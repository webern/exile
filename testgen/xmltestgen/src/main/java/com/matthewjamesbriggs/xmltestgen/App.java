package com.matthewjamesbriggs.xmltestgen;

import org.apache.commons.io.FileUtils;
import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.Node;
import org.w3c.dom.NodeList;
import org.xml.sax.SAXException;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.FactoryConfigurationError;
import javax.xml.parsers.ParserConfigurationException;
import java.io.*;
import java.net.URI;
import java.net.URISyntaxException;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
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
        FileOutputStream modRsStream = openFile(modRs);
        writeln(modRsStream, "// generated file, do not edit");
        writeln(modRsStream, "");

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
            FileOutputStream os = openFile(testFile);
            writeln(modRsStream, "mod %s;", id);
            writeln(os, "// generated file, do not edit");

            writeln(os, "use std::path::PathBuf;");
            writeln(os, "const MANIFEST_DIR: &str = env!(\"CARGO_MANIFEST_DIR\");");
            writeln(os, "const INPUT_DATA: &str = \"input_data\";");
            writeln(os, "const FILENAME: &str = \"%s\";", t.getFileRename());

            writeln(os, "");
            writeln(os, "fn path() -> PathBuf {");
            writeln(os, "    let p = PathBuf::from(MANIFEST_DIR)");
            writeln(os, "        .join(\"tests\")");
            writeln(os, "        .join(INPUT_DATA)");
            writeln(os, "        .join(FILENAME);");
            writeln(os, "    p.canonicalize()");
            writeln(os, "        .expect(format!(\"bad path: {}\", p.display()).as_str())");
            writeln(os, "}");

            writeln(os, "");
            writeln(os, "#[test]");
            writeln(os, "fn %s() {", id);
            writeln(os, "    let path = path();");
            writeln(os, "    let _doc = exile::load(&path).unwrap();");
            writeln(os, "}");


            closeStream(testFile, os);

            copyXmlTestFile(t, inputData);
        }

        closeStream(modRs, modRsStream);

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

    private static void closeStream(File file, FileOutputStream stream) throws TestGenException {
        try {
            stream.close();
        } catch (IOException e) {
            throw new TestGenException("unable to close file: " + file.getPath(), e);
        }
    }


    private static FileOutputStream openFile(File file) throws TestGenException {
        try {
            return FileUtils.openOutputStream(file);
        } catch (IOException e) {
            throw new TestGenException("could not open for writing: " + file.getPath(), e);
        }
    }


    private static void write(FileOutputStream os, String format, Object... args) throws TestGenException {
        String line = String.format(format, args);
        try {
            os.write(line.getBytes());
            os.flush();
        } catch (IOException e) {
            throw new TestGenException("unable to write to stream: " + os.toString(), e);
        }
    }

    private static void writeln(FileOutputStream os, String format, Object... args) throws TestGenException {
        write(os, format + "\n", args);
    }
}
