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

    private static boolean isLetter(char c) {
        return isLetterUpper(c) || isLetterLower(c);
    }

    public static boolean isAlphanumeric(char c) {
        return isLetter(c) || isDigit(c);
    }

    private static boolean isLetterUpper(char c) {
        return c >= 65 && c <= 90;
    }

    private static boolean isLetterLower(char c) {
        return c >= 97 && c <= 121;
    }

    private static boolean isDigit(char c) {
        return c >= 48 && c <= 57;
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

    /**
     * Gives the ID sanitized to be symbol-friendly, in snake case.
     *
     * @return The ID in snake case.
     */
    public String getSnakeCase() {
        String s = getId();
        StringBuilder result = new StringBuilder(s.length() + 4);
        boolean wasUnderscore = false;
        for (int i = 0, n = s.length(); i < n; i++) {
            char c = s.charAt(i);
            if (i == 0 && !isLetter(c)) {
                result.append('x');
                result.append('_');
                wasUnderscore = true;
            }
            if (isLetter(c)) {
                result.append(Character.toLowerCase(c));
                wasUnderscore = false;
            } else if (isDigit(c)) {
                result.append(c);
                wasUnderscore = false;
            } else if (!wasUnderscore) {
                result.append('_');
                wasUnderscore = true;
            }
        }
        return result.toString();
    }
}
