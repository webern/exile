package com.matthewjamesbriggs.xmltestgen;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;

@AllArgsConstructor @NoArgsConstructor public class ExileTestMetadataDeclaration {
    @Getter
    private String version;
    @Getter
    private String encoding;

    Recommendation getRecommendation() throws TestGenException {
        XmlVersion v = getXmlVersion();
        if (v == XmlVersion.V10) {
            return Recommendation.NS1_0;
        } else {
            return Recommendation.NS1_1;
        }
    }

    public XmlVersion getXmlVersion() throws TestGenException {
        if (version == null) {
            return XmlVersion.V10;
        } else if (version.equals("v10")) {
            return XmlVersion.V10;
        } else if (version.equals("v11")) {
            return XmlVersion.V11;
        } else {
            throw new TestGenException("bad version value: " + version);
        }
    }
}
