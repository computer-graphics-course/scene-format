#[macro_use] extern crate log;
extern crate custom_error;

use std::fs::File;
use std::io::Write;

use custom_error::custom_error;
use prost::Message;
use serde_json::{Map, Number, Value};

include!(concat!(env!("OUT_DIR"), "/scene_format.rs"));

custom_error!{pub SceneIOError
    FailedToEncode{description: String} = "Failed to encode",
    FailedToDecode{description: String} = "Failed to decode",
    IOError {source: std::io::Error} = "IO Error: {source}",
}

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
    let value: Value = match serde_json::from_slice(data) {
        Ok(v) => v,
        Err(err) => {
            debug!("Failed to decode as json, trying binary: {:?}", err);
            return Scene::decode(&*data.to_vec()).map_err(|_| SceneIOError::FailedToDecode { description: err.to_string() })
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

        scene.insert("renderOptions".to_string(), Value::Object(post_process_render_options(render_options)?));
    }

    if let Some(cameras) = scene.get("cameras") {
        let cameras = match &cameras {
            Value::Array(arr) => arr,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected cameras to be an array".to_string(),
            })
        };

        scene.insert("cameras".to_string(), Value::Array(post_process_cameras(&cameras)?));
    }

    if let Some(scene_objects) = scene.get("sceneObjects") {
        let scene_objects = match scene_objects {
            Value::Array(arr) => arr,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected sceneObjects to be an array".to_string(),
            })
        };

        scene.insert("sceneObjects".to_string(), Value::Array(post_process_scene_objects(&scene_objects)?));
    }

    if let Some(lights) = scene.get("lights") {
        let lights = match &lights {
            Value::Array(arr) => arr,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected lights to be an array".to_string(),
            })
        };

        scene.insert("lights".to_string(), Value::Array(post_process_lights(&lights)?));
    } else {
        scene.insert("lights".to_string(), Value::Array(Vec::new()));
    }

    if !scene.contains_key("materials") {
        scene.insert("materials".to_string(), Value::Array(Vec::new()));
    }

    Ok(serde_json::from_str(&match serde_json::to_string(&scene) {
        Ok(v) => v,
        Err(err) => return Err(SceneIOError::FailedToEncode {
            description: err.to_string(),
        }),
    }).expect("expected json to be valid"))
}

fn post_process_render_options(render_options: &Map<String, Value>) -> Result<Map<String, Value>, SceneIOError> {
    let mut render_options = render_options.clone();

    if !render_options.contains_key("customProperties") {
        render_options.insert("customProperties".to_string(), Value::Array(Vec::new()));
    }

    Ok(render_options)
}

fn post_process_cameras(cameras: &Vec<Value>) -> Result<Vec<Value>, SceneIOError> {
    let mut new_cameras = Vec::new();

    for camera in cameras {
        new_cameras.push(Value::Object(match &camera {
            Value::Object(camera) => post_process_camera(camera)?,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected camera to be an object".to_string(),
            })
        }));
    }

    Ok(new_cameras)
}

fn post_process_camera(camera: &Map<String, Value>) -> Result<Map<String, Value>, SceneIOError> {
    let mut camera = camera.clone();

    if let Some(transform) = camera.get("transform") {
        camera["transform"] = Value::Object(match &transform {
            Value::Object(transform) => post_process_transform(transform)?,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected transform to be an object".to_string(),
            })
        })
    }

    Ok(camera)
}


fn post_process_scene_objects(scene_objects: &Vec<Value>) -> Result<Vec<Value>, SceneIOError> {
    let mut objects = Vec::new();

    for object in scene_objects {
        objects.push(Value::Object(match object {
            Value::Object(scene_object) => post_process_scene_object(scene_object)?,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected scene object to be an object".to_string(),
            })
        }))
    }

    Ok(objects)
}

fn post_process_scene_object(scene_object: &Map<String, Value>) -> Result<Map<String, Value>, SceneIOError> {
    let mut scene_object = scene_object.clone();

    if let Some(transform) = scene_object.get("transform") {
        scene_object["transform"] = Value::Object(match &transform {
            Value::Object(transform) => post_process_transform(transform)?,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected transform to be an object".to_string(),
            })   
        })
    }

    Ok(scene_object)
}

fn post_process_lights(lights: &Vec<Value>) -> Result<Vec<Value>, SceneIOError> {
    let mut new_lights = Vec::new();

    for light in lights {
        new_lights.push(Value::Object(match light {
            Value::Object(light) => post_process_light(light)?,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected light to be an object".to_string(),
            })
        }))
    }

    Ok(new_lights)
}

fn post_process_light(light: &Map<String, Value>) -> Result<Map<String, Value>, SceneIOError> {
    let mut light = light.clone();

    if let Some(transform) = light.get("transform") {
        light["transform"] = Value::Object(match &transform {
            Value::Object(transform) => post_process_transform(transform)?,
            _ => return Err(SceneIOError::FailedToDecode {
                description: "Expected transform to be an object".to_string(),
            })
        })
    }

    Ok(light)
}

fn post_process_transform(transform: &Map<String, Value>) -> Result<Map<String, Value>, SceneIOError> {
    let mut transform = transform.clone();

    if !transform.contains_key("parentId") {
        transform.insert("parentId".to_string(), Value::Number(0.into()));
    }

    Ok(transform)
}

pub fn read(read_from: &str) -> Result<Scene, SceneIOError> {
    let data = std::fs::read(read_from)?;
    decode(&data)
}

#[cfg(test)]
mod tests {

    use super::*;
    use env_logger::Env;

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
                        reference: "cow.obj".to_string(),
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
        read("./examples/5.cowscene").unwrap();
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
        read("./examples/13.cowscene").unwrap();
    }
}