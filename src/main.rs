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

fn main() {
    let path = "input.txt";

    let mut previous = -1;
    let mut increase_count = 0;

    let lines = lines_from_file(&path);
    let numbers = lines.map(|l| l.parse::<i32>().unwrap());

    let _result: Vec<i32> = numbers.map(|current| {
        if first_record(previous) {
            println!("skipping first");
            previous = current;
            return increase_count;
        }

        if current > previous {
            print_evaluation(&previous, &current, "increased");
            increase_count = increase_count + 1;
        } else {
            print_evaluation(&previous, &current, "decreased");
        }
        previous = current;
        return increase_count;
    }).collect();

    println!("Total increase count = {}", increase_count);
}

fn first_record(previous: i32) -> bool {
    previous == -1
}

fn print_evaluation(previous: &i32, current: &i32, direction_label: &str) {
    println!("{} -> {} == {}", previous, current, direction_label);
}