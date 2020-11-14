package com.matthewjamesbriggs.xmltestgen;

import org.junit.Test;

import java.io.File;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;


public class ExileFileNamesTest {
    @Test
    public void isExileTest() {
        assertTrue(ExileFileNames.isExile(new File("exile_x")));
        assertFalse(ExileFileNames.isExile(new File("exile")));
        assertTrue(ExileFileNames.isExileDisabled(new File("exile_disabled")));
        assertFalse(ExileFileNames.isExileDisabled(new File("exile_disable")));
    }

    @Test
    public void isEnabledExileInputTest() {
        assertTrue(ExileFileNames.isExileInput(new File("exile_anything.xml")));
        assertFalse(ExileFileNames.isExileInput(new File("exile_anything.output.xml")));
        assertFalse(ExileFileNames.isExileInput(new File("exile_anything.metadata.json")));
        assertFalse(ExileFileNames.isExileInput(new File("exile_anything.weird")));
        assertTrue(ExileFileNames.isEnabledExileInput(new File("exile_anything.xml")));
        assertFalse(ExileFileNames.isEnabledExileInput(new File("exile_disabled_anything.xml")));
        assertFalse(ExileFileNames.isEnabledExileInput(new File("exile_anything.json")));
        assertTrue(ExileFileNames.isExileDisabledInput(new File("exile_disabled_anything.xml")));
        assertTrue(ExileFileNames.isExileDisabledInput(new File("exile_disabled.xml")));
        assertFalse(ExileFileNames.isExileDisabledInput(new File("exile_disabled_anything.json")));
        assertFalse(ExileFileNames.isExileDisabledInput(new File("exile_anything.xml")));
        assertFalse(ExileFileNames.isExileDisabledInput(new File("exile_disabled_anything.output.xml")));
    }


    @Test
    public void isMetadataTest() {
        assertTrue(ExileFileNames.isExileMetadata(new File("exile_anything.metadata.json")));
        assertTrue(ExileFileNames.isExileMetadata(new File("exile_disabled_anything.metadata.json")));
        assertFalse(ExileFileNames.isExileMetadata(new File("exile_disabled_anything.metadata.xml")));
    }

    @Test
    public void isExileOutput() {
        assertTrue(ExileFileNames.isExileOutput(new File("exile_anything.output.xml")));
        assertTrue(ExileFileNames.isExileOutput(new File("exile_disabled_anything.output.xml")));
        assertFalse(ExileFileNames.isExileOutput(new File("exile_disabled_anything.output.json")));
    }
}