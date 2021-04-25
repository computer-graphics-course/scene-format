using System;
using System.IO;
using System.Text;
using Com.Github.ComputerGraphicsCourse;
using Google.Protobuf;

namespace SceneFormat
{
    public class SceneIO : ISceneIO
    {
        private readonly JsonFormatter _jsonFormatter = JsonFormatter.Default;
        
        public Scene Read(Stream input)
        {
            var content = ReadToEnd(input);

            try
            {
                return Scene.Parser.ParseJson(Encoding.UTF8.GetString(content));
            }
            catch (InvalidProtocolBufferException e)
            {
                return Scene.Parser.ParseFrom(content);
            }
        }

        public Scene Read(string inputPath)
        {
            throw new NotImplementedException();
        }

        public void Save(Scene scene, Stream output)
        {
            scene.WriteTo(output);
        }

        public void Save(Scene scene, string outputPath)
        {
            Save(scene, OpenOutputFile(outputPath));
        }

        public void SaveAsJson(Scene scene, Stream output)
        {
            output.Write(Encoding.UTF8.GetBytes(_jsonFormatter.Format(scene)));
        }

        public void SaveAsJson(Scene scene, string outputPath)
        {
            SaveAsJson(scene, OpenOutputFile(outputPath));
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
    }
}