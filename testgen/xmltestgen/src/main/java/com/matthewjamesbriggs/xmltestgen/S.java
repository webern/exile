package com.matthewjamesbriggs.xmltestgen;

public class S {
    static String getSnakeCase(String s) {
        StringBuilder result = new StringBuilder(s.length() + 4);
        boolean wasUnderscore = false;
        for (int i = 0, n = s.length(); i < n; i++) {
            char c = s.charAt(i);
            if (i == 0 && !isLetter(c)) {
                result.append('x');
                result.append(ExileConstants.SEPARATOR);
                wasUnderscore = true;
            }
            if (isLetter(c)) {
                result.append(Character.toLowerCase(c));
                wasUnderscore = false;
            } else if (isDigit(c)) {
                result.append(c);
                wasUnderscore = false;
            } else if (!wasUnderscore) {
                result.append(ExileConstants.SEPARATOR);
                wasUnderscore = true;
            }
        }
        return result.toString();
    }

    static boolean isLetter(char c) {
        return isLetterUpper(c) || isLetterLower(c);
    }

    static boolean isAlphanumeric(char c) {
        return isLetter(c) || isDigit(c);
    }

    static boolean isLetterUpper(char c) {
        return c >= 65 && c <= 90;
    }

    static boolean isLetterLower(char c) {
        return c >= 97 && c <= 122;
    }

    static boolean isDigit(char c) {
        return c >= 48 && c <= 57;
    }
}
