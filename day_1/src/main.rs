use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let inputs: Result<Vec<i64>, &'static str> = stdin
        .lock()
        .lines()
        .filter_map(|line_result| {
            line_result
                .map_err(|_| "Could not read a line.")
                .map(|line| if line.trim() == "" {
                    None
                } else {
                    Some(line)
                })
                .transpose()
                .map(|line_result| line_result.and_then(|line|
                    line.parse::<i64>().map_err(|_| "A line was invalid.")
                ))
        }).collect();


    match inputs {
        Ok(changes) => {
            let result = calibrate(changes);
            println!("{}", result)
        },
        Err(error) => eprintln!("The input was invalid: {}", error),
    }
}


fn calibrate(changes: Vec<i64>) -> i64 {
    changes.iter().fold(0, |acc, change| acc + change)
}