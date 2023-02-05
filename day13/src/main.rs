use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Debug)]
struct Pair {
    left: Item,
    right: Item,
}

impl Pair {
    fn parse(input: &Vec<char>, index: &mut usize) -> Self {
        let left = Item::parse(input, index);
        *index += 1;
        let right = Item::parse(input, index);
        *index += 2;
        Pair { left, right }
    }
}

#[derive(Debug, Clone, Eq)]
enum Item {
    List(Vec<Item>),
    Value(u32),
}

impl Item {
    fn parse(input: &Vec<char>, index: &mut usize) -> Self {
        match input[*index] {
            '[' => {
                *index += 1;
                let mut subitems = Vec::new();
                while input[*index] != ']' {
                    subitems.push(Self::parse(input, index));
                    while input[*index] == ',' {
                        *index += 1
                    }
                }
                *index += 1;
                Self::List(subitems)
            }
            c if c.is_ascii_digit() => {
                *index += 1;
                let mut val = c.to_digit(10).unwrap();
                while '0' <= input[*index] && input[*index] <= '9' {
                    val = 10 * val + input[*index].to_digit(10).unwrap();
                    *index += 1;
                }
                Self::Value(val)
            }
            _ => panic!("Unexpected char: "),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::List(llist), Item::List(rlist)) => {
                let (mut liter, mut riter) = (llist.iter(), rlist.iter());
                loop {
                    let (litem, ritem) = (liter.next(), riter.next());
                    match (litem, ritem) {
                        (Some(l), Some(r)) => {
                            let s = l.cmp(r);
                            if s != Ordering::Equal {
                                return s;
                            }
                        }
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (None, None) => return Ordering::Equal,
                    };
                }
            }
            (Item::List(_), Item::Value(_)) => return self.cmp(&Item::List(vec![other.clone()])),
            (Item::Value(_), Item::List(_)) => return Item::List(vec![self.clone()]).cmp(other),
            (Item::Value(lval), Item::Value(rval)) => {
                if lval < rval {
                    return Ordering::Less;
                } else if lval == rval {
                    return Ordering::Equal;
                } else {
                    return Ordering::Greater;
                }
            }
        };
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn main() {
    // Read input
    let input = read_to_string("./data/input.txt").unwrap();
    let data: Vec<char> = input.chars().collect();

    let mut pairs: Vec<Pair> = vec![];
    let mut index = 0;
    while index < data.len() - 1 {
        let pair = Pair::parse(&data, &mut index);
        pairs.push(pair);
    }

    // Part 1
    let mut sum = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if pair.left.cmp(&pair.right) == Ordering::Less {
            sum += i + 1
        };
    }
    println!("Part 1: {:?}", sum);

    // Part 2
    let mut packets: Vec<&Item> = pairs
        .iter()
        .flat_map(|pair| [&pair.left, &pair.right])
        .collect();
    let two = Item::List(vec![Item::List(vec![Item::Value(2)])]);
    let six = Item::List(vec![Item::List(vec![Item::Value(6)])]);
    packets.push(&two);
    packets.push(&six);
    packets.sort();
    let itwo = packets
        .iter()
        .enumerate()
        .find(|&(_, &item)| item.cmp(&two) == Ordering::Equal)
        .unwrap()
        .0;
    let isix = packets
        .iter()
        .enumerate()
        .find(|&(_, &item)| item.cmp(&six) == Ordering::Equal)
        .unwrap()
        .0;
    println!("Part 2: {:?}", (itwo + 1) * (isix + 1));
}
