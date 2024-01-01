use std::collections::{BTreeMap, VecDeque};

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{self, alpha1, one_of},
    combinator::opt,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

#[derive(Debug)]
enum Op {
    Remove,
    Add,
}

impl Op {
    fn from(c: char) -> Op {
        match c {
            '-' => Op::Remove,
            '=' => Op::Add,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    op: Op,
    focal_len: Option<u32>,
    hash: usize,
}

impl<'a> Lens<'a> {
    fn get_lens(input: &str) -> IResult<&str, (&str, char, Option<u32>)> {
        tuple((alpha1, one_of("=-"), opt(complete::u32)))(input)
    }

    fn new(input: &str) -> Lens {
        let (_, (label, op, focal_len)) = Lens::get_lens(input).unwrap();
        let mut hash: usize = 0;
        for c in label.chars() {
            hash += c as usize;
            hash *= 17;
            hash %= 256;
        }

        Lens {
            label,
            op: Op::from(op),
            focal_len,
            hash,
        }
    }
}

fn get_lenses(input: &str) -> IResult<&str, Vec<Lens>> {
    terminated(
        separated_list1(tag(","), take_till(|c| c == ',' || c == '\n').map(|s| Lens::new(s))),
        tag("\n"),
    )(input)
}

fn process(input: &str) -> String {
    let (_, lenses) = get_lenses(input).unwrap();
    let mut boxes: Vec<VecDeque<&str>> = (0..256).map(|_| VecDeque::new()).collect();
    let mut focal_lengths: BTreeMap<&str, u32> = BTreeMap::new();

    for lens in lenses {
        match lens.op {
            Op::Remove => {
                if let Some(idx) = boxes.iter().position(|lenses| lenses.contains(&lens.label)) {
                    let index = boxes
                        .get(idx)
                        .unwrap()
                        .iter()
                        .position(|l| l == &lens.label)
                        .unwrap();
                    boxes.get_mut(idx).unwrap().remove(index);
                    focal_lengths.remove(lens.label);
                }
            }
            Op::Add => {
                if !boxes.get(lens.hash).unwrap().contains(&lens.label) {
                    boxes.get_mut(lens.hash).unwrap().push_back(lens.label);
                }
                focal_lengths.insert(lens.label, lens.focal_len.unwrap());
            }
        }
    }

    let res: usize = boxes
        .into_iter()
        .enumerate()
        .map(|(i, container)| {
            container
                .into_iter()
                .enumerate()
                .map(|(j, lens)| {
                    let focal_len = focal_lengths.get(lens).unwrap().to_owned() as usize;
                    
                    (i + 1) * (j + 1) * focal_len
                })
                .sum::<usize>()
        })
        .sum();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("145", process(input));
    }
}
