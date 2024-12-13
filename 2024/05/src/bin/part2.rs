use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn parse_orderings(input: &str) -> IResult<&str, HashMap<u32, BTreeSet<u32>>> {
    let (input, rules) = separated_list1(
        line_ending,
        separated_pair(complete::u32, tag("|"), complete::u32),
    )(input)?;

    let orderings = rules
        .into_iter()
        .into_grouping_map()
        .collect::<BTreeSet<_>>();

    Ok((input, orderings))
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, separated_list1(tag(","), complete::u32))(input)
}

fn parse_manuals(input: &str) -> IResult<&str, (HashMap<u32, BTreeSet<u32>>, Vec<Vec<u32>>)> {
    let (input, orderings) = parse_orderings(input)?;
    let (input, _) = count(line_ending, 2)(input)?;
    let (input, updates) = parse_updates(input)?;

    Ok((input, (orderings, updates)))
}

fn fix_order(orderings: &HashMap<u32, BTreeSet<u32>>, update: Vec<u32>) -> Vec<u32> {
    let update_set = update.clone().into_iter().collect::<BTreeSet<_>>();
    update
        .into_iter()
        .sorted_by_key(|page| {
            -(orderings
                .get(&page)
                .map(|pages| pages.intersection(&update_set).count())
                .unwrap_or(0) as isize)
        })
        .collect::<Vec<_>>()
}

fn process(input: &str) -> String {
    let (_, (orderings, updates)) = parse_manuals(input).unwrap();

    updates
        .into_iter()
        .filter(|update| {
            !(0..update.len()).all(|i| {
                update[i + 1..].into_iter().all(|num| {
                    orderings.contains_key(&update[i]) && orderings[&update[i]].contains(num)
                })
            })
        })
        .map(|update| fix_order(&orderings, update))
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input));
    }
}
