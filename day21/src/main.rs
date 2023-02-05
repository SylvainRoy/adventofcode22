use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn parse(token: char) -> Self {
        match token {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Mul,
            '/' => Self::Div,
            _ => panic!("Unknown operation!"),
        }
    }

    fn process(&self, left: isize, right: isize) -> isize {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }
}

#[derive(Debug)]
enum Monkey {
    Op {
        left: String,
        op: Operation,
        right: String,
    },
    Value {
        val: isize,
    },
    Unknown,
}

impl Monkey {
    fn parse(line: &str) -> (String, Self) {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        let name = tokens[0].replace(':', "").to_owned();
        match tokens.len() {
            2 => (
                name,
                Monkey::Value {
                    val: tokens[1].parse::<isize>().unwrap(),
                },
            ),
            4 => (
                name,
                Monkey::Op {
                    left: tokens[1].to_owned(),
                    op: Operation::parse(tokens[2].chars().nth(0).unwrap()),
                    right: tokens[3].to_owned(),
                },
            ),
            _ => panic!("This line isn't properly formatted!"),
        }
    }

    fn eval(&self, monkeys: &HashMap<String, Monkey>) -> Option<isize> {
        match self {
            Monkey::Value { val } => Some(*val),
            Monkey::Op { left, op, right } => {
                let lres = monkeys[left].eval(monkeys);
                let rres = monkeys[right].eval(monkeys);
                match (lres, rres) {
                    (Some(lvalue), Some(rvalue)) => Some(op.process(lvalue, rvalue)),
                    _ => None,
                }
            }
            Monkey::Unknown => None,
        }
    }

    fn solve(&self, res: isize, monkeys: &HashMap<String, Monkey>) -> isize {
        match self {
            Monkey::Op { left, op, right } => {
                match (monkeys[left].eval(monkeys), monkeys[right].eval(monkeys)) {
                    (Some(lval), None) => {
                        match op {
                            Operation::Add => monkeys[right].solve(res - lval, monkeys),
                            Operation::Sub => monkeys[right].solve(lval - res, monkeys),
                            Operation::Mul => monkeys[right].solve(res / lval, monkeys),
                            Operation::Div => monkeys[right].solve(lval / res, monkeys),
                        }
                    }
                    (None, Some(rval)) => {
                        match op {
                            Operation::Add => monkeys[left].solve(res - rval, monkeys),
                            Operation::Sub => monkeys[left].solve(rval + res, monkeys),
                            Operation::Mul => monkeys[left].solve(res / rval, monkeys),
                            Operation::Div => monkeys[left].solve(rval * res, monkeys),
                        }
                    }
                    _ => panic!("This should never happen."),
                }
            }
            Monkey::Unknown => res,
            _ => panic!("Should never get here."),
        }
    }
}

fn main() {
    // Read input
    let input = read_to_string("./data/input.txt").unwrap();
    let mut monkeys: HashMap<String, Monkey> =
        input.lines().map(|line| Monkey::parse(line)).collect();

    // Part 1
    let res = monkeys["root"].eval(&monkeys).unwrap();
    println!("Part 1: {:?}", res);

    // Part 2
    let root = monkeys.remove("root").unwrap();
    if let Monkey::Op { left, op: _, right } = root {
        monkeys.insert(
            "root".to_owned(),
            Monkey::Op {
                left,
                op: Operation::Sub,
                right,
            },
        );
    }
    monkeys.remove("humn").unwrap();
    monkeys.insert("humn".to_owned(), Monkey::Unknown);

    let res = monkeys["root"].solve(0, &monkeys);
    println!("Part 2: {:?}", res);
}
