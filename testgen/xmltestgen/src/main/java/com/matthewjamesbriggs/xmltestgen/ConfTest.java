package com.matthewjamesbriggs.xmltestgen;

import lombok.AllArgsConstructor;
import org.apache.commons.io.FilenameUtils;
import org.w3c.dom.Element;
import lombok.Getter;

import java.io.File;
import java.nio.file.Path;


@AllArgsConstructor class ConfTest {
    private static final String SEPARATOR = "_";
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
    private final XmlVersion xmlVersion;
    @Getter
    private final String prefix;

    ConfTest(Element element, Path path, ConfTestCases confTestCases) throws TestGenException {
        this.path = path;
        this.confTestCases = confTestCases;
        id = X.getRequiredAttribute(element, "ID");
        String entitiesStr = X.getOptionalAttribute(element, "ENTITIES"); // both, general, none, parameter, <absent>
        entities = Entities.fromString(entitiesStr);
        namespace = X.getOptionalAttribute(element, "NAMESPACE").equals("yes");
        output = X.getOptionalAttribute(element, "OUTPUT");
        String recommendationStr =
                X.getOptionalAttribute(element, "RECOMMENDATION"); // NS1.0, XML1.0, XML1.0-errata2e, XML1.1
        recommendation = Recommendation.fromString(recommendationStr);
        sections = X.getOptionalAttribute(element, "SECTIONS");
        String typeStr = X.getOptionalAttribute(element, "TYPE"); // error, invalid, not-wf, valid
        confType = ConfType.fromString(typeStr);
        if (X.getOptionalAttribute(element, "VERSION").equals("1.1")) {
            xmlVersion = XmlVersion.V11;
        } else {
            xmlVersion = XmlVersion.V10;
        }
        prefix = confTestCases.getPrefix();
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
                "ConfTest: %s, Entities: %s, Path: %s, Output: %s, Namespace: %b, ConfType: %s, XmlVersion: %s, Sections: %s",
                id,
                entities.toString(),
                path.toString(),
                output,
                namespace,
                confType.toString(),
                xmlVersion.toString(),
                sections);
    }

    /**
     * Gives the ID sanitized to be symbol-friendly, in snake case.
     *
     * @return The ID in snake case.
     */
    String getSnakeCase() {
        String s = getId();
        StringBuilder result = new StringBuilder(s.length() + 4);
        boolean wasUnderscore = false;
        for (int i = 0, n = s.length(); i < n; i++) {
            char c = s.charAt(i);
            if (i == 0 && !isLetter(c)) {
                result.append('x');
                result.append(SEPARATOR);
                wasUnderscore = true;
            }
            if (isLetter(c)) {
                result.append(Character.toLowerCase(c));
                wasUnderscore = false;
            } else if (isDigit(c)) {
                result.append(c);
                wasUnderscore = false;
            } else if (!wasUnderscore) {
                result.append(SEPARATOR);
                wasUnderscore = true;
            }
        }
        return result.toString();
    }

    String getTestName() {
        return getPrefix() + SEPARATOR + getSnakeCase();
    }

    String getFileRename() {
        return getTestName() + ".xml";
    }

    /**
     * Returns true if this test is a 'custom' exile test, i.e. an exile test, and did not come from W3C.
     */
    boolean isExileTest() {
        return getPrefix().equals(ExileFileNames.EXILE_PREFIX);
    }
}
