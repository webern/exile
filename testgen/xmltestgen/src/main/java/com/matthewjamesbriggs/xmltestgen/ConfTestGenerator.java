package com.matthewjamesbriggs.xmltestgen;

import com.google.gson.Gson;
import lombok.AllArgsConstructor;
import lombok.Getter;
import org.apache.commons.io.FileUtils;
import org.w3c.dom.*;

import java.io.*;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.MatchResult;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

class ConfTestGenerator {
    /// The maximum number of W3C tests of ConfType.Valid that will be generated.
    private static final int MAX_VALID = 20;
    /// The maximum number of W3C tests of ConfType.NotWellFormed that will be generated.
    private static final int MAX_NOT_WELL_FORMED = 5;
    /// The tests directory, e.g. exile_repo/exile/tests
    private final File outDir;
    /// The root of the generated tests, e.g. exile_repo/exile/tests/generated
    private final File generatedDir;
    /// The place to copy xml files, e.g. exile_repo/exile/tests/input_data
    private final File testDataDir;
    /// The root of the rust workspace, e.g. exile_repo
    private final File rustWorkspaceDir;
    /// The parsed conf tests to use as our source for generating rust tests
    private final List<ConfTest> tests;
    /// The mod.rs file for our generated rust tests to be compiled, e.g. exile_repo/exile/tests/generated/mod.rs
    private final File modRs;
    /// The number of W3C tests of ConfType.Valid that have been generated.
    private int validTestCount;
    /// The number of W3C tests of ConfType.NotWellFormed that have been generated.
    private int notWellFormedTestCount;

    ConfTestGenerator(List<ConfTest> tests, ProgramOptions opts) throws TestGenException {
        outDir = F.canonicalize(opts.getXmlOutdir());
        F.checkDir(outDir);
        generatedDir = F.canonicalize(new File(opts.getXmlOutdir(), "generated"));
        F.checkDir(generatedDir);
        testDataDir = opts.getRustDataDir();
        F.checkDir(testDataDir);
        rustWorkspaceDir = F.canonicalize(opts.getRustRoot());
        F.checkDir(rustWorkspaceDir);
        this.tests = tests;
        modRs = F.canonicalize(new File(generatedDir, "mod.rs"));
    }

    @AllArgsConstructor private static class FoundDecl {
        @Getter
        private final String version;
        @Getter
        private final String encoding;

        boolean hasVersion() {
            return version != null && version.length() == 3;
        }

        boolean hasEncoding() {
            return encoding != null && encoding.length() > 0;
        }

        XmlVersion getVersion() throws TestGenException {
            if (!hasVersion()) {
                throw new TestGenException("function only works when there is a version string");
            }
            if (version.equals("1.0")) {
                return XmlVersion.V10;
            } else if (version.equals("1.1")) {
                return XmlVersion.V11;
            }
            throw new TestGenException("bad version string: " + version);
        }
    }

    // TODO - get rid of this
    @AllArgsConstructor private static class PI {
        @Getter
        private final String target;
        @Getter
        private final String data;
    }

    /**
     * Filters the file list to include only those that are *non* permanent, custom, exile files. That is, a list of
     * files that we should delete each time we generate tests.
     */
    private static class FilesToDelete implements FilenameFilter {
        @Override
        public boolean accept(File dir, String name) {
            File f = new File(name);
            if (!ExileConstants.isExile(f)) {
                return false;
            }
            boolean isMetadata = ExileConstants.isExileMetadata(f);
            boolean isOutput = ExileConstants.isExileOutput(f);
            boolean isInput = ExileConstants.isExileInput(f);
            return !isInput && !isOutput && !isMetadata;
        }
    }

    void generateTests() throws TestGenException {
        F.createOrReplaceDir(generatedDir);
        deleteNonExileXmlFiles(testDataDir);

        // create the mod.rs file
        OutputStreamWriter mod = F.createAndOpen(modRs);
        writeCodeFileHeader(mod);
        F.writeln(mod, "");

        // create test files
        for (ConfTest t : tests) {
            generateTest(t, mod);
        }

        F.writeln(mod, "");
        F.closeStream(modRs, mod);
        Cmd.fmt(rustWorkspaceDir);
        Cmd.clippy(rustWorkspaceDir);
    }

