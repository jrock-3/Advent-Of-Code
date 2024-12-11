use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, multispace0},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(
        line_ending,
        tuple((complete::u32, multispace0, complete::u32)).map(|(l, _, r)| (l, r)),
    )(input)
}

fn process(input: &str) -> String {
    let (_, pairs) = parse_lines(input).unwrap();
    let (mut llist, rlist) = pairs.iter().fold(
        (Vec::new(), Vec::new()),
        |(mut llist, mut rlist), (l, r)| {
            llist.push(l);
            rlist.push(r);
            (llist, rlist)
        },
    );

    llist.sort();
    let rfreqs = rlist.into_iter().counts();

    llist
        .into_iter()
        .map(|&l| (l as usize) * *rfreqs.get(&l).unwrap_or(&0))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input));
    }
}
