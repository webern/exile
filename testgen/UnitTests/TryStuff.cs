using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Xml;
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
            var textReader = new XmlTextReader(file.FullName)
            {
                EntityHandling = EntityHandling.ExpandEntities,
                DtdProcessing = DtdProcessing.Parse,
                XmlResolver = new XmlUrlResolver()
            };
            doc.Load(textReader);
            Assert.AreEqual("#document", doc.Name);
            var root = doc.DocumentElement;
            var conformanceTests = FindTests(root);
            ProcessTests(conformanceTests);
        }

        private void ProcessTests(List<ConformanceTest> conformanceTests)
        {
            foreach (var conformanceTest in conformanceTests)
            {
                // TODO - Switch to Xerces because XML 1.1 is not supported in .net. 
                if (conformanceTest.Type == "valid" && conformanceTest.Namespace != "no" &&
                    conformanceTest.Version != "1.1")
                {
                    ProcessTest(conformanceTest);
                }
            }
        }

        /// <summary>
        /// Digs into the private interface of XmlTextReader, using reflection, and disables entity checking. This
        /// allows us to read DTD entity references with following them and throwing if they cannot be resolved.
        /// </summary>
        /// <param name="xmlTextReader">The XmlTextReader that will be altered to disable entity checking.</param>
        /// <exception cref="Exception">If reflection fails, an exception will be thrown.</exception>
        private static void DisableUndeclaredEntityCheck(XmlTextReader xmlTextReader)
        {
            var binderFlags = BindingFlags.Instance | BindingFlags.Public | BindingFlags.NonPublic;
            var xmlTextReaderType = xmlTextReader.GetType();
            var implPropertyName = "Impl";
            PropertyInfo implPropertyInfo = xmlTextReaderType.GetProperty(
                implPropertyName, binderFlags);
            if (implPropertyInfo == null)
            {
                throw new Exception("Could not find property Impl");
            }

            var impl = implPropertyInfo.GetValue(xmlTextReader);
            if (impl == null)
            {
                throw new Exception("implPropertyInfo.GetValue(obj) returned null");
            }
            var implType = impl.GetType();
            PropertyInfo propertyInfo = implType.GetProperty(
                "DisableUndeclaredEntityCheck", binderFlags);
            if (propertyInfo == null)
            {
                throw new Exception("Could not find property DisableUndeclaredEntityCheck");
            }

            propertyInfo.SetValue(impl, true);
        }

        // TODO - give this function a better name and document it.
        private static void ProcessTest(ConformanceTest conformanceTest)
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
            textReader.Namespaces = true;
            var doc = new XmlDocument();
            doc.Load(textReader);
            // TODO - do something interesting with the test
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

        public XmlNode AsNode(Object maybe)
        {
            var node = (XmlNode) maybe;
            if (node == null)
            {
                throw new Exception("wtf? not an XmlNode?");
            }

            return node;
        }

        public void PrintChildren(XmlNode node)
        {
            if (!(node is XmlElement))
            {
                return;
            }

            var element = (XmlElement) node;
            foreach (var child in element.ChildNodes)
            {
                switch (child)
                {
                    case XmlElement xmlElement:
                        Console.WriteLine(xmlElement.Name);
                        PrintChildren(xmlElement);
                        break;
                    case XmlEntityReference entityReference:
                    {
                        var n = entityReference.Name;
                        Console.WriteLine("Reference: {0}", n);
                        break;
                    }
                }
            }
        }

        public List<ConformanceTest> FindTests(XmlNode root)
        {
            var tests = new List<ConformanceTest>();
            FindTestsRecursive((XmlElement) root, tests, new List<String>(), "");
            return tests;
        }

        private void FindTestsRecursive(XmlElement element, List<ConformanceTest> outTests, List<string> inProfiles,
            string basePath)
        {
            var profiles = new List<string>(inProfiles);
            // a base case, we do not follow children
            if (element.Name == "TEST")
            {
                var conformanceTest = ParseTest(element, profiles, basePath, Paths.Instance.DataPaths);
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
            }

            foreach (var child in children(element))
            {
                FindTestsRecursive(child, outTests, profiles, basePath);
            }
        }

        private string GetAttributeValue(XmlElement element, string attributeName)
        {
            foreach (var attribute in attributes(element).Where(attribute => attribute.Name == attributeName))
            {
                return attribute.Value;
            }

            return "";
        }

        private ConformanceTest ParseTest(XmlElement element, List<String> profiles, string basePath, DataPaths paths)
        {
            var t = new ConformanceTest
            {
                Profile = String.Join(", ", profiles),
                BasePath = basePath,
                Type = GetAttributeValue(element, "TYPE"),
                Sections = GetAttributeValue(element, "SECTIONS")
            };
            var entities = GetAttributeValue(element, "ENTITIES");
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
            t.Uri = GetAttributeValue(element, "URI");
            t.Namespace = GetAttributeValue(element, "NAMESPACE");
            t.Id = GetAttributeValue(element, "ID");
            t.Recommendation = GetAttributeValue(element, "RECOMMENDATION");
            t.Version = GetAttributeValue(element, "VERSION");

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

                if (f.EndsWith(t.Uri!) && !filterEduniOut)
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
                if (childNode is XmlElement node)
                {
                    children.Add(node);
                }
            }

            return children;
        }

        private List<XmlAttribute> attributes(XmlElement element)
        {
            var attributes = new List<XmlAttribute>();
            foreach (var thing in element.Attributes)
            {
                if (thing is XmlAttribute attribute)
                {
                    attributes.Add(attribute);
                }
            }

            return attributes;
        }
    }
}