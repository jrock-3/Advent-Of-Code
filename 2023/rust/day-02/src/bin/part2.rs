use std::str::FromStr;

fn main() {
    let input = include_str!("../in/in1.txt");
    let res = process(input);
    dbg!(res);
}

enum Marble {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct ParseMarbleError;

impl FromStr for Marble {
    type Err = ParseMarbleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Marble::Red),
            "green" => Ok(Marble::Green),
            "blue" => Ok(Marble::Blue),
            _ => Err(ParseMarbleError),
        }
    }
}

fn process(input: &str) -> String {
    let mut res = 0;
    for line in input.lines() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for round in line[line.find(':').expect("should be index") + 1..].split(';') {
            println!("---");
            for word in round.split(',').inspect(|_marble| {
                dbg!(_marble);
            }) {
                let mut marble = word[1..].split(' ');
                let num = marble
                    .next()
                    .expect("should be string")
                    .parse::<u32>()
                    .expect("should be number");
                let m_type = marble
                    .next()
                    .expect("should be string")
                    .parse::<Marble>()
                    .expect("should be marble");
                match m_type {
                    Marble::Red => {
                        min_red = std::cmp::max(min_red, num);
                    }
                    Marble::Green => {
                        min_green = std::cmp::max(min_green, num);
                    }
                    Marble::Blue => {
                        min_blue = std::cmp::max(min_blue, num);
                    }
                }
            }
        }
        println!(
            "{} {} {} {}",
            min_red,
            min_green,
            min_blue,
            min_red * min_blue * min_green
        );
        res += min_red * min_blue * min_green;
    }
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        let res = process(input);
        assert_eq!(res, "286")
    }
}
