using System;
using System.IO;
using System.Reflection;
using System.Runtime.CompilerServices;
using xmltests;

namespace UnitTests
{
    public sealed class Paths
    {
        public DataPaths DataPaths { get; }
        private Paths()
        {
            var path = Directory.GetParent(myPath()).FullName;
            if(path == null) { 
                throw new NullReferenceException("Directory.GetParent(myPath()).FullName;");
            }

            path = Path.Combine(path, "..", "..", "data");
            var x = Path.Combine(path, "xmlconf");
            var s = Path.Combine(path, "xmlschema");
            var xDir = new DirectoryInfo(x);
            var sDir = new DirectoryInfo(s);
            this.DataPaths = new DataPaths(xDir, sDir);
        }

        private string myPath([CallerFilePath] string sourceFilePath = "")
        {
            return sourceFilePath;
        }

        public static Paths Instance { get { return Nested.InnerInstance; } }

        private class Nested
        {
            // Explicit static constructor to tell C# compiler
            // not to mark type as beforefieldinit
            static Nested()
            {
            }

            internal static Paths InnerInstance { get; } = new Paths();
        }
    }
}