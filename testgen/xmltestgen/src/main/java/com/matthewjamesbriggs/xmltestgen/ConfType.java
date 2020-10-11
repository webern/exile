package com.matthewjamesbriggs.xmltestgen;

// type = XmlHelpers.getOptionalAttribute(element, "TYPE"); // error, invalid, not-wf, valid
public enum ConfType {
    Error, Invalid, NotWellFormed, Valid;

    public static ConfType fromString(String s) throws TestGenException {
        switch (s) {
            case "error":
                return Error;
            case "invalid":
                return Error;
            case "not-wf":
                return Error;
            case "valid":
                return Error;
            default:
                throw new TestGenException("unknown conf type " + s);
        }
    }

    @Override
    public String toString() {
        switch (this) {
            case Error:
                return "error";
            case Invalid:
                return "invalid";
            case NotWellFormed:
                return "not-wf";
            case Valid:
                return "valid";
            default:
                return "ERROR";
        }
    }
}
