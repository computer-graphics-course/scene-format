using System;
using System.IO;
using System.Text;
using System.Text.Json;
using Google.Protobuf;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using JsonSerializer = System.Text.Json.JsonSerializer;

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
            Scene scene;
            
            try
            {
                scene = Scene.Parser.ParseFrom(content);
            }
            catch (InvalidProtocolBufferException e)
            {
                var contentAsString = Encoding.UTF8.GetString(content).Replace("\uFEFF", "");
                try
                {
                    scene = Scene.Parser.ParseJson(contentAsString);
                }
                catch (InvalidJsonException ex)
                {
                    try
                    {
                        JToken.Parse(contentAsString);
                        throw new SceneIOException("Protobuf failed to parse what seems to be a valid json. Please report this problem to SceneFormat team.");
                    }
                    catch (JsonReaderException jex)
                    {
                        throw new SceneIOException("Failed to parse json: " + jex.Message);
                    }
                }
            }

            return ScenePostProcessing.PostProcessAndValidate(scene);
        }

        public Scene Read(string inputPath)
        {
            return PostProcessSceneAfterReadFromFile(Read(File.Open(inputPath, FileMode.Open)), inputPath);
        }

        public void Save(Scene scene, Stream output)
        {
            ScenePostProcessing.PostProcessAndValidate(scene).WriteTo(output);
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
            var jsonData = _jsonFormatter.Format(ScenePostProcessing.PostProcessAndValidate(scene));
         
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
            if (scene != null && scene.SceneObjects != null)
            {
                foreach (var sceneObject in scene.SceneObjects)
                {
                    if (sceneObject.MeshedObject != null)
                    {
                        var meshedObject = sceneObject.MeshedObject;
                        if (meshedObject.Reference != null && !Path.IsPathRooted(meshedObject.Reference))
                        {
                            meshedObject.Reference =
                                Path.Combine(Path.GetDirectoryName(filePath), meshedObject.Reference);
                        }
                    }
                }
            }

            return scene;
        }
    }
}