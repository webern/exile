using System;
using System.IO;

namespace xmltests
{
    public class DataPaths
    {
        public DirectoryInfo XmlTestDir { get; }
        public DirectoryInfo SchemaTestDir { get; }

        public FileInfo XmlConfFile
        {
            get
            {
                var info = new FileInfo(Path.Combine(XmlTestDir.FullName, "xmlconf.xml"));
                if (!info.Exists)
                {
                    throw new FileNotFoundException("Could not find xmlconf.xml");
                }

                return info;
            }
        }
        
        public FileInfo SchemaSuiteFile
        {
            get
            {
                var info = new FileInfo(Path.Combine(SchemaTestDir.FullName, "suite.xml"));
                if (!info.Exists)
                {
                    throw new FileNotFoundException("Could not find suite.xml");
                }

                return info;
            }
        }
        
        public DataPaths(DirectoryInfo xmlTestDir, DirectoryInfo schemaTestDir)
        {
            XmlTestDir = xmlTestDir;
            SchemaTestDir = schemaTestDir;

            if (XmlTestDir == null) throw new NullReferenceException("XmlTestDir");

            if (!XmlTestDir.Exists)
                throw new DirectoryNotFoundException($"No directory at {XmlTestDir.FullName}");

            if (SchemaTestDir == null) throw new NullReferenceException("SchemaTestDir");

            if (!SchemaTestDir.Exists)
                throw new DirectoryNotFoundException($"No directory at {SchemaTestDir.FullName}");
        }

    }
}