use std::fmt::{Display, Formatter};

/// Can be observed if something goes wrong.
#[derive(Debug)]
pub enum Error {
    /// A null pointer was observed where it wasn't expected.
    Null,

    /// An operation was requested that is not supported.
    Unsupported,

    /// Given string is not valid Ascii.
    Ascii,

    /// A test failed to execute.
    TestFailed,

    /// A specified file was not found.
    FileNotFound,
}
