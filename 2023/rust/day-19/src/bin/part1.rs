use std::{collections::BTreeMap, ops::BitXor};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, one_of},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl Category {
    fn from(s: char) -> Category {
        match s {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Comparator {
    LessThan,
    GreaterThan,
}

impl Comparator {
    fn from(s: char) -> Comparator {
        match s {
            '<' => Self::LessThan,
            '>' => Self::GreaterThan,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rule<'a> {
    cat: Category,
    comp: Comparator,
    val: u32,
    dest: &'a str,
}

impl Rule<'_> {
    fn from(cat: char, comp: char, val: u32, dest: &str) -> Rule {
        Rule {
            cat: Category::from(cat),
            comp: Comparator::from(comp),
            val,
            dest,
        }
    }

    fn passes(&self, rating: &[u32]) -> bool {
        let val = rating[(self.cat as i32) as usize];
        match self.comp {
            Comparator::LessThan => val < self.val,
            Comparator::GreaterThan => val > self.val,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    default_dest: &'a str,
}

impl Workflow<'_> {
    fn get_next(&self, rating: &[u32]) -> &str {
        for rule in self.rules.iter() {
            if rule.passes(rating) {
                return rule.dest;
            }
        }
        self.default_dest
    }
}

fn get_parts(input: &str) -> IResult<&str, (BTreeMap<&str, Workflow>, Vec<Vec<u32>>)> {
    let (input, workflows) = terminated(
        separated_list1(
            newline,
            tuple((
                alpha1,
                delimited(
                    tag("{"),
                    tuple((
                        many1(
                            tuple((
                                one_of("xmas"),
                                one_of("<>"),
                                complete::u32,
                                tag(":"),
                                alpha1,
                                tag(","),
                            ))
                            .map(|(cat, comp, val, _, dest, _)| Rule::from(cat, comp, val, dest)),
                        ),
                        alpha1,
                    ))
                    .map(|(rules, default_dest)| Workflow {
                        rules,
                        default_dest,
                    }),
                    tag("}"),
                ),
            )),
        )
        .map(BTreeMap::from_iter),
        tuple((newline, newline)),
    )(input)?;

    let (input, ratings) = separated_list1(
        newline,
        delimited(
            tag("{"),
            separated_list1(tag(","), tuple((one_of("xmas"), tag("="), complete::u32)))
                .map(|values| values.iter().map(|(_, _, val)| *val).collect::<Vec<_>>()),
            tag("}"),
        ),
    )(input)?;

    Ok((input, (workflows, ratings)))
}

fn process(input: &str) -> String {
    let (_, (workflows, ratings)) = get_parts(input).unwrap();

    let res = ratings
        .into_iter()
        .filter_map(|rating| {
            let mut curr_workflow = "in";
            loop {
                curr_workflow = workflows[curr_workflow].get_next(&rating);
                if curr_workflow == "A" {
                    return Some(rating.iter().sum::<u32>());
                } else if curr_workflow == "R" {
                    return None;
                }
            }
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
        assert_eq!("19114", process(input));
    }
}
