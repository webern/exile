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
import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
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

        // create the mod.rs file
        File modRs = new File(dir, "mod.rs");
        createFile(modRs);
        FileOutputStream modRsStream = openFile(modRs);
        write(modRsStream, "// Hello %s", "World");

        // create NUM_TESTS test files
        int testCount = 0;
        for (int i = 0; i < confTests.size() && testCount < NUM_TESTS; ++i) {
            ConfTest t = confTests.get(i);
            if (t.getConfType() != ConfType.Valid) {
                continue;
            }
            ++testCount;
        }

        closeStream(modRs, modRsStream);

        // run rust fmt
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
        //        System.out.println(root.getTagName());
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
        //        System.out.println("TESTCASES - " + profile);
        List<Element> children = getChildren(element);
        for (Element child : children) {
            if (child.getTagName().equals("TESTCASES")) {
                parseTestCases(child, outConfTests);
            } else if (child.getTagName().equals("TEST")) {
                outConfTests.add(parseTest(child));
            } else {
                throw new TestGenException("Expected TESTCASES or TEST, got " + child.getTagName());
            }
        }
    }

    private static ConfTest parseTest(Element element) throws TestGenException {
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
            // System.out.println(element.getTagName() + " - " + id + " - " + filepath);
            ConfTest confTest = new ConfTest(element, filepath, new ConfTestCases());
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
