using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Xml;
using System.Xml.Resolvers;
using NUnit.Framework;
using xmltests;

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
                if (conformanceTest.Type == "valid" && conformanceTest.Namespace != "no" &&
                    conformanceTest.Version != "1.1")
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
                conformanceTest.BasePath + conformanceTest.Uri);
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
            // Console.WriteLine(fileInfo.FullName);
            doc.Load(textReader);
            // foreach (var docChild in doc.ChildNodes)
            // {
            //     var node = asNode(docChild);
            //     switch (node.NodeType)
            //     {
            //         case XmlNodeType.None:
            //             throw new Exception("XmlNodeType.None");
            //             break;
            //         case XmlNodeType.Element:
            //             Console.WriteLine("Element : '{0}'", node.Value);
            //             break;
            //         case XmlNodeType.Attribute:
            //             Console.WriteLine("Attribute : '{0}'", node.Value);
            //             break;
            //         case XmlNodeType.Text:
            //             Console.WriteLine("Text : '{0}'", node.Value);
            //             break;
            //         case XmlNodeType.CDATA:
            //             Console.WriteLine("CDATA : '{0}'", node.Value);
            //             break;
            //         case XmlNodeType.EntityReference:
            //             Console.WriteLine("EntityReference : '{0}'", node.Value);
            //             break;
            //         case XmlNodeType.Entity:
            //             Console.WriteLine("Entity : '{0}'", node.Value);
            //             break;
            //         case XmlNodeType.ProcessingInstruction:
            //             Console.WriteLine("ProcessingInstruction : '{0}'", node.Value);
            //             break;
            //         case XmlNodeType.Comment:
            //             Console.WriteLine("Comment : '{0}'", node.Value);
            //             break;
            //         case XmlNodeType.Document:
            //             Console.WriteLine("Document : '{0}'", node.Value);
            //             break;
            //         case XmlNodeType.DocumentType:
            //             Console.WriteLine("DocumentType : '{0}'", node.Value);
            //             break;
            //         case XmlNodeType.DocumentFragment:
            //             throw new Exception("XmlNodeType.DocumentFragment");
            //             break;
            //         case XmlNodeType.Notation:
            //             throw new Exception("XmlNodeType.Notation");
            //             break;
            //         case XmlNodeType.Whitespace:
            //             Console.WriteLine("Whitespace : '{0}'", node.Value);
            //             break;
            //         case XmlNodeType.SignificantWhitespace:
            //             Console.WriteLine("SignificantWhitespace : '{0}'", node.Value);
            //             break;
            //         case XmlNodeType.EndElement:
            //             throw new Exception("XmlNodeType.EndElement");
            //             break;
            //         case XmlNodeType.EndEntity:
            //             throw new Exception("XmlNodeType.Exception");
            //             break;
            //         case XmlNodeType.XmlDeclaration:
            //             Console.WriteLine("XmlDeclaration : '{0}'", node.Value);
            //             break;
            //         default:
            //             throw new ArgumentOutOfRangeException();
            //     }
            // }
            Console.WriteLine(conformanceTest.Entities);
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
            findTestsRecursive((XmlElement) root, tests, new List<String>(), "");
            return tests;
        }

        private void findTestsRecursive(XmlElement element, List<ConformanceTest> outTests, List<string> inProfiles,
            string basePath)
        {
            var profiles = new List<string>(inProfiles);
            // a base case, we do not follow children
            if (element.Name == "TEST")
            {
                var conformanceTest = parseTest(element, profiles, basePath, Paths.Instance.DataPaths);
                outTests.Add(conformanceTest);
                return;
            }

            if (element.Name == "TESTCASES")
            {
                foreach (var a in attributes(element))
                {
                    if (a.Name == "PROFILE")
                    {
                        profiles.Add(a.Value);
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
                findTestsRecursive(child, outTests, profiles, basePath);
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

        private ConformanceTest parseTest(XmlElement element, List<String> profiles, string basePath, DataPaths paths)
        {
            var t = new ConformanceTest();
            t.Profile = String.Join(", ", profiles);
            t.BasePath = basePath;
            t.Type = getAttributeValue(element, "TYPE");
            t.Sections = getAttributeValue(element, "SECTIONS");
            var entities = getAttributeValue(element, "ENTITIES");
            switch (entities)
            {
                case "both":
                    t.Entities = Entities.Both;
                    break;
                case "":
                    t.Entities = Entities.Empty;
                    break;
                case "general":
                    t.Entities = Entities.General;
                    break;
                case "none":
                    t.Entities = Entities.None;
                    break;
                case "parameter":
                    t.Entities = Entities.Parameter;
                    break;
                default:
                    throw new Exception($"unknown entities value '{entities}'");
            }
            t.Uri = getAttributeValue(element, "URI");
            t.Namespace = getAttributeValue(element, "NAMESPACE");
            t.Id = getAttributeValue(element, "ID");
            t.Recommendation = getAttributeValue(element, "RECOMMENDATION");
            t.Version = getAttributeValue(element, "VERSION");

            if (t.BasePath == "")
            {
                if (t.Profile == "Richard Tobin's XML 1.0 2nd edition errata test suite 21 Jul 2003")
                {
                    t.BasePath = "eduni/errata-2e/";
                }
                else if (t.Profile == "Richard Tobin's XML 1.1 test suite 13 Feb 2003")
                {
                    t.BasePath = "eduni/xml-1.1/";
                }
                else if (t.Profile == "Richard Tobin's XML Namespaces 1.0 test suite 14 Feb 2003")
                {
                    t.BasePath = "eduni/namespaces/1.0/";
                }
                else if (t.Profile == "Richard Tobin's XML Namespaces 1.1 test suite 14 Feb 2003")
                {
                    t.BasePath = "eduni/namespaces/1.1/";
                }
            }
            else if (t.BasePath == "ibm/" && t.Version == "1.1")
            {
                if (t.Profile.Contains("IBM Invalid Conformance Tests for XML 1.1 CR October 15, 2002") ||
                    t.Profile.Contains("IBM Not-WF Conformance Tests for XML 1.1 CR October 15, 2002") ||
                    t.Profile.Contains("IBM Valid Conformance Tests for XML 1.1 CR October 15, 2002"))
                {
                    t.BasePath = "ibm/xml-1.1";
                }
            }

            // TODO find the filepath to the test xml file within BasePath

            // what follows is stupid. i don't know how to find the file from the given XML objects, so instead i
            // search for the file by name.
            var searchIn = Path.Combine(paths.XmlConfDir.FullName, t.BasePath);


            var dir = new DirectoryInfo(searchIn);
            if (!dir.Exists)
            {
                throw new DirectoryNotFoundException(dir.FullName);
            }


            string[] files = Directory.GetFiles(dir.FullName, t.Uri, SearchOption.AllDirectories);
            var filteredFiles = new List<String>();
            foreach (var f in files)
            {
                var filterEduniOut = t.BasePath.StartsWith("eduni") && f.Contains($"out/{Path.GetFileName(t.Uri)}");

                if (f.EndsWith(t.Uri) && !filterEduniOut)
                {
                    filteredFiles.Add(f);
                }
            }

            if (filteredFiles.Count > 1)
            {
                throw new Exception($"more than one candidate file found for {t.Uri}");
            }
            else if (filteredFiles.Count == 0)
            {
                throw new FileNotFoundException($"could not find {t.Uri}");
            }

            var fileinfo = new FileInfo(files[0]);
            if (!fileinfo.Exists)
            {
                throw new FileLoadException("Could not find file", files[0]);
            }

            t.XmlFile = fileinfo;
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
}