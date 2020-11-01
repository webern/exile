package com.matthewjamesbriggs.xmltestgen;

import lombok.AllArgsConstructor;
import lombok.Getter;
import org.w3c.dom.*;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

class ConfTestGenerator {
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


    ConfTestGenerator(List<ConfTest> tests, ProgramOptions opts) throws TestGenException {
        outDir = F.canonicalize(opts.getXmlOutdir());
        F.checkDir(outDir);
        generatedDir = F.canonicalize(new File(opts.getXmlOutdir(), "generated"));
        F.checkDir(generatedDir);
        testDataDir = F.canonicalize(new File(opts.getXmlOutdir(), "input_data"));
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


    void generateTests(int maxTests) throws TestGenException {
        F.createOrReplaceDir(generatedDir);

        // create the mod.rs file
        FileOutputStream mod = F.createAndOpen(modRs);
        writeCodeFileHeader(mod);
        F.writeln(mod, "");

        // create test files
        int testCount = 0;
        for (int i = 0; i < tests.size(); ++i) {
            ConfTest t = tests.get(i);
            if (t.getConfType() != ConfType.Valid) {
                continue;
            }
            // we always generate all of our own custom tests, but if is a w3c test then we increment the testCount to
            // ensure we only generate as many w3c tests as specified by 'maxTests'.
            if (t.getPrefix() == "exile" && t.getConfType() == ConfType.Valid) {
                generateValidTest(t, mod);
            } else if (t.getConfType() == ConfType.Valid && testCount < maxTests) {
                ++testCount;
                generateValidTest(t, mod);
            }
        }

        F.writeln(mod, "");
        F.closeStream(modRs, mod);
        Cmd.fmt(rustWorkspaceDir);
        Cmd.clippy(rustWorkspaceDir);
    }

    private void generateValidTest(ConfTest t, FileOutputStream mod) throws TestGenException {
        if (t.getConfType() != ConfType.Valid) {
            throw new TestGenException("wrong test type, expected 'valid', got " + t.getConfType().toString());
        }
        File testFile = new File(generatedDir, t.getTestName() + ".rs");
        FileOutputStream os = F.createAndOpen(testFile);
        F.writeln(mod, "mod %s;", t.getTestName());
        writeCodeFileHeader(os);
        F.writeln(os, "");
        FoundDecl foundDecl = findDecl(t);
        writeUseStatements(foundDecl, os);
        F.writeln(os, "");
        writeConstDeclarations(t, os);
        F.writeln(os, "");
        writePathFunction(t, os);
        F.writeln(os, "");
        writeTestFunction(t, os);
        F.writeln(os, "");
        writeExpectedFunction(t, foundDecl, os);
        copyXmlTestFile(t);

        // close the stream, we are done writing to the test file
        F.closeStream(testFile, os);
    }

    private static void writeCodeFileHeader(FileOutputStream os) throws TestGenException {
        F.writeln(os, "// generated file, do not edit");
    }

    private static void writeUseStatements(FoundDecl foundDecl, FileOutputStream os) throws TestGenException {
        F.writeln(os, "use exile::Document;");
        F.writeln(os, "use std::path::PathBuf;");
        F.writeln(os, "use xdoc::Declaration;");
        if (foundDecl.hasVersion()) {
            F.writeln(os, "use xdoc::Version;");
        }
        if (foundDecl.hasEncoding()) {
            F.writeln(os, "use xdoc::Encoding;");
        }
    }

    private static void writeConstDeclarations(ConfTest t, FileOutputStream os) throws TestGenException {
        F.writeln(os, "const MANIFEST_DIR: &str = env!(\"CARGO_MANIFEST_DIR\");");
        F.writeln(os, "const INPUT_DATA: &str = \"input_data\";");
        F.writeln(os, "const FILENAME: &str = \"%s\";", t.getFileRename());
    }

    private static void writePathFunction(ConfTest t, FileOutputStream os) throws TestGenException {
        F.writeln(os, "fn path() -> PathBuf {");
        F.writeln(os, "    let p = PathBuf::from(MANIFEST_DIR)");
        F.writeln(os, "        .join(\"tests\")");
        F.writeln(os, "        .join(INPUT_DATA)");
        F.writeln(os, "        .join(FILENAME);");
        F.writeln(os, "    p.canonicalize()");
        F.writeln(os, "        .unwrap_or_else(|e| panic!(\"bad path: {}: {}\", p.display(), e))");
        F.writeln(os, "}");
    }

    private static void writeTestFunction(ConfTest t, FileOutputStream os) throws TestGenException {
        F.writeln(os, "#[test]");
        F.writeln(os, "fn %s() {", t.getSnakeCase());
        F.writeln(os, "    let path = path();");
        F.writeln(os, "    let loaded = exile::load(&path).unwrap();");
        F.writeln(os, "    let expected = expected();");
        F.writeln(os, "    if loaded != expected {");
        F.writeln(os, "        let loaded_str = loaded.to_string();");
        F.writeln(os, "        let expected_str = expected.to_string();");
        F.writeln(os, "        if loaded_str != expected_str {");
        F.writeln(os, "            assert_eq!(loaded_str, expected_str);");
        F.writeln(os, "        } else {");
        F.writeln(os, "            assert_eq!(loaded, expected);");
        F.writeln(os, "        }");
        F.writeln(os, "    }");
        F.writeln(os, "}");
    }

    private static void writeExpectedFunction(ConfTest t,
                                              FoundDecl foundDecl,
                                              FileOutputStream os) throws TestGenException {

        F.writeln(os, "fn expected() -> Document {");
        Document doc = X.loadShallow(t.getPath().toFile());
        F.writeln(os, "let mut doc = Document::new();");
        writeExpectedXmlDeclaration(foundDecl, os);
        List<Node> prelude = findPrelude(doc);
        List<Node> postlude = findPostlude(doc);
        DocumentType doctype = doc.getDoctype();
        if (doctype != null) {
            writeExpectedDoctype(doctype, os);
        }
        writeExpectedPrelude(prelude, os);
        writeExpectedContents(t, doc, os);
        writeExpectedPostlude(prelude, os);
        F.writeln(os, "doc");
        F.writeln(os, "}");
    }


    private static void writeExpectedPrelude(List<Node> prelude, FileOutputStream os) throws TestGenException {
        for (Node node : prelude) {
            XType xtype = XType.fromNode(node);
            if (xtype == XType.ProcessingInstruction) {
                ProcessingInstruction pi = (ProcessingInstruction) node;
                String target = pi.getTarget();
                String data = pi.getData();
                String[] split = data.split("\\s");
                List<String> instructions = new ArrayList<>();
                for (String s : split) {
                    String trimmed = s.trim();
                    if (!trimmed.isEmpty()) {
                        instructions.add(trimmed);
                    }
                }
                F.writeln(os, "doc.push_prolog_misc(xdoc::Misc::PI(xdoc::PI {");
                F.writeln(os, "target: r#\"%s\"#.into(),", target);
                F.writeln(os, "instructions: vec![");
                for (String instruction : instructions) {
                    F.writeln(os, "r#\"%s\"#.to_owned(),", instruction);
                }
                F.writeln(os, "],");
                F.writeln(os, "}));");
            }

        }
        /*
            doc.push_prolog_misc(xdoc::Misc::PI(PI {
                target: "foo".into(),
                instructions: vec!["".to_owned()],
            }));
         */
    }

    private static void writeExpectedPostlude(List<Node> prelude, FileOutputStream os) throws TestGenException {
    }


    private static void writeExpectedContents(ConfTest t, Document doc, FileOutputStream os) throws TestGenException {
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
                                      FileOutputStream os) throws TestGenException {
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
                case CData:
                case EntityReference:
                case Entity:
                case ProcessingInstruction:
                case Comment:
                case Document:
                case DocumentType:
                case DocumentFragment:
                case Notation:
                case Unknown:
                default:
                    System.out.println(String.format("%s: %s", xtype.toString(), child.getNodeName()));
                    break;
            }
        }
    }

    private static void writeTextChild(String parentVariableName,
                                       int parentGeneration,
                                       int i,
                                       Text child,
                                       ConfTest t,
                                       Document doc,
                                       FileOutputStream os) throws TestGenException {
        // TODO - if we start to support ignorable whitespace nodes or preserve directives, this will not work
        if (child.isElementContentWhitespace()) {
            return;
        }
        // HACK - this is a super-funky way of figuring out whether the text node is ignoreable whitespace
        if (child.getWholeText().trim().isEmpty()) {
            return;
        }
        // TODO - this probably not work once we get into more complicated test cases (e.g. CData and entities, etc)
        String text = child.getWholeText();
        F.writeln(os, "%s.add_text(r#\"%s\"#);", parentVariableName, text);
    }

    private static void writeElementChild(String parentVariableName,
                                          int parentGeneration,
                                          int childIndex,
                                          Element child,
                                          ConfTest t,
                                          Document doc,
                                          FileOutputStream os) throws TestGenException {
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

    private static void writeExpectedXmlDeclaration(FoundDecl foundDecl, FileOutputStream os) throws TestGenException {
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

    private static void writeExpectedDoctype(DocumentType dt, FileOutputStream os) throws TestGenException {
        if (dt == null) {
            return;
        }
        F.writeln(os, "// TODO - write doctype information");
    }

    private void copyXmlTestFile(ConfTest t) throws TestGenException {
        F.checkDir(testDataDir);
        File original = new File(t.getPath().toString());
        F.checkFile(original);
        File copied = new File(testDataDir, t.getFileRename());
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
