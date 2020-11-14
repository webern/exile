package com.matthewjamesbriggs.xmltestgen;

import java.util.List;

/**
 * The entrypoint class for a program that generates exile XML tests.
 */
public class App {
    private static final int SUCCESS = 0;
    private static final int FAILURE = 1;
    private static final int BAD_USAGE = 2;
    private static final int NUM_TESTS = 5;

    public static void main(String[] args) {
        ProgramOptions opts = null;
        try {
            opts = ProgramOptions.parse(args);
        } catch (TestGenException e) {
            System.exit(BAD_USAGE);
        }
        try {
            generateConfTests(opts);
        } catch (TestGenException e) {
            System.out.println(e.getMessage());
            e.printStackTrace();
            System.exit(FAILURE);
        }
        System.exit(SUCCESS);
    }

    private static void generateConfTests(ProgramOptions opts) throws TestGenException {
        List<ConfTest> confTests = ConfTestParser.parse(opts.getW3cXml().getPath());
        List<ConfTest> exileTests = ConfTestParser.parseExileTests(opts.getRustDataDir());
        confTests.addAll(exileTests);
        ConfTestGenerator confTestGenerator = new ConfTestGenerator(confTests, opts);
        confTestGenerator.generateTests(NUM_TESTS);
    }
}
