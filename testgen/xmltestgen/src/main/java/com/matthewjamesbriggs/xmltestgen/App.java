package com.matthewjamesbriggs.xmltestgen;

import org.apache.commons.io.FileUtils;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.util.List;

// http://www.java2s.com/Tutorials/Java/XML/How_to_get_root_element_from_Java_DOM_parser.htm

public class App {
    private static final int SUCCESS = 0;
    private static final int FAILURE = 1;
    private static final int BAD_USAGE = 2;
    private static final int NUM_TESTS = 5;

    public static void main(String[] args) {
        ProgramOptions opts = null;
        try {
            opts = ProgramOptions.parse(args);
        } catch (TestGenException e) {
            System.exit(BAD_USAGE);
        }
        try {
            List<ConfTest> confTests = ConfTestParser.parse(opts.getW3cXml().getPath());
            doThings(confTests, opts);
        } catch (TestGenException e) {
            System.out.println(e.getMessage());
            e.printStackTrace();
            System.exit(FAILURE);
        }
        System.exit(SUCCESS);
    }

    private static void doThings(List<ConfTest> confTests, ProgramOptions opts) throws TestGenException {
        // delete and recreate the directory named "generated"
        File dir = new File(opts.getXmlOutdir(), "generated");
        dir = F.canonicalize(dir);
        F.createOrReplaceDir(dir);
        File inputData = new File(opts.getXmlOutdir(), "input_data");
        inputData = F.canonicalize(inputData);
        F.createOrReplaceDir(inputData);
        File rustRoot = new File(opts.getRustRoot().getPath());
        rustRoot = F.canonicalize(rustRoot);
        F.checkDir(rustRoot);

        // create the mod.rs file
        File modRs = new File(dir, "mod.rs");
        modRs = F.canonicalize(modRs);
        F.createFile(modRs);
        FileOutputStream modRsStream = F.openFile(modRs);
        F.writeln(modRsStream, "// generated file, do not edit");
        F.writeln(modRsStream, "");

        // create test files
        int testCount = 0;
        for (int i = 0; i < confTests.size() && testCount < NUM_TESTS; ++i) {
            ConfTest t = confTests.get(i);
            if (t.getConfType() != ConfType.Valid) {
                continue;
            }
            ++testCount;
            String id = t.getSnakeCase();
            File testFile = new File(dir, id + ".rs");
            F.createFile(testFile);
            FileOutputStream os = F.openFile(testFile);
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

            copyXmlTestFile(t, inputData);
        }

        F.closeStream(modRs, modRsStream);

        File exileCrate = new File(rustRoot, "exile");
        exileCrate = F.canonicalize(exileCrate);
        File manifestPath = new File(exileCrate, "Cargo.toml");
        manifestPath = F.canonicalize(manifestPath);
        F.checkFile(manifestPath);
        Cmd.fmt(rustRoot);
    }

    private static void copyXmlTestFile(ConfTest t, File copyToDir) throws TestGenException {
        F.checkDir(copyToDir);
        File original = new File(t.getPath().toString());
        F.checkFile(original);
        File copied = new File(copyToDir, t.getFileRename());
        F.copy(original, copied);
    }
}
