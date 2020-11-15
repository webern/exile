package com.matthewjamesbriggs.xmltestgen;

import org.junit.Test;

import static org.junit.Assert.*;

public class STest {
    @Test
    public void getSnakeCaseTest() {
        String result = S.getSnakeCase("ezfile");
        assertEquals("ezfile", result);
    }

    public static void isLetterTest() {
        assertTrue(S.isLetter('z'));
        assertTrue(S.isLetter('Z'));
        assertTrue(S.isLetter('a'));
        assertTrue(S.isLetter('A'));
        assertFalse(S.isLetter('0'));
        assertFalse(S.isLetter('9'));
        assertFalse(S.isLetter(' '));
    }
}
