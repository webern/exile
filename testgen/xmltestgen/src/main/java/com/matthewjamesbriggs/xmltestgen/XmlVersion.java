package com.matthewjamesbriggs.xmltestgen;

public enum XmlVersion {
    V10, V11;

    public static final String S10 = "1.0";
    public static final String S11 = "1.1";

    public static XmlVersion fromString(String s) throws TestGenException {
        switch (s) {
            case S10:
                return V10;
            case S11:
                return V11;
            default:
                throw new TestGenException("unknown xml version " + s);
        }
    }

    @Override
    public String toString() {
        switch (this) {
            case V11:
                return S11;
            case V10:
            default:
                return S10;
        }
    }
}
