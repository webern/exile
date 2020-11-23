/// is the char one of `(#x20 | #x9 | #xD | #xA)`, i.e. space, tab, carriage return, or line feed.
pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

// contains `(#x20 | #x9 | #xD | #xA)`, i.e. space, tab, carriage return, or line feed.
// pub fn contains_whitespace<S: AsRef<str>>(s: S) -> bool {
//     for c in s.as_ref().chars() {
//         if is_whitespace(c) {
//             return true;
//         }
//     }
//     false
// }
