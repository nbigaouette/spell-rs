use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<Error>>;

use spell::map::LcsMap;

fn main() -> Result<()> {
    let path = "data/var/log/messages";

    let max_lines: usize = std::env::args()
        .nth(1)
        .map(|i| i.parse().unwrap())
        .unwrap_or_else(|| usize::max_value());

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut map = LcsMap::new();

    buffered
        .lines()
        .filter_map(|line| match line {
            Ok(line) => {
                map.insert(&line);
                Some(())
                // println!("map: {:?}", map);
                // println!("======================\n{}", line);
                // println!("{}", serde_json::to_string(&map).unwrap());
            }
            Err(err) => {
                eprintln!("Error processing line: {:?}", err);
                None
            }
        })
        .enumerate()
        .take_while(|(i, _line)| *i + 1 < max_lines)
        .for_each(|(_i, _line)| {});

    Ok(())
}
