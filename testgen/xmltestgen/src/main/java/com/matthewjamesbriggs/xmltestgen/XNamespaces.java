package com.matthewjamesbriggs.xmltestgen;

public enum XNamespaces {
    ON, OFF;

    public static XNamespaces fromBoolean(boolean isNamespacesOn) {
        if (isNamespacesOn) {
            return ON;
        } else {
            return OFF;
        }
    }
}
