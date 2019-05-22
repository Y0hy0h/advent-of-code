use std::io;
use std::io::BufRead;

use fallible_iterator::{self, FallibleIterator};

fn main() {
    let stdin = io::stdin();
    let input =
        fallible_iterator::convert(stdin.lock().lines()).map_err(|_| "A line could not be read.");
    let input = sanitize(input);
    let result = frequency(input);

    match result {
        Ok(res) => println!("{}", res),
        Err(error) => eprintln!("The input was invalid: {}", error),
    }
}

type Error = &'static str;

fn sanitize(
    input: impl FallibleIterator<Item = String, Error = Error>,
) -> impl FallibleIterator<Item = i64, Error = Error> {
    input
        .filter(|line| Ok(line.trim() != ""))
        .map(|line| line.parse::<i64>().map_err(|_| "A line was invalid."))
}

fn frequency(changes: impl FallibleIterator<Item = i64, Error = Error>) -> Result<i64, Error> {
    changes.fold(0, |acc, change| Ok(acc + change))
}
