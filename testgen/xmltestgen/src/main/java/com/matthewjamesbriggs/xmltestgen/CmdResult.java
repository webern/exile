package com.matthewjamesbriggs.xmltestgen;

import lombok.AllArgsConstructor;
import lombok.Getter;

@AllArgsConstructor public class CmdResult {
    @Getter
    private final String stdout;
    @Getter
    private final String stderr;
    @Getter
    private final int exit;
}
