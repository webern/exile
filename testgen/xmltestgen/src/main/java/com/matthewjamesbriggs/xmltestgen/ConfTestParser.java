package com.matthewjamesbriggs.xmltestgen;

import org.w3c.dom.Document;
import org.xml.sax.SAXException;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.FactoryConfigurationError;
import javax.xml.parsers.ParserConfigurationException;
import java.io.File;
import java.io.IOException;
import java.util.List;

public class ConfTestParser {
    public static List<ConfTest> parseTests(String w3cXmlFilepath) throws TestGenException {
        Document doc = loadXconf(w3cXmlFilepath);

    }

    private static Document loadXconf(String w3cXmlFilepath) throws TestGenException {
        File xmlConfFile = new File(w3cXmlFilepath);
        xmlConfFile = F.canonicalize(xmlConfFile);
        F.checkFile(xmlConfFile);
        String uri = xmlConfFile.toPath().toUri().toString();

        try {
            DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
            DocumentBuilder builder = factory.newDocumentBuilder();
            Document document = builder.parse(uri);
            return document;
        } catch (FactoryConfigurationError e) {
            throw new TestGenException("unable to get a document builder factory", e);
        } catch (ParserConfigurationException e) {
            throw new TestGenException("parser was unable to be configured", e);
        } catch (SAXException e) {
            throw new TestGenException("parsing error", e);
        } catch (IOException e) {
            throw new TestGenException("i/o error", e);
        } catch (Throwable t) {
            throw new TestGenException("weird error", t);
        }
    }
}
