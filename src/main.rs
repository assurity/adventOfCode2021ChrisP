use std::fs::File;
// use std::fs::OpenOptions;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt";

    // let mut output = File::create(path)?;
    // let mut output = OpenOptions.``
    // write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut previous = -1;
    let mut current;
    let mut increase_count = 0;
    for line in buffered.lines() {
        // println!("{}", line?);
        let my_int = line?.parse::<i32>().unwrap();
        // println!("{}", my_int);
        current = my_int;
        if previous != -1 {
            if current > previous {
                print_evaluation(&previous, &current, "increased");
                increase_count = increase_count + 1;
            } else {
                print_evaluation(&previous, &current, "decreased");
            }
        } else {
            println!("skipping first");
        }
        previous = current;

    }

    println!("Total increase count = {}", increase_count);
    Ok(())
}

fn print_evaluation(previous: &i32, current: &i32, direction_label: &str) {
    println!("{} -> {} == {}", previous, current, direction_label);
}