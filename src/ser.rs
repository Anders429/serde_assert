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
