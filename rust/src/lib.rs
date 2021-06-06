#[macro_use] extern crate log;
extern crate custom_error;

pub mod errors;
pub mod obj;

use std::{env, fs::File, path::Path};
use std::io::Write;

use prost::Message;
use serde_json::{Map, Value};

use errors::SceneIOError;
use obj::read_obj_file;

include!(concat!(env!("OUT_DIR"), "/scene_format.rs"));

pub fn encode(scene: &Scene) -> Result<Vec<u8>, SceneIOError> {
    let mut buf = Vec::with_capacity(scene.encoded_len());
    scene.encode(&mut buf).map_err(|err| SceneIOError::FailedToEncode {
        description: err.to_string(),
    })?;
    Ok(buf)
}

pub fn encode_json(scene: &Scene) -> Result<Vec<u8>, SceneIOError> {
    serde_json::to_vec_pretty(&scene).map_err(|err| SceneIOError::FailedToEncode {
        description: err.to_string(),
    })
}

pub fn save(scene: &Scene, save_to: &str) -> Result<(), SceneIOError> {
    let encoded = encode(scene)?;
    let mut file = File::create(save_to)?;
    file.write(&encoded)?;
    Ok(())
}

pub fn save_json(scene: &Scene, save_to: &str) -> Result<(), SceneIOError> {
    let encoded = encode_json(scene)?;
    let mut file = File::create(save_to)?;
    file.write(&encoded)?;
    Ok(())
}

pub fn decode(data: &[u8]) -> Result<Scene, SceneIOError> {
    decode_with_context(data, None)
}

pub fn decode_with_context(data: &[u8], context: Option<&Path>) -> Result<Scene, SceneIOError> {
    let value: Value = match serde_json::from_slice(data) {
        Ok(v) => v,
        Err(err) => {
            debug!("Failed to decode as json, trying binary: {:?}", err);
            return Scene::decode(&*data.to_vec())
                .map_err(|_| SceneIOError::FailedToDecode { description: err.to_string() })
                .and_then(|scene| post_process_scene(&scene, context))
        }
    };

    let mut scene: Map<String, Value> = match value {
        Value::Object(obj) => obj,
        _ => return Err(SceneIOError::FailedToDecode {
            description: "Expected top level structure to be object".to_string(),
        }),
    };

    if let Some(render_options) = scene.get("renderOptions") {
        let render_options = match &render_options {
            Value::Object(obj) => obj,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected render options to be an object".to_string(),
            })
        };

        scene.insert("renderOptions".to_string(), Value::Object(pre_process_render_options(render_options)?));
    }

    if let Some(cameras) = scene.get("cameras") {
        let cameras = match &cameras {
            Value::Array(arr) => arr,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected cameras to be an array".to_string(),
            })
        };

        scene.insert("cameras".to_string(), Value::Array(pre_process_cameras(&cameras)?));
    }

    if let Some(scene_objects) = scene.get("sceneObjects") {
        let scene_objects = match scene_objects {
            Value::Array(arr) => arr,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected sceneObjects to be an array".to_string(),
            })
        };

        scene.insert("sceneObjects".to_string(), Value::Array(pre_process_scene_objects(&scene_objects)?));
    }

    if let Some(lights) = scene.get("lights") {
        let lights = match &lights {
            Value::Array(arr) => arr,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected lights to be an array".to_string(),
            })
        };

        scene.insert("lights".to_string(), Value::Array(pre_process_lights(&lights)?));
    } else {
        scene.insert("lights".to_string(), Value::Array(Vec::new()));
    }

    if !scene.contains_key("materials") {
        scene.insert("materials".to_string(), Value::Array(Vec::new()));
    }

    let scene: Scene = serde_json::from_str(&match serde_json::to_string(&scene) {
        Ok(v) => v,
        Err(err) => return Err(SceneIOError::FailedToEncode {
            description: err.to_string(),
        }),
    }).expect("expected json to be valid");

    post_process_scene(&scene, context)
}

fn pre_process_render_options(render_options: &Map<String, Value>) -> Result<Map<String, Value>, SceneIOError> {
    let mut render_options = render_options.clone();

    if !render_options.contains_key("customProperties") {
        render_options.insert("customProperties".to_string(), Value::Array(Vec::new()));
    }

    Ok(render_options)
}

