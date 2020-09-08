using System;
using System.IO;

namespace xmltests
{
    enum Entities
    {
        Both,
        Empty,
        General,
        None,
        Parameter,
    }

    /// <summary>
    /// Represents an XML conformance test from the official XML conformance test suite.
    /// </summary>
    public class ConformanceTest
    {
        /// <summary>
        /// The ID of the test, as given in the <code>TESTCASE</code> element.
        /// </summary>
        public String Id { get; set; }

        /// <summary>
        /// The combined Profile, as given by one or more <code>TESTCASES</code> elements. Joined by ", ".
        /// </summary>
        public String Profile { get; set; }

        /// <summary>
        /// The path to the directory where the XML file with the <code>TESTCASES</code> element lives (I think).
        /// </summary>
        public String BasePath
        {
            get => _basePath;
            set => SetBasePath(value);
        }

        /// <summary>
        /// Which sections of the XML specification are being tested. 
        /// </summary>
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