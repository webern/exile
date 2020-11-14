package com.matthewjamesbriggs.xmltestgen;

import lombok.AllArgsConstructor;
import lombok.Getter;

import java.io.File;

/**
 * The data filenames associated with a custom exile test, e.g. exile_foo.xml, exile_foo.output.xml,
 * exile_foo.metaddata.json.
 */
@AllArgsConstructor public class ExileFiles {
    @Getter
    private final boolean isDisabled;
    @Getter
    private final String coreName;
    @Getter
    private final File directory;

    ExileFiles(File file) throws TestGenException {
        if (!ExileConstants.isExileInput(file)) {
            throw new TestGenException("'%s' is not an exile input file", file.toString());
        }
        isDisabled = ExileConstants.isExileDisabledInput(file);
        coreName = ExileConstants.getCoreName(file);
        directory = F.canonicalize(new File(file.getParent()));
        F.checkDir(directory);
    }

    String getFilenamePrefix() {
        if (isDisabled) {
            return ExileConstants.EXILE_DISABLED_FILE_PREFIX;
        } else {
            return ExileConstants.EXILE_FILE_PREFIX;
        }
    }

    String getInputFilename() {
        return getFilenamePrefix() + getCoreName() + ".xml";
    }

    String getOutputFilename() {
        return getFilenamePrefix() + getCoreName() + ExileConstants.OUTPUT_FILE_SUFFIX;
    }

    String getMetadataFilename() {
        return getFilenamePrefix() + getCoreName() + ExileConstants.METADATA_FILE_SUFFIX;
    }

    File getInputFile() {
        return new File(directory, getInputFilename());
    }

    File getOutputFile() {
        return new File(directory, getOutputFilename());
    }

    File getMetadataFile() {
        return new File(directory, getMetadataFilename());
    }
}
