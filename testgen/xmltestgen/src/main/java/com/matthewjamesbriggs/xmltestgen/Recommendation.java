package com.matthewjamesbriggs.xmltestgen;

public enum Recommendation {
    NS1_0,   /// NS1.0
    NS1_1,   /// NS1.1
    XML1_0,  /// XML1.0
    XML1_0E, /// XML1.0-errata2e
    XML1_1;  /// XML1.1

    public static Recommendation fromString(String s) throws TestGenException {
        switch (s) {
            case "NS1.0":
                return NS1_0;
            case "NS1.1":
                return NS1_1;
            case "XML1.0":
                return XML1_0;
            case "XML1.0-errata2e":
                return XML1_0E;
            case "XML1.1":
                return XML1_1;
            default:
                throw new TestGenException("unknown recommendation " + s);
        }
    }

    @Override
    public String toString() {
        switch (this) {
            case NS1_0:
                return "NS1.0";
            case NS1_1:
                return "NS1.1";
            case XML1_0:
                return "XML1.0";
            case XML1_0E:
                return "XML1.0-errata2e";
            case XML1_1:
                return "XML1.1";
            default:
                return "ERROR";
        }
    }
}
