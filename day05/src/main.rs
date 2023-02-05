use std::fs::read_to_string;

fn main() {
    // Read input
    let input = read_to_string("./data/input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let groups = lines.split(|l| l.len() == 0).collect::<Vec<_>>();
    let mut cargo: Vec<&str> = groups[0].into();
    let procedure = groups[1];

    // Build stack structure
    cargo.reverse();
    let numstacks = cargo[0]
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = (0..numstacks).into_iter().map(|_| Vec::new()).collect();
    for line in cargo.iter().skip(1) {
        for (i, chunk) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            match chunk {
                [_, c, _, _] => {
                    if *c != ' ' {
                        stacks[i].push(*c);
                    }
                }
                [_, c, _] => {
                    if *c != ' ' {
                        stacks[i].push(*c);
                    }
                }
                _ => break,
            }
        }
    }

    // Build commands
    let commands = procedure
        .iter()
        .map(|l| {
            let words = l.split_whitespace().collect::<Vec<&str>>();
            (
                words[1].parse::<usize>().unwrap(),
                words[3].parse::<usize>().unwrap(),
                words[5].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize, usize)>>();

    // Run the program of part 1
    let mut stacks1 = stacks.clone();
    for (num, from, to) in &commands {
        for _ in 0..*num {
            let val = stacks1[from - 1].pop().unwrap();
            stacks1[to - 1].push(val);
        }
    }
    let result: String = stacks.iter().map(|s| s.last().unwrap()).collect();
    println!("Part 1: {:?}", result);

    // Run the program of part 2
    for (num, from, to) in &commands {
        let mut temp = Vec::new();
        for _ in 0..*num {
            temp.push(stacks[from - 1].pop().unwrap());
        }
        for _ in 0..*num {
            stacks[to - 1].push(temp.pop().unwrap());
        }
    }
    let result: String = stacks.iter().map(|s| s.last().unwrap()).collect();
    println!("Part 2: {:?}", result);
}
