package com.matthewjamesbriggs.xmltestgen;

import lombok.AllArgsConstructor;
import lombok.Getter;

@AllArgsConstructor class ConfTestCases {
    @Getter
    private final String prefix;
    @Getter
    private final String profile;
}
