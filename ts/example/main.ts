import { Scene, Camera, Vector3, SceneFormatIO } from '@computer-graphics-course/scene-format';

(async () => {
    const scene: Scene = {
        version: 42,
        cameras: [<Camera>{
            id: 0,
            position: <Vector3> {
                x: 1.01,
                y: 2.76,
                z: 3,
            }
        }]
    };
    
    await SceneFormatIO.save(scene, 'example_binary.cowscene');
    await SceneFormatIO.saveAsJson(scene, 'example_json.cowscene');
    
    const readResultBinary = await SceneFormatIO.read('example_binary.cowscene');
    console.log(`Camera X is ${readResultBinary.cameras[0]?.position?.x} when reading binary`);

    const readResultJson = await SceneFormatIO.read('example_json.cowscene');
    console.log(`Camera X is ${readResultJson.cameras[0]?.position?.x} when reading json`);
})();