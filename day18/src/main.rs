use std::collections::HashSet;
use std::fs::read_to_string;

const MAX: usize = 20;
const NEIGHBOURS: [(isize, isize, isize); 6] = [
    (0, 0, 1),
    (0, 0, -1),
    (0, 1, 0),
    (0, -1, 0),
    (1, 0, 0),
    (-1, 0, 0),
];

struct Space {
    vol: [[[bool; MAX + 4]; MAX + 4]; MAX + 4],
}

impl Space {
    fn new() -> Self {
        Space {
            vol: [[[false; MAX + 4]; MAX + 4]; MAX + 4],
        }
    }

    fn set(&mut self, x: isize, y: isize, z: isize, val: bool) {
        self.vol[(x + 2) as usize][(y + 2) as usize][(z + 2) as usize] = val;
    }

    fn get(&self, x: isize, y: isize, z: isize) -> bool {
        self.vol[(x + 2) as usize][(y + 2) as usize][(z + 2) as usize]
    }

    fn within(&self, x: isize, y: isize, z: isize) -> bool {
        -1 <= x
            && x <= MAX as isize + 1
            && -1 <= y
            && y <= MAX as isize + 1
            && -1 <= z
            && z <= MAX as isize + 1
    }
}

fn main() {
    // read input
    let input = read_to_string("./data/input.txt").unwrap();
    let cubes: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|token| token.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();

    let mut space = Space::new();
    for cube in &cubes {
        space.set(cube[0], cube[1], cube[2], true);
    }

    // Part 1
    let mut opensides = 0;
    for cube in &cubes {
        for (x, y, z) in NEIGHBOURS {
            if !space.get(cube[0] + x, cube[1] + y, cube[2] + z) {
                opensides += 1;
            }
        }
    }
    println!("Part 1: {:?}", opensides);

    // Part 2
    let mut open = Space::new();
    open.set(0, 0, 0, true);
    setopen2(&mut open, &space);

    let mut opensides = 0;
    for cube in &cubes {
        for (x, y, z) in NEIGHBOURS {
            if !space.get(cube[0] + x, cube[1] + y, cube[2] + z)
                && open.get(cube[0] + x, cube[1] + y, cube[2] + z)
            {
                opensides += 1;
            }
        }
    }
    println!("Part 2: {:?}", opensides);
}

fn setopen2(open: &mut Space, space: &Space) {
    let mut todo = Vec::new();
    let mut done = HashSet::new();
    open.set(0, 0, 0, true);
    todo.push((0, 0, 0));
    loop {
        match todo.pop() {
            None => return,
            Some((x, y, z)) => {
                done.insert((x, y, z));
                for (xx, yy, zz) in NEIGHBOURS {
                    let (x_, y_, z_) = (x + xx, y + yy, z + zz);
                    if space.within(x_, y_, z_)
                        && !done.contains(&(x_, y_, z_))
                        && !space.get(x_, y_, z_)
                    {
                        open.set(x_, y_, z_, true);
                        todo.push((x_, y_, z_));
                    }
                }
            }
        }
    }
}