    private static void deleteNonExileXmlFiles(File testDataDir) throws TestGenException {
        F.checkDir(testDataDir);
        File[] files = testDataDir.listFiles(new FilesToDelete());
        if (files == null) {
            throw new TestGenException("null file list");
        }
        for (File file : files) {
            FileUtils.deleteQuietly(file);
        }
    }

    private void generateTest(ConfTest t, OutputStreamWriter mod) throws TestGenException {
        // Copy W3C tests to the 'generated' directory.
        if (!isMaxedOut(t) && !t.isExileTest()) {
            copyXmlTestFile(t);
        }
        switch (t.getConfType()) {
            case Valid:
                generateValidTest(t, mod);
                break;
            case NotWellFormed:
                generateNotWellFormedTest(t, mod);
                break;
            case Invalid:
            case Error:
            default:
        }
    }

    private boolean isMaxedOut(ConfTest t) {
        if (t.isExileTest()) {
            // we always generate all of the exile tests
            return false;
        }
        switch (t.getConfType()) {
            case Valid:
                return validTestCount >= MAX_VALID;
            case NotWellFormed:
                return notWellFormedTestCount >= MAX_NOT_WELL_FORMED;
            case Error:
            case Invalid:
            default:
                return true;
        }
    }

    private void incrementCount(ConfTest t) {
        if (t.isExileTest()) {
            // we generate all exile test and they don't could toward the max W3C test counts
            return;
        }
        switch (t.getConfType()) {
            case NotWellFormed:
                notWellFormedTestCount++;
                break;
            case Valid:
                validTestCount++;
                break;
            case Error:
            case Invalid:
            default:
                break;
        }
    }

    private void generateNotWellFormedTest(ConfTest t, OutputStreamWriter mod) throws TestGenException {
        if (isMaxedOut(t)) {
            return;
        }
        incrementCount(t);
        if (t.getConfType() != ConfType.NotWellFormed) {
            throw new TestGenException("wrong test type, expected '%s', got '%s'",
                    ConfType.NotWellFormed,
                    t.getConfType().toString());
        }
        ExileTestMetadata metadata = null;
        if (t.hasMetadataFile()) {
            File m = t.getMetadataFile();
            Gson gson = new Gson();

            try {
                Reader reader = Files.newBufferedReader(m.toPath());
                metadata = gson.fromJson(reader, ExileTestMetadata.class);
            } catch (IOException e) {
                throw new TestGenException(e, "unable to load %s", m.getPath());
            }
        }
        String description =
                String.format("A not-well-formed test file from the W3C conformance test suite: %s", t.getId());
        ExileTestMetadataBad bad = null;
        if (metadata != null) {
            description = metadata.getDescription();
            bad = metadata.getSyntax().getBad();
        }
        File testFile = new File(generatedDir, t.getTestName() + ".rs");
        OutputStreamWriter os = F.createAndOpen(testFile);
        F.writeln(mod, "mod %s;", t.getTestName());
        writeCodeFileHeader(os);
        F.writeln(os, "");
        F.writeln(os, "use crate::test_utils::run_not_well_formed_test;");
        if (bad != null) {
            F.writeln(os, "use exile::parser::XmlSite;");
        }
        F.writeln(os, "");
        writeConstDeclarations(t, os);
        F.writeln(os, "");
        F.writeln(os, "#[test]");
        F.writeln(os, "/// %s", description);
        F.writeln(os, "fn %s_test() {", t.getSnakeCase());
        F.writeln(os, "    run_not_well_formed_test(");
        F.writeln(os, "        INPUT_FILE,");
        if (bad != null) {
            F.writeln(os, "           Some(XmlSite{");
            F.writeln(os, "                   line: %d,", bad.getLine());
            F.writeln(os, "                   column: %d,", bad.getColumn());
            F.writeln(os, "                   position: %d,", bad.getPosition());
            F.writeln(os, "                   character: '%s',", bad.getCharacter());
            F.writeln(os, "                   }"); // closes the XmlSite struct
            F.writeln(os, "                ),"); // closes the Some variant
            F.writeln(os, "           );"); // closes the function call
        } else {
            F.writeln(os, "    None);");
        }
        final boolean positionAsserted = false;
        F.writeln(os, "}");
    }

