use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
enum Command {
    Noop,
    Addx(isize),
}

impl Command {
    fn parse(str: &str) -> Self {
        let mut tokens = str.split_whitespace();
        match tokens.next() {
            Some("noop") => Command::Noop,
            Some("addx") => Command::Addx(tokens.next().unwrap().parse::<isize>().unwrap()),
            _ => panic!("Unsupported command."),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read input
    let input = read_to_string("./data/input.txt")?;
    let commands = input.lines().map(Command::parse);

    // Part 1
    let registers = commands
        .flat_map(|command| match command {
            Command::Addx(value) => {
                vec![0, value]
            }
            Command::Noop => {
                vec![0]
            }
        })
        .fold(vec![1, 1], |mut acc, value| {
            let prev_reg = if let Some(val) = acc.last() { *val } else { 1 };
            acc.push(prev_reg + value);
            acc
        });
    let score: isize = registers
        .iter()
        .enumerate()
        .map(|(cycle, reg)| {
            if (cycle as isize - 20) % 40 == 0 {
                cycle as isize * *reg
            } else {
                0
            }
        })
        .sum();
    println!("Part 1: {:?}", score);

    // Part 2
    let mut crt = String::new();
    let mut reg = registers.iter().skip(1);
    for i in 1..=240 {
        let regindex = reg.next().unwrap() + 1;
        let crtindex = (i - 1) % 40 + 1;
        if regindex - 1 <= crtindex && crtindex <= regindex + 1 {
            crt.push('#');
        } else {
            crt.push(' ');
        }
    }
    println!("Part 2:");
    for i in 0..6 {
        println!("        {}", crt.get(i * 40..(i + 1) * 40).unwrap());
    }

    Ok(())
}
