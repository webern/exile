package com.matthewjamesbriggs.xmltestgen;

import lombok.Getter;
import org.w3c.dom.DocumentType;
import org.w3c.dom.Node;
import org.w3c.dom.Text;

/**
 * Realizes all of the data fields of an XML text node so that we can see the values.
 */
public class XText {
    @Getter
    private final String parentNodeName;
    @Getter
    private final String parentLocalName;
    @Getter
    private final XType parentNodeType;
    @Getter
    private final String data;
    @Getter
    private final String wholeText;
    @Getter
    private final String textContent;
    @Getter
    private final int numChildNodes;
    @Getter
    private final int length;
    @Getter
    private final XType xtype;
    @Getter
    private final String nodeValue;
    @Getter
    private final boolean isElementContentWhitespace;
    @Getter
    private final DocumentType docType;

    public XText(Text text) {
        Node parent = text.getParentNode();
        if (parent != null) {
            parentNodeName = parent.getNodeName();
            parentLocalName = parent.getLocalName();
            parentNodeType = XType.fromNode(parent);
        } else {
            parentNodeName = "";
            parentLocalName = "";
            parentNodeType = XType.Unknown;
        }
        data = text.getData();
        wholeText = text.getWholeText();
        textContent = text.getTextContent();
        numChildNodes = text.getChildNodes().getLength();
        length = text.getLength();
        xtype = XType.fromNode(text);
        nodeValue = text.getNodeValue();
        isElementContentWhitespace = text.isElementContentWhitespace();
        docType = text.getOwnerDocument().getDoctype();
    }
}
