use monparse::{any_char};

fn main() {
    let any_parser = any_char();
    let input = "abc";
    let result = any_parser.run(input);

    match result {
        Ok(tuple) => println!("{:?}", tuple),
        Err(e) => println!("{}", e),
    }
}