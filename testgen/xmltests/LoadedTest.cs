using System.Xml;

namespace xmltests
{
    public class LoadedTest
    {
        public ConformanceTest Info { get; }
        public XmlDocument Document { get; }

        public LoadedTest(ConformanceTest info, XmlDocument document)
        {
            Info = info;
            Document = document;
        }
    }
}