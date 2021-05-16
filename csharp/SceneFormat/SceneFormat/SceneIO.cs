using System;
using System.IO;
using System.Text;
using System.Text.Json;
using Google.Protobuf;

namespace SceneFormat
{
    public class SceneIO : ISceneIO
    {
        private readonly JsonFormatter _jsonFormatter = JsonFormatter.Default;
        private readonly bool _prettifyJson;
        
        public SceneIO(bool prettifyJson)
        {
            _prettifyJson = prettifyJson;
        }

        public SceneIO() : this(true)
        {
        }

        public Scene Read(Stream input)
        {
            var content = ReadToEnd(input);
            try
            {
                return Scene.Parser.ParseJson(Encoding.UTF8.GetString(content));
            }
            catch (InvalidJsonException e)
            {
                return Scene.Parser.ParseFrom(content);
            }
        }

        public Scene Read(string inputPath)
        {
            return PostProcessSceneAfterReadFromFile(Read(File.Open(inputPath, FileMode.Open)), inputPath);
        }

        public void Save(Scene scene, Stream output)
        {
            scene.WriteTo(output);
        }

        public void Save(Scene scene, string outputPath)
        {
            var outputStream = OpenOutputFile(outputPath);
            Save(scene, outputStream);
            outputStream.Flush();
            outputStream.Close();
        }

        public void SaveAsJson(Scene scene, Stream output)
        {
            var jsonData = _jsonFormatter.Format(scene);
         
            if (this._prettifyJson)
            {
                jsonData = PrettifyJson(jsonData);
            }
            
            output.Write(Encoding.UTF8.GetBytes(jsonData));
        }

        public void SaveAsJson(Scene scene, string outputPath)
        {
            var outputStream = OpenOutputFile(outputPath);
            SaveAsJson(scene, outputStream);
            outputStream.Flush();
            outputStream.Close();
        }

        private Stream OpenOutputFile(String path)
        {
            return File.Open(path, FileMode.OpenOrCreate);
        }

        private byte[] ReadToEnd(Stream stream)
        {
            using var memoryStream = new MemoryStream();
            stream.CopyTo(memoryStream);
            return memoryStream.ToArray();
        }
        
        private string PrettifyJson(string json)
        {
            var options = new JsonSerializerOptions {
                WriteIndented = true
            };

            var jsonElement = JsonSerializer.Deserialize<JsonElement>(json);

            return JsonSerializer.Serialize(jsonElement, options);
        }

        private Scene PostProcessSceneAfterReadFromFile(Scene scene, string filePath)
        {
            foreach (var sceneObject in scene.SceneObjects)
            {
                if (sceneObject.MeshedObject != null)
                {
                    var meshedObject = sceneObject.MeshedObject;
                    if (meshedObject.Reference != null && !Path.IsPathRooted(meshedObject.Reference))
                    {
                        meshedObject.Reference = Path.Combine(Path.GetDirectoryName(filePath), meshedObject.Reference);
                    }
                }
            }

            return scene;
        }
    }
}