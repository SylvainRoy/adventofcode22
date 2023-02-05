use std::fs::read_to_string;

#[derive(Clone)]
enum Operation {
    Mult(Value),
    Add(Value),
}

#[derive(Clone)]
enum Value {
    Number(usize),
    Old,
}

impl Operation {
    fn parse<'a, I>(tokens: &mut I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let (op, param) = (tokens.nth(3), tokens.next());
        let value = match param {
            Some(str) if str == "old" => Value::Old,
            Some(str) => Value::Number(str.parse::<usize>().unwrap()),
            _ => panic!("Unsupported parameter."),
        };
        match op {
            Some(str) if str == "+" => Operation::Add(value),
            Some(str) if str == "*" => Operation::Mult(value),
            _ => panic!("Unsupported operation."),
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    actions: (usize, usize),
    inspections: usize,
}

impl Monkey {
    fn parse<'a, I>(tokens: &mut I) -> Option<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        // Check if monkeys left
        if !matches!(tokens.next(), Some(str) if str == "Monkey") {
            return None;
        };
        // Parse ID
        let _id = tokens
            .next()
            .unwrap()
            .replace(':', "")
            .parse::<usize>()
            .unwrap();
        // Parse items
        tokens.next();
        tokens.next();
        let mut items = vec![];
        loop {
            match tokens.next() {
                Some(str) if str == "Operation:" => break,
                Some(str) => items.push(str.replace(',', "").parse::<usize>().unwrap()),
                _ => panic!("The keyword 'operation:' should follow the list of items."),
            }
        }
        // Parse operation
        let operation = Operation::parse(tokens);
        let test = tokens.nth(3).unwrap().parse::<usize>().unwrap();
        // Parse actions
        let iftrue = tokens.nth(5).unwrap().parse::<usize>().unwrap();
        let iffalse = tokens.nth(5).unwrap().parse::<usize>().unwrap();
        Some(Monkey {
            items,
            operation,
            test,
            actions: (iftrue, iffalse),
            inspections: 0,
        })
    }

    fn run(&mut self, worry: usize, divider: Option<usize>) -> Vec<(usize, usize)> {
        let mut out = vec![];
        for old in self.items.iter() {
            self.inspections += 1;
            let mut new = match self.operation {
                Operation::Mult(Value::Number(val)) => old * val,
                Operation::Add(Value::Number(val)) => old + val,
                Operation::Mult(Value::Old) => old * old,
                Operation::Add(Value::Old) => old + old,
            };
            new /= worry;
            if let Some(value) = divider {
                new %= value;
            }
            if new % self.test == 0 {
                out.push((self.actions.0, new));
            } else {
                out.push((self.actions.1, new));
            }
        }
        self.items.clear();
        out.reverse();
        out
    }
}

fn main() {
    // Read input
    let input = read_to_string("./data/input.txt").unwrap();
    let mut tokens = input.split_whitespace();
    let mut monkeys: Vec<Monkey> = vec![];
    loop {
        let monkey = Monkey::parse(&mut tokens);
        match monkey {
            Some(monkey) => monkeys.push(monkey.clone()),
            None => break,
        }
    }
    let mut monkeys2 = monkeys.clone();

    // Part 1
    for _round in 0..20 {
        for m in 0..monkeys.len() {
            for (n, item) in monkeys[m].run(3, None).iter() {
                monkeys[*n as usize].items.push(*item);
            }
        }
    }
    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    println!(
        "Part 1: {:?}",
        inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
    );

    // // Part 2
    let divider: usize = monkeys2
        .iter()
        .map(|m| m.test)
        .reduce(|acc, t| acc * t)
        .unwrap();
    for _round in 0..10000 {
        for m in 0..monkeys2.len() {
            for (n, item) in monkeys2[m].run(1, Some(divider)).iter() {
                monkeys2[*n as usize].items.push(*item);
            }
        }
    }
    let mut inspections: Vec<usize> = monkeys2.iter().map(|m| m.inspections).collect();
    inspections.sort();
    println!(
        "Part 2: {:?}",
        inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
    );
}
