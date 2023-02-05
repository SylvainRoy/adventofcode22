use std::fs::read_to_string;
use std::fmt;
use std::fmt::Display;

struct Digit {
    val: isize,
}

impl Digit {
    fn from_car(car: char) -> Self {
        let val = match car {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Unsupported char!"),
        };
        Self { val }
    }

    fn from_int(val: isize) -> Self {
        if !(-2..=2).contains(&val) {
            panic!("Unsupported val");
        }
        Self { val }
    }

    fn to_car(&self) -> char {
        match self.val {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => panic!("Unsupported val!"),
        }
    }

    fn to_int(&self) -> isize {
        self.val
    }
}

struct Snafu {
    val: isize,
}

impl Snafu {
    fn from_string(snafu: &str) -> Self {
        let mut val = 0;
        for car in snafu.chars() {
            val = 5 * val + Digit::from_car(car).to_int();
        }
        Snafu { val }
    }

    fn from_int(val: isize) -> Self {
        Self { val }
    }

    fn to_int(&self) -> isize {
        self.val
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut value = self.val;
        let mut res = String::new();
        let mut i = 0;
        while value / 5_isize.pow(i) > 2 {
            i += 1
        }
        i += 1;
        loop {
            let div: f32 = value as f32 / (5_isize.pow(i)) as f32;
            let digit = if (div - div.floor()).abs() < (div - div.ceil()).abs() {
                div.floor()
            } else {
                div.ceil()
            } as isize;
            value -= digit * 5_isize.pow(i);
            res.push(Digit::from_int(digit).to_car());
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        let res = res.trim_start_matches('0').to_string();
        write!(f, "{}", res)
    }
}

fn main() {
    // Read input
    let input = read_to_string("./data/input.txt").unwrap();

    let total: isize = input
        .lines()
        .map(|line| Snafu::from_string(line).to_int())
        .sum();
    println!("Part1: {}", Snafu::from_int(total));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_to_snafu() {
        assert_eq!(Snafu::from_int(1).to_string(), "1");
        assert_eq!(Snafu::from_int(2).to_string(), "2");
        assert_eq!(Snafu::from_int(3).to_string(), "1=");
        assert_eq!(Snafu::from_int(4).to_string(), "1-");
        assert_eq!(Snafu::from_int(5).to_string(), "10");
        assert_eq!(Snafu::from_int(6).to_string(), "11");
        assert_eq!(Snafu::from_int(7).to_string(), "12");
        assert_eq!(Snafu::from_int(8).to_string(), "2=");
        assert_eq!(Snafu::from_int(9).to_string(), "2-");
        assert_eq!(Snafu::from_int(10).to_string(), "20");
        assert_eq!(Snafu::from_int(15).to_string(), "1=0");
        assert_eq!(Snafu::from_int(20).to_string(), "1-0");
        assert_eq!(Snafu::from_int(2022).to_string(), "1=11-2");
        assert_eq!(Snafu::from_int(12345).to_string(), "1-0---0");
        assert_eq!(Snafu::from_int(314159265).to_string(), "1121-1110-1=0");
    }

    #[test]
    fn int_to_snafu_and_back_many_times() {
        for i in 0..10000 {
            let snafu = Snafu::from_int(i).to_string();
            let val = Snafu::from_string(&snafu).to_int();
            assert_eq!(i, val);
        }
    }

    #[test]
    fn check_input_file() {
        let input = read_to_string("./data/input.txt").unwrap();
        for line in input.lines() {
            assert_eq!(line, Snafu::from_string(line).to_string());
        }
    }
}