fn pre_process_cameras(cameras: &Vec<Value>) -> Result<Vec<Value>, SceneIOError> {
    let mut new_cameras = Vec::new();

    for camera in cameras {
        new_cameras.push(Value::Object(match &camera {
            Value::Object(camera) => pre_process_camera(camera)?,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected camera to be an object".to_string(),
            })
        }));
    }

    Ok(new_cameras)
}

fn pre_process_camera(camera: &Map<String, Value>) -> Result<Map<String, Value>, SceneIOError> {
    let mut camera = camera.clone();

    if let Some(transform) = camera.get("transform") {
        camera["transform"] = Value::Object(match &transform {
            Value::Object(transform) => pre_process_transform(transform)?,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected transform to be an object".to_string(),
            })
        })
    }

    Ok(camera)
}


fn pre_process_scene_objects(scene_objects: &Vec<Value>) -> Result<Vec<Value>, SceneIOError> {
    let mut objects = Vec::new();

    for object in scene_objects {
        objects.push(Value::Object(match object {
            Value::Object(scene_object) => pre_process_scene_object(scene_object)?,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected scene object to be an object".to_string(),
            })
        }))
    }

    Ok(objects)
}

fn pre_process_scene_object(scene_object: &Map<String, Value>) -> Result<Map<String, Value>, SceneIOError> {
    let mut scene_object = scene_object.clone();

    if let Some(transform) = scene_object.get("transform") {
        scene_object["transform"] = Value::Object(match &transform {
            Value::Object(transform) => pre_process_transform(transform)?,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected transform to be an object".to_string(),
            })   
        })
    }

    if let Some(sphere) = scene_object.get("sphere") {
        let sphere = match sphere {
            Value::Object(v) => v.clone(),
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected sphere to be an object".to_string(),
            })
        };

        scene_object.insert("mesh".to_string(), Value::Object({
            let mut map = Map::new();
            map.insert("sphere".to_string(), Value::Object(sphere));
            map
        }));
    }

    if let Some(cube) = scene_object.get("cube") {
        let cube = match cube {
            Value::Object(v) => v.clone(),
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected cube to be an object".to_string(),
            })
        };

        scene_object.insert("mesh".to_string(), Value::Object({
            let mut map = Map::new();
            map.insert("cube".to_string(), Value::Object(cube));
            map
        }));
    }

    if let Some(plane) = scene_object.get("plane") {
        let plane = match plane {
            Value::Object(v) => v.clone(),
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected plane to be an object".to_string(),
            })
        };

        scene_object.insert("mesh".to_string(), Value::Object({
            let mut map = Map::new();
            map.insert("plane".to_string(), Value::Object(plane));
            map
        }));
    }

    if let Some(disk) = scene_object.get("disk") {
        let disk = match disk {
            Value::Object(v) => v.clone(),
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected disk to be an object".to_string(),
            })
        };

        scene_object.insert("mesh".to_string(), Value::Object({
            let mut map = Map::new();
            map.insert("disk".to_string(), Value::Object(disk));
            map
        }));
    }

    if let Some(meshed_object) = scene_object.get("meshed_object") {
        let meshed_object = match meshed_object {
            Value::Object(v) => v.clone(),
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected meshed object to be an object".to_string(),
            })
        };

        scene_object.insert("mesh".to_string(), Value::Object({
            let mut map = Map::new();
            map.insert("meshedObject".to_string(), Value::Object(meshed_object));
            map
        }));
    }

    Ok(scene_object)
}

fn pre_process_lights(lights: &Vec<Value>) -> Result<Vec<Value>, SceneIOError> {
    let mut new_lights = Vec::new();

    for light in lights {
        new_lights.push(Value::Object(match light {
            Value::Object(light) => pre_process_light(light)?,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected light to be an object".to_string(),
            })
        }))
    }

    Ok(new_lights)
}

