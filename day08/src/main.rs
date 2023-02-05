use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    // Read input
    let input = read_to_string("./data/input.txt")?;
    let map: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let (numrows, numcols) = (map.len(), map[0].len());

    // Part 1
    let mut visibles = HashSet::new();
    for row in 0..numrows {
        // Left to right
        let (mut height, mut col) = (-1, 0);
        while height < 9 && col < numcols {
            if height < map[row][col] {
                height = map[row][col];
                visibles.insert((row, col));
            }
            col += 1;
        }
        // Right to left
        let (mut height, mut col) = (-1, numcols - 1);
        while height < 9 && 0 < col {
            if height < map[row][col] {
                height = map[row][col];
                visibles.insert((row, col));
            }
            col -= 1;
        }
    }
    for col in 0..numcols {
        // Top down
        let (mut height, mut row) = (-1, 0);
        while height < 9 && row < numrows {
            if height < map[row][col] {
                height = map[row][col];
                visibles.insert((row, col));
            }
            row += 1;
        }
        // Bottop up
        let (mut height, mut row) = (-1, numrows - 1);
        while height < 9 && 0 < row {
            if height < map[row][col] {
                height = map[row][col];
                visibles.insert((row, col));
            }
            row -= 1;
        }
    }
    println!("Part 1: {:?}", visibles.len());

    // Part 2
    let mut scenic: Vec<Vec<usize>> = (0..numrows).map(|_| vec![1; numcols]).collect();
    for row in 0..numrows {
        // Left to right
        let mut dist_to_higher = vec![0; 10];
        for col in 0..numcols {
            // Update score of tree
            scenic[row][col] *= dist_to_higher[map[row][col] as usize];
            // Update distance for next tree
            for height in 0..10 {
                if height > map[row][col] as usize {
                    dist_to_higher[height] += 1;
                } else {
                    dist_to_higher[height] = 1;
                }
            }
        }
        // Right to left
        let mut dist_to_higher = vec![0; 10];
        for col in (0..numcols).rev() {
            // Update score of tree
            scenic[row][col] *= dist_to_higher[map[row][col] as usize];
            // Update distance for next tree
            for height in 0..10 {
                if height > map[row][col] as usize {
                    dist_to_higher[height] += 1;
                } else {
                    dist_to_higher[height] = 1;
                }
            }
        }
    }
    for col in 0..numcols {
        // Top-down
        let mut dist_to_higher = vec![0; 10];
        for row in 0..numrows {
            // Update score of tree
            scenic[row][col] *= dist_to_higher[map[row][col] as usize];
            // Update distance for next tree
            for height in 0..10 {
                if height > map[row][col] as usize {
                    dist_to_higher[height] += 1;
                } else {
                    dist_to_higher[height] = 1;
                }
            }
        }
        // Bottom-up
        let mut dist_to_higher = vec![0; 10];
        for row in (0..numrows).rev() {
            // Update score of tree
            scenic[row][col] *= dist_to_higher[map[row][col] as usize];
            // Update distance for next tree
            for height in 0..10 {
                if height > map[row][col] as usize {
                    dist_to_higher[height] += 1;
                } else {
                    dist_to_higher[height] = 1;
                }
            }
        }
    }

    let max = scenic
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap();
    println!("Part 2: {:?}", max);

    Ok(())
}
