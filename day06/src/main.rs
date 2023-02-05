use std::collections::HashMap;
use std::fs::read_to_string;

fn find_n_diff_char(input: &str, n: usize) -> Option<usize> {
    // Init counters
    let firstchar = input.chars().next().unwrap();
    let mut counters: HashMap<char, usize> = HashMap::new();
    counters.insert(firstchar, n);
    let mut window: Vec<char> = (0..n).map(|_| firstchar).collect();
    // Go through input
    for (i, char) in input.chars().enumerate() {
        // Remove old char
        let oldchar = window[i % n];
        let counter = counters.entry(oldchar).and_modify(|c| *c -= 1).or_insert(0);
        if *counter == 0 {
            counters.remove(&oldchar);
        }
        // Add new char
        window[i % n] = char;
        counters.entry(char).and_modify(|c| *c += 1).or_insert(1);
        // Check if 14 different chars
        if counters.len() == n {
            return Some(i + 1);
        }
    }
    None
}

fn main() {
    // Read input
    let input = read_to_string("./data/input.txt").unwrap();

    // Part 1
    let start = find_n_diff_char(&input, 4).unwrap();
    println!("Part 1: {:?}", start);

    // Part 2
    let start = find_n_diff_char(&input, 14).unwrap();
    println!("Part 2: {:?}", start);
}
