package com.matthewjamesbriggs.xmltestgen;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;

@AllArgsConstructor @NoArgsConstructor class ExileTestMetadataSyntax {
    @Getter
    private ExileTestMetadataGood good;
    @Getter
    private ExileTestMetadataBad bad;
}
