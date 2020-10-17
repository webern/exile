package com.matthewjamesbriggs.xmltestgen;

import org.apache.commons.io.FileUtils;

import java.io.File;
import java.io.IOException;

/**
 * File and path related helper functions.
 */
public class F {

    /**
     * If the directory exists, deletes it and replaces it. Otherwise just creates it.
     *
     * @param directory The directory to create.
     * @throws TestGenException
     */
    static void createOrReplaceDir(File directory) throws TestGenException {
        try {
            directory = new File(directory.getCanonicalPath());
        } catch (IOException e) {
            throw new TestGenException("unable to canonicalize: " + directory.getPath());
        }
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
     * @throws TestGenException If the directory does not exist or is not a directory.
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
     * @throws TestGenException If the file does not exist or is not a file.
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
     * @throws TestGenException If the cannonicalization failes.
     */
    public static File canonicalize(File path) throws TestGenException {
        try {
            return new File(path.getCanonicalFile().getPath());
        } catch (IOException e) {
            throw new TestGenException("unable to cannonicalize path, " + path.getPath() + ": " + e.getMessage());
        }
    }
}
