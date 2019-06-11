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
        Ok(changes) => println!("{}", checksum(changes)),
        Err(error) => eprintln!("The input was invalid: {}", error),
    }
}

type Error = &'static str;

fn sanitize(
    input: impl FallibleIterator<Item = String, Error = Error>,
) -> impl FallibleIterator<Item = String, Error = Error> {
    input.filter(|line| Ok(line.trim() != ""))
}

fn checksum(boxes: Vec<String>) -> i64 {
    let mut doubles = 0;
    let mut triples = 0;

    for label in boxes {
        let mut counts = HashMap::new();
        for c in label.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        if counts.values().any(|&count| count == 2) {
            doubles += 1;
        };
        if counts.values().any(|&count| count == 3) {
            triples += 1;
        }
    }

    doubles * triples
}
