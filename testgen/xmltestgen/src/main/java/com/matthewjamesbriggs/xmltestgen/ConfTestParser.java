package com.matthewjamesbriggs.xmltestgen;

import org.w3c.dom.Document;
import org.w3c.dom.Element;
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

class ConfTestParser {
    static List<ConfTest> parse(String w3cXmlFilepath) throws TestGenException {
        Document doc = X.loadComplete(new File(w3cXmlFilepath));
        return parseDocument(doc);
    }

    private static List<ConfTest> parseDocument(Document document) throws TestGenException {
        Element root = document.getDocumentElement();
        List<Element> children = X.getChildren(root);
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
        List<Element> children = X.getChildren(element);
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
                throw new TestGenException("this does not exist: " + basePathFileObject.getPath());
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
}
