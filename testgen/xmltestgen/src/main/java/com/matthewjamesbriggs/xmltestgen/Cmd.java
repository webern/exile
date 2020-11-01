package com.matthewjamesbriggs.xmltestgen;

import java.io.*;

class Cmd {
    /**
     * Execute a command at the shell.
     *
     * @param cmd        The shell command to run, e.g. "echo hello".
     * @param contextDir The directory to be in when running the command.
     * @return The result of the command.
     * @throws TestGenException if the command cannot be executed.
     */
    private static CmdResult exec(String cmd, File contextDir) throws TestGenException {
        Process process;
        try {
            process = Runtime.getRuntime().exec(cmd, null, contextDir);
        } catch (IOException e) {
            throw new TestGenException("cmd io exception", e);
        }
        int exitCode;
        try {
            exitCode = process.waitFor();
        } catch (InterruptedException e) {
            throw new TestGenException("process failed", e);
        }
        String stdout = getStdOut(process);
        String stderr = getStdErr(process);
        return new CmdResult(stdout, stderr, exitCode);
    }

    /**
     * Run <code>cargo fmt</code> in the specified directory.
     *
     * @param directoryToFmt The directory to be in when running <code>cargo fmt</code>
     * @throws TestGenException if <code>cargo fmt</code> cannot be executed, or exits non-zero.
     */
    static void fmt(File directoryToFmt) throws TestGenException {
        CmdResult result = Cmd.exec("cargo fmt", directoryToFmt);
        if (result.getExit() != 0) {
            throw new TestGenException(String.format("cargo fmt failed with exit: %d\n%s",
                    result.getExit(),
                    result.getStderr()));
        }
    }

    /**
     * Run <code>cargo clippy --tests --locked -- -D warnings</code> in the specified directory.
     *
     * @param directory The directory to be in when running <code>cargo fmt</code>
     * @throws TestGenException if <code>cargo clippy --tests --locked -- -D warnings</code> cannot be executed, or
     *                          exits non-zero.
     */
    static void clippy(File directory) throws TestGenException {
        CmdResult result = Cmd.exec("cargo clippy --tests --locked -- -D warnings", directory);
        if (result.getExit() != 0) {
            throw new TestGenException(String.format("cargo fmt failed with exit: %d\n%s",
                    result.getExit(),
                    result.getStderr()));
        }
    }

    private static String getStdErr(Process process) throws TestGenException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(process.getErrorStream()));
        StringBuilder everything = new StringBuilder();
        String line = "";
        try {
            while ((line = reader.readLine()) != null) {
                everything.append(line);
                everything.append('\n');
            }
        } catch (IOException e) {
            throw new TestGenException("proccess results could not be read", e);
        }
        return everything.toString();
    }

    private static String getStdOut(Process process) throws TestGenException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()));
        StringBuilder everything = new StringBuilder();
        String line = "";
        try {
            while ((line = reader.readLine()) != null) {
                everything.append(line);
                everything.append('\n');
            }
        } catch (IOException e) {
            throw new TestGenException("proccess results could not be read", e);
        }
        return everything.toString();
    }
}
