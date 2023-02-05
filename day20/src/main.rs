use std::collections::VecDeque;
use std::fs::read_to_string;

#[derive(Debug)]
struct Number {
    index: usize,
    value: isize,
}

fn main() {
    // Read input
    let input = read_to_string("./data/input.txt").unwrap();

    // Part 1
    let mut numbers: VecDeque<Number> = input
        .lines()
        .enumerate()
        .map(|(i, line)| Number {
            index: i,
            value: line.parse::<isize>().unwrap(),
        })
        .collect();
    for index in 0..numbers.len() {
        shift(&mut numbers, index);
    }
    let izero = numbers
        .iter()
        .enumerate()
        .find(|(_, n)| n.value == 0)
        .unwrap()
        .0;
    let res: isize = (1..=3)
        .map(|i| numbers[(izero + i * 1000).rem_euclid(numbers.len())].value)
        .sum();
    println!("Part 1: {:?}", res);

    // Part 2
    let key = 811589153;
    let mut numbers: VecDeque<Number> = input
        .lines()
        .enumerate()
        .map(|(i, line)| Number {
            index: i,
            value: key * line.parse::<isize>().unwrap(),
        })
        .collect();
    for _ in 0..10 {
        for index in 0..numbers.len() {
            shift(&mut numbers, index);
        }
    }
    let izero = numbers
        .iter()
        .enumerate()
        .find(|(_, n)| n.value == 0)
        .unwrap()
        .0;
    let res: isize = (1..=3)
        .map(|i| numbers[(izero + i * 1000).rem_euclid(numbers.len())].value)
        .sum();
    println!("Part 2: {:?}", res);
}

fn shift(numbers: &mut VecDeque<Number>, index: usize) {
    while numbers[0].index != index {
        numbers.rotate_left(1);
    }
    let number = numbers.pop_front().unwrap();
    if number.value > 0 {
        numbers.rotate_left((number.value as usize).rem_euclid(numbers.len()));
    } else if number.value < 0 {
        numbers.rotate_right((-number.value as usize).rem_euclid(numbers.len()));
    }
    numbers.push_front(number);
}
