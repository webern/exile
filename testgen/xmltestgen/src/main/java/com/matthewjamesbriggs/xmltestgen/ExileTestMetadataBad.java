package com.matthewjamesbriggs.xmltestgen;

import com.google.gson.annotations.SerializedName;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;

@AllArgsConstructor @NoArgsConstructor public class ExileTestMetadataBad {
    @SerializedName("character_position")
    @Getter
    private int characterPosition;
    @Getter
    private int line;
    @Getter
    private int column;
}
