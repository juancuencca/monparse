use std::rc::Rc;

type ParseResult<'a, A> = Result<(A, &'a str), String>;
type ParseFn<'a, A> = Rc<dyn Fn(&'a str) -> ParseResult<'a, A> + 'a>;

pub struct Parser<'a, A> {
    parse: ParseFn<'a, A>,
}

impl<'a, A: 'a> Parser<'a, A> {
    fn new<F>(f: F) -> Self
    where 
        F: Fn(&'a str) -> ParseResult<'a, A> + 'a
    {
        Parser {
            parse: Rc::new(f),
        }
    }

    pub fn run(&self, input: &'a str) -> ParseResult<'a, A> {
        (self.parse)(input)
    }

    pub fn map<F, B: 'a>(self, f: F) -> Parser<'a, B> 
    where
        F: Fn(A) -> B + 'a 
    {
        Parser::new(move |input| {
            self.run(input).map(|(a, slice)| (f(a), slice))
        })
    }
}

pub fn char_parser<'a>(c: char) -> Parser<'a, char> {
    Parser::new(move |input: &'a str| {
        match input.chars().next() {
            Some(first) if first == c => Ok((c, &input[c.len_utf8()..])),
            _ => Err(format!("Expected '{}', but got something else", c)),
        }
    })
}

pub fn any_char<'a>() -> Parser<'a, char> {
    Parser::new(move |input: &'a str| {
        match input.chars().next() {
            Some(first) => Ok((first, &input[first.len_utf8()..])),
            None => Err(format!("Expected a character but got an end of a string")),
        }
    })
}