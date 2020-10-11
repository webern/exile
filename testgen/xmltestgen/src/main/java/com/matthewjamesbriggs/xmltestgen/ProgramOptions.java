package com.matthewjamesbriggs.xmltestgen;

import org.apache.commons.cli.*;

import java.io.File;

public class ProgramOptions {
    private static final String OPT_W3C_XML = "w3c-xml";
    private static final String OPT_W3C_SCHEMA = "w3c-schema";
    private static final String OPT_CUSTOM_XML = "custom-xml";
    private static final String OPT_CUSTOM_SCHEMA = "custom-schema";
    private static final String OPT_XML_OUTDIR = "xml-outdir";
    private static final String OPT_SCHEMA_OUTDIR = "schema-outdir";
    private final File w3cXml;
    private final File w3cSchema;
    private final File customXml;
    private final File customSchema;
    private final File xmlOutdir;
    private final File schemaOutdir;

    /**
     * Private all-args constructor. Use the static parse function to construct from command line arguments.
     *
     * @param w3cXml:       the path to xmlconf/xmlconf.xml
     * @param w3cSchema:    the path to xmlschema/suite.xml
     * @param customXml:    the path to custom xml test definitions file.
     * @param customSchema: the path to custom schema test definitions file.
     * @param xmlOutdir:    the directory into which rust xml tests will be written.
     * @param schemaOutdir: the directory into which rust schema tests will be written.
     */
    private ProgramOptions(File w3cXml,
                           File w3cSchema,
                           File customXml,
                           File customSchema,
                           File xmlOutdir,
                           File schemaOutdir) {
        this.w3cXml = w3cXml;
        this.w3cSchema = w3cSchema;
        this.customXml = customXml;
        this.customSchema = customSchema;
        this.xmlOutdir = xmlOutdir;
        this.schemaOutdir = schemaOutdir;
    }

    /**
     * Parses the command line arguments.
     *
     * @param args the command line arguments.
     * @return true if successful.
     * @throws TestGenException if the parsing fails or any of the named files or directories do not exist.
     */
    public static ProgramOptions parse(String[] args) throws TestGenException {
        Options options = new Options();

        Option wx = new Option("x", OPT_W3C_XML, true, "path to w3c xmlconf.xml");
        wx.setRequired(true);
        options.addOption(wx);

        Option ws = new Option("s", OPT_W3C_SCHEMA, true, "path to w3c schema.xml");
        ws.setRequired(true);
        options.addOption(ws);

        Option cx = new Option("m", OPT_CUSTOM_XML, true, "path to my own parser tests.xml");
        cx.setRequired(true);
        options.addOption(cx);

        Option cs = new Option("c", OPT_CUSTOM_SCHEMA, true, "path to my own schema tests.xml");
        cs.setRequired(true);
        options.addOption(cs);

        Option ox = new Option("l", OPT_XML_OUTDIR, true, "directory to write rust parser xml tests");
        ox.setRequired(true);
        options.addOption(ox);

        Option os = new Option("h", OPT_SCHEMA_OUTDIR, true, "directory to write rust schema xml tests");
        os.setRequired(true);
        options.addOption(os);

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
        } catch (TestGenException e) {
            System.out.println(e.getMessage());
            formatter.printHelp("xmltestgen", options);
            throw e;
        }

        return new ProgramOptions(w3cXml, w3cSchema, customXml, customSchema, xmlOutdir, schemaOutdir);
    }

    public File getW3cXml() {
        return w3cXml;
    }

    public File getW3cSchema() {
        return w3cSchema;
    }

    public File getCustomXml() {
        return customXml;
    }

    public File getCustomSchema() {
        return customSchema;
    }

    public File getXmlOutdir() {
        return xmlOutdir;
    }

    public File getSchemaOutdir() {
        return schemaOutdir;
    }
}
