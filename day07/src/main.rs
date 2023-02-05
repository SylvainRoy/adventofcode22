use std::collections::HashMap;
use std::fs::read_to_string;

enum Command {
    Cd(String),
    Ls(Vec<Item>),
}

impl Command {
    fn parse(desc: &str) -> Self {
        let mut lines = desc.lines();
        let tokens = lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();
        match tokens[..] {
            ["cd", dir] => Self::Cd(dir.into()),
            ["ls"] => Self::Ls(lines.map(Item::parse).collect::<Vec<Item>>()),
            _ => panic!("unsupported command"),
        }
    }
}

#[derive(Debug, Clone)]
enum Item {
    File(String, usize),
    Dir(String, HashMap<String, Item>, Box<usize>),
}

impl Item {
    fn parse(desc: &str) -> Self {
        match desc.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            ["dir", name] => Self::Dir(name.into(), HashMap::new(), Box::new(0)),
            [size, name] => Self::File(name.into(), size.parse::<usize>().unwrap()),
            _ => panic!("Unsupported format!"),
        }
    }

    fn set_content<I>(&mut self, commands: &mut I) -> usize
    where
        I: Iterator<Item = Command>,
    {
        let mut size: usize = 0;
        match self {
            Self::File(_, _) => panic!("Cannot set the content of a file."),
            Self::Dir(_, dircontent, _) => loop {
                match commands.next() {
                    Some(Command::Cd(name)) if name == ".." => break,
                    Some(Command::Cd(name)) => {
                        let mut newdir = Item::Dir(name.clone(), HashMap::new(), Box::new(0));
                        size += newdir.set_content(commands);
                        dircontent.insert(name, newdir);
                    }
                    Some(Command::Ls(subs)) => {
                        for sub in subs {
                            if let Item::File(name, fsize) = &sub {
                                size += fsize;
                                dircontent.insert(name.clone(), sub);
                            }
                        }
                    }
                    None => break,
                }
            },
        }
        // Set size of directory
        if let Self::Dir(_, _, dirsize) = self {
            *dirsize = Box::new(size)
        }
        size
    }

    fn size(&self) -> usize {
        match self {
            Self::Dir(_, _, size) => **size,
            Self::File(_, size) => *size,
        }
    }

    fn filter_content<F>(&self, res: &mut Vec<Item>, fun: &F)
    where
        F: Fn(&Item) -> bool,
    {
        if fun(self) {
            res.push((*self).clone())
        }
        if let Self::Dir(_, childs, _) = self {
            for child in childs.values() {
                child.filter_content(res, fun)
            }
        }
    }
}

fn main() {
    // Read input and parse commands.
    let input = read_to_string("./data/input.txt").unwrap();
    let mut commands = input
        .split('$')
        .filter(|string| !string.is_empty())
        .map(Command::parse);

    // Build the file system
    let mut root = Item::Dir("/".into(), HashMap::new(), Box::new(0));
    root.set_content(&mut commands);

    // Part 1
    let mut res = Vec::new();
    root.filter_content(
        &mut res,
        &|item| matches!(item, Item::Dir(_, _, size) if **size <= 100000),
    );
    let total: usize = res.iter().map(|item| item.size()).sum();
    println!("Part 1: {:?}", total);

    // Part 2
    let used_space = root.size();
    let must_delete = used_space - 40000000;
    let mut res = Vec::new();
    root.filter_content(
        &mut res,
        &|item| matches!(item, Item::Dir(_, _, size) if **size >= must_delete),
    );
    let min: usize = res.iter().map(|item| item.size()).min().unwrap();
    println!("Part 2: {:?}", min);
}
