/// is the char one of `(#x20 | #x9 | #xD | #xA)`, i.e. space, tab, carriage return, or line feed.
pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}