    private void generateValidTest(ConfTest t, OutputStreamWriter mod) throws TestGenException {
        if (isMaxedOut(t)) {
            return;
        }
        incrementCount(t);
        if (t.getConfType() != ConfType.Valid) {
            throw new TestGenException("wrong test type, expected '%s', got '%s'",
                    ConfType.Valid,
                    t.getConfType().toString());
        }
        File testFile = new File(generatedDir, t.getTestName() + ".rs");
        OutputStreamWriter os = F.createAndOpen(testFile);
        F.writeln(mod, "mod %s;", t.getTestName());
        writeCodeFileHeader(os);
        F.writeln(os, "");
        FoundDecl foundDecl = findDecl(t);
        String foundDoctype = findDoctype(t);
        writeUseStatements(t, foundDecl, os);
        F.writeln(os, "");
        writeConstDeclarations(t, os);
        F.writeln(os, "");
        writeTestFunction(t, os);
        F.writeln(os, "");
        if (t.hasOutputFile()) {
            writeSerializationTestFunction(t, os);
        }
        F.writeln(os, "");
        writeExpectedFunction(t, foundDecl, foundDoctype, os);

        // close the stream, we are done writing to the test file
        F.closeStream(testFile, os);
    }

    private static String findDoctype(ConfTest t) throws TestGenException {
        String contents;
        try {
            byte[] encoded = Files.readAllBytes(t.getPath());
            contents = new String(encoded, StandardCharsets.UTF_8);
        } catch (IOException e) {
            throw new TestGenException("could not load " + t.getPath().toString(), e);
        }
        Pattern pattern = Pattern.compile("<!DOCTYPE[^<>]*(?:<![^<>]*>[^<>]*)*>");
        Matcher matcher = pattern.matcher(contents);
        if (matcher.find()) {
            try {
                String result = matcher.group(0);
                return result;
            } catch (Throwable e) {
                return "";
            }
        } else {
            return "";
        }

    }

    private static void writeCodeFileHeader(OutputStreamWriter os) throws TestGenException {
        F.writeln(os, "// generated file, do not edit");
    }

    private static void writeUseStatements(ConfTest t,
                                           FoundDecl foundDecl,
                                           OutputStreamWriter os) throws TestGenException {
        if (t.hasOutputFile()) {
            F.writeln(os, "use crate::test_utils::{run_output_test, run_parse_test};");
        } else {
            F.writeln(os, "use crate::test_utils::run_parse_test;");
        }
        List<String> structs = new ArrayList<>();
        structs.add("Document");
        structs.add("Declaration");
        if (foundDecl.hasVersion()) {
            structs.add("Version");
        }
        if (foundDecl.hasEncoding()) {
            structs.add("Encoding");
        }
        String importThese = String.join(",", structs);
        F.writeln(os, "use exile::{%s};", importThese);
    }

    private static void writeConstDeclarations(ConfTest t, OutputStreamWriter os) throws TestGenException {
        F.writeln(os, "const INPUT_FILE: &str = \"%s\";", t.getXmlFilename());
        if (t.hasOutputFile()) {
            F.writeln(os, "const OUTPUT_FILE: &str = \"%s\";", t.getOutputFile().getName());
        }
    }

