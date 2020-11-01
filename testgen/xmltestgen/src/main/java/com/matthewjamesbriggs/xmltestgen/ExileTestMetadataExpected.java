package com.matthewjamesbriggs.xmltestgen;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;

@AllArgsConstructor @NoArgsConstructor class ExileTestMetadataExpected {
    @Getter
    private ExileTestMetadataDeclaration declaration;

    Recommendation getRecommentation() throws TestGenException {
        if (declaration == null) {
            return Recommendation.NS1_0;
        } else {
            return declaration.getRecommendation();
        }
    }

    public XmlVersion getXmlVersion() throws TestGenException {
        if (declaration == null) {
            return XmlVersion.V10;
        } else {
            return declaration.getXmlVersion();
        }
    }
}
