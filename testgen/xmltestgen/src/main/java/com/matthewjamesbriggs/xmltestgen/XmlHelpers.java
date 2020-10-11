package com.matthewjamesbriggs.xmltestgen;

import com.matthewjamesbriggs.xmltestgen.TestGenException;
import org.w3c.dom.Element;

class XmlHelpers {
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
}