    private static void writeTestFunction(ConfTest t, OutputStreamWriter os) throws TestGenException {
        ExileTestMetadata metadata = null;
        if (t.hasMetadataFile()) {
            File m = t.getMetadataFile();
            Gson gson = new Gson();

            try {
                Reader reader = Files.newBufferedReader(m.toPath());
                metadata = gson.fromJson(reader, ExileTestMetadata.class);
            } catch (IOException e) {
                throw new TestGenException(e, "unable to load %s", m.getPath());
            }
        }
        String description = String.format("A valid XML file from the W3C conformance test suite: %s", t.getId());
        if (metadata != null) {
            description = metadata.getDescription();
        }
        F.writeln(os, "#[test]");
        F.writeln(os, "/// %s", description);
        F.writeln(os, "fn %s_parse() {", t.getSnakeCase());
        F.writeln(os, "    run_parse_test(INPUT_FILE, &expected());");
        F.writeln(os, "}");
    }


    private static void writeSerializationTestFunction(ConfTest t, OutputStreamWriter os) throws TestGenException {
        F.writeln(os, "#[test]");
        F.writeln(os, "/// Check that the serialization of this XML document matches what we expect.");
        F.writeln(os, "fn %s_serialize() {", t.getSnakeCase());
        F.writeln(os, "    run_output_test(OUTPUT_FILE, &expected());");
        F.writeln(os, "}");
    }

    private static void writeExpectedFunction(ConfTest t,
                                              FoundDecl foundDecl,
                                              String foundDoctype,
                                              OutputStreamWriter os) throws TestGenException {

        F.writeln(os, "fn expected() -> Document {");
        Document doc = X.loadShallow(t.getPath().toFile(), t.getNamespaces());
        F.writeln(os, "let mut doc = Document::new();");
        writeExpectedXmlDeclaration(foundDecl, os);
        List<Node> prelude = findPrelude(doc);
        List<Node> postlude = findPostlude(doc);
        DocumentType doctype = doc.getDoctype();
        if (doctype != null) {
            writeExpectedDoctype(doctype, os, foundDoctype);
        }
        writeExpectedPrelude(prelude, os);
        writeExpectedContents(t, doc, os);
        writeExpectedPostlude(postlude, os);
        F.writeln(os, "doc");
        F.writeln(os, "}");
    }

    private static void writeExpectedPrelude(List<Node> prelude, OutputStreamWriter os) throws TestGenException {
        for (Node node : prelude) {
            XType xtype = XType.fromNode(node);
            if (xtype == XType.ProcessingInstruction) {
                ProcessingInstruction piNode = (ProcessingInstruction) node;
                PI pi = parseProcessingInstruction(piNode);
                F.write(os, "doc.add_prolog_pi(");
                constructProcessingInstruction(pi, os);
                F.writeln(os, ");");
            } else if (xtype == XType.Comment) {
                Comment comment = (Comment) node;
                F.write(os, "doc.add_prolog_comment(%s).unwrap();", rustStringLiteral(comment.getData()));
            }
        }
    }

    private static void writeExpectedPostlude(List<Node> postlude, OutputStreamWriter os) throws TestGenException {
        for (Node node : postlude) {
            XType xtype = XType.fromNode(node);
            if (xtype == XType.ProcessingInstruction) {
                ProcessingInstruction piNode = (ProcessingInstruction) node;
                PI pi = parseProcessingInstruction(piNode);
                F.write(os, "doc.add_epilog_pi(");
                constructProcessingInstruction(pi, os);
                F.writeln(os, ");");
            } else if (xtype == XType.Comment) {
                Comment comment = (Comment) node;
                F.write(os, "doc.add_epilog_comment(%s).unwrap();", rustStringLiteral(comment.getData()));
            }
        }
    }

    private static PI parseProcessingInstruction(ProcessingInstruction pi) throws TestGenException {
        String target = pi.getTarget();
        String data = pi.getData();
        return new PI(target, data);
    }

    private static void constructProcessingInstruction(PI pi, OutputStreamWriter os) throws TestGenException {
        F.writeln(os, "exile::Pi::new(");
        F.writeln(os, "%s,", rustStringLiteral(pi.getTarget()));
        F.writeln(os, "%s,", rustStringLiteral(pi.getData()));
        F.writeln(os, ").unwrap()");
    }

