package com.matthewjamesbriggs.xmltestgen;

import org.w3c.dom.Element;
import lombok.Getter;

import java.nio.file.Path;


class ConfTest {
    @Getter
    private final ConfTestCases confTestCases;
    @Getter
    private final Path path;
    @Getter
    private final Entities entities;
    @Getter
    private final String id;
    @Getter
    private final String output;
    @Getter
    private final Recommendation recommendation;
    @Getter
    private final String sections;
    @Getter
    private final boolean namespace;
    @Getter
    private final ConfType confType;
    @Getter
    private final boolean isVersion1_1;

    ConfTest(Element element, Path path, ConfTestCases confTestCases) throws TestGenException {
        this.path = path;
        this.confTestCases = confTestCases;
        id = XmlHelpers.getRequiredAttribute(element, "ID");
        String entitiesStr =
                XmlHelpers.getOptionalAttribute(element, "ENTITIES"); // both, general, none, parameter, <absent>
        entities = Entities.fromString(entitiesStr);
        namespace = XmlHelpers.getOptionalAttribute(element, "NAMESPACE").equals("yes");
        output = XmlHelpers.getOptionalAttribute(element, "OUTPUT");
        String recommendationStr =
                XmlHelpers.getOptionalAttribute(element, "RECOMMENDATION"); // NS1.0, XML1.0, XML1.0-errata2e, XML1.1
        recommendation = Recommendation.fromString(recommendationStr);
        sections = XmlHelpers.getOptionalAttribute(element, "SECTIONS");
        String typeStr = XmlHelpers.getOptionalAttribute(element, "TYPE"); // error, invalid, not-wf, valid
        confType = ConfType.fromString(typeStr);
        isVersion1_1 = XmlHelpers.getOptionalAttribute(element, "VERSION").equals("1.1");
    }

    @Override
    public String toString() {
        return String.format(
                "ConfTest: %s, Entities: %s, Path: %s, Output: %s, Namespace: %b, ConfType: %s, IsVersion1_1: %b, Sections: %s",
                id,
                entities.toString(),
                path.toString(),
                output,
                namespace,
                confType.toString(),
                isVersion1_1,
                sections);
    }
}
