use monparse::{Input, Parser};

fn main() {
    let parser = Parser::new(|input: Input| {
        let new_input = Input {
            text: input.text.clone(),
            pos: input.pos + 1,
        };

        (new_input, Ok(()))
    });

    let input = Input::new("example");
    let (new_input, result) = (parser.run)(input);

    println!("New position: {}", new_input.pos);
    match result {
        Ok(_) => println!("Parsing succeed!"),
        Err(err) => println!("Parsing failed: {}", err),
    }
}