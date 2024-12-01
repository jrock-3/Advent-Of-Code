use std::collections::BinaryHeap;

use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, multispace0},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../in/in.txt");
    dbg!(process(input));
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(
        line_ending,
        tuple((complete::u32, multispace0, complete::u32)).map(|(l, _, r)| (l, r)),
    )(input)
}

fn process(input: &str) -> String {
    // sort both lists
    let mut total = 0;
    let mut llist = Vec::new();
    let mut rlist = Vec::new();

    let (_, pairs) = parse_lines(input).unwrap();
    for (left, right) in pairs {
        llist.push(left);
        rlist.push(right);
    }

    llist.sort();
    let lfreqs = llist.iter().counts();
    rlist.sort();
    let rfreqs = rlist.into_iter().counts();

    dbg!(&lfreqs, &rfreqs);
    for num in llist {
        total += (num as usize) * *rfreqs.get(&num).unwrap_or(&0);
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("31", process(input));
    }
}
