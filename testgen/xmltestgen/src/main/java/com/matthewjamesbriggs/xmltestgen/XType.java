package com.matthewjamesbriggs.xmltestgen;

import org.w3c.dom.Node;

/// Use an enum instead of an integer for XML Node type.
enum XType {
    Element(Node.ELEMENT_NODE, "element"),
    Attribute(Node.ATTRIBUTE_NODE, "Attribute"),
    Text(Node.TEXT_NODE, "Text"),
    CData(Node.CDATA_SECTION_NODE, "CData"),
    EntityReference(Node.ENTITY_REFERENCE_NODE, "EntityReference"),
    Entity(Node.ENTITY_NODE, "Entity"),
    ProcessingInstruction(Node.PROCESSING_INSTRUCTION_NODE, "ProcessingInstruction"),
    Comment(Node.COMMENT_NODE, "Comment"),
    Document(Node.DOCUMENT_NODE, "Document"),
    DocumentType(Node.DOCUMENT_TYPE_NODE, "DocumentType"),
    DocumentFragment(Node.DOCUMENT_FRAGMENT_NODE, "DocumentFragment"),
    Notation(Node.NOTATION_NODE, "Notation"),
    Unknown((short) -1, "unknown");

    /// This is the NodeType integer from the Java XML Node class.
    private final short nodeType;

    /// This is the name for toString calls;
    private final String name;

    XType(short nodeType, String name) {
        this.nodeType = nodeType;
        this.name = name;
    }

    @Override
    public String toString() {
        return name;
    }

    /// Returns the short value as defined by org.w3c.dom.Node
    public short getNodeType() {
        return nodeType;
    }

    public static XType fromNode(Node node) {
        short nodeType = node.getNodeType();
        for (XType xType : XType.values()) {
            if (xType.getNodeType() == nodeType) {
                return xType;
            }
        }
        return XType.Unknown;
    }
}
