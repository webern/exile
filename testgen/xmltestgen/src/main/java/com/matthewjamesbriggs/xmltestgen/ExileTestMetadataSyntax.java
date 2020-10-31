package com.matthewjamesbriggs.xmltestgen;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;

@AllArgsConstructor @NoArgsConstructor class ExileTestMetadataSyntax {
    @Getter
    private ExileTestMetadataGood good;
    @Getter
    private ExileTestMetadataBad bad;

    ConfType getConfType() throws TestGenException {
        if (good != null) {
            return ConfType.Valid;
        } else if (bad != null) {
            return ConfType.NotWellFormed;
        } else {
            throw new TestGenException("uninizialed");
        }
    }
}
