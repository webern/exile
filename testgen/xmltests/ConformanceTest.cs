using System;
using System.IO;

namespace xmltests
{
    /// <summary>
    /// Represents an XML conformance test from the official XML conformance test suite.
    /// </summary>
    public class ConformanceTest
    {
        /// <summary>
        /// The ID of the test, as given in the <code>TESTCASE</code> element.
        /// </summary>
        public String Id { get; set; }
        public String Profile { get; set; }
        public String BasePath {get => _basePath; set => SetBasePath(value); }
        
        public String Sections { get; set; }
        public String Entities { get; set; }
        public String Uri { get; set; }
        public String Type { get; set; }
        public String Namespace { get; set; }
        public String Recommendation { get; set; }
        public String Version { get; set; }
        public FileInfo XmlFile { get; set; }

        /// <summary>
        /// The internal BasePath backing field.
        /// </summary>
        private string _basePath;

        /// <summary>
        /// Sets the BasePath, ensuring that a trailing slash exists.
        /// </summary>
        /// <param name="inPath">The BasePath to set.</param>
        private void SetBasePath(String inPath)
        {
            if (inPath.Length > 0 && inPath[^1] != '/')
            {
                _basePath = $"{inPath}/";
            }
            else
            {
                _basePath = inPath;
            }
        }
    }
}