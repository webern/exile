package com.matthewjamesbriggs.xmltestgen;

public class TestGenException extends Exception {
    private static final long serialVersionUID = -617463356193004953L;

    public TestGenException(String message) {
        super(message);
    }

    public TestGenException(String message, Throwable cause) {
        super(message, cause);
    }
}
