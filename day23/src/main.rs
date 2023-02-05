use std::collections::HashMap;
use std::fs::read_to_string;

type Position = (isize, isize);
type Positions = HashMap<Position, Option<Position>>;
type Hints = HashMap<Position, isize>;

const DELTAS: [Position; 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

const DELTAS_STRATEGY: [[Position; 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)], // NE, N, NW
    [(-1, 1), (0, 1), (1, 1)],    // SE, S, SW
    [(-1, -1), (-1, 0), (-1, 1)], // NW, W, SW
    [(1, -1), (1, 0), (1, 1)],    // NW, W, SW
];

fn get_wish(pos: &Position, positions: &Positions, strategy: usize) -> Option<Position> {
    // If adjacent Elves, do not move.
    let has_neighbours = DELTAS
        .iter()
        .map(|d| positions.get(&(pos.0 + d.0, pos.1 + d.1)).is_some())
        .any(|v| v);
    if !has_neighbours {
        return None;
    };
    // Look for possible move.
    for i in 0..4 {
        let deltas = DELTAS_STRATEGY[(i + strategy) % 4];
        let occupied = deltas
            .iter()
            .map(|d| positions.get(&(pos.0 + d.0, pos.1 + d.1)).is_some())
            .any(|v| v);
        if !occupied {
            let d = deltas[1];
            return Some((pos.0 + d.0, pos.1 + d.1));
        }
    }
    None
}

fn get_wishes(positions: &Positions, strategy: usize) -> (Positions, Hints) {
    let mut new_positions: Positions = HashMap::new();
    let mut wishes: Hints = HashMap::new();
    for pos in positions.keys() {
        let wish = get_wish(pos, positions, strategy);
        if let Some(wish_pos) = wish {
            wishes.entry(wish_pos).and_modify(|e| *e += 1).or_insert(1);
            new_positions.insert(*pos, wish);
        } else {
            new_positions.insert(*pos, None);
        }
    }
    (new_positions, wishes)
}

fn do_moves(positions: &Positions, wishes: &Hints) -> Positions {
    let mut new_positions = HashMap::new();
    for (pos, wish) in positions {
        if let Some(wish_pos) = wish {
            if wishes[wish_pos] == 1 {
                new_positions.insert(*wish_pos, None);
                continue;
            }
        }
        new_positions.insert(*pos, None);
    }
    new_positions
}

fn main() {
    // Decode input
    let input = read_to_string("./data/input.txt").expect("Can't read input");
    let mut positions: Positions = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, car)| {
                if car == '#' {
                    Some((col as isize, row as isize))
                } else {
                    None
                }
            })
        })
        .map(|pos| (pos, None))
        .collect();

    // Part 1
    for i in 0..10 {
        let wishes;
        (positions, wishes) = get_wishes(&positions, i);
        positions = do_moves(&positions, &wishes);
    }
    let xmin = positions.keys().map(|p| p.0).min().unwrap();
    let xmax = positions.keys().map(|p| p.0).max().unwrap();
    let ymin = positions.keys().map(|p| p.1).min().unwrap();
    let ymax = positions.keys().map(|p| p.1).max().unwrap();
    let result = (xmax - xmin + 1) * (ymax - ymin + 1) - positions.len() as isize;
    println!("Part1: {}", result);

    // Part 2
    let mut newpositions;
    let mut wishes;
    let mut i = 10;
    loop {
        (newpositions, wishes) = get_wishes(&positions, i);
        newpositions = do_moves(&newpositions, &wishes);
        if positions == newpositions {
            break;
        }
        i += 1;
        positions = newpositions;
    }
    println!("Part2: {}", i + 1);
}
