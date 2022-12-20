use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("./advent/src/data.txt") {
        let mut calories: u64 = 0u64;
        let mut elves: Vec<u64> = Vec::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip == "".to_string() {
                    if elves.len() < 3 {
                        elves.push(calories);
                    } else {
                        let min = *elves.iter().min().unwrap();
                        if calories > min {
                            elves.remove(elves.iter().position(|x| *x == min).unwrap());
                            elves.push(calories);
                        }
                    }
                    // println!("{}", calories);
                    calories = 0u64;
                }

                if ip != "".to_string() {
                    calories += ip.parse::<u64>().unwrap();
                }
            }
        }

        let sum: u64 = elves.iter().sum();
        println!("{}", sum);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
