using System.Xml;
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
            doc.Load(file.FullName);
            Assert.AreEqual("#document", doc.Name);
            var root = doc.DocumentElement;
            foreach (var child in root.ChildNodes)
            {
                if (child is XmlElement)
                {
                    var element = (XmlElement)child;
                    var n = element.Name;
                    System.Console.WriteLine(n);

                }
            }
        }
    }
}