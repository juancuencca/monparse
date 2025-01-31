type ParseResult<'a, A> = Result<(A, &'a str), String>;

pub struct Parser<'a, 'b, A> {
    parse: Box<dyn Fn(&'a str) -> ParseResult<'a, A> + 'b>,
}

impl<'a, 'b, A> Parser<'a, 'b, A> {
    fn new<F>(f: F) -> Self 
    where
        F: Fn(&'a str) -> ParseResult<'a, A> + 'b
    {
        Parser { parse: Box::new(f), }
    }

    pub fn run(&self, input: &'a str) -> ParseResult<'a, A> {
        (self.parse)(input)
    }
}

pub fn char_parser<'a, 'b>(c: char) -> Parser<'a, 'b, char> {
    Parser::new(move |input: &'a str| {
        match input.chars().next() {
            Some(first) if first == c => Ok((first, &input[first.len_utf8()..])),
            _ => Err(format!("Expected character {}, but got something else", c)),
        }
    })
}

pub fn any_char<'a, 'b>() -> Parser<'a, 'b, char> {
    Parser::new(|input: &'a str| {
        match input.chars().next() {
            Some(first) => Ok((first, &input[first.len_utf8()..])),
            None => Err(format!("Expected a character, but got end of input")),
        }
    })
}