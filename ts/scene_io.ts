import { promises } from 'fs';

import { Material, Scene } from './proto/scene';

const readFileAsync = promises.readFile;
const writeFileAsync = promises.writeFile;

export type SceneIO = {

    decode: (data: string | Buffer) => Scene,
    read: (path: string) => Promise<Scene>,

    encode: (scene: Scene) => Uint8Array,
    save: (scene: Scene, path: string) => Promise<void>,
    encodeAsJson: (scene: Scene) => string,
    saveAsJson: (scene: Scene, path: string) => Promise<void>,
};

const preprocessScene = (scene: Scene) => {
    // workaround of what seems to be ts-proto bug.
    scene.sceneObjects
        .filter(obj => obj.material != undefined)
        .forEach(obj => (obj.material as Material).id = '');
    
    return scene;
};

const decode = (data: Buffer | string) => {
    try {
        return Scene.fromJSON(JSON.parse(data.toString()));
    } catch (e) {
        if (Buffer.isBuffer(data)) {
            return Scene.decode(data);
        } else {
            throw Error('Failed to decode as scene format: input is string, but not a valid json: ' + e);
        }
    }
};

const encode = (scene: Scene) => Scene.encode(preprocessScene(scene)).finish();
const encodeAsJson = (scene: Scene) => JSON.stringify(Scene.toJSON(scene), null, 4);

export const SceneFormatIO : SceneIO = {

    decode,
    read: async (path: string) => decode(await readFileAsync(path)),
    encode,
    save: async (scene: Scene, path: string) => await writeFileAsync(path, Buffer.from(encode(scene))),
    encodeAsJson,
    saveAsJson: async (scene: Scene, path: string) => await writeFileAsync(path, encodeAsJson(scene)),
};
