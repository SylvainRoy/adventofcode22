use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;

#[derive(Debug)]
struct Sand {
    x: isize,
    y: isize,
}

impl Sand {
    fn new() -> Self {
        Self { x: 500, y: 0 }
    }

    fn mv1(&mut self, rocks: &HashMap<(isize, isize), char>, bottom_rock: isize) -> State {
        if !rocks.contains_key(&(self.x, self.y + 1)) {
            self.y += 1;
            if self.y > bottom_rock {
                State::Freefall
            } else {
                State::Fall
            }
        } else if !rocks.contains_key(&(self.x - 1, self.y + 1)) {
            self.x -= 1;
            self.y += 1;
            if self.y > bottom_rock {
                State::Freefall
            } else {
                State::Fall
            }
        } else if !rocks.contains_key(&(self.x + 1, self.y + 1)) {
            self.x += 1;
            self.y += 1;
            if self.y > bottom_rock {
                State::Freefall
            } else {
                State::Fall
            }
        } else {
            State::Stopped
        }
    }

    fn mv2(&mut self, rocks: &HashMap<(isize, isize), char>, floor: isize) -> State {
        if self.y + 1 == floor {
            State::Stopped
        } else if !rocks.contains_key(&(self.x, self.y + 1)) {
            self.y += 1;
            State::Fall
        } else if !rocks.contains_key(&(self.x - 1, self.y + 1)) {
            self.x -= 1;
            self.y += 1;
            State::Fall
        } else if !rocks.contains_key(&(self.x + 1, self.y + 1)) {
            self.x += 1;
            self.y += 1;
            State::Fall
        } else {
            State::Stopped
        }
    }
}

enum State {
    Freefall,
    Fall,
    Stopped,
}

fn main() {
    // Read input and store rocks in a hasmap
    let input = read_to_string("./data/input.txt").unwrap();
    let walls: Vec<Vec<(isize, isize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pos| {
                    let mut coord = pos.split(',');
                    (
                        coord.next().unwrap().parse::<isize>().unwrap(),
                        coord.next().unwrap().parse::<isize>().unwrap(),
                    )
                })
                .collect::<Vec<(isize, isize)>>()
        })
        .collect();
    let mut rocks: HashMap<(isize, isize), char> = HashMap::new();
    let mut bottom_rock = 0;
    for wall in walls {
        for (org, dest) in zip(wall.iter(), wall.iter().skip(1)) {
            bottom_rock = bottom_rock.max(org.1).max(dest.1);
            if org.0 < dest.0 {
                for x in org.0..=dest.0 {
                    rocks.insert((x, org.1), '#');
                }
            } else if org.0 > dest.0 {
                for x in dest.0..=org.0 {
                    rocks.insert((x, org.1), '#');
                }
            } else if org.1 < dest.1 {
                for y in org.1..=dest.1 {
                    rocks.insert((org.0, y), '#');
                }
            } else if org.1 > dest.1 {
                for y in dest.1..=org.1 {
                    rocks.insert((org.0, y), '#');
                }
            } else {
                panic!("This shouldn't happen.");
            }
        }
    }
    let mut rocks2 = rocks.clone();

    // Part 1
    let mut num_sand = 0;
    'moresand: loop {
        // Create a unit of sand and move it down until stopped or free fall.
        let mut sand = Sand::new();
        loop {
            match sand.mv1(&rocks, bottom_rock) {
                State::Freefall => break 'moresand,
                State::Fall => (),
                State::Stopped => {
                    num_sand += 1;
                    rocks.insert((sand.x, sand.y), 'o');
                    break;
                }
            }
        }
    }
    println!("Part 1: {:?}", num_sand);

    // Part 2
    let mut num_sand = 0;
    'moresand: loop {
        // Create a unit of sand and move it down until stopped
        let mut sand = Sand::new();
        loop {
            match sand.mv2(&rocks2, bottom_rock + 2) {
                State::Freefall => panic!("This shouldn't happen."),
                State::Fall => (),
                State::Stopped => {
                    num_sand += 1;
                    rocks2.insert((sand.x, sand.y), 'o');
                    if sand.y == 0 {
                        break 'moresand;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    println!("Part 2: {:?}", num_sand);
}
