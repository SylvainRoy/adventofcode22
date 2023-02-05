use pathfinding::prelude::astar;
use std::collections::HashSet;
use std::fs::read_to_string;

type Pos = (isize, isize);

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct TimePos(isize, isize, usize);

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Blizzard {
    dir: Dir,
    x: isize,
    y: isize,
}

struct TimeMap {
    t: usize,
    blizzards_t: Vec<Blizzard>,
    maps: Vec<HashSet<Pos>>,
    xdim: isize,
    ydim: isize,
    x0: isize,
}

impl TimeMap {
    fn new(blizzards: Vec<Blizzard>, xdim: isize, ydim: isize, x0: isize) -> Self {
        let map: HashSet<Pos> = blizzards
            .iter()
            .map(|blizzard| (blizzard.x, blizzard.y))
            .collect();
        Self {
            t: 0,
            blizzards_t: blizzards,
            maps: vec![map],
            xdim,
            ydim,
            x0,
        }
    }

    fn map(&mut self, t: usize) -> &HashSet<Pos> {
        // Compjute all intermediate maps if needed.
        while self.t < t {
            self.t += 1;
            self.blizzards_t = self
                .blizzards_t
                .iter()
                .map(|blizzard| match blizzard.dir {
                    Dir::Up => Blizzard {
                        dir: blizzard.dir,
                        x: blizzard.x,
                        y: (blizzard.y - 1).rem_euclid(self.ydim),
                    },
                    Dir::Right => Blizzard {
                        dir: blizzard.dir,
                        x: (blizzard.x + 1).rem_euclid(self.xdim),
                        y: blizzard.y,
                    },
                    Dir::Down => Blizzard {
                        dir: blizzard.dir,
                        x: blizzard.x,
                        y: (blizzard.y + 1).rem_euclid(self.ydim),
                    },
                    Dir::Left => Blizzard {
                        dir: blizzard.dir,
                        x: (blizzard.x - 1).rem_euclid(self.xdim),
                        y: blizzard.y,
                    },
                })
                .collect();
            let map_t: HashSet<Pos> = self
                .blizzards_t
                .iter()
                .map(|blizzard| (blizzard.x, blizzard.y))
                .collect();
            self.maps.push(map_t);
        }
        &self.maps[t]
    }

    fn successors(&mut self, tpos: &TimePos) -> Vec<(TimePos, isize)> {
        let mut successors = Vec::new();
        // Wait in place
        if !self.map(tpos.2 + 1).contains(&(tpos.0, tpos.1)) {
            successors.push((TimePos(tpos.0, tpos.1, tpos.2 + 1), 1));
        }
        // Move from start to board
        if tpos.1 == -1 && !self.map(tpos.2 + 1).contains(&(tpos.0, tpos.1 + 1)) {
            successors.push((TimePos(tpos.0, tpos.1 + 1, tpos.2 + 1), 1));
        }
        // Move from board to finish
        if tpos.1 == self.ydim - 1 && tpos.0 == self.xdim - 1 {
            successors.push((TimePos(tpos.0, tpos.1 + 1, tpos.2 + 1), 1));
        }
        // Move from one board cell to another
        if 0 <= tpos.0 && tpos.0 < self.xdim && 0 <= tpos.1 && tpos.1 < self.ydim {
            for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let spos = (tpos.0 + delta.0, tpos.1 + delta.1);
                if !self.map(tpos.2 + 1).contains(&spos)
                    && 0 <= spos.0
                    && spos.0 < self.xdim
                    && 0 <= spos.1
                    && spos.1 < self.ydim
                {
                    successors.push((TimePos(spos.0, spos.1, tpos.2 + 1), 1));
                }
            }
        }
        // Move from board to start
        if tpos.0 == self.x0 && tpos.1 == 0 {
            successors.push((TimePos(tpos.0, tpos.1 - 1, tpos.2 + 1), 1));
        }
        // Move from finish to board
        if tpos.0 == self.xdim - 1
            && tpos.1 == self.ydim
            && !self.map(tpos.2 + 1).contains(&(tpos.0, tpos.1 - 1))
        {
            successors.push((TimePos(tpos.0, tpos.1 - 1, tpos.2 + 1), 1));
        }
        successors
    }
}

fn main() {
    // Parse input
    let input = read_to_string("./data/input.txt").unwrap();
    let blizzards: Vec<Blizzard> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, car)| match car {
                    '<' => Some(Blizzard {
                        dir: Dir::Left,
                        x: x as isize - 1,
                        y: y as isize - 1,
                    }),
                    '^' => Some(Blizzard {
                        dir: Dir::Up,
                        x: x as isize - 1,
                        y: y as isize - 1,
                    }),
                    'v' => Some(Blizzard {
                        dir: Dir::Down,
                        x: x as isize - 1,
                        y: y as isize - 1,
                    }),
                    '>' => Some(Blizzard {
                        dir: Dir::Right,
                        x: x as isize - 1,
                        y: y as isize - 1,
                    }),
                    '.' => None,
                    '#' => None,
                    _ => panic!("Unexpected car!"),
                })
        })
        .collect();
    let xdim = input.lines().next().unwrap().len() as isize - 2;
    let ydim = input.lines().count() as isize - 2;
    let x0 = input
        .chars()
        .enumerate()
        .find(|(_, car)| *car == '.')
        .unwrap()
        .0 as isize
        - 1;

    // Part 1
    let mut timemap = TimeMap::new(blizzards, xdim, ydim, x0);

    let result = astar(
        &TimePos(x0, -1, 0),
        |tpos| timemap.successors(tpos),
        |tpos| tpos.0.abs_diff(xdim - 1) as isize + tpos.1.abs_diff(ydim) as isize,
        |tpos| tpos.0 == xdim - 1 && tpos.1 == ydim,
    );
    let trip = result.unwrap().1 as usize;
    println!("Part1: {:?}", trip);

    let result = astar(
        &TimePos(xdim - 1, ydim, trip),
        |tpos| timemap.successors(tpos),
        |tpos| tpos.0.abs_diff(x0) as isize + tpos.1.abs_diff(-1) as isize,
        |tpos| tpos.0 == x0 && tpos.1 == -1,
    );
    let back = result.unwrap().1 as usize;
    println!("back: {:?}", back);
    let result = astar(
        &TimePos(x0, -1, trip + back),
        |tpos| timemap.successors(tpos),
        |tpos| tpos.0.abs_diff(xdim - 1) as isize + tpos.1.abs_diff(ydim) as isize,
        |tpos| tpos.0 == xdim - 1 && tpos.1 == ydim,
    );
    let trip2 = result.unwrap().1 as usize;
    println!("Part2: {:?}", trip + back + trip2);
    // It's not 803
}