    private static void writeProcessingInstruction(ProcessingInstruction pi,
                                                   String parentVariableName,
                                                   OutputStreamWriter os) throws TestGenException {
        F.write(os, "%s.add_pi(", parentVariableName);
        PI parsed = parseProcessingInstruction(pi);
        constructProcessingInstruction(parsed, os);
        F.writeln(os, ");");
    }

    private static void writeExpectedContents(ConfTest t, Document doc, OutputStreamWriter os) throws TestGenException {
        Element root = doc.getDocumentElement();
        String name = root.getNodeName();
        F.writeln(os, "let root = doc.root_mut();");
        F.writeln(os, "root.set_name(r#\"%s\"#);", name);
        NamedNodeMap attributes = root.getAttributes();
        int numAttributes = attributes.getLength();
        for (int i = 0; i < numAttributes; ++i) {
            Node item = attributes.item(i);
            String key = item.getNodeName();
            String val = item.getNodeValue();
            F.writeln(os, "root.add_attribute(r#\"%s\"#, r#\"%s\"#);", key, val);
        }
        writeChildren(root, "root", 0, t, doc, os);
    }

    private static void writeChildren(Node parent,
                                      String parentVariableName,
                                      int parentGeneration,
                                      ConfTest t,
                                      Document doc,
                                      OutputStreamWriter os) throws TestGenException {
        NodeList children = parent.getChildNodes();
        int childCount = children.getLength();
        for (int i = 0; i < childCount; ++i) {
            Node child = children.item(i);
            XType xtype = XType.fromNode(child);
            switch (xtype) {
                case Element:
                    writeElementChild(parentVariableName, parentGeneration, i, (Element) child, t, doc, os);
                    break;
                case Attribute:
                    throw new TestGenException("We should not encounter an attribute node.");
                case Text:
                    writeTextChild(parentVariableName, parentGeneration, i, (Text) child, t, doc, os);
                    break;
                case ProcessingInstruction:
                    writeProcessingInstruction((ProcessingInstruction) child, parentVariableName, os);
                    break;
                case CData:
                    writeCdataChild(parentVariableName, parentGeneration, i, (CDATASection) child, t, doc, os);
                    break;
                case Comment:
                    writeComment(parentVariableName, parentGeneration, i, (Comment) child, t, doc, os);
                    break;
                case EntityReference:
                case Entity:
                case Document:
                case DocumentType:
                case DocumentFragment:
                case Notation:
                case Unknown:
                default:
                    System.out.println(String.format("Unhandled node in '%s' %s: %s",
                            t.getXmlFilename(),
                            xtype.toString(),
                            child.getNodeName()));
                    break;
            }
        }
    }

    private static void writeComment(String parentVariableName,
                                     int parentGeneration,
                                     int i,
                                     Comment child,
                                     ConfTest t,
                                     Document doc,
                                     OutputStreamWriter os) throws TestGenException {
        String data = child.getData();
        F.writeln(os, "%s.add_comment(%s).unwrap();", parentVariableName, rustStringLiteral(data));
    }

    private static void writeTextChild(String parentVariableName,
                                       int parentGeneration,
                                       int i,
                                       Text child,
                                       ConfTest t,
                                       Document doc,
                                       OutputStreamWriter os) throws TestGenException {
        XText xtext = new XText(child);
        // HACK: this is quite difficult. The DOM presents us with 'ignorable whitespace' but does not mark it as such
        // unless the parser is in validation mode. In the presence of a doctype, when an element is specified as
        // containing other elements and not PCDATA, then the DOM marks isElementContentWhitespace true. But sometimes
        // we have no doctype and we essentially have to guess.
        if (xtext.getDocType() == null && xtext.getData().trim().length() == 0) {
            // Because there is no doctype and the text is nothing but whitespace, we are assuming that this is just the
            // newlines and whitespace pretty-printing between elements. Not something we want to add to the exile DOM.
            System.out.println("skipping what is likely element whitespace");
        } else if (!xtext.isElementContentWhitespace()) {
            String data = xtext.getData();
            // this is a little bit scary. the exile parser will always treat whitespace as 'replace', which is what
            // many(?) parsers do. but the Java parser is more correct than that. it only does so when validating. so
            // here we hand-rolled the replacing and collapsing algs to view the string as exile intends to.
            data = normalizeWhitespace(data);
            if (data.isEmpty()) {
                return;
            }
            data = rustStringLiteral(data);
            F.writeln(os, "%s.add_text(%s);", parentVariableName, data);
        }
    }

