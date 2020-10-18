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
import java.util.ArrayList;
import java.util.List;

/**
 * XML generic helper functions.
 */
class X {
    /**
     * Gets the child elements from an element.
     * <p>
     * Note, there is some helpful information here:
     * <a href="http://www.java2s.com/Tutorials/Java/XML/How_to_get_root_element_from_Java_DOM_parser.htm">info</a>
     *
     * @param parent The element to get the children from.
     * @return The child elements.
     * @throws TestGenException if something bad happens.
     */
    static List<Element> getChildren(Element parent) throws TestGenException {
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

    static String getRequiredAttribute(Element element, String key) throws TestGenException {
        String value = element.getAttribute(key);
        if (value == null) {
            throw new TestGenException("no attribute named " + key);
        } else if (value.isEmpty()) {
            throw new TestGenException("empty value for attribute " + key);
        }
        return value;
    }

    static String getOptionalAttribute(Element element, String key) {
        String value = element.getAttribute(key);
        if (value == null) {
            return "";
        } else if (value.isEmpty()) {
            return "";
        }
        return value;
    }

    /**
     * Loads an XML Document following entity references.
     *
     * @param file
     * @return
     * @throws TestGenException
     */
    static Document loadComplete(File file) throws TestGenException {
        file = F.canonicalize(file);
        F.checkFile(file);
        String uri = file.toPath().toUri().toString();

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
