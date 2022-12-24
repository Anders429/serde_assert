use alloc::string::String;
use alloc::string::ToString;
use core::fmt;
use core::fmt::Display;
use serde::ser;

#[derive(Debug)]
pub struct Serializer {
    is_human_readable: bool,
}

impl Serializer {
    pub fn builder() -> Builder {
        Builder::default()
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    is_human_readable: Option<bool>,
}

impl Builder {
    pub fn is_human_readable(&mut self, is_human_readable: bool) -> &mut Self {
        self.is_human_readable = Some(is_human_readable);
        self
    }

    pub fn build(&mut self) -> Serializer {
        Serializer {
            is_human_readable: self.is_human_readable.unwrap_or(true),
        }
    }
}

#[derive(Debug)]
pub struct Error(pub String);

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl ser::StdError for Error {}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self where T: Display {
        Self(msg.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use alloc::format;
    use serde::ser::Error as _;

    #[test]
    fn custom_error() {
        let error = Error::custom("foo");

        assert_eq!(error.0, "foo");
    }

    #[test]
    fn display_error() {
        let formatted = format!("{}", Error::custom("foo"));

        assert_eq!(formatted, "foo");
    }
}
