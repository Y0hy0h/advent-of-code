use std::collections::HashMap;
use std::io;
use std::io::BufRead;

use fallible_iterator::{self, FallibleIterator};

fn main() {
    let stdin = io::stdin();
    let input =
        fallible_iterator::convert(stdin.lock().lines()).map_err(|_| "A line could not be read.");
    let input = sanitize(input).collect();

    match input {
        Ok(inputs) => println!("{}", common(inputs)),
        Err(error) => eprintln!("The input was invalid: {}", error),
    }
}

type Error = &'static str;

fn sanitize(
    input: impl FallibleIterator<Item = String, Error = Error>,
) -> impl FallibleIterator<Item = String, Error = Error> {
    input.filter(|line| Ok(line.trim() != ""))
}

fn common(boxes: Vec<String>) -> String {
    let mut common: Option<String> = None;

    for label in &boxes {
        let length = label.len();

        for other in &boxes {
            let in_common = common_chars(label, other);
            if in_common.len() == length - 1 {
                common = Some(in_common);
            }
        }
    }

    common.unwrap()
}

fn common_chars(first: &String, second: &String) -> String {
    let mut common = String::with_capacity(first.len());

    for (c1, c2) in first.chars().zip(second.chars()) {
        if c1 == c2 {
            common.push(c1)
        }
    }

    common
}
