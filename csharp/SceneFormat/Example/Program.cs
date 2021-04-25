using System;
using System.IO;
using System.Text;
using System.Text.Json;
using Google.Protobuf;
using SceneFormat;

namespace Example
{
    class Example
    {
        private static readonly JsonFormatter _jsonFormatter = new JsonFormatter(JsonFormatter.Settings.Default);
        private static readonly ISceneIO _sceneIO = new SceneIO();
        
        static void Main(string[] args)
        {
            var scene = new Scene
            {
                Version = 42,
                Cameras =
                {
                    new Scene.Types.Camera()
                    {
                        Id = 0,
                        Position = new Scene.Types.Vector3 {
                            X = 1.01,
                            Y = 2.76,
                            Z = 3,
                        },
                    }
                }
            };
            
            _sceneIO.Save(scene, "example_binary.cowscene");
            _sceneIO.SaveAsJson(scene, "example_json.cowscene");

            var readResultBinary = _sceneIO.Read("example_binary.cowscene");
            Console.WriteLine($"Camera X is {readResultBinary.Cameras[0].Position.X} when reading binary");

            var readResultJson = _sceneIO.Read("example_json.cowscene");
            Console.WriteLine($"Camera X is {readResultJson.Cameras[0].Position.X} when reading json");
        }
    }
}