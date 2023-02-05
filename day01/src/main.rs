use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./data/input.txt")?;
    let mut elves: Vec<u32> = input
        .lines()
        .collect::<Vec<_>>()
        .split(|l| l.len() == 0)
        .map(|group| group.iter().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect();

    let max = elves.iter().max();
    println!("Part 1: {:?}", max.unwrap());

    elves.sort_unstable();
    let len = elves.len();
    println!(
        "Part 2: {:?}",
        elves[len - 1] + elves[len - 2] + elves[len - 3]
    );

    Ok(())
}
