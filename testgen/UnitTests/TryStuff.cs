using System;
using System.Collections.Generic;
using System.Xml;
using System.Xml.Resolvers;
using NUnit.Framework;

namespace UnitTests
{
    public class TryStuff
    {
        [Test]
        public void Test()
        {
            var file = Paths.Instance.DataPaths.XmlConfFile;
            var doc = new XmlDocument();

            // var xmlReaderSettings = new XmlReaderSettings();
            // xmlReaderSettings.DtdProcessing = DtdProcessing.Parse;
            var textReader = new XmlTextReader(file.FullName);
            textReader.EntityHandling = EntityHandling.ExpandEntities;
            textReader.DtdProcessing = DtdProcessing.Parse;
            textReader.XmlResolver = new XmlUrlResolver();
            doc.Load(textReader);
            Assert.AreEqual("#document", doc.Name);
            var root = doc.DocumentElement;
            printChildren(root);
            var conformanceTests = findTests(root);
        }

        public void printChildren(XmlNode node)
        {
            if (!(node is XmlElement))
            {
                return;
            }

            var element = (XmlElement) node;
            foreach (var child in element.ChildNodes)
            {
                if (child is XmlElement)
                {
                    System.Console.WriteLine(((XmlElement) child).Name);
                    printChildren((XmlNode) child);
                }
                else if (child is XmlEntityReference)
                {
                    var reference = (XmlEntityReference) child;
                    var n = reference.Name;
                    System.Console.WriteLine("Reference: {0}", n);
                    // reference.
                }
            }
        }

        public List<ConformanceTest> findTests(XmlNode root)
        {
            var tests = new List<ConformanceTest>();
            findTestsRecursive(root, tests, "unknown", "");
            return tests;
        }

        private void findTestsRecursive(XmlNode current, List<ConformanceTest> outTests, string currentSuite, string basePath)
        {
            if (!(current is XmlElement))
            {
                return;
            }

            var element = (XmlElement) current;
            
            // a base case, we do not follow children
            if (element.Name == "TEST")
            {
                var conformanceTest = parseTest(element, currentSuite, basePath);
                outTests.Add(conformanceTest);
                return;
            }
            
            if (element.Name == "TESTCASES")
            {
                foreach (var attribute in element.Attributes)
                {
                    if (attribute is XmlAttribute)
                    {
                        var a = (XmlAttribute) attribute;
                        if (a.Name == "PROFILE")
                        {
                            currentSuite = a.Value;
                        }
                        else if (a.Name == "base")
                        {
                            basePath = a.Value;
                        }
                        else if (a.Name == "xml:base")
                        {
                            basePath = a.Value;
                        }
                    }
                }
            }
            
            foreach (var childNode in element.ChildNodes)
            {
                if (childNode is XmlElement)
                {
                    findTestsRecursive((XmlNode) childNode, outTests, currentSuite, basePath);
                }
            }
        }

        private ConformanceTest parseTest(XmlElement element, string currentSuite, string basePath)
        {
            var t = new ConformanceTest();
            t.Profile = currentSuite;
            t.BasePath = basePath;
            return t;
        }
    }

    public class ConformanceTest
    {
        public String Profile { get; set; }
        public String BasePath { get; set; }
    }
}