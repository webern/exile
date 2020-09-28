using System;

namespace xmltests
{
    public class ExileTestWriter
    {
        public string Outdir { get; }
        public LoadedTest LoadedTest { get; }

        public ExileTestWriter(String outdir, LoadedTest loadedTest)
        {
            Outdir = outdir;
            LoadedTest = loadedTest;
        }

        public void Write()
        {
            Console.WriteLine("{0}", LoadedTest.Info.Id);
        }
    }
}