using System;
using System.IO;

namespace xmltests
{
    public class DataPaths
    {
        public DirectoryInfo XmlConfDir { get; }
        public DirectoryInfo SchemaTestDir { get; }

        public FileInfo XmlConfFile
        {
            get
            {
                var info = new FileInfo(Path.Combine(XmlConfDir.FullName, "xmlconf.xml"));
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
        
        public DataPaths(DirectoryInfo xmlConfDir, DirectoryInfo schemaTestDir)
        {
            XmlConfDir = xmlConfDir;
            SchemaTestDir = schemaTestDir;

            if (XmlConfDir == null) throw new NullReferenceException("XmlTestDir");

            if (!XmlConfDir.Exists)
                throw new DirectoryNotFoundException($"No directory at {XmlConfDir.FullName}");

            if (SchemaTestDir == null) throw new NullReferenceException("SchemaTestDir");

            if (!SchemaTestDir.Exists)
                throw new DirectoryNotFoundException($"No directory at {SchemaTestDir.FullName}");
        }
    }
}