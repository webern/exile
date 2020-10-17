package com.matthewjamesbriggs.xmltestgen;

import org.w3c.dom.Element;
import org.w3c.dom.Node;
import org.w3c.dom.NodeList;

import java.util.ArrayList;
import java.util.List;

/**
 * XML generic helper functions.
 */
public class X {
    /**
     * Gets the child elements from an element.
     *
     * @param parent The element to get the children from.
     * @return The child elements.
     * @throws TestGenException if something bad happens.
     */
    public static List<Element> getChildren(Element parent) throws TestGenException {
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
