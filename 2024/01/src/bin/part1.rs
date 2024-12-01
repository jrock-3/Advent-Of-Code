use std::collections::BinaryHeap;

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

fn parse_lines(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(
        line_ending,
        tuple((complete::i32, multispace0, complete::i32)).map(|(l, _, r)| (l, r)),
    )(input)
}

fn process(input: &str) -> String {
    // sort both lists
    let mut total = 0;
    let mut llist = BinaryHeap::new();
    let mut rlist = BinaryHeap::new();

    let (_, pairs) = parse_lines(input).unwrap();
    for (left, right) in pairs {
        llist.push(left);
        rlist.push(right);
    }

    loop {
        if llist.is_empty() {
            break;
        }
        total += (llist.pop().unwrap() - rlist.pop().unwrap()).abs();
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("11", process(input));
    }
}
