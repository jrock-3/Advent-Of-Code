use std::collections::{BTreeMap, BTreeSet};

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
    starts: BTreeSet<String>,
}

impl Map {
    fn apply_dirs(&self, init: String) -> String {
        self.dirs.iter().fold(init, |acc, dir| match dir {
            Dir::Left => self.maps.get(&acc).unwrap().left.clone(),
            Dir::Right => self.maps.get(&acc).unwrap().right.clone(),
        })
    }

    fn count_to_end(&self, start: &str) -> u128 {
        let mut cnt = 0;
        let mut curr = start.to_string();
        while !curr.ends_with('Z') {
            cnt += 1;
            curr = self.apply_dirs(curr);
        }
        cnt * self.dirs.len() as u128
    }

    fn get_simul_end(&self) -> u128 {
        self.starts.iter().fold(1, |acc, start| {
            num::integer::lcm(acc, self.count_to_end(start))
        })
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

    let maps = lines
        .clone()
        .into_iter()
        .map(|(src, _, left, _, right, _)| (src, Node { left, right }))
        .collect::<BTreeMap<_, _>>();

    let starts = lines
        .into_iter()
        .filter_map(|(src, ..)| {
            if src.ends_with('A') {
                Some(src.clone())
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();

    Ok((input, Map { dirs, maps, starts }))
}

fn process(input: &str) -> String {
    let (_, map) = get_maps(input).unwrap();

    let res = map.get_simul_end();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in3() {
        let input = include_str!("../in/sample3.txt");
        assert_eq!("6", process(input));
    }
}
