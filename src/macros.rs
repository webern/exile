/// This macro is used internally to obtain the current file and line (in the sourcecode).
macro_rules! throw_site {
    () => {
        crate::error::ThrowSite {
            file: file!().to_owned(),
            line: line!(),
        }
    };
}
