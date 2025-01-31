use monparse::{char_parser};

fn main() {
    let a_parser = char_parser('a').map(|c| c.to_uppercase().to_string());
    let input = "abc";
    let result = a_parser.run(input);

    match result {
        Ok(tuple) => println!("{:?}", tuple),
        Err(e) => println!("{}", e),
    }
}