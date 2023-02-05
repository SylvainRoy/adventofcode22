/*
TODO:
- have dist stored in sensor
- organise sensor per region
- skip cells to end of sensor zone directly
*/

use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    closest: (isize, isize),
    dist: isize,
}

fn distance(ax: isize, ay: isize, bx: isize, by: isize) -> isize {
    (ax.abs_diff(bx) + ay.abs_diff(by)) as isize
}

fn main() {
    // Params
    let input = "./data/input.txt";
    let linenum = 2000000;
    let maxcoord = 4000000;
    // let input = "./data/example.txt";
    // let linenum = 10;
    // let maxcoord = 20;

    // Read input
    let input = read_to_string(input).unwrap();
    let sensors: Vec<Sensor> = input
        .lines()
        .map(|line| {
            let mut tokens = line.split_ascii_whitespace();
            let x = tokens.nth(2).unwrap()[2..]
                .replace(",", "")
                .parse::<isize>()
                .unwrap();
            let y = tokens.next().unwrap()[2..]
                .replace(":", "")
                .parse::<isize>()
                .unwrap();
            let beacon_x = tokens.nth(4).unwrap()[2..]
                .replace(",", "")
                .parse::<isize>()
                .unwrap();
            let beacon_y = tokens.next().unwrap()[2..].parse::<isize>().unwrap();
            Sensor {
                x,
                y,
                closest: (beacon_x, beacon_y),
                dist: distance(x, y, beacon_x, beacon_y),
            }
        })
        .collect();
    let mut devices = HashSet::new();
    for sensor in &sensors {
        devices.insert(sensor.closest);
        devices.insert((sensor.x, sensor.y));
    }

    // Part 1
    let further_closest = sensors
        .iter()
        .map(|s| s.dist)
        .max()
        .unwrap();
    let min_x = sensors.iter().map(|s| s.x).min().unwrap() - further_closest - 2;
    let max_x = sensors.iter().map(|s| s.x).max().unwrap() + further_closest + 2;

    // Part 1
    let mut cannot = 0;
    let y = linenum;
    'nextslot: for x in min_x..=max_x {
        for sensor in &sensors {
            if distance(sensor.x, sensor.y, x, y) <= sensor.dist && !devices.contains(&(x, y))
            {
                cannot += 1;
                continue 'nextslot;
            }
        }
    }
    println!("Part 1: {:?}", cannot);



    // Part 2
    let (mut dx, mut dy) = (0, 0);
    'nextline: for y in 0..=maxcoord {
        let mut x = 0;
        'nextcell: loop {
            for sensor in &sensors {
                // If the slot is already occupied by a device
                // if devices.contains(&(x, y)) {
                //     x += 1;
                //     continue 'nextcell;
                // }
                // If in range of a sensor, go to next cell out of range
                if distance(sensor.x, sensor.y, x, y) <= sensor.dist {
                    x = sensor.x + sensor.dist + 1 - (sensor.y.abs_diff(y) as isize);
                    if x <= maxcoord {
                        continue 'nextcell;
                    } else {
                        continue 'nextline;
                    }
                }
            }
            // There can be a beacon
            (dx, dy) = (x, y);
            break 'nextline;
        }
    }
    println!("Part 2a: {:?}", 4000000 * dx + dy);
    // println!("with x: {:?} y: {:?}", dx, dy);


    // // Part 2b
    // let (mut dx, mut dy) = (0, 0);
    // 'nextline: for y in 0..=maxcoord {
    //     'nextcell: for x in 0..=maxcoord {
    //         for sensor in &sensors {
    //             // If there cannot be a beacon
    //             if distance(sensor.x, sensor.y, x, y) <= sensor.dist || devices.contains(&(x, y))
    //             {
    //                 // go to next space
    //                 continue 'nextcell;
    //             }
    //         }
    //         // There can be a beacon
    //         (dx, dy) = (x, y);
    //         break 'nextline;
    //     }
    // }
    // println!("Part 2b: {:?}", 4000000 * dx + dy);

}
