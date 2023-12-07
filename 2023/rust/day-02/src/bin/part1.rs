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
struct Marbles(u32, Marble);

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn is_valid_round(round: &mut impl Iterator<Item = Marbles>) -> bool {
    for marbles in round {
        let maximum = match marbles.1 {
            Marble::Red => MAX_RED,
            Marble::Blue => MAX_BLUE,
            Marble::Green => MAX_GREEN,
        };
        if marbles.0 > maximum {
            return false;
        }
    }
    true
}

fn process(input: &str) -> String {
    let mut cnt: u32 = 0;
    let res = input
        .lines()
        .filter_map(move |line| {
            cnt += 1;
            dbg!(cnt);
            for round in line[line.find(':').expect("should be index") + 1..].split(';') {
                let mut marbles = round
                    .split(',')
                    .inspect(|&marble| {
                        dbg!(marble);
                    })
                    .map(|word| {
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
                        Marbles(num, m_type)
                    });
                if !is_valid_round(&mut marbles) {
                    return None;
                }
            }
            dbg!(cnt);
            Some(cnt)
        })
        .sum::<u32>();
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        let res = process(input);
        assert_eq!(res, "8")
    }
}
