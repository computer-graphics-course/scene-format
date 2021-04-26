import { readFile, writeFile } from 'fs';

import { Scene } from './proto/scene';

export type SceneIO = {

    decode: (data: string | Buffer) => Scene,
    read: (path: string) => Promise<Scene>,

    encode: (scene: Scene) => Uint8Array,
    save: (scene: Scene, path: string) => Promise<void>,
    encodeAsJson: (scene: Scene) => string,
    saveAsJson: (scene: Scene, path: string) => Promise<void>,
};

const decode = (data: Buffer | string) => {
    try {
        return Scene.fromJSON(JSON.parse(data.toString()));
    } catch (e) {
        if (Buffer.isBuffer(data)) {
            return Scene.decode(data);
        } else {
            throw Error('Failed to decode as scene format: input is string, but not a valid json');
        }
    }
};

const encode = (scene: Scene) => Scene.encode(scene).finish();
const encodeAsJson = (scene: Scene) => JSON.stringify(Scene.toJSON(scene), null, 4);

export const SceneFormatIO : SceneIO = {

    decode,
    read: async (path: string) => new Promise((resolve, reject) => {
        readFile(path, (err, data) => {
            if (err) reject(err);
            resolve(decode(data));
        });
    }),

    encode,
    save: async (scene: Scene, path: string) => new Promise((resolve, reject) => {
        writeFile(path, Buffer.from(encode(scene)), err => {
            if (err) reject(err);
            resolve();
        });
    }),
    encodeAsJson,
    saveAsJson: async (scene: Scene, path: string) => new Promise((resolve, reject) => {
        writeFile(path, encodeAsJson(scene), err => {
            if (err) reject(err);
            resolve();
        });
    }),
};