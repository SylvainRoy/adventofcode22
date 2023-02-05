use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    // Read input
    let input = read_to_string("./data/input.txt")?;
    let pairs = input
        .lines()
        .map(|line| {
            let vals: Vec<u32> = line
                .split(|c| c == ',' || c == '-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            ((vals[0], vals[1]), (vals[2], vals[3]))
        })
        .collect::<Vec<((u32, u32), (u32, u32))>>();

    // Part 1
    let full_overlaps: u32 = pairs
        .iter()
        .map(|((a, b), (c, d))| {
            if a <= c && d <= b {
                1
            } else if c <= a && b <= d {
                1
            } else {
                0
            }
        })
        .sum();
    println!("Part 1: {:?}", full_overlaps);

    // Part 2
    let partial_overlaps: u32 = pairs
        .iter()
        .map(|((a, b), (c, d))| {
            if a <= c && c <= b {
                1
            } else if a <= d && d <= b {
                1
            } else if c <= a && b <= d {
                1
            } else {
                0
            }
        })
        .sum();
    println!("Part 2: {:?}", partial_overlaps);

    Ok(())
}
