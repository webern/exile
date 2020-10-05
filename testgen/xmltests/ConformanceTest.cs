using System;
using System.IO;

namespace xmltests
{
    /// <summary>
    /// Some tests may have a field identifying the kinds of external Entities a nonvalidating processor must include
    /// (parameter, general, or both) to be able to detect any errors in that test case.
    /// </summary>
    public enum Entities
    {
        /// <summary>
        /// The `ENTITIES` field is not present.
        /// </summary>
        None,
        
        /// <summary>
        /// The `ENTITIES` field is present and has the value "none".
        /// </summary>
        Empty,

        /// <summary>
        /// The `ENTITIES` field is present and has the value "both".
        /// </summary>
        Both,
        
        /// <summary>
        /// The `ENTITIES` field is present and has the value "general".
        /// </summary>
        General,
        
        /// <summary>
        /// The `ENTITIES` field is present and has the value "parameter".
        /// </summary>
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

        /// <summary>
        /// Some tests may have a field identifying the kinds of external Entities a nonvalidating processor must
        /// include (parameter, general, or both) to be able to detect any errors in that test case.
        /// </summary>
        public Entities Entities { get; set; }
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
        /// <param name="value">The BasePath to set.</param>
        private void SetBasePath(String value)
        {
            if (value.Length > 0 && value[^1] != '/')
            {
                _basePath = $"{value}/";
            }
            else
            {
                _basePath = value;
            }
        }
    }
}