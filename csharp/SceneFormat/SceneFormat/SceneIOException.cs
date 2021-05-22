using System;

namespace SceneFormat
{
    public class SceneIOException : Exception
    {
        
        public SceneIOException()
        {
        }

        public SceneIOException(string message) : base(message)
        {
        }

        public SceneIOException(string message, Exception inner) : base(message, inner)
        {
        }
    }
}