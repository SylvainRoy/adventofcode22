use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

fn priority(char: char) -> usize {
    let val = char as usize;
    if val > 96 {
        val - 96
    } else {
        val - 38
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read input
    let input = read_to_string("./data/input.txt")?;

    // Part 1
    let mut sum = 0;
    for line in input.lines() {
        let len = line.len();
        let comp1 = line[..len / 2].chars().collect::<HashSet<char>>();
        let comp2 = line[len / 2..].chars().collect::<HashSet<char>>();
        let item = comp1.intersection(&comp2).next().unwrap();
        sum += priority(*item);
    }
    println!("Part 1: {:?}", sum);

    // Part 2
    let sum: usize = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let inter = chunk
                .iter()
                .map(|e| e.chars().collect::<HashSet<char>>())
                .reduce(|acc, rucksack| &acc & &rucksack)
                .unwrap();
            priority(*inter.iter().next().unwrap())
        })
        .sum();
    println!("Part 2: {:?}", sum);

    Ok(())
}
