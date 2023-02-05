use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

const LEVELS: usize = 2048;

#[derive(Debug, Clone)]
enum Form {
    Hline,
    Cross,
    Corner,
    Vline,
    Square,
}

#[derive(Clone)]
struct Rock {
    form: Form,
    x: isize,
    y: isize,
}

impl Rock {
    fn next_rock(num_rock: &mut isize, y: isize) -> Self {
        *num_rock += 1;
        match *num_rock % 5 {
            1 => Self {
                form: Form::Hline,
                x: 2,
                y,
            },
            2 => Self {
                form: Form::Cross,
                x: 2,
                y,
            },
            3 => Self {
                form: Form::Corner,
                x: 2,
                y,
            },
            4 => Self {
                form: Form::Vline,
                x: 2,
                y,
            },
            0 => Self {
                form: Form::Square,
                x: 2,
                y,
            },
            _ => panic!("This should not happen!"),
        }
    }

    fn pieces(&self) -> HashSet<(isize, isize)> {
        let x = self.x;
        let y = self.y;
        match self.form {
            Form::Hline => [
                (0 + x, 0 + y),
                (1 + x, 0 + y),
                (2 + x, 0 + y),
                (3 + x, 0 + y),
            ]
            .into_iter()
            .collect(),
            Form::Cross => [
                (1 + x, 0 + y),
                (0 + x, 1 + y),
                (1 + x, 1 + y),
                (2 + x, 1 + y),
                (1 + x, 2 + y),
            ]
            .into_iter()
            .collect(),
            Form::Corner => [
                (0 + x, 0 + y),
                (1 + x, 0 + y),
                (2 + x, 0 + y),
                (2 + x, 1 + y),
                (2 + x, 2 + y),
            ]
            .into_iter()
            .collect(),
            Form::Vline => [
                (0 + x, 0 + y),
                (0 + x, 1 + y),
                (0 + x, 2 + y),
                (0 + x, 3 + y),
            ]
            .into_iter()
            .collect(),
            Form::Square => [
                (0 + x, 0 + y),
                (1 + x, 0 + y),
                (0 + x, 1 + y),
                (1 + x, 1 + y),
            ]
            .into_iter()
            .collect(),
        }
    }

    fn overlap(&self, other: &Rock) -> bool {
        self.pieces().intersection(&other.pieces()).next() != None
    }

    fn max_x(&self) -> isize {
        match &self.form {
            Form::Hline => 3,
            Form::Cross => 4,
            Form::Corner => 4,
            Form::Vline => 6,
            Form::Square => 5,
        }
    }

    fn apply_wind(&mut self, chamber: &Chamber, direction: isize) {
        self.x += direction;
        if self.x < 0 || self.max_x() < self.x {
            self.x -= direction;
        }
        for level in 0.max(self.y - 3)..=self.y + 3 {
            for rock in chamber.levels(level as usize) {
                if self.overlap(&rock) {
                    self.x -= direction;
                    return;
                }
            }
        }
    }

    fn apply_gravity(&mut self, chamber: &Chamber) -> bool {
        self.y -= 1;
        if self.y < 0 {
            self.y += 1;
            return true;
        }
        for level in 0.max(self.y - 3)..=self.y + 3 {
            for rock in chamber.levels(level as usize) {
                if self.overlap(&rock) {
                    self.y += 1;
                    return true;
                }
            }
        }
        false
    }
}

struct Chamber {
    levels: Vec<Vec<Rock>>,
    start: usize,
    cols: [isize; 7],
}

impl Chamber {
    fn _print(&self, header: &str, block: &Rock) {
        println!("\n{}", header);
        let mut max_y = 0;
        let mut screen = HashMap::new();
        for level in &self.levels {
            for block in level {
                for piece in block.pieces() {
                    screen.insert(piece, '#');
                    max_y = max_y.max(piece.1);
                }
            }
        }
        for piece in block.pieces() {
            screen.insert(piece, '@');
            max_y = max_y.max(piece.1);
        }
        for y in (0..=max_y).rev() {
            for x in 0..7 {
                match screen.get(&(x, y)) {
                    Some(c) => print!("{}", c),
                    None => print!("{}", '.'),
                }
            }
            println!("");
        }
    }

    fn top_of_stack(&self) -> isize {
        *self.cols.iter().max().unwrap()
    }

    fn lowest_reachable(&self) -> isize {
        *self.cols.iter().min().unwrap()
    }

    fn levels(&self, level: usize) -> &Vec<Rock> {
        &self.levels[level % LEVELS]
    }

    fn levels_mut(&mut self, level: usize) -> &mut Vec<Rock> {
        &mut self.levels[level % LEVELS]
    }

    fn update_unreachable(&mut self) {
        let level = (self.lowest_reachable() - 4).min(0) as usize % LEVELS as usize;
        for i in (self.start % LEVELS)..(level % LEVELS) {
            self.levels[i].clear();
        }
        self.start = level;
    }
}

fn main() {
    let jets: Vec<isize> = read_to_string("./data/input.txt")
        .unwrap()
        .trim_end()
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!("Unkown direction!"),
        })
        .collect();

    let (mut r0, mut r1, mut r2) = (0, 0, 0);
    let (mut a0, mut a1) = (0, 0);
    let mut n1 = 0;
    let rt: isize = 1000000000000;

    let mut chamber = Chamber {
        levels: (0..LEVELS)
            .into_iter()
            .map(|_| Vec::new())
            .collect::<Vec<Vec<Rock>>>(),
        start: 0,
        cols: [0; 7],
    };

    let mut num_rock = 0;
    let mut num_jet = 0;
    let mut jets_iter = jets.iter().cycle();
    let numjets = jets.len();

    loop {
        let mut rock = Rock::next_rock(&mut num_rock, chamber.top_of_stack() + 3);
        loop {
            rock.apply_wind(&chamber, *jets_iter.next().unwrap());
            num_jet += 1;
            let touched_down = rock.apply_gravity(&chamber);
            if touched_down {
                for piece in rock.pieces() {
                    chamber.cols[piece.0 as usize] =
                        chamber.cols[piece.0 as usize].max(piece.1 + 1);
                }
                chamber.levels_mut(rock.y as usize).push(rock.clone());
                chamber.update_unreachable();
                if num_rock == 2022 {
                    println!("Part 1: {}", chamber.top_of_stack());
                }
            }
            if num_jet == 5 * numjets {
                a0 = chamber.top_of_stack();
                r0 = num_rock;
                // println!("Section 0:\n  a0: {a0}\n  r0: {r0} [num_jet: {num_jet}]");
            } else if num_jet == 10 * numjets {
                a1 = chamber.top_of_stack() - a0;
                r1 = num_rock - r0;
                n1 = (rt - r0) / r1;
                r2 = rt - r0 - n1 * r1;
                // println!("Section 1:\n  a1: {a1}\n  r1: {r1}\n  n1: {n1}\n  r2: {r2}");
            } else if num_jet > 10 * numjets && num_rock == r0 + r1 + r2 && touched_down {
                let a2 = chamber.top_of_stack() - a1 - a0;
                let at = a0 + n1 * a1 + a2;
                // println!("Section 2:\n  a2: {a2}\n  at: {at}");
                println!("Part 2: {}", at);
                return;
            }
            if touched_down {
                break;
            }
        }
    }
}
