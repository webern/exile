macro_rules! wrap_err {
    // Base case:
    ($err:expr) => (Err($crate::xdoc::error::XDocErr {
        message: "an error occurred".to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    }));
    ($err:expr, $msg:expr) => (Err($crate::xdoc::error::XDocErr {
        message: $msg.to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    }));
    ($err:expr, $fmt:expr, $($arg:expr),+) => (Err($crate::xdoc::error::XDocErr {
        message: format!($fmt, $($arg),+),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    }));
}

macro_rules! better_wrap {
    ($result:expr) => {
        match $result {
            Ok(value) => Ok(value),
            Err(e) => wrap_err!(e),
        }
    };
}

// a convenience macro for creating a Result::Err
macro_rules! raise {
    // Base case:
    ($msg:expr) => (Err($crate::xdoc::error::XDocErr {
        message: $msg.to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: None,
    }));
    ($fmt:expr, $($arg:expr),+) => ($Err($crate::xdoc::error::XErr {
        message: format!($fmt, $($arg),+),
        file: file!().to_string(),
        line: line!() as u64,
        source: None,
    }));
}

macro_rules! xwrite {
    ($writer:expr, $fmt:expr) => ({
        better_wrap!(write!($writer, $fmt))
    });
    ($writer:expr, $fmt:expr, $($arg:expr),+) => ({
        better_wrap!(write!($writer, $fmt, $($arg)*))
    });
}
