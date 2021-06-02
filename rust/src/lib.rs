#[macro_use] extern crate log;
extern crate custom_error;

use std::fs::File;
use std::io::Write;

use custom_error::custom_error;
use prost::Message;

include!(concat!(env!("OUT_DIR"), "/scene_format.rs"));

custom_error!{pub SceneIOError
    FailedToEncode = "Failed to encode",
    FailedToDecode{description: String} = "Failed to decode",
    IOError {source: std::io::Error} = "IO Error: {source}",
}

pub fn encode(scene: &Scene) -> Result<Vec<u8>, SceneIOError> {
    let mut buf = Vec::with_capacity(scene.encoded_len());
    scene.encode(&mut buf).map_err(|_| SceneIOError::FailedToEncode)?;
    Ok(buf)
}

pub fn encode_json(scene: &Scene) -> Result<Vec<u8>, SceneIOError> {
    serde_json::to_vec_pretty(&scene).map_err(|_| SceneIOError::FailedToEncode)
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
    return match serde_json::from_slice(data) {
        Ok(v) => Ok(v),
        Err(err) => {
            debug!("Failed to decode as json, trying binary: {:?}", err);
            Scene::decode(&*data.to_vec()).map_err(|_| SceneIOError::FailedToDecode { description: err.to_string() })
        }
    }
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
}