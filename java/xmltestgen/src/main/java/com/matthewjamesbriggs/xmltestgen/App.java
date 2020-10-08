package com.matthewjamesbriggs.xmltestgen;

import org.apache.xerces.parsers.DOMParser;
import org.apache.xerces.xni.parser.XMLInputSource;
import org.w3c.dom.Document;
import org.w3c.dom.Node;
import org.w3c.dom.NodeList;
import org.xml.sax.SAXException;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.FactoryConfigurationError;
import javax.xml.parsers.ParserConfigurationException;
import java.io.IOException;

public class App {
    private static Object SAXException;

    public static void main(String[] args) {
        final String xmlFile = "file:///Users/mjb/repos/mx/Documents/musicxml.xsd";
        try {
            DocumentBuilderFactory factory =
                    DocumentBuilderFactory.newInstance();
            DocumentBuilder builder = factory.newDocumentBuilder();
            Document document = builder.parse(xmlFile);
            NodeList childNodes = document.getChildNodes();
            if (childNodes.getLength() != 1) {
                System.err.println("No root node.");
                System.exit(1);
            } else {
                Node node = childNodes.item(0);
                System.out.println(node.getNodeName());
            }
        } catch (FactoryConfigurationError e) {
            // unable to get a document builder factory
            System.out.println(e.getMessage());
        } catch (ParserConfigurationException e) {
            // parser was unable to be configured
            System.out.println(e.getMessage());
        } catch (SAXException e) {
            // parsing error
            System.out.println(e.getMessage());
        } catch (IOException e) {
            // i/o error
            System.out.println(e.getMessage());
        }
        System.out.println("Hello World!");
    }
}
