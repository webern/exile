using System;
using System.Collections.Generic;
using System.Linq;
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
            // printChildren(root);
            var conformanceTests = findTests(root);
            foreach (var conformanceTest in conformanceTests)
            {
                Console.WriteLine("{0} {1} {2} {3}", conformanceTest.Profile, conformanceTest.BasePath,
                    conformanceTest.Type, conformanceTest.ID);
            }
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
            findTestsRecursive((XmlElement) root, tests, "unknown", "");
            return tests;
        }

        private void findTestsRecursive(XmlElement element, List<ConformanceTest> outTests, string currentSuite,
            string basePath)
        {
            // a base case, we do not follow children
            if (element.Name == "TEST")
            {
                var conformanceTest = parseTest(element, currentSuite, basePath);
                outTests.Add(conformanceTest);
                return;
            }

            if (element.Name == "TESTCASES")
            {
                foreach (var a in attributes(element))
                {
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

                // foreach (var attribute in element.Attributes)
                // {
                //     if (attribute is XmlAttribute)
                //     {
                //         var a = (XmlAttribute) attribute;
                //         if (a.Name == "PROFILE")
                //         {
                //             currentSuite = a.Value;
                //         }
                //         else if (a.Name == "base")
                //         {
                //             basePath = a.Value;
                //         }
                //         else if (a.Name == "xml:base")
                //         {
                //             basePath = a.Value;
                //         }
                //     }
                // }
            }

            foreach (var child in children(element))
            {
                findTestsRecursive(child, outTests, currentSuite, basePath);
            }
        }

        private string getAttributeValue(XmlElement element, string attributeName)
        {
            foreach (var attribute in attributes(element).Where(attribute => attribute.Name == attributeName))
            {
                return attribute.Value;
            }

            return "";
        }

        private ConformanceTest parseTest(XmlElement element, string currentSuite, string basePath)
        {
            var t = new ConformanceTest();
            t.Profile = currentSuite;
            t.BasePath = basePath;
            t.Type = getAttributeValue(element, "TYPE");
            t.Sections = getAttributeValue(element, "SECTIONS");
            t.Entities = getAttributeValue(element, "ENTITIES");
            t.URI = getAttributeValue(element, "URI");
            return t;
        }

        private List<XmlElement> children(XmlElement element)
        {
            var children = new List<XmlElement>();
            foreach (var childNode in element.ChildNodes)
            {
                if (childNode is XmlElement)
                {
                    children.Add((XmlElement) childNode);
                }
            }

            return children;
        }

        private List<XmlAttribute> attributes(XmlElement element)
        {
            var attributes = new List<XmlAttribute>();
            foreach (var thing in element.Attributes)
            {
                if (thing is XmlAttribute)
                {
                    attributes.Add((XmlAttribute) thing);
                }
            }

            return attributes;
        }
    }

    public class ConformanceTest
    {
        public String Profile { get; set; }
        public String BasePath { get; set; }
        public String ID { get; set; }
        public String Sections { get; set; }
        public String Entities { get; set; }
        public String URI { get; set; }
        public String Type { get; set; }
    }
}