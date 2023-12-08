use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, multispace1, newline, one_of},
    combinator::opt,
    multi::{count, many1},
    sequence::{terminated, tuple},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn from(raw: char) -> Dir {
        match raw {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Map {
    dirs: Vec<Dir>,
    maps: BTreeMap<String, Node>,
}

impl Map {
    fn apply_dirs(&self, init: String) -> String {
        self.dirs.iter().fold(init, |acc, dir| match dir {
            Dir::Left => self.maps.get(&acc).unwrap().left.clone(),
            Dir::Right => self.maps.get(&acc).unwrap().right.clone(),
        })
    }

    fn count_to_end(&self) -> u32 {
        let mut cnt = 0;
        let mut curr = String::from("AAA");
        while curr != "ZZZ" {
            cnt += 1;
            curr = self.apply_dirs(curr);
        }
        cnt * self.dirs.len() as u32
    }
}

fn get_maps(input: &str) -> IResult<&str, Map> {
    let (input, (dirs, _)) = tuple((many1(one_of("RL").map(Dir::from)), multispace1))(input)?;

    let (input, lines) = many1(terminated(
        tuple((
            count(anychar, 3).map(|chars| chars.iter().collect::<String>()),
            tag(" = ("),
            count(anychar, 3).map(|chars| chars.iter().collect::<String>()),
            tag(", "),
            count(anychar, 3).map(|chars| chars.iter().collect::<String>()),
            tag(")"),
        )),
        opt(newline),
    ))(input)?;

    let mut maps = BTreeMap::new();
    for (src, _, left, _, right, _) in lines {
        maps.insert(src, Node { left, right });
    }

    Ok((input, Map { dirs, maps }))
}

fn process(input: &str) -> String {
    let (_, map) = get_maps(input).unwrap();

    let res = map.count_to_end();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("2", process(input));
    }

    #[test]
    fn test_in2() {
        let input = include_str!("../in/sample2.txt");
        assert_eq!("6", process(input));
    }
}
