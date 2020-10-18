package com.matthewjamesbriggs.xmltestgen;

import java.io.File;
import java.io.FileOutputStream;
import java.util.List;

public class ConfTestGenerator {
    private final File outDir;
    private final File generatedDir;
    private final File testDataDir;
    private final File rustWorkspaceDir;
    private final List<ConfTest> tests;

    public ConfTestGenerator(List<ConfTest> tests, ProgramOptions opts) throws TestGenException {
        outDir = F.canonicalize(opts.getXmlOutdir());
        F.checkDir(outDir);
        generatedDir = F.canonicalize(new File(opts.getXmlOutdir(), "generated"));
        F.checkDir(generatedDir);
        testDataDir = F.canonicalize(new File(opts.getXmlOutdir(), "input_data"));
        F.checkDir(testDataDir);
        rustWorkspaceDir = F.canonicalize(opts.getRustRoot());
        F.checkDir(rustWorkspaceDir);
        this.tests = tests;
    }

    public void generateTests(int maxTests) throws TestGenException {
        File dir = generatedDir;
        F.createOrReplaceDir(generatedDir);

        // create the mod.rs file
        File modRs = F.canonicalize(new File(dir, "mod.rs"));
        FileOutputStream modRsStream = F.createAndOpen(modRs);
        F.writeln(modRsStream, "// generated file, do not edit");
        F.writeln(modRsStream, "");

        // create test files
        int testCount = 0;
        for (int i = 0; i < tests.size() && testCount < maxTests; ++i) {
            ConfTest t = tests.get(i);
            if (t.getConfType() != ConfType.Valid) {
                continue;
            }
            ++testCount;
            String id = t.getSnakeCase();
            File testFile = new File(dir, id + ".rs");
            FileOutputStream os = F.createAndOpen(testFile);
            F.writeln(modRsStream, "mod %s;", id);
            F.writeln(os, "// generated file, do not edit");

            F.writeln(os, "use std::path::PathBuf;");
            F.writeln(os, "const MANIFEST_DIR: &str = env!(\"CARGO_MANIFEST_DIR\");");
            F.writeln(os, "const INPUT_DATA: &str = \"input_data\";");
            F.writeln(os, "const FILENAME: &str = \"%s\";", t.getFileRename());

            F.writeln(os, "");
            F.writeln(os, "fn path() -> PathBuf {");
            F.writeln(os, "    let p = PathBuf::from(MANIFEST_DIR)");
            F.writeln(os, "        .join(\"tests\")");
            F.writeln(os, "        .join(INPUT_DATA)");
            F.writeln(os, "        .join(FILENAME);");
            F.writeln(os, "    p.canonicalize()");
            F.writeln(os, "        .expect(format!(\"bad path: {}\", p.display()).as_str())");
            F.writeln(os, "}");

            F.writeln(os, "");
            F.writeln(os, "#[test]");
            F.writeln(os, "fn %s() {", id);
            F.writeln(os, "    let path = path();");
            F.writeln(os, "    let _doc = exile::load(&path).unwrap();");
            F.writeln(os, "}");


            F.closeStream(testFile, os);

            copyXmlTestFile(t);
        }

        F.closeStream(modRs, modRsStream);

        File exileCrate = new File(rustWorkspaceDir, "exile");
        exileCrate = F.canonicalize(exileCrate);
        File manifestPath = new File(exileCrate, "Cargo.toml");
        manifestPath = F.canonicalize(manifestPath);
        F.checkFile(manifestPath);
        Cmd.fmt(rustWorkspaceDir);
    }

    private void generateTest(ConfTest t) throws TestGenException {

    }

    private void copyXmlTestFile(ConfTest t) throws TestGenException {
        F.checkDir(testDataDir);
        File original = new File(t.getPath().toString());
        F.checkFile(original);
        File copied = new File(testDataDir, t.getFileRename());
        F.copy(original, copied);
    }
}
