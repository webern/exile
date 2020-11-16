package com.matthewjamesbriggs.xmltestgen;

public enum Entities {
    Both, General, None, Parameter;

    public static Entities fromString(String s) throws TestGenException {
        switch (s) {
            case "both":
                return Both;
            case "general":
                return General;
            case "":
            case "none":
                return None;
            case "parameter":
                return Parameter;
        }
        throw new TestGenException("unknown entities type " + s);
    }
}
