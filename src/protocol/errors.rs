//! The error types used
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        FromUtf8(::std::string::FromUtf8Error);
    }
}
