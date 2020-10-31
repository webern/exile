package com.matthewjamesbriggs.xmltestgen;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;

@AllArgsConstructor @NoArgsConstructor class ExileTestMetadata {
    @Getter
    private String description;
    @Getter
    private ExileTestMetadataSyntax syntax;
    @Getter
    private ExileTestMetadataExpected expected;

    Recommendation getRecommendation() throws TestGenException {
        if (expected == null) {
            return Recommendation.NS1_0;
        } else {
            return expected.getRecommentation();
        }
    }

    public XmlVersion getXmlVersion() throws TestGenException {
        if (expected == null) {
            return XmlVersion.V10;
        } else {
            return expected.getXmlVersion();
        }
    }

    static boolean getNamespace() {
        // TODO - get this from the JSON if we decide to write non-namespace tests
        return true;
    }

    static Entities getEntities() {
        // TODO - get this from the JSON if we decide to support entity tests
        return Entities.None;
    }
}
