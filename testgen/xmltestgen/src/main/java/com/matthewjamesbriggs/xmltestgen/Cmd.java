package com.matthewjamesbriggs.xmltestgen;

import java.io.*;

class Cmd {
    static CmdResult exec(String cmd, File contextDir) throws TestGenException {
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
