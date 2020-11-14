package com.matthewjamesbriggs.xmltestgen;

import org.apache.commons.io.FilenameUtils;

import java.io.File;

class ExileConstants {
    static final String DIRECTORY = "input_data";
    static final String EXILE = "exile";
    static final String METADATA_FILE_SUFFIX = ".metadata.json";
    static final String OUTPUT_FILE_SUFFIX = ".output.xml";
    static final String DISABLED = "disabled";
    static final String SEPARATOR = "_";
    static final String EXILE_FILE_PREFIX = EXILE + SEPARATOR;
    static final String EXILE_DISABLED = EXILE_FILE_PREFIX + DISABLED;
    static final String EXILE_DISABLED_FILE_PREFIX = EXILE_DISABLED + ".";

    /**
     * Returns true if this file starts with the exile file prefix.
     */
    static boolean isExile(File file) {
        return file.getName().startsWith(EXILE_FILE_PREFIX);
    }

    /**
     * Returns true if this file starts with exile_disabled prefix.
     */
    static boolean isExileDisabled(File file) {
        return file.getName().startsWith(EXILE_DISABLED_FILE_PREFIX);
    }

    /**
     * Given a file, determines whether or not the file is the XML input file of a permanent exile test. This will
     * return true for both enabled and disabled exile tests.
     *
     * @param file The file to query.
     * @return true if the file is an enabled or disabled exile test input XML file.
     */
    static boolean isExileInput(File file) {
        String ext = FilenameUtils.getExtension(file.getName());
        if (!ext.equals("xml")) {
            return false;
        } else if (isExileOutput(file)) {
            return false;
        } else if (isExileMetadata(file)) {
            return false;
        } else {
            return isExile(file) || isExileDisabled(file);
        }
    }

    /**
     * Given a file, determines whether or not the file is the XML input file of a *disabled* exile test.
     *
     * @param file The file to query.
     * @return true if the file is a disabled exile test input XML file.
     */
    static boolean isExileDisabledInput(File file) {
        if (!isExileInput(file)) {
            return false;
        } else {
            return isExileDisabled(file);
        }
    }

    /**
     * Given a file, determines whether or not the file is the XML input file of a *enabled* exile test.
     *
     * @param file The file to query.
     * @return true if the file is a enabled exile test input XML file.
     */
    static boolean isEnabledExileInput(File file) {
        if (!isExileInput(file)) {
            return false;
        }
        return !isExileDisabled(file);
    }

    /**
     * Given a file, determines whether the file is the metadata JSON file for an exile test (enabled or disabled).
     *
     * @param file The file to query.
     * @return true if the file is a enabled or disabled exile test JSON metadata file.
     */
    static boolean isExileMetadata(File file) {
        if (!file.getName().startsWith(EXILE_FILE_PREFIX)) {
            return false;
        }
        return file.getName().endsWith(METADATA_FILE_SUFFIX);
    }

    /**
     * Given a file, determines whether the file is the expected output XML file for an exile test (enabled or disabled).
     *
     * @param file The file to query.
     * @return true if the file is a enabled or disabled output XML file for an exile test.
     */
    static boolean isExileOutput(File file) {
        if (!file.getName().startsWith(EXILE_FILE_PREFIX)) {
            return false;
        }
        return file.getName().endsWith(OUTPUT_FILE_SUFFIX);
    }

    /**
     * Given a valid exile test input filename, e.g. exile_foo.xml or exile_disabled.foo.xml, return `foo`.
     *
     * @param file The file to parse the core name from.
     * @return The core name.
     */
    static String getCoreName(File file) throws TestGenException {
        if (!isExileInput(file)) {
            throw new TestGenException("not an exile input file '%s'", file.getName());
        }
        String coreName = file.getName();
        if (isEnabledExileInput(file)) {
            coreName = coreName.replaceFirst(EXILE_FILE_PREFIX, "");
        } else {
            coreName = coreName.replaceFirst(EXILE_DISABLED_FILE_PREFIX, "");
        }
        return FilenameUtils.removeExtension(coreName);
    }
}
