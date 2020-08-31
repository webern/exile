using NUnit.Framework;

namespace UnitTests
{
    public class DataPathsTest
    {
        [Test]
        public void Test()
        {
            Assert.True(Paths.Instance.DataPaths.XmlTestDir.Exists);
            Assert.True(Paths.Instance.DataPaths.SchemaTestDir.Exists);
            Assert.True(Paths.Instance.DataPaths.XmlConfFile.Exists);
            Assert.True(Paths.Instance.DataPaths.SchemaSuiteFile.Exists);
            System.Console.WriteLine(Paths.Instance.DataPaths.XmlTestDir.FullName);
        }
    }
}