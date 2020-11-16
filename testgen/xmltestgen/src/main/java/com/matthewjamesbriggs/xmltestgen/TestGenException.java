package com.matthewjamesbriggs.xmltestgen;

import java.io.OutputStreamWriter;

public class TestGenException extends Exception {
    private static final long serialVersionUID = -617463356193004953L;

    public TestGenException(String message) {
        super(message);
    }

    public TestGenException(String message, Throwable cause) {
        super(message, cause);
    }

    public TestGenException(Throwable cause, String format, Object... args) {
        super(String.format(format, args), cause);
    }

    public TestGenException(String format, Object... args) {
        super(String.format(format, args));
    }
}
