use std::collections::HashSet;
use std::io;
use std::io::BufRead;

use fallible_iterator::{self, FallibleIterator};

fn main() {
    let stdin = io::stdin();
    let input =
        fallible_iterator::convert(stdin.lock().lines()).map_err(|_| "A line could not be read.");
    let input = sanitize(input).collect();

    match input {
        Ok(changes) => println!("{}", calibrate(changes)),
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

fn calibrate(changes: Vec<i64>) -> i64 {
    let mut visited_set = HashSet::with_capacity(changes.len());
    visited_set.insert(0);
    changes
        .iter()
        .cycle()
        .try_fold((0, visited_set), |(acc, mut visited), change| {
            let frequency = acc + change;
            if visited.insert(frequency) {
                Ok((frequency, visited))
            } else {
                Err(frequency)
            }
        })
        .unwrap_err()
}
