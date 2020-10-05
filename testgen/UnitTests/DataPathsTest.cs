using NUnit.Framework;

namespace UnitTests
{
    public class DataPathsTest
    {
        [Test]
        public void Test()
        {
            Assert.True(Paths.Instance.DataPaths.XmlConfDir.Exists);
            Assert.True(Paths.Instance.DataPaths.SchemaTestDir.Exists);
            Assert.True(Paths.Instance.DataPaths.XmlConfFile.Exists);
            Assert.True(Paths.Instance.DataPaths.SchemaSuiteFile.Exists);
            System.Console.WriteLine(Paths.Instance.DataPaths.XmlConfDir.FullName);
        }
    }
}