fn pre_process_light(light: &Map<String, Value>) -> Result<Map<String, Value>, SceneIOError> {
    let mut light = light.clone();

    if let Some(transform) = light.get("transform") {
        light["transform"] = Value::Object(match &transform {
            Value::Object(transform) => pre_process_transform(transform)?,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected transform to be an object".to_string(),
            })
        })
    }

    if let Some(point) = light.get("point") {
        let point_light = match point {
            Value::Object(v) => v.clone(),
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected point light to be an object".to_string(),
            })
        };

        light.insert("light".to_string(), Value::Object({
            let mut map = Map::new();
            map.insert("point".to_string(), Value::Object(point_light));
            map
        }));
    }

    if let Some(directional) = light.get("directional") {
        let directional_light = match directional {
            Value::Object(v) => v.clone(),
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected directional light to be an object".to_string(),
            })
        };

        light.insert("light".to_string(), Value::Object({
            let mut map = Map::new();
            map.insert("directional".to_string(), Value::Object(directional_light));
            map
        }));
    }

    if let Some(environment) = light.get("environment") {
        let environment_light = match environment {
            Value::Object(v) => v.clone(),
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected environment light to be an object".to_string(),
            })
        };

        light.insert("environment".to_string(), Value::Object({
            let mut map = Map::new();
            map.insert("environment".to_string(), Value::Object(environment_light));
            map
        }));
    }

    Ok(light)
}

fn pre_process_transform(transform: &Map<String, Value>) -> Result<Map<String, Value>, SceneIOError> {
    let mut transform = transform.clone();

    if !transform.contains_key("parentId") {
        transform.insert("parentId".to_string(), Value::Number(0.into()));
    }

    Ok(transform)
}

fn post_process_scene(scene: &Scene, context: Option<&Path>) -> Result<Scene, SceneIOError> {
    let mut scene = scene.clone();

    for i in 0..scene.scene_objects.len() {
        scene.scene_objects[i] = post_process_scene_object(&scene.scene_objects[i], context)?;
    }

    Ok(scene)
}

fn post_process_scene_object(scene_object: &SceneObject, context: Option<&Path>) -> Result<SceneObject, SceneIOError> {
    let mut scene_object = scene_object.clone();
    let mesh = match &scene_object.mesh {
        Some(v) => v.clone(),
        None => return Err(SceneIOError::FailedToDecode {
            description: "Expected scene object to contain mesh".to_string(),
        })
    };

    if let scene_object::Mesh::MeshedObject(meshed_object) = &mesh {
        let mut meshed_object = meshed_object.clone();

        if let Some(context) = context {
            if meshed_object.reference != "" {
                meshed_object.reference = context.join(meshed_object.reference).to_str().ok_or(SceneIOError::FailedToDecode {
                    description: "Failed to join reference path with context".to_string(),
                })?.to_string();

                meshed_object.obj = Some(read_obj_file(&meshed_object.reference)?);
            }
        }

        scene_object.mesh = Some(scene_object::Mesh::MeshedObject(meshed_object));
    }

    Ok(scene_object)
}

pub fn read(read_from: &str) -> Result<Scene, SceneIOError> {
    let file_path = Path::new(read_from);
    let parent_directory_path = file_path.parent();

    let data = std::fs::read(file_path)?;
    decode_with_context(&data, parent_directory_path)
}

#[cfg(test)]
mod tests {

    use super::*;
    use env_logger::Env;

    const DELTA: f64 = 0.00001;

    #[ctor::ctor]
    fn init() {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    }

    #[test]
    fn example() {
        let scene = Scene {
            version: 1,
            render_options: Some(RenderOptions {
                camera_id: 0,
                width: 1000,
                height: 1000,
                custom_properties: Vec::new(),
            }),
            scene_objects: vec![
                SceneObject {
                    id: 0,
                    transform: Some(Transform {
                        parent_id: 0,
                        position: Some(Vector3 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        }),
                        rotation: None,
                        scale: None,
                    }),
                    object_material: Some(scene_object::ObjectMaterial::Material(Material {
                        id: "".to_string(),
                        material: Some(material::Material::LambertReflection(LambertReflectionMaterial {
                            color: Some(Color {
                                r: 1.0,
                                g: 1.0,
                                b: 1.0,
                            })
                        })),
                    })),
                    mesh: Some(scene_object::Mesh::MeshedObject(MeshedObject {
                        reference: "examples/assets/cow.obj".to_string(),
                        obj: None,
                    })),
                },
            ],
            cameras: vec![
                Camera {
                    id: 0,
                    transform: Some(Transform {
                        parent_id: 0,
                        position: Some(Vector3 {
                            x: 1.01,
                            y: 2.76,
                            z: 3.0,
                        }),
                        rotation: None,
                        scale: None,
                    }),
                    camera: Some(camera::Camera::Perspective(PerspectiveCamera {
                        fov: 60.0,
                    }))
                }
            ],
            lights: vec![],
            materials: vec![],
        };

