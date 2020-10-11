package com.matthewjamesbriggs.xmltestgen;

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

    public static void main(String[] args) {
        ProgramOptions opt = null;
        try {
            opt = ProgramOptions.parse(args);
        } catch (TestGenException e) {
            System.exit(BAD_USAGE);
        }
        try {
            Document document = loadXconf(opt.getW3cXml().getPath());
            doThings(document);
        } catch (TestGenException e) {
            System.out.println(e.getMessage());
            e.printStackTrace();
            System.exit(FAILURE);
        }
        System.exit(SUCCESS);
    }

    private static void doThings(Document document) throws TestGenException {
        Element root = document.getDocumentElement();
        System.out.println(root.getTagName());
        List<Element> children = getChildren(root);
        for (Element child : children) {
            if (!child.getTagName().equals("TESTCASES")) {
                throw new TestGenException("Expected TESTCASES, got " + child.getTagName());
            }
            parseTestCases(child);
        }
    }

    private static void parseTestCases(Element element) throws TestGenException {
        String profile = element.getAttribute("PROFILE");
        System.out.println("TESTCASES - " + profile);
        List<Element> children = getChildren(element);
        for (Element child : children) {
            if (child.getTagName().equals("TESTCASES")) {
                parseTestCases(child);
            } else if (child.getTagName().equals("TEST")) {
                parseTest(child);
            } else {
                throw new TestGenException("Expected TESTCASES or TEST, got " + child.getTagName());
            }
        }
    }

    private static void parseTest(Element element) throws TestGenException {
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
            System.out.println(confTest.toString());
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
