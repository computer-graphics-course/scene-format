using System;
using SceneFormat;

namespace Example
{
    static class Example
    {
        private static readonly ISceneIO _sceneIO = new SceneIO();
        
        static void Main()
        {
            var scene = new Scene
            {
                Version = 1,
                SceneObjects =
                {
                    new SceneObject
                    {
                        Id = 0,
                        Transform = new Transform
                        {
                            Position = new Vector3 { X = 1.0, Y = 1.0, Z = 1.0 }
                        },
                        Material = new Material
                        {
                            LambertReflection = new LambertReflectionMaterial {},
                        },
                        MeshedObject = new MeshedObject
                        {
                            Reference = "cow.obj"
                        }
                    }
                },
                Cameras =
                {
                    new Camera
                    {
                        Id = 0,
                        Transform = new Transform {
                            Position = new Vector3 { X = 1.01, Y = 2.76, Z = 3, }
                        },
                        Perspective = new PerspectiveCamera {
                            Fov = 60,
                        },
                    }
                }
            };
            
            _sceneIO.Save(scene, "example_binary.cowscene");
            _sceneIO.SaveAsJson(scene, "example_json.cowscene");

            var readResultBinary = _sceneIO.Read("example_binary.cowscene");
            Console.WriteLine($"Camera X is {readResultBinary.Cameras[0].Transform.Position.X} when reading binary");

            var readResultJson = _sceneIO.Read("example_json.cowscene");
            Console.WriteLine($"Camera X is {readResultJson.Cameras[0].Transform.Position.X} when reading json");
        }
    }
}