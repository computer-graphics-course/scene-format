import { Scene, Camera, Vector3, Transform, SceneFormatIO, SceneObject, Material, SolidMaterial, MeshedObject, PerspectiveCamera } from '@computer-graphics-course/scene-format';

(async () => {
    const scene: Scene = {
        version: 1,
        renderOptions: {
            cameraId: 0,
            width: 1920,
            height: 1280,
        },
        sceneObjects: [<SceneObject> {
            id: 0,
            transform: <Transform> {
                position: <Vector3> { x: 1, y: 1, z: 1},
            },
            material: <Material> {
                solid: <SolidMaterial>{},
            },
            meshedObject: <MeshedObject> {
                reference: 'cow.obj',
            }
        }],
        lights: [],
        cameras: [<Camera>{
            id: 0,
            transform: <Transform> {
                position: <Vector3> {
                    x: 1.01,
                    y: 2.76,
                    z: 3,
                }
            },
            perspective: <PerspectiveCamera> {
                fov: 60,
            }
        }],
        materials: [],
    };
    
    await SceneFormatIO.save(scene, 'example_binary.cowscene');
    await SceneFormatIO.saveAsJson(scene, 'example_json.cowscene');
    
    const readResultBinary = await SceneFormatIO.read('example_binary.cowscene');
    console.log(`Camera X is ${readResultBinary.cameras[0]?.transform?.position?.x} when reading binary`);

    const readResultJson = await SceneFormatIO.read('example_json.cowscene');
    console.log(`Camera X is ${readResultJson.cameras[0]?.transform?.position?.x} when reading json`);
})();