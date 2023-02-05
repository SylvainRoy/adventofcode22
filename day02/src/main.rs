use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    // Read input
    let input = read_to_string("./data/input.txt")?;

    // Part 1
    let score: i32 = input.lines().map(|line| play1(line)).sum();
    println!("Part 1: {:?}", score);

    // Part 2
    let score: i32 = input.lines().map(|line| play2(line)).sum();
    println!("Part 2: {:?}", score);

    Ok(())
}

fn play1(line: &str) -> i32 {
    let mut shapes = line.split_ascii_whitespace();
    let opponent = Shape::new(shapes.next().unwrap());
    let player = Shape::new(shapes.next().unwrap());
    player.play(&opponent)
}

fn play2(line: &str) -> i32 {
    let mut shapes = line.split_ascii_whitespace();
    let opponent = Shape::new(shapes.next().unwrap());
    let expected = shapes.next().unwrap();
    let player = match (expected, &opponent) {
        ("X", Shape::Rock) => Shape::Scissor,
        ("X", Shape::Paper) => Shape::Rock,
        ("X", Shape::Scissor) => Shape::Paper,
        ("Y", Shape::Rock) => Shape::Rock,
        ("Y", Shape::Paper) => Shape::Paper,
        ("Y", Shape::Scissor) => Shape::Scissor,
        ("Z", Shape::Rock) => Shape::Paper,
        ("Z", Shape::Paper) => Shape::Scissor,
        ("Z", Shape::Scissor) => Shape::Rock,
        _ => panic!("Unexpected line!"),
    };
    player.play(&opponent)
}

enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn new(letter: &str) -> Self {
        match letter {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissor,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissor,
            _ => panic!("Unsuported letter!"),
        }
    }

    fn value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }

    fn play(&self, opponent: &Shape) -> i32 {
        let won = match (self, opponent) {
            (Self::Rock, Self::Rock) => 3,
            (Self::Rock, Self::Paper) => 0,
            (Self::Rock, Self::Scissor) => 6,
            (Self::Paper, Self::Paper) => 3,
            (Self::Paper, Self::Scissor) => 0,
            (Self::Paper, Self::Rock) => 6,
            (Self::Scissor, Self::Rock) => 0,
            (Self::Scissor, Self::Paper) => 6,
            (Self::Scissor, Self::Scissor) => 3,
        };
        won + self.value()
    }
}