    /**
     * All occurrences of #x9 (tab), #xA (line feed) and #xD (carriage return) are replaced with #x20 (space).
     */
    private static boolean isWhite(int c) {
        return (c == ' ') || (c == '\t') || (c == '\n') || (c == '\r');
    }

    /**
     * Subsequent to the replacements specified above under replace, contiguous sequences of #x20s are collapsed to a
     * single #x20, and initial and/or final #x20s are deleted.
     */
    private static String normalizeWhitespace(String s) {
        boolean hasNonWhite = false;
        boolean spaceBuffer = false;
        StringBuilder result = new StringBuilder(s.length());
        int l = s.length();
        for (int i = 0; i < l; i++) {
            int c = s.codePointAt(i);
            boolean isW = isWhite(c);
            if (isW) {
                if (!hasNonWhite) {
                    continue;
                }
                if (!spaceBuffer) {
                    spaceBuffer = true;
                }
            } else {
                hasNonWhite = true;
                if (spaceBuffer) {
                    result.append(' ');
                    spaceBuffer = false;
                }
                result.append((char) c);
            }
        }
        return result.toString();
    }

    private static String rustStringLiteral(String s) {
        if (s.contains("\r") ||
                s.contains("\t") ||
                s.contains("\b") ||
                s.contains("\n") ||
                s.contains("\f") ||
                s.contains("\u00a0")) {
            return String.format("\"%s\"", rustEscape(s));
        } else {
            if (s.contains("\"#")) {
                return String.format("r###\"%s\"###", s);
            } else {
                return String.format("r#\"%s\"#", s);
            }
        }
    }

    private static String rustEscape(String s) {
        s = s.replaceAll("\\\\", "\\\\");
        s = s.replaceAll("\"", "\\\"");
        s = s.replaceAll("\n", "\\\\n");
        s = s.replaceAll("\r", "\\\\r");
        s = s.replaceAll("\t", "\\\\t");
        s = s.replaceAll("\b", "\\\\b");
        s = s.replaceAll("\f", "\\\\f");
        s = s.replaceAll("\u00a0", "\\\\u{00a0}");
        return s;
    }

    private static void writeCdataChild(String parentVariableName,
                                        int parentGeneration,
                                        int i,
                                        CDATASection child,
                                        ConfTest t,
                                        Document doc,
                                        OutputStreamWriter os) throws TestGenException {
        F.writeln(os, "%s.add_cdata(%s).unwrap();", parentVariableName, rustStringLiteral(child.getData()));
    }

    private static void writeElementChild(String parentVariableName,
                                          int parentGeneration,
                                          int childIndex,
                                          Element child,
                                          ConfTest t,
                                          Document doc,
                                          OutputStreamWriter os) throws TestGenException {
        int myGeneration = parentGeneration + 1;
        String varName = String.format("gen%dn%d", myGeneration, childIndex);
        F.writeln(os, "let %s = %s.add_new_child().unwrap();", varName, parentVariableName);
        F.writeln(os, "%s.set_name(r#\"%s\"#);", varName, child.getNodeName());
        NamedNodeMap attributes = child.getAttributes();
        int numAttributes = attributes.getLength();
        for (int i = 0; i < numAttributes; ++i) {
            Node item = attributes.item(i);
            String key = item.getNodeName();
            String val = item.getNodeValue();
            F.writeln(os, "%s.add_attribute(r#\"%s\"#, r#\"%s\"#);", varName, key, val);
        }
        writeChildren(child, varName, myGeneration, t, doc, os);
    }

