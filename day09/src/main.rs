use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

enum Dir {
    U,
    R,
    D,
    L,
}
struct Move {
    dir: Dir,
    len: usize,
}

impl Move {
    fn parse(desc: &str) -> Self {
        let mut tokens = desc.split_whitespace();
        let dir = tokens.next().unwrap();
        let len: usize = tokens.next().unwrap().parse().unwrap();
        match dir {
            "U" => Move { dir: Dir::U, len },
            "R" => Move { dir: Dir::R, len },
            "D" => Move { dir: Dir::D, len },
            "L" => Move { dir: Dir::L, len },
            _ => panic!("Unkown command"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn mv(&mut self, dir: &Dir) {
        match dir {
            Dir::U => self.y += 1,
            Dir::R => self.x += 1,
            Dir::D => self.y -= 1,
            Dir::L => self.x -= 1,
        }
    }

    fn follow(&mut self, head: &Position) {
        let diffx = head.x - self.x;
        let diffy = head.y - self.y;
        if diffx.abs() > 1 || diffy.abs() > 1 {
            self.x += diffx.signum();
            self.y += diffy.signum();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read input
    let input = read_to_string("./data/input.txt")?;
    let moves = input.lines().map(Move::parse);

    // Part 1
    let mut path = HashSet::new();
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    for mv in moves.clone() {
        for _i in 0..mv.len {
            head.mv(&mv.dir);
            tail.follow(&head);
            path.insert(tail.clone());
        }
    }
    println!("Part 1: {:?}", path.len());

    // Part 2
    let mut path = HashSet::new();
    let mut knots: Vec<Position> = (0..10).map(|_| Position { x: 0, y: 0 }).collect();
    for mv in moves {
        for _step in 0..mv.len {
            knots[0].mv(&mv.dir);
            for i in 0..9 {
                let k = knots[i].clone();
                knots[i + 1].follow(&k);
            }
            path.insert(knots[9].clone());
        }
    }
    println!("Part 2: {:?}", path.len());

    Ok(())
}
