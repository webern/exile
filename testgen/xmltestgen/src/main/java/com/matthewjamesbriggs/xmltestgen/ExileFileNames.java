package com.matthewjamesbriggs.xmltestgen;

import org.apache.commons.io.FilenameUtils;

import java.io.File;

class ExileFileNames {
    static final String EXILE_PREFIX = "exile";
    private static final String DISABLED = "disabled";
    private static final String SEPARATOR = "_";
    static final String EXILE_FILE_PREFIX = EXILE_PREFIX + SEPARATOR;
    private static final String EXILE_DISABLED_FILE_PREFIX = EXILE_FILE_PREFIX + DISABLED;
    private static final String EXILE_METADATA_FILE_SUFFIX = ".metadata.json";
    private static final String EXILE_OUTPUT_FILE_SUFFIX = ".output.xml";

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
        return file.getName().endsWith(EXILE_METADATA_FILE_SUFFIX);
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
        return file.getName().endsWith(EXILE_OUTPUT_FILE_SUFFIX);
    }
}
