// name start char range boundries
const U_00C0: char = '\u{00C0}';
const U_00D6: char = '\u{00D6}';
const U_00D8: char = '\u{00D8}';
const U_00F6: char = '\u{00F6}';
const U_00F8: char = '\u{00F8}';
const U_02FF: char = '\u{02FF}';
const U_0370: char = '\u{0370}';
const U_037D: char = '\u{037D}';
const U_037F: char = '\u{037F}';
const U_1FFF: char = '\u{1FFF}';
const U_200C: char = '\u{200C}';
const U_200D: char = '\u{200D}';
const U_2070: char = '\u{2070}';
const U_218F: char = '\u{218F}';
const U_2C00: char = '\u{2C00}';
const U_2FEF: char = '\u{2FEF}';
const U_3001: char = '\u{3001}';
const U_D7FF: char = '\u{D7FF}';
const U_F900: char = '\u{F900}';
const U_FDCF: char = '\u{FDCF}';
const U_FDF0: char = '\u{FDF0}';
const U_FFFD: char = '\u{FFFD}';
const U_10000: char = '\u{10000}';
const U_EFFFF: char = '\u{EFFFF}';

// name char range boundries
const U_00B7: char = '\u{00B7}';
const U_0300: char = '\u{0300}';
const U_036F: char = '\u{036F}';
const U_203F: char = '\u{203F}';
const U_2040: char = '\u{2040}';

pub(crate) fn is_name_start_char(c: char) -> bool {
    // https://www.w3.org/TR/2008/REC-xml-20081126/#NT-NameStartChar
    // [4]   	NameStartChar	   ::=   	":" | [A-Z] | "_" | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] |
    // [#xF8-#x2FF] | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] |
    // [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
    // let x = c as u64;
    matches!(c,
        'A'..='Z' |
        'a'..='z' |
        ':' |
        '_' |
        U_00C0..=U_00D6 |
        U_00D8..=U_00F6 |
        U_00F8..=U_02FF |
        U_0370..=U_037D |
        U_037F..=U_1FFF |
        U_200C..=U_200D |
        U_2070..=U_218F |
        U_2C00..=U_2FEF |
        U_3001..=U_D7FF |
        U_F900..=U_FDCF |
        U_FDF0..=U_FFFD |
        U_10000..=U_EFFFF)
}

pub(crate) fn is_name_char(c: char) -> bool {
    // https://www.w3.org/TR/2008/REC-xml-20081126/#NT-NameChar
    // [4a] NameChar ::= NameStartChar | "-" | "." | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]
    if is_name_start_char(c) {
        return true;
    }
    matches!(c, U_00B7 | U_0300..=U_036F | U_203F..=U_2040 | '0'..='9' | '-' | '.')
}
