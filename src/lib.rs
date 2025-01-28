use std::fmt;

pub struct Input {
    pub text: String,
    pub pos: usize,
}

impl Input {
    pub fn new(text: impl Into<String>) -> Self {
        Input { text: text.into(), pos: 0, }
    }
}

pub struct Error {
    desc: String,
    pos: usize,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed at {}: {}", self.pos, self.desc)
    }
}
pub struct Parser<'a, A> {
    pub run: Box<dyn Fn(Input) -> (Input, Result<A, Error>) + 'a>,
}

impl<'a, A> Parser<'a, A> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(Input) -> (Input, Result<A, Error>) + 'a
    {
        Parser { run: Box::new(f), }
    } 
}