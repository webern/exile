package com.matthewjamesbriggs.xmltestgen;

import org.w3c.dom.Element;
import org.w3c.dom.NamedNodeMap;
import org.w3c.dom.Node;

import java.nio.file.Path;


class ConfTest {
    private final ConfTestCases confTestCases;
    private final Path path;
    private final Entities entities;
    private final String id;
    private final String output;
    private final Recommendation recommendation;
    private final String sections;
    private final boolean namespace;
    private final ConfType confType;
    private final boolean isVersion1_1;

    ConfTest(Element element, Path path, ConfTestCases confTestCases) throws TestGenException {
        this.path = path;
        this.confTestCases = confTestCases;
        id = XmlHelpers.getRequiredAttribute(element, "ID");
        String entitiesStr = XmlHelpers.getOptionalAttribute(element, "ENTITIES"); // both, general, none, parameter, <absent>
        entities = Entities.fromString(entitiesStr);
        namespace = XmlHelpers.getOptionalAttribute(element, "NAMESPACE").equals("yes");
        output = XmlHelpers.getOptionalAttribute(element, "OUTPUT");
        String recommendationStr = XmlHelpers.getOptionalAttribute(element, "RECOMMENDATION"); // NS1.0, XML1.0, XML1.0-errata2e, XML1.1
        recommendation = Recommendation.fromString(recommendationStr);
        sections = XmlHelpers.getOptionalAttribute(element, "SECTIONS");
        String typeStr = XmlHelpers.getOptionalAttribute(element, "TYPE"); // error, invalid, not-wf, valid
        confType = ConfType.fromString(typeStr);
        isVersion1_1 = XmlHelpers.getOptionalAttribute(element, "VERSION").equals("1.1");
    }

    @Override
    public String toString() {
        return String.format("ConfTest: %s, Entities: %s, Path: %s, Output: %s, Namespace: %b, ConfType: %s, IsVersion1_1: %b, Sections: %s", id, entities.toString(), path.toString(), output, namespace, confType.toString(), isVersion1_1, sections);
    }
}
