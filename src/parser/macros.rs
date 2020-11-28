/// Creates a ParseError object.
/// parser_state: required as the first argument
/// message: optional, can be a string or a format
macro_rules! create_parser_error {
    // required: first argument must be the ParserState object
    ($parser_state:expr) => {
        crate::parser::error::parse_err(
            $parser_state,
            throw_site!(),
            Option::<String>::None,
            Option::<crate::parser::error::ParseError>::None,
        )
    };
    // optional: second argument can be a simple string message
    ($parser_state:expr, $msg:expr) => {
        crate::parser::error::parse_err(
            $parser_state,
            throw_site!(),
            Some($msg),
            Option::<crate::parser::error::ParseError>::None,
        )
    };
    ($parser_state:expr, $fmt:expr, $($arg:expr),+) => {
        crate::parser::error::parse_err(
            $parser_state,
            throw_site!(),
            Some(format!($fmt, $($arg),+)),
            Option::<crate::parser::error::ParseError>::None,
        )
    };
}

/// Creates a ParseError object, requires an 'Iter' and the expected 'char'.
macro_rules! expect {
    ($iter:expr, $c:expr) => {
        $iter.expect($c, throw_site!())
    };
}

/// Creates a Result populated by a ParseError
/// iter: required as the first argument, `Iter`
/// message: optional, can be a string or a format
macro_rules! parse_err {
    // required: first argument must be the Iter object
    ($iter:expr) => { Err(create_parser_error!(&$iter.st)) };
    // optional: second argument can be a simple string message
    ($iter:expr, $msg:expr) => { Err(create_parser_error!(&$iter.st, $msg) ) };
    // optional: format!
    ($iter:expr, $fmt:expr, $($arg:expr),+) => {
        Err(create_parser_error!(&$iter.st, $fmt, $($arg),+))
    };
}

macro_rules! from_xe {
    ($iter:expr, $xdoc_error:expr) => {{
        create_parser_error!(&$iter.st, "{}", $xdoc_error)
    }};
}
