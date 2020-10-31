package com.matthewjamesbriggs.xmltestgen;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;

@AllArgsConstructor @NoArgsConstructor public class ExileTestMetadataDeclaration {
    @Getter
    private String version;
    @Getter
    private String encoding;
}
