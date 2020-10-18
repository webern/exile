package com.matthewjamesbriggs.xmltestgen;

import java.io.File;
import java.io.FileOutputStream;
import java.util.List;

class ConfTestGenerator {
    /// The tests directory, e.g. exile_repo/exile/tests
    private final File outDir;
    /// The root of the generated tests, e.g. exile_repo/exile/tests/generated
    private final File generatedDir;
    /// The place to copy xml files, e.g. exile_repo/exile/tests/input_data
    private final File testDataDir;
    /// The root of the rust workspace, e.g. exile_repo
    private final File rustWorkspaceDir;
    /// The parsed conf tests to use as our source for generating rust tests
    private final List<ConfTest> tests;
    /// The mod.rs file for our generated rust tests to be compiled, e.g. exile_repo/exile/tests/generated/mod.rs
    private final File modRs;


    ConfTestGenerator(List<ConfTest> tests, ProgramOptions opts) throws TestGenException {
        outDir = F.canonicalize(opts.getXmlOutdir());
        F.checkDir(outDir);
        generatedDir = F.canonicalize(new File(opts.getXmlOutdir(), "generated"));
        F.checkDir(generatedDir);
        testDataDir = F.canonicalize(new File(opts.getXmlOutdir(), "input_data"));
        F.checkDir(testDataDir);
        rustWorkspaceDir = F.canonicalize(opts.getRustRoot());
        F.checkDir(rustWorkspaceDir);
        this.tests = tests;
        modRs = F.canonicalize(new File(generatedDir, "mod.rs"));
    }

    void generateTests(int maxTests) throws TestGenException {
        F.createOrReplaceDir(generatedDir);

        // create the mod.rs file
        FileOutputStream mod = F.createAndOpen(modRs);
        writeCodeFileHeader(mod);
        F.writeln(mod, "");

        // create test files
        int testCount = 0;
        for (int i = 0; i < tests.size() && testCount < maxTests; ++i) {
            ConfTest t = tests.get(i);
            if (t.getConfType() != ConfType.Valid) {
                continue;
            }
            ++testCount;
            generateValidTest(t, mod);
        }

        F.writeln(mod, "");
        F.closeStream(modRs, mod);
        Cmd.fmt(rustWorkspaceDir);
    }

    private void generateValidTest(ConfTest t, FileOutputStream mod) throws TestGenException {
        if (t.getConfType() != ConfType.Valid) {
            throw new TestGenException("wrong test type, expected 'valid', got " + t.getConfType().toString());
        }
        File testFile = new File(generatedDir, t.getTestName() + ".rs");
        FileOutputStream os = F.createAndOpen(testFile);
        F.writeln(mod, "mod %s;", t.getTestName());
        writeCodeFileHeader(os);
        F.writeln(os, "");
        writeUseStatements(t, os);
        F.writeln(os, "");
        writeConstDeclarations(t, os);
        F.writeln(os, "");
        writePathFunction(t, os);
        F.writeln(os, "");
        writeTestFunction(t, os);
        F.closeStream(testFile, os);
        copyXmlTestFile(t);
    }

    private static void writeCodeFileHeader(FileOutputStream os) throws TestGenException {
        F.writeln(os, "// generated file, do not edit");
    }

    private static void writeUseStatements(ConfTest t, FileOutputStream os) throws TestGenException {
        F.writeln(os, "use std::path::PathBuf;");
    }

    private static void writeConstDeclarations(ConfTest t, FileOutputStream os) throws TestGenException {
        F.writeln(os, "const MANIFEST_DIR: &str = env!(\"CARGO_MANIFEST_DIR\");");
        F.writeln(os, "const INPUT_DATA: &str = \"input_data\";");
        F.writeln(os, "const FILENAME: &str = \"%s\";", t.getFileRename());
    }

    private static void writePathFunction(ConfTest t, FileOutputStream os) throws TestGenException {
        F.writeln(os, "fn path() -> PathBuf {");
        F.writeln(os, "    let p = PathBuf::from(MANIFEST_DIR)");
        F.writeln(os, "        .join(\"tests\")");
        F.writeln(os, "        .join(INPUT_DATA)");
        F.writeln(os, "        .join(FILENAME);");
        F.writeln(os, "    p.canonicalize()");
        F.writeln(os, "        .expect(format!(\"bad path: {}\", p.display()).as_str())");
        F.writeln(os, "}");
    }

    private static void writeTestFunction(ConfTest t, FileOutputStream os) throws TestGenException {
        F.writeln(os, "#[test]");
        F.writeln(os, "fn %s() {", t.getSnakeCase());
        F.writeln(os, "    let path = path();");
        F.writeln(os, "    let _doc = exile::load(&path).unwrap();");
        F.writeln(os, "}");
    }

    private void copyXmlTestFile(ConfTest t) throws TestGenException {
        F.checkDir(testDataDir);
        File original = new File(t.getPath().toString());
        F.checkFile(original);
        File copied = new File(testDataDir, t.getFileRename());
        F.copy(original, copied);
    }
}
