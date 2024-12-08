use std::collections::{BTreeSet, VecDeque};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn parse_equations(input: &str) -> IResult<&str, Vec<(u128, Vec<u128>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::u128,
            tag(": "),
            separated_list1(space1, complete::u128),
        ),
    )(input)
}

fn process(input: &str) -> String {
    let (_input, equations) = parse_equations(input).unwrap();
    // dbg!(&equations);

    equations
        .into_iter()
        .filter_map(|(goal, nums)| {
            // dbg!(&goal, &nums);
            let mut queue = VecDeque::new();
            queue.push_back((nums[0], 1));

            while !queue.is_empty() {
                let (curr, idx) = queue.pop_back().unwrap();
                // dbg!(&curr, idx);

                if curr > goal {
                    continue;
                }

                if idx == nums.len() {
                    if curr == goal {
                        return Some(goal);
                    }
                    continue;
                }

                queue.push_back((curr + nums[idx], idx + 1));
                queue.push_back((curr * nums[idx], idx + 1));
                queue.push_back((
                    curr * 10u128.pow(nums[idx].ilog10() + 1) + nums[idx],
                    idx + 1,
                ));
            }

            None
        })
        // .inspect(|x| {
        //     dbg!(x);
        // })
        .sum::<u128>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", process(input));
    }
}
