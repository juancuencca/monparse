use std::fmt;

struct Input {
    text: String,
    position: usize,
}

impl Input {
    fn new(text: impl Into<String>) -> Self {
        Input { text: text.into(), position: 0, }
    }

    fn sub(&self, start: usize, len: usize) -> Self {
        let text = self.text.get(start..start + len).unwrap_or("").to_string();

        Input { text, position: self.position + start, }
    }
}

struct Error {
    description: String,
    position: usize,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed at {}: {}", self.position, self.description)
    }
}

struct Parser<'a, A> {
    run: Box<dyn Fn(Input) -> (Input, Result<A, String>) + 'a>,
}

impl<'a, A> Parser<'a, A> {
    fn new<F>(f: F) -> Self
    where
        F: Fn(Input) -> (Input, Result<A, String>) + 'a
    {
        Parser { run: Box::new(f), }
    } 

    fn fail<E>(error: E) -> Self
    where 
        E: Into<String> + Clone + 'a 
    {
        Parser::new(move |input: Input| {
            (input, Err(error.clone().into()))
        })
    }

    fn wrap(value: A) -> Self
    where
        A: Clone + 'a 
    {
        Parser::new(move |input: Input| {
            (input, Ok(value.clone()))
        })
    }
}