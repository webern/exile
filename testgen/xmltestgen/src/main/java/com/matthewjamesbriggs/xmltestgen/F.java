package com.matthewjamesbriggs.xmltestgen;

import org.apache.commons.io.FileUtils;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;

/**
 * File and path related helper functions.
 */
class F {

    /**
     * If the directory exists, deletes it and replaces it. Otherwise just creates it.
     *
     * @param directory The directory to create.
     * @throws TestGenException if the directory could not be created
     */
    static void createOrReplaceDir(File directory) throws TestGenException {
        directory = new File(canonicalize(directory).getPath());
        if (directory.exists() && directory.isDirectory()) {
            try {
                FileUtils.deleteDirectory(directory);
            } catch (IOException e) {
                throw new TestGenException("unable to delete dir: " + directory.getPath());
            }
        } else if (directory.exists() && directory.isFile()) {
            FileUtils.deleteQuietly(directory);
        }
        try {
            FileUtils.forceMkdir(directory);
        } catch (IOException e) {
            throw new TestGenException("unable to create directory: " + directory.getPath());
        }
    }

    /**
     * Throws an exception if the directory does not exist, or if it is not a directory.
     *
     * @param dir The directory to check.
     * @throws TestGenException if the directory does not exist or is not a directory.
     */
    static void checkDir(File dir) throws TestGenException {
        if (!dir.exists()) {
            throw new TestGenException("dir does not exist: " + dir.getPath());
        }
        if (!dir.isDirectory()) {
            throw new TestGenException("not a dir: " + dir.getPath());
        }
    }

    /**
     * Throws an exception if the file does not exist, or if it is not a file.
     *
     * @param file The file to check.
     * @throws TestGenException if the file does not exist or is not a file.
     */
    static void checkFile(File file) throws TestGenException {
        if (!file.exists()) {
            throw new TestGenException("file does not exist: " + file.getPath());
        }
        if (!file.isFile()) {
            throw new TestGenException("not a file: " + file.getPath());
        }
    }

    /**
     * Canonicalizes a file path.
     *
     * @param path The path to cannonicalize.
     * @return The cannonicalized file path.
     * @throws TestGenException if the cannonicalization failes.
     */
    static File canonicalize(File path) throws TestGenException {
        try {
            return new File(path.getCanonicalFile().getPath());
        } catch (IOException e) {
            throw new TestGenException("unable to cannonicalize path, " + path.getPath() + ": " + e.getMessage());
        }
    }

    /**
     * Copies a file from 'original' to 'destination'.
     *
     * @param original    The filepath to the file to be copied.
     * @param destination The filepath to copy the original file to.
     * @throws TestGenException if the operation fails.
     */
    static void copy(File original, File destination) throws TestGenException {
        try {
            FileUtils.copyFile(original, destination);
        } catch (IOException e) {
            throw new TestGenException("unable to copy file from: " +
                    original.getPath() +
                    ", to: " +
                    destination.getPath());
        }
    }

    /**
     * Creates (or replaces if it exists) a file.
     *
     * @param file The file to create.
     * @throws TestGenException if the file could not be created.
     */
    static void createFile(File file) throws TestGenException {
        if (file.exists()) {
            if (file.isFile()) {
                FileUtils.deleteQuietly(file);
            } else if (file.isDirectory()) {
                try {
                    FileUtils.deleteDirectory(file);
                } catch (IOException e) {
                    throw new TestGenException("a directory could not be deleted: " + file, e);
                }
            } else {
                throw new TestGenException("something exists but i don't know what: " + file);
            }
        }
        try {
            if (!file.createNewFile()) {
                throw new TestGenException("file already exists: " + file.getPath());
            }
        } catch (IOException e) {
            throw new TestGenException("unable to create file: " + file.getPath(), e);
        }
    }

    /**
     * Creates (or replaces if it exists) a file, then opens a stream to that file.
     *
     * @param file The file to create and open.
     * @return The stream to the new file.
     * @throws TestGenException if the file could not be created or opened.
     */
    static FileOutputStream createAndOpen(File file) throws TestGenException {
        createFile(file);
        return openFile(file);
    }

    static void closeStream(File file, FileOutputStream stream) throws TestGenException {
        try {
            stream.close();
        } catch (IOException e) {
            throw new TestGenException("unable to close file: " + file.getPath(), e);
        }
    }


    static FileOutputStream openFile(File file) throws TestGenException {
        try {
            return FileUtils.openOutputStream(file);
        } catch (IOException e) {
            throw new TestGenException("could not open for writing: " + file.getPath(), e);
        }
    }


    private static void write(FileOutputStream os, String format, Object... args) throws TestGenException {
        String line = String.format(format, args);
        try {
            os.write(line.getBytes());
            os.flush();
        } catch (IOException e) {
            throw new TestGenException("unable to write to stream: " + os.toString(), e);
        }
    }

    static void writeln(FileOutputStream os, String format, Object... args) throws TestGenException {
        write(os, format + "\n", args);
    }
}
