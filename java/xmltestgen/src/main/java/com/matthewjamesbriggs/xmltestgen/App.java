package com.matthewjamesbriggs.xmltestgen;

import org.apache.xerces.parsers.DOMParser;
import org.apache.xerces.xni.parser.XMLInputSource;
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
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

// http://www.java2s.com/Tutorials/Java/XML/How_to_get_root_element_from_Java_DOM_parser.htm

public class App {
    //    private static Object SAXException;

    public static void main(String[] args) {

        if (args.length != 2) {
            System.err.println("--testdata path is required.");
            System.out.println("Example usage: ./testgen --testdata /path/to/dir");
            System.exit(255);
        }
        String p = args[1];
        File f = new File(System.getProperty("user.dir"), p);
        String dir = "";
        try {
            dir = f.getCanonicalPath();
        } catch (IOException e) {
            System.out.println(String.format("Unable to find directory %s from parent %s", p, System.getProperty("user.dir")));
            System.exit(254);
        }
        String path = String.format("%s/xmlconf/xmlconf.xml", dir);
        String uri = String.format("file://%s", path);

        //        String pathToTestData = String.format("file://%s/xmlconf/xmlconf.xml", p);
        try {
            DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
            DocumentBuilder builder = factory.newDocumentBuilder();
            Document document = builder.parse(uri);
            doThings(document);
        } catch (TestGenException e) {
            // unable to do things
            System.out.println(e.getMessage());
            System.exit(250);
        } catch (FactoryConfigurationError e) {
            // unable to get a document builder factory
            System.out.println(e.getMessage());
            System.exit(249);
        } catch (ParserConfigurationException e) {
            // parser was unable to be configured
            System.out.println(e.getMessage());
            System.exit(248);
        } catch (SAXException e) {
            // parsing error
            System.out.println(e.getMessage());
            System.exit(247);
        } catch (IOException e) {
            // i/o error
            System.out.println(e.getMessage());
            System.exit(246);
        }
        System.out.println("Hello World!");
    }

    private static void doThings(Document document) throws TestGenException {
        Element root = document.getDocumentElement();
        System.out.println(root.getTagName());
        List<Element> children = getChildren(root);
        for (Element child : children) {
            System.out.println(child.getTagName());
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
}
