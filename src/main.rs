use std::fs::File;
// use std::fs::OpenOptions;
use std::io::{BufReader, BufRead, Lines};
use std::iter::Map;
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> Map<Lines<BufReader<File>>, fn(std::io::Result<String>) -> String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
}

struct Accumulator {
    previous: i32,
    accumulation: i32,
}

fn main() {
    let path = "input.txt";

    let lines = lines_from_file(&path);
    let numbers = lines.map(|l| l.parse::<i32>().unwrap());

    let result: Accumulator = numbers.fold(Accumulator {
        previous: -1,
        accumulation: 0,
    }, evaluate_current_and_previous);

    println!("Total increase count = {}", result.accumulation);
}

fn evaluate_current_and_previous(accumulator: Accumulator, current: i32) -> Accumulator {
    let mut new_accumulator = Accumulator {
        previous: current,
        accumulation: accumulator.accumulation, // no increment!
    };
    if is_first_record(accumulator.previous) {
        println!("skipping first");
        return new_accumulator;
    }

    let increment_by = evaluate_if_increase_or_decrease(accumulator.previous, current);

    new_accumulator.accumulation = accumulator.accumulation + increment_by;
    return new_accumulator;
}

fn evaluate_if_increase_or_decrease(previous: i32, current: i32) -> i32 {
    return if current > previous {
        print_evaluation(previous, current, "increased");
        1
    } else {
        print_evaluation(previous, current, "decreased");
        0
    }
}

fn is_first_record(previous: i32) -> bool {
    previous == -1
}

fn print_evaluation(previous: i32, current: i32, direction_label: &str) {
    println!("{} -> {} == {}", previous, current, direction_label);
}