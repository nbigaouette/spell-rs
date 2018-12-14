use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<Error>>;

use spell::map::LcsMap;

fn main() -> Result<()> {
    let path = "data/var/log/messages";

    let max_lines: usize = std::env::args()
        .nth(1)
        .map(|i| i.parse())
        .ok_or_else(|| usize::max_value())
        .unwrap()
        .unwrap();

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut map = LcsMap::new();

    buffered
        .lines()
        .enumerate()
        .take_while(|(i, _line)| *i < max_lines)
        .for_each(|(_i, line)| match line {
            Ok(line) => {
                map.insert(&line);
                println!("map: {:?}", map);
            }
            Err(err) => eprintln!("Error processing line: {:?}", err),
        });

    Ok(())
}
