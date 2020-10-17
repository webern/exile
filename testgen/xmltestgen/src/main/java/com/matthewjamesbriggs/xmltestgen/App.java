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
            Document document = loadXconf(opts.getW3cXml().getPath());
            doThings(document, opts);
        } catch (TestGenException e) {
            System.out.println(e.getMessage());
            e.printStackTrace();
            System.exit(FAILURE);
        }
        System.exit(SUCCESS);
    }

    private static void doThings(Document document, ProgramOptions opts) throws TestGenException {
        List<ConfTest> confTests = parseConfTests(document);

        // delete and recreate the directory named "generated"
        File dir = new File(opts.getXmlOutdir(), "generated");
        createOrReplaceDir(dir);
        dir = canonicalize(dir);
        File inputData = new File(opts.getXmlOutdir(), "input_data");
        createOrReplaceDir(inputData);
        inputData = canonicalize(inputData);
        File rustRoot = new File(opts.getRustRoot().getPath());
        checkDir(rustRoot);
        canonicalize(rustRoot);

        // create the mod.rs file
        File modRs = new File(dir, "mod.rs");
        modRs = canonicalize(modRs);
        createFile(modRs);
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
            createFile(testFile);
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
        exileCrate = canonicalize(exileCrate);
        File manifestPath = new File(exileCrate, "Cargo.toml");
        manifestPath = canonicalize(manifestPath);
        checkFile(manifestPath);
        Process process;
        try {
            process = Runtime.getRuntime().exec("cargo fmt --manifest-path " + manifestPath.getPath(), null, rustRoot);
        } catch (IOException e) {
            throw new TestGenException("cargo fmt did not work", e);
        }
        int exitCode = 0;
        try {
            exitCode = process.waitFor();
        } catch (InterruptedException e) {
            throw new TestGenException("process interrupted", e);
        }
        if (exitCode != 0) {
            String processOutput = getStdErr(process);
            throw new TestGenException(String.format("cargo fmt failed with exit: %d\n%s", exitCode, processOutput));
        } else {
            String processOutput = getStdOut(process);
            System.out.println(processOutput);
        }
    }

    private static File canonicalize(File path) throws TestGenException {
        try {
            return new File(path.getCanonicalFile().getPath());
        } catch (IOException e) {
            throw new TestGenException("unable to cannonicalize path, " + path.getPath() + ": " + e.getMessage());
        }
    }

    private static void copyXmlTestFile(ConfTest t, File copyToDir) throws TestGenException {
        checkDir(copyToDir);
        File original = new File(t.getPath().toString());
        checkFile(original);
        File copied = new File(copyToDir, t.getFileRename());
        try {
            FileUtils.copyFile(original, copied);
        } catch (IOException e) {
            throw new TestGenException("unable to copy file from: " + original.getPath() + ", to: " + copied.getPath());
        }
    }

    private static void checkFile(File file) throws TestGenException {
        if (!file.exists()) {
            throw new TestGenException("file does not exist: " + file.getPath());
        }
        if (!file.isFile()) {
            throw new TestGenException("not a file: " + file.getPath());
        }
    }

    private static void checkDir(File dir) throws TestGenException {
        if (!dir.exists()) {
            throw new TestGenException("dir does not exist: " + dir.getPath());
        }
        if (!dir.isDirectory()) {
            throw new TestGenException("not a dir: " + dir.getPath());
        }
    }

    private static String getStdErr(Process process) throws TestGenException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(process.getErrorStream()));
        StringBuilder everything = new StringBuilder();
        String line = "";
        try {
            while ((line = reader.readLine()) != null) {
                everything.append(line);
                everything.append('\n');
            }
        } catch (IOException e) {
            throw new TestGenException("proccess results could not be read", e);
        }
        return everything.toString();
    }

    private static String getStdOut(Process process) throws TestGenException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()));
        StringBuilder everything = new StringBuilder();
        String line = "";
        try {
            while ((line = reader.readLine()) != null) {
                everything.append(line);
                everything.append('\n');
            }
        } catch (IOException e) {
            throw new TestGenException("proccess results could not be read", e);
        }
        return everything.toString();
    }

    private static void closeStream(File file, FileOutputStream stream) throws TestGenException {
        try {
            stream.close();
        } catch (IOException e) {
            throw new TestGenException("unable to close file: " + file.getPath(), e);
        }
    }

    private static void createFile(File file) throws TestGenException {
        if (file.exists()) {
            if (file.isFile()) {
                FileUtils.deleteQuietly(file);
            } else if (file.isDirectory()) {
                try {
                    FileUtils.deleteDirectory(file);
                } catch (IOException e) {
                    throw new TestGenException("a directory could not be deleted: " + file, e);
                }
            } else {
                throw new TestGenException("something exists but i don't know what: " + file);
            }
        }
        try {
            if (!file.createNewFile()) {
                throw new TestGenException("file already exists: " + file.getPath());
            }
        } catch (IOException e) {
            throw new TestGenException("unable to create file: " + file.getPath(), e);
        }
    }

    private static FileOutputStream openFile(File file) throws TestGenException {
        try {
            return FileUtils.openOutputStream(file);
        } catch (IOException e) {
            throw new TestGenException("could not open for writing: " + file.getPath(), e);
        }
    }

    private static void createOrReplaceDir(File directory) throws TestGenException {
        try {
            directory = new File(directory.getCanonicalPath());
        } catch (IOException e) {
            throw new TestGenException("unable to canonicalize: " + directory.getPath());
        }
        if (directory.exists() && directory.isDirectory()) {
            try {
                FileUtils.deleteDirectory(directory);
            } catch (IOException e) {
                throw new TestGenException("unable to delete dir: " + directory.getPath());
            }
        } else if (directory.exists() && directory.isFile()) {
            FileUtils.deleteQuietly(directory);
        }
        try {
            FileUtils.forceMkdir(directory);
        } catch (IOException e) {
            throw new TestGenException("unable to create directory: " + directory.getPath());
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

    private static List<ConfTest> parseConfTests(Document document) throws TestGenException {
        Element root = document.getDocumentElement();
        List<Element> children = getChildren(root);
        List<ConfTest> confTests = new ArrayList<>();
        for (Element child : children) {
            if (!child.getTagName().equals("TESTCASES")) {
                throw new TestGenException("Expected TESTCASES, got " + child.getTagName());
            }
            parseTestCases(child, confTests);
        }

        return confTests;
    }

    private static void parseTestCases(Element element, List<ConfTest> outConfTests) throws TestGenException {
        String profile = element.getAttribute("PROFILE");
        String prefix = "unknown";
        if (profile.equals("James Clark XMLTEST cases, 18-Nov-1998")) {
            prefix = "jclark";
        } else if (profile.equals("Fuji Xerox Japanese Text Tests")) {
            prefix = "xjapan";
        } else if (profile.equals("Sun Microsystems XML Tests")) {
            prefix = "sun";
        } else if (profile.equals("OASIS/NIST TESTS, 1-Nov-1998")) {
            prefix = "nist";
        } else if (profile.equals("IBM XML Tests")) {
            prefix = "ibm";
        } else if (profile.equals("IBM XML Conformance Test Suite - invalid tests")) {
            prefix = "ibminv";
        } else if (profile.equals("IBM XML Conformance Test Suite - not-wf tests")) {
            prefix = "ibmnotwf";
        } else if (profile.startsWith("IBM XML Conformance Test Suite - Production ")) {
            prefix = "ibmprod";
        } else if (profile.equals("IBM XML Conformance Test Suite - valid tests")) {
            prefix = "ibmval";
        } else if (profile.equals("IBM Invalid Conformance Tests for XML 1.1 CR October 15, 2002")) {
            prefix = "ibm11";
        } else if (profile.equals("IBM XML Conformance Test Suite")) {
            prefix = "ibmconf";
        } else if (profile.equals("IBM Not-WF Conformance Tests for XML 1.1 CR October 15, 2002")) {
            prefix = "ibmnw11";
        } else if (profile.equals("IBM Valid Conformance Tests for XML 1.1 CR October 15, 2002")) {
            prefix = "ibmval11";
        } else if (profile.equals("Richard Tobin's XML 1.0 2nd edition errata test suite 21 Jul 2003")) {
            prefix = "edunierr";
        } else if (profile.equals("Richard Tobin's XML 1.1 test suite 13 Feb 2003")) {
            prefix = "eduni11";
        } else if (profile.equals("Richard Tobin's XML Namespaces 1.0 test suite 14 Feb 2003")) {
            prefix = "edunins10";
        } else if (profile.equals("Richard Tobin's XML Namespaces 1.1 test suite 14 Feb 2003")) {
            prefix = "edunins11";
        } else {
            throw new TestGenException("unknown profile: " + profile);
        }
        ConfTestCases confTestCases = new ConfTestCases(prefix, profile);
        List<Element> children = getChildren(element);
        for (Element child : children) {
            if (child.getTagName().equals("TESTCASES")) {
                parseTestCases(child, outConfTests);
            } else if (child.getTagName().equals("TEST")) {
                outConfTests.add(parseTest(child, confTestCases));
            } else {
                throw new TestGenException("Expected TESTCASES or TEST, got " + child.getTagName());
            }
        }
    }

    private static ConfTest parseTest(Element element, ConfTestCases confTestCases) throws TestGenException {
        try {
            String id = element.getAttribute("ID");
            URI baseUri = new URI(element.getBaseURI());
            Path basePath = Paths.get(baseUri);
            // a) in most cases, an `xml:base` is not specified, in which case baseUri is the current XML filepath.
            // b) in other cases, an `xml:base` attribute is given, providing the parent of the current XML filepath.
            // for case a, we need to get the parent before proceeding so that we can join with the test file paths.
            File basePathFileObject = new File(basePath.toString());
            if (basePathFileObject.exists() && basePathFileObject.isFile()) {
                basePath = basePath.getParent();
            } else if (!basePathFileObject.exists()) {
                throw new TestGenException("we're fucked because this does not exist: " + basePathFileObject.getPath());
            }
            String uriAttribute = element.getAttribute("URI");
            if (uriAttribute == null) {
                throw new TestGenException("URI attribute not found for test " + id);
            }
            Path relativeFilepath = Paths.get(uriAttribute);
            Path filepath = basePath.resolve(relativeFilepath);
            File f = new File(filepath.toString());
            if (!f.exists()) {
                throw new TestGenException("File does not exist for " + id + ", " + filepath.toString());
            } else if (f.isDirectory()) {
                throw new TestGenException("Directory instead of file for " + id + ", " + filepath.toString());
            }
            ConfTest confTest = new ConfTest(element, filepath, confTestCases);
            return confTest;
        } catch (URISyntaxException e) {
            throw new TestGenException("malformed uri " + element.getBaseURI(), e);
        }
    }

    private static List<Element> getChildren(Element parent) throws TestGenException {
        NodeList nodeList = parent.getChildNodes();
        List<Element> children = new ArrayList<>();
        for (int i = 0; i < nodeList.getLength(); ++i) {
            Node node = nodeList.item(i);
            if (node instanceof Element) {
                Element element = ((Element) node);
                children.add(element);
            }
        }
        return children;
    }

    private static Document loadXconf(String w3cXmlFilepath) throws TestGenException {
        File xmlConfFile = new File(w3cXmlFilepath);
        if (!xmlConfFile.exists()) {
            throw new TestGenException("this path does not exist: " + w3cXmlFilepath);
        } else if (!xmlConfFile.isFile()) {
            throw new TestGenException("this path is not a file: " + w3cXmlFilepath);
        }

        String dir = "";
        try {
            dir = xmlConfFile.getCanonicalPath();
        } catch (IOException e) {
            throw new TestGenException(String.format("Unable to find canonical form of %s", w3cXmlFilepath));
        }

        String uri = String.format("file://%s", dir);

        try {
            DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
            DocumentBuilder builder = factory.newDocumentBuilder();
            Document document = builder.parse(uri);
            return document;
        } catch (FactoryConfigurationError e) {
            throw new TestGenException("unable to get a document builder factory", e);
        } catch (ParserConfigurationException e) {
            throw new TestGenException("parser was unable to be configured", e);
        } catch (SAXException e) {
            throw new TestGenException("parsing error", e);
        } catch (IOException e) {
            throw new TestGenException("i/o error", e);
        } catch (Throwable t) {
            throw new TestGenException("weird error", t);
        }
    }
}