    private static List<String> readAllLines(Path path, Charset cs) throws TestGenException {
        try {
            return Files.readAllLines(path, cs);
        } catch (IOException e) {
            throw new TestGenException("Unable to read lines of '" + path.toString() + "': " + e.getMessage());
        }
    }

    private static void writeExpectedXmlDeclaration(FoundDecl foundDecl,
                                                    OutputStreamWriter os) throws TestGenException {
        String rsVersion = "None";
        if (foundDecl.hasVersion()) {
            XmlVersion version = foundDecl.getVersion();
            if (version == XmlVersion.V10) {
                rsVersion = "Some(Version::V10)";
            } else if (version == XmlVersion.V11) {
                rsVersion = "Some(Version::V11)";
            }
        }
        String rsEncoding = "None";
        if (foundDecl.hasEncoding()) {
            if (foundDecl.getEncoding().equals("UTF-8")) {
                rsEncoding = "Some(Encoding::Utf8)";
            } else {
                throw new TestGenException("Unsupported XML encoding parsed: " + foundDecl.getEncoding());
            }
        }
        F.writeln(os, "doc.set_declaration(Declaration{ version: %s, encoding: %s });", rsVersion, rsEncoding);
    }

    private static void writeExpectedDoctype(DocumentType dt,
                                             OutputStreamWriter os,
                                             String fromRegex) throws TestGenException {
        if (dt == null) {
            return;
        }
        if (fromRegex == null || fromRegex.isEmpty()) {
            return;
        }
        F.writeln(os, "// TODO - support doctype https://github.com/webern/exile/issues/22");
        F.writeln(os, "doc.set_doctype(");
        F.writeln(os, "%s", rustStringLiteral(fromRegex));
        F.writeln(os, ").unwrap();");
    }

    private void copyXmlTestFile(ConfTest t) throws TestGenException {
        F.checkDir(testDataDir);
        File original = new File(t.getPath().toString());
        F.checkFile(original);
        File copied = new File(testDataDir, t.getXmlFilename());
        F.copy(original, copied);
    }

    private static FoundDecl findDecl(ConfTest t) throws TestGenException {
        List<String> lines = readAllLines(t.getPath(), StandardCharsets.UTF_8);
        Pattern rxVersion = Pattern.compile("version=[\"']([0-9]+.[0-9]+)[\"']", 0);
        Pattern rxEncoding = Pattern.compile("encoding=[\"'](.+)[\"']", 0);
        String version = null;
        String encoding = null;
        for (String line : lines) {
            if (line.contains("<?xml")) {
                Matcher versionMatcher = rxVersion.matcher(line);
                if (versionMatcher.find()) {
                    try {
                        version = versionMatcher.group(1);
                    } catch (Throwable e) {
                        // ignore
                    }
                }
                Matcher encodingMatcher = rxEncoding.matcher(line);
                if (encodingMatcher.find()) {
                    try {
                        encoding = encodingMatcher.group(1);
                    } catch (Throwable e) {
                        // ignore
                    }
                }
                break;
            }
        }
        return new FoundDecl(version, encoding);
    }

    private static List<Node> findPrelude(Document doc) throws TestGenException {
        List<Node> result = new ArrayList<>();
        NodeList children = doc.getChildNodes();
        int len = children.getLength();
        for (int i = 0; i < len; ++i) {
            Node node = children.item(i);
            if (XType.fromNode(node) == XType.Element) {
                // we reached the root node of the document, prelude is done
                return result;
            } else {
                result.add(node);
            }
        }
        return result;
    }

    private static List<Node> findPostlude(Document doc) throws TestGenException {
        List<Node> result = new ArrayList<>();
        NodeList children = doc.getChildNodes();
        int len = children.getLength();
        boolean foundRoot = false;
        for (int i = 0; i < len; ++i) {
            Node node = children.item(i);
            if (XType.fromNode(node) == XType.Element) {
                foundRoot = true;
            } else if (foundRoot) {
                result.add(node);
            }
        }
        return result;
    }
}
