use std::fs::File;
use std::io::{self};

fn main() {
    println!("Hello, world!");

    let file = File::open("./advent/src/data.txt").unwrap();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(io::BufReader::new(file));

    let mut val: u64 = 0u64;
    for result in rdr.records() {
        let record = result.unwrap();
        val += play(&record[0], &record[1]);
    }

    println!("{}", val);
}

fn play(opp: &str, user: &str) -> u64 {
    println!("{} {}", opp, user);

    let score: u64 = match opp {
        "A" => match user {
            "X" => 3u64,
            "Y" => 4u64,
            "Z" => 8u64,
            _ => todo!(),
        },
        "B" => match user {
            "X" => 1u64,
            "Y" => 3u64 + 2u64,
            "Z" => 6u64 + 3u64,
            _ => todo!(),
        },
        "C" => match user {
            "X" => 0u64 + 2u64,
            "Y" => 3u64 + 3u64,
            "Z" => 6u64 + 1u64,
            _ => todo!(),
        },
        _ => todo!(),
    };

    return score;
}
