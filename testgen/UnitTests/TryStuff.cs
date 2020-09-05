using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
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
            // foreach (var conformanceTest in conformanceTests)
            // {
            //     Console.WriteLine("{0} {1} {2} {3}", conformanceTest.Profile, conformanceTest.BasePath,
            //         conformanceTest.Type, conformanceTest.ID);
            // }

            doMoreStuff(conformanceTests);
        }

        private void doMoreStuff(List<ConformanceTest> conformanceTests)
        {
            foreach (var conformanceTest in conformanceTests)
            {
                if (conformanceTest.Type == "valid" && conformanceTest.Namespace != "no")
                {
                    doOneThing(conformanceTest);
                }
            }
        }

        private static void DisableUndeclaredEntityCheck(XmlTextReader obj)
        {
            
            var binderFlagsA = BindingFlags.Instance | BindingFlags.Public | BindingFlags.NonPublic;
            var binderFlagsB = BindingFlags.Public | BindingFlags.NonPublic;
            var xmlTextReaderType = obj.GetType();
            var implPropertyName = "Impl";
            PropertyInfo implPropertyInfo = xmlTextReaderType.GetProperty(
                implPropertyName, binderFlagsA);
            if (implPropertyInfo == null)
            {
                throw new Exception("Could not find property Impl");
            }

            var impl = implPropertyInfo.GetValue(obj);
            var implType = impl.GetType();
            PropertyInfo propertyInfo = implType.GetProperty(
                "DisableUndeclaredEntityCheck", binderFlagsA);
            if (propertyInfo == null)
            {
                throw new Exception("Could not find property DisableUndeclaredEntityCheck");
            }

            propertyInfo.SetValue(impl, true);
        }

        private void doOneThing(ConformanceTest conformanceTest)
        {
            var path = Path.Combine(Paths.Instance.DataPaths.XmlConfDir.FullName,
                conformanceTest.BasePath + conformanceTest.URI);
            var fileInfo = new FileInfo(path);
            if (!fileInfo.Exists)
            {
                throw new FileNotFoundException(path);
            }

            var textReader = new XmlTextReader(fileInfo.FullName);
            DisableUndeclaredEntityCheck(textReader);
            textReader.EntityHandling = EntityHandling.ExpandCharEntities;
            textReader.DtdProcessing = DtdProcessing.Parse;
            // textReader.XmlResolver = null;
            textReader.Namespaces = true;
            var doc = new XmlDocument();
            Console.WriteLine(fileInfo.FullName);
            doc.Load(textReader);
            foreach (var docChild in doc.ChildNodes)
            {
                var node = asNode(docChild);
                switch (node.NodeType)
                {
                    case XmlNodeType.None:
                        throw new Exception("XmlNodeType.None");
                        break;
                    case XmlNodeType.Element:
                        Console.WriteLine("Element : '{0}'", node.Value);
                        break;
                    case XmlNodeType.Attribute:
                        Console.WriteLine("Attribute : '{0}'", node.Value);
                        break;
                    case XmlNodeType.Text:
                        Console.WriteLine("Text : '{0}'", node.Value);
                        break;
                    case XmlNodeType.CDATA:
                        Console.WriteLine("CDATA : '{0}'", node.Value);
                        break;
                    case XmlNodeType.EntityReference:
                        Console.WriteLine("EntityReference : '{0}'", node.Value);
                        break;
                    case XmlNodeType.Entity:
                        Console.WriteLine("Entity : '{0}'", node.Value);
                        break;
                    case XmlNodeType.ProcessingInstruction:
                        Console.WriteLine("ProcessingInstruction : '{0}'", node.Value);
                        break;
                    case XmlNodeType.Comment:
                        Console.WriteLine("Comment : '{0}'", node.Value);
                        break;
                    case XmlNodeType.Document:
                        Console.WriteLine("Document : '{0}'", node.Value);
                        break;
                    case XmlNodeType.DocumentType:
                        Console.WriteLine("DocumentType : '{0}'", node.Value);
                        break;
                    case XmlNodeType.DocumentFragment:
                        throw new Exception("XmlNodeType.DocumentFragment");
                        break;
                    case XmlNodeType.Notation:
                        throw new Exception("XmlNodeType.Notation");
                        break;
                    case XmlNodeType.Whitespace:
                        Console.WriteLine("Whitespace : '{0}'", node.Value);
                        break;
                    case XmlNodeType.SignificantWhitespace:
                        Console.WriteLine("SignificantWhitespace : '{0}'", node.Value);
                        break;
                    case XmlNodeType.EndElement:
                        throw new Exception("XmlNodeType.EndElement");
                        break;
                    case XmlNodeType.EndEntity:
                        throw new Exception("XmlNodeType.Exception");
                        break;
                    case XmlNodeType.XmlDeclaration:
                        Console.WriteLine("XmlDeclaration : '{0}'", node.Value);
                        break;
                    default:
                        throw new ArgumentOutOfRangeException();
                }
            }
        }

        public XmlNode asNode(Object maybe)
        {
            var node = (XmlNode) maybe;
            if (node == null)
            {
                throw new Exception("wtf? not an XmlNode?");
            }

            return node;
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
            t.Namespace = getAttributeValue(element, "NAMESPACE");
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
        public String Namespace { get; set; }
    }
}