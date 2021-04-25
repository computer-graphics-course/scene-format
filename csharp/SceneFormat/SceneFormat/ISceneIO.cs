using System;
using System.IO;
using Com.Github.ComputerGraphicsCourse;

namespace SceneFormat
{
    public interface ISceneIO
    {

        Scene Read(Stream input);
        Scene Read(String inputPath);
        
        void Save(Scene scene, Stream output);
        void Save(Scene scene, String outputPath);
        
        void SaveAsJson(Scene scene, Stream output);
        void SaveAsJson(Scene scene, String outputPath);
    }
}