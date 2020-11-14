package com.matthewjamesbriggs.xmltestgen;

import org.junit.Test;

import java.io.File;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;
import static org.junit.Assert.assertEquals;

public class ExileConstantsTest {
    @Test
    public void isExileTest() {
        assertTrue(ExileConstants.isExile(new File("exile_x")));
        assertFalse(ExileConstants.isExile(new File("exile")));
        assertTrue(ExileConstants.isExileDisabled(new File("exile_disabled")));
        assertFalse(ExileConstants.isExileDisabled(new File("exile_disable")));
    }

    @Test
    public void isEnabledExileInputTest() {
        assertTrue(ExileConstants.isExileInput(new File("exile_anything.xml")));
        assertFalse(ExileConstants.isExileInput(new File("exile_anything.output.xml")));
        assertFalse(ExileConstants.isExileInput(new File("exile_anything.metadata.json")));
        assertFalse(ExileConstants.isExileInput(new File("exile_anything.weird")));
        assertTrue(ExileConstants.isEnabledExileInput(new File("exile_anything.xml")));
        assertFalse(ExileConstants.isEnabledExileInput(new File("exile_disabled_anything.xml")));
        assertFalse(ExileConstants.isEnabledExileInput(new File("exile_anything.json")));
        assertTrue(ExileConstants.isExileDisabledInput(new File("exile_disabled_anything.xml")));
        assertTrue(ExileConstants.isExileDisabledInput(new File("exile_disabled.xml")));
        assertFalse(ExileConstants.isExileDisabledInput(new File("exile_disabled_anything.json")));
        assertFalse(ExileConstants.isExileDisabledInput(new File("exile_anything.xml")));
        assertFalse(ExileConstants.isExileDisabledInput(new File("exile_disabled_anything.output.xml")));
    }


    @Test
    public void isMetadataTest() {
        assertTrue(ExileConstants.isExileMetadata(new File("exile_anything.metadata.json")));
        assertTrue(ExileConstants.isExileMetadata(new File("exile_disabled_anything.metadata.json")));
        assertFalse(ExileConstants.isExileMetadata(new File("exile_disabled_anything.metadata.xml")));
    }

    @Test
    public void isExileOutput() {
        assertTrue(ExileConstants.isExileOutput(new File("exile_anything.output.xml")));
        assertTrue(ExileConstants.isExileOutput(new File("exile_disabled_anything.output.xml")));
        assertFalse(ExileConstants.isExileOutput(new File("exile_disabled_anything.output.json")));
    }
}