use pathfinding::directed::astar::astar;
use pathfinding::directed::dijkstra::dijkstra;
use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Cell {
    i: usize,
    j: usize,
    height: usize,
}

fn main() {
    // Read input
    let input = read_to_string("./data/input.txt").unwrap();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, car)| {
                    let height;
                    if car == 'S' {
                        start = (i, j);
                        height = 'a' as usize;
                    } else if car == 'E' {
                        end = (i, j);
                        height = 'z' as usize;
                    } else {
                        height = car as usize;
                    };
                    Cell { i, j, height }
                })
                .collect()
        })
        .collect();
    let numrows = map.len();
    let numcols = map[0].len();

    // Part 1
    let successors = |cell: &Cell| {
        let mut succ = Vec::new();
        let maxheight = map[cell.i][cell.j].height + 2;
        if 0 < cell.i && map[cell.i - 1][cell.j].height < maxheight {
            succ.push((map[cell.i - 1][cell.j], 1))
        }
        if cell.i < numrows - 1 && map[cell.i + 1][cell.j].height < maxheight {
            succ.push((map[cell.i + 1][cell.j], 1))
        }
        if 0 < cell.j && map[cell.i][cell.j - 1].height < maxheight {
            succ.push((map[cell.i][cell.j - 1], 1))
        }
        if cell.j < numcols - 1 && map[cell.i][cell.j + 1].height < maxheight {
            succ.push((map[cell.i][cell.j + 1], 1))
        }
        succ
    };

    let path = astar(
        &map[start.0][start.1],
        successors,
        |cell| cell.i.abs_diff(end.0) + cell.j.abs_diff(end.1),
        |cell| cell.i == end.0 && cell.j == end.1,
    );
    println!("Part 1: {:?}", path.unwrap().1);

    // Part 2
    let predecessors = |cell: &Cell| {
        let mut succ = Vec::new();
        let minheight = map[cell.i][cell.j].height - 2;
        if 0 < cell.i && minheight < map[cell.i - 1][cell.j].height {
            succ.push((map[cell.i - 1][cell.j], 1))
        }
        if cell.i < numrows - 1 && minheight < map[cell.i + 1][cell.j].height {
            succ.push((map[cell.i + 1][cell.j], 1))
        }
        if 0 < cell.j && minheight < map[cell.i][cell.j - 1].height {
            succ.push((map[cell.i][cell.j - 1], 1))
        }
        if cell.j < numcols - 1 && minheight < map[cell.i][cell.j + 1].height {
            succ.push((map[cell.i][cell.j + 1], 1))
        }
        succ
    };

    let path = dijkstra(&map[end.0][end.1], predecessors, |cell| {
        cell.height == 'a' as usize
    });
    println!("Part 2: {:?}", path.unwrap().1);
}
