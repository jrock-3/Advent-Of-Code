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

fn parse_lines(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(
        line_ending,
        tuple((complete::i32, multispace0, complete::i32)).map(|(l, _, r)| (l, r)),
    )(input)
}

fn process(input: &str) -> String {
    let (_, pairs) = parse_lines(input).unwrap();

    let (mut llist, mut rlist) = pairs.iter().fold(
        (Vec::new(), Vec::new()),
        |(mut llist, mut rlist), (l, r)| {
            llist.push(l);
            rlist.push(r);
            (llist, rlist)
        },
    );

    llist.sort();
    rlist.sort();

    std::iter::zip(llist, rlist)
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<u32>()
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
        assert_eq!("11", process(input));
    }
}