        save(&scene, "example_binary.cowscene").unwrap();
        save_json(&scene, "example_json.cowscene").unwrap();

        let read_result_binary = read("example_binary.cowscene").unwrap();
        println!("Camera X is {} when reading binary", read_result_binary.cameras[0].transform.as_ref().unwrap().position.as_ref().unwrap().x);

        let read_result_json = read("example_json.cowscene").unwrap();
        println!("Camera X is {} when reading json", read_result_json.cameras[0].transform.as_ref().unwrap().position.as_ref().unwrap().x);
    }

    #[test]
    fn example_from_docs1() {
        read("./examples/1.cowscene").unwrap();
    }

    #[test]
    fn example_from_docs2() {
        read("./examples/2.cowscene").unwrap();
    }

    #[test]
    fn example_from_docs3() {
        read("./examples/3.cowscene").unwrap();
    }

    #[test]
    fn example_from_docs4() {
        read("./examples/4.cowscene").unwrap();
    }

    #[test]
    fn example_from_docs5() {
        let result = read("./examples/5.cowscene").unwrap();

        if let Some(meshed_object) = result.scene_objects.get(0).unwrap().mesh.as_ref() {
            if let scene_object::Mesh::MeshedObject(meshed_object) = meshed_object {
                assert_eq!("./examples/assets/cow.obj", meshed_object.reference);

                let obj = meshed_object.obj.as_ref().unwrap();

                assert_eq!(2574, obj.vertices.len());

                assert!((0.14922 - obj.vertices[0].x).abs() < DELTA);
                assert!((0.0940258 - obj.vertices[0].y).abs() < DELTA);
                assert!((-0.0463043 - obj.vertices[0].z).abs() < DELTA);
                assert!((1.0 - obj.vertices[0].w).abs() < DELTA);

                assert_eq!(2574, obj.vertex_normals.len());
                assert!((0.372948 - obj.vertex_normals[0].x).abs() < DELTA);
                assert!((0.780945 - obj.vertex_normals[0].y).abs() < DELTA);
                assert!((-0.501034 - obj.vertex_normals[0].z).abs() < DELTA);

                assert_eq!(5144, obj.faces.len());
                assert_eq!(3, obj.faces[0].elements.len());
                assert_eq!(5, obj.faces[0].elements[0].vertex_index);
                assert_eq!(1, obj.faces[0].elements[0].normal_index);
            } else {
                panic!("Expected scene object to be meshed object");
            }
        } else {
            panic!("Expected meshed object to be present");
        }
    }

    #[test]
    fn example_from_docs6() {
        read("./examples/6.cowscene").unwrap();
    }

    #[test]
    fn example_from_docs7() {
        read("./examples/7.cowscene").unwrap();
    }

    #[test]
    fn example_from_docs8() {
        read("./examples/8.cowscene").unwrap();
    }

    #[test]
    fn example_from_docs9() {
        read("./examples/9.cowscene").unwrap();
    }

    #[test]
    fn example_from_docs10() {
        read("./examples/10.cowscene").unwrap();
    }

    #[test]
    fn example_from_docs11() {
        read("./examples/11.cowscene").unwrap();
    }

    #[test]
    fn example_from_docs12() {
        read("./examples/12.cowscene").unwrap();
    }

    #[test]
    fn example_from_docs13() {
        let scene = read("./examples/13.cowscene").unwrap();

        if let Some(light) = scene.lights.get(0).unwrap().light.as_ref() {
            match light {
                light::Light::Directional(_) => {
                    // ok
                },
                other => panic!("Expected light to be directional, instead got: {:?}", other),
            };
        } else {
            panic!("Expected light to be present");
        }
    }
}
