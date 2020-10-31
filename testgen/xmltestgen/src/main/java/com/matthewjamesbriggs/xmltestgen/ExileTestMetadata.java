package com.matthewjamesbriggs.xmltestgen;

import lombok.AllArgsConstructor;
import lombok.NoArgsConstructor;

@AllArgsConstructor @NoArgsConstructor public class ExileTestMetadata {
    private String description;
    private ExileTestMetadataSyntax syntax;
    private ExileTestMetadataExpected expected;
}
