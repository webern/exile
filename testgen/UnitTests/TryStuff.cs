using System;
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
            foreach (var child in root.ChildNodes)
            {
                if (child is XmlElement)
                {
                    var element = (XmlElement) child;
                    var n = element.Name;
                    System.Console.WriteLine(n);
                }
                else if (child is XmlEntityReference)
                {
                    var reference = (XmlEntityReference) child;
                    var n = reference.Name;
                    System.Console.WriteLine(n);
                    // reference.
                }
            }

            // var set = new XmlReaderSettings();
            // set.DtdProcessing = DtdProcessing.Parse;
            //
            // var reader = XmlReader.Create(file.FullName, set);
            // Console.WriteLine("foo");
            // while (reader.Read())
            // {
            //     switch (reader.NodeType)
            //     {
            //         case XmlNodeType.Element:
            //             Console.Write("<{0}>", reader.Name);
            //             break;
            //         case XmlNodeType.Text:
            //             Console.Write(reader.Value);
            //             break;
            //         case XmlNodeType.CDATA:
            //             Console.Write("<![CDATA[{0}]]>", reader.Value);
            //             break;
            //         case XmlNodeType.ProcessingInstruction:
            //             Console.Write("<?{0} {1}?>", reader.Name, reader.Value);
            //             break;
            //         case XmlNodeType.Comment:
            //             Console.Write("<!--{0}-->", reader.Value);
            //             break;
            //         case XmlNodeType.XmlDeclaration:
            //             Console.Write("<?xml version='1.0'?>");
            //             break;
            //         case XmlNodeType.Document:
            //             break;
            //         case XmlNodeType.DocumentType:
            //             Console.Write("<!DOCTYPE {0} [{1}]", reader.Name, reader.Value);
            //             break;
            //         case XmlNodeType.EntityReference:
            //             Console.Write(reader.Name);
            //             break;
            //         case XmlNodeType.EndElement:
            //             Console.Write("</{0}>", reader.Name);
            //             break;
            //         case XmlNodeType.EntityReference:
            //             Console.WriteLine("Entity reference: {0}", reader.Name);
            //             break;
            //     }
            // }
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
            }
        }
    }
}