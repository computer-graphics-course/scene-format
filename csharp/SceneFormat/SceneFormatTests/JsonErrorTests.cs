using System.IO;
using NUnit.Framework;
using SceneFormat;

namespace SceneFormatTests
{
    public class JsonErrorTests
    {
        private static ISceneIO _sceneIo = new SceneIO();
        
        [Test]
        public void Test1()
        {
            try
            {
                _sceneIo.Read("test_files/simple_trailing_comma.json");
                Assert.Fail();
            }
            catch (SceneIOException e)
            {
                Assert.AreEqual("Failed to parse json: After parsing a value an unexpected character was encountered: o. Path  'sceneObjects', line 3, position 21.", e.Message);
            }
        }
    }
}