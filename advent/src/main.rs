use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("./advent/src/data.txt") {
        let mut count: u64 = 0u64;
        let mut point: u64 = 0u64;
        let mut calories: u64 = 0u64;
        let mut elf: u64 = 0u64;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip == "".to_string() {
                    println!("{} {}", elf, calories);

                    if elf > calories {
                        calories = elf;
                        point = count;
                    }

                    count += 1u64;
                    elf = 0u64;
                }

                if ip != "".to_string() {
                    elf += ip.parse::<u64>().unwrap();
                }
            }
        }

        println!("{}", count);
        println!("{}", calories);
        println!("{}", point);
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
