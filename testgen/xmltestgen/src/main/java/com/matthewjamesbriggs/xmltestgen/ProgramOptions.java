package com.matthewjamesbriggs.xmltestgen;

import lombok.Getter;
import org.apache.commons.cli.*;

import java.io.File;

class ProgramOptions {
    private static final String OPT_W3C_XML = "w3c-xml";
    private static final String OPT_W3C_SCHEMA = "w3c-schema";
    private static final String OPT_CUSTOM_XML = "custom-xml";
    private static final String OPT_CUSTOM_SCHEMA = "custom-schema";
    private static final String OPT_XML_OUTDIR = "xml-outdir";
    private static final String OPT_SCHEMA_OUTDIR = "schema-outdir";
    private static final String OPT_RUST_ROOT = "rust-root";
    private static final String OPT_EXILE_TESTS = "exile-tests";

    @Getter
    private final File w3cXml;
    @Getter
    private final File w3cSchema;
    @Getter
    private final File customXml;
    @Getter
    private final File customSchema;
    @Getter
    private final File xmlOutdir;
    @Getter
    private final File schemaOutdir;
    @Getter
    private final File rustRoot;
    @Getter
    private final File exileTests;

    /**
     * Private all-args constructor. Use the static parse function to construct from command line arguments.
     *
     * @param w3cXml:       the path to xmlconf/xmlconf.xml
     * @param w3cSchema:    the path to xmlschema/suite.xml
     * @param customXml:    the path to custom xml test definitions file.
     * @param customSchema: the path to custom schema test definitions file.
     * @param xmlOutdir:    the directory into which rust xml tests will be written.
     * @param schemaOutdir: the directory into which rust schema tests will be written.
     * @param rustRoot:     the path to the root of the Rust workspace containing the exile crate.
     * @param exileTests:   the path to the to the custom XML tests created for the exile library.
     */
    private ProgramOptions(File w3cXml,
                           File w3cSchema,
                           File customXml,
                           File customSchema,
                           File xmlOutdir,
                           File schemaOutdir,
                           File rustRoot,
                           File exileTests) {
        this.w3cXml = w3cXml;
        this.w3cSchema = w3cSchema;
        this.customXml = customXml;
        this.customSchema = customSchema;
        this.xmlOutdir = xmlOutdir;
        this.schemaOutdir = schemaOutdir;
        this.rustRoot = rustRoot;
        this.exileTests = exileTests;
    }

    /**
     * Parses the command line arguments.
     *
     * @param args the command line arguments.
     * @return true if successful.
     * @throws TestGenException if the parsing fails or any of the named files or directories do not exist.
     */
    static ProgramOptions parse(String[] args) throws TestGenException {
        Options options = new Options();

        Option wx = new Option("a", OPT_W3C_XML, true, "path to w3c xmlconf.xml");
        wx.setRequired(true);
        options.addOption(wx);

        Option ws = new Option("b", OPT_W3C_SCHEMA, true, "path to w3c schema.xml");
        ws.setRequired(true);
        options.addOption(ws);

        Option cx = new Option("c", OPT_CUSTOM_XML, true, "path to my own parser tests.xml");
        cx.setRequired(true);
        options.addOption(cx);

        Option cs = new Option("d", OPT_CUSTOM_SCHEMA, true, "path to my own schema tests.xml");
        cs.setRequired(true);
        options.addOption(cs);

        Option ox = new Option("e", OPT_XML_OUTDIR, true, "directory to write rust parser xml tests");
        ox.setRequired(true);
        options.addOption(ox);

        Option os = new Option("f", OPT_SCHEMA_OUTDIR, true, "directory to write rust schema xml tests");
        os.setRequired(true);
        options.addOption(os);

        Option rr = new Option("g", OPT_RUST_ROOT, true, "directory the Rust workspace's Cargo.toml");
        rr.setRequired(true);
        options.addOption(rr);

        Option exile = new Option("h",
                OPT_EXILE_TESTS,
                true,
                "directory where the custom XML tests, created for the exile library, reside");
        exile.setRequired(true);
        options.addOption(exile);

        CommandLineParser parser = new DefaultParser();
        HelpFormatter formatter = new HelpFormatter();
        CommandLine cmd;

        try {
            cmd = parser.parse(options, args);
        } catch (ParseException e) {
            System.out.println(e.getMessage());
            formatter.printHelp("xmltestgen", options);
            throw new TestGenException("unable to parse command line arguments", e);
        }

        File w3cXml = new File(cmd.getOptionValue(OPT_W3C_XML));
        File w3cSchema = new File(cmd.getOptionValue(OPT_W3C_SCHEMA));
        File customXml = new File(cmd.getOptionValue(OPT_CUSTOM_XML));
        File customSchema = new File(cmd.getOptionValue(OPT_CUSTOM_SCHEMA));
        File xmlOutdir = new File(cmd.getOptionValue(OPT_XML_OUTDIR));
        File schemaOutdir = new File(cmd.getOptionValue(OPT_SCHEMA_OUTDIR));
        File rustRoot = new File(cmd.getOptionValue(OPT_RUST_ROOT));
        File exileTests = new File(cmd.getOptionValue(OPT_EXILE_TESTS));

        try {
            if (!w3cXml.exists() || !w3cXml.isFile()) {
                throw new TestGenException("Unable to verify " + OPT_W3C_XML + ", " + w3cXml.getPath());
            }
            if (!w3cSchema.exists() || !w3cSchema.isFile()) {
                throw new TestGenException("Unable to verify " + OPT_W3C_SCHEMA + ", " + w3cSchema.getPath());
            }
            // TODO - verify customXml
            // TODO - verify customSchema
            if (!xmlOutdir.exists() || !xmlOutdir.isDirectory()) {
                throw new TestGenException("Unable to verify " + OPT_XML_OUTDIR + ", " + xmlOutdir.getPath());
            }
            // TODO - verify schemaOutdir
            if (!exileTests.exists() || !exileTests.isDirectory()) {
                throw new TestGenException("Unable to verify " + OPT_EXILE_TESTS + ", " + exileTests.getPath());
            }
        } catch (TestGenException e) {
            System.out.println(e.getMessage());
            formatter.printHelp("xmltestgen", options);
            throw e;
        }

        return new ProgramOptions(w3cXml,
                w3cSchema,
                customXml,
                customSchema,
                xmlOutdir,
                schemaOutdir,
                rustRoot,
                exileTests);
    }
}
