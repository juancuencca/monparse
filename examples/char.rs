use monparse::{char_parser};

fn main() {
    let a_parser = char_parser('a');
    let input = "abc";
    let result = a_parser.run(input);

    match result {
        Ok(tuple) => println!("{:?}", tuple),
        Err(e) => println!("{}", e),
    }
}