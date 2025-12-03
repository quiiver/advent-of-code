use std::usize;
use std::env;
use std::fs;

mod solutions;

mod utils {
    use super::*;
    pub fn read_file(day: u8, test: bool) -> String {
        let cwd = env::current_dir().unwrap();
        let test_suffix = if test { "-test" } else { "" };
        let fname = format!("day{:02}{}.txt", day, test_suffix);
        println!("Opening day file: {}", fname);
        let filepath = cwd
            .join("inputs")
            .join(fname);

        let f = fs::read_to_string(filepath);
        f.expect("could not open input file")
    }
}

static SOLUTIONS: &[fn(&String)] = &[
    solutions::day01::solution,
    solutions::day02::solution,
];

fn main() {
    let day_arg = env::args().nth(1).map(|x| x.parse::<usize>());
    let test_arg = env::args().nth(2).map(|s| s == "test").unwrap_or(false);
    match day_arg {
        Some(Ok(day)) => {
            if day <= SOLUTIONS.len() {
                SOLUTIONS[day-1](&utils::read_file(day as u8, test_arg))
            } else {
                println!("Day argument out of bounds")
            }
        },
        Some(Err(_)) => {
            println!("Invalid day argument");
        },
        None => {
            for s in SOLUTIONS.into_iter().enumerate() {
                match s {
                    (idx, solution) => {
                        solution(&utils::read_file((idx + 1) as u8, test_arg))
                    }
                }
            }
        }
    }
}
