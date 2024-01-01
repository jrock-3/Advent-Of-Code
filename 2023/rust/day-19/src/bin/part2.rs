use std::collections::{BTreeMap, BTreeSet};

use indicatif::{ParallelProgressIterator, ProgressBarIter};

use rayon::prelude::*;

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
    val: u128,
    dest: &'a str,
}

impl Rule<'_> {
    fn from(cat: char, comp: char, val: u128, dest: &str) -> Rule {
        Rule {
            cat: Category::from(cat),
            comp: Comparator::from(comp),
            val,
            dest,
        }
    }

    fn passes(&self, rating: &[u128]) -> bool {
        let val = rating[(self.cat as i32) as usize];
        match self.comp {
            Comparator::LessThan => val < self.val,
            Comparator::GreaterThan => val > self.val,
        }
    }

    fn get_diffs(&self) -> (BTreeSet<u32>, BTreeSet<u32>, BTreeSet<u32>, BTreeSet<u32>) {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    default_dest: &'a str,
}

impl Workflow<'_> {
    fn get_next(&self, rating: &[u128]) -> &str {
        for rule in self.rules.iter() {
            if rule.passes(rating) {
                return rule.dest;
            }
        }
        self.default_dest
    }
}

fn get_parts(input: &str) -> IResult<&str, BTreeMap<&str, Workflow>> {
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
                                complete::u128,
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

    Ok((input, workflows))
}

fn _parallelized_brute_force(workflows: &BTreeMap<&str, Workflow<'_>>) -> u128 {
    (1..=4000)
        .into_par_iter()
        .map(|x| {
            (1..=4000)
                .into_par_iter()
                .progress_count(4000u64)
                .map(|m| {
                    (1..=4000)
                        .into_par_iter()
                        .map(|a| {
                            (1..=4000)
                                .into_par_iter()
                                .map(|s| {
                                    let rating = vec![x, m, a, s];

                                    let mut curr_workflow = "in";
                                    loop {
                                        curr_workflow = workflows[curr_workflow].get_next(&rating);
                                        if curr_workflow == "A" {
                                            return 1;
                                        } else if curr_workflow == "R" {
                                            return 0;
                                        }
                                    }
                                })
                                .sum::<u128>()
                        })
                        .sum::<u128>()
                })
                .sum::<u128>()
        })
        .sum::<u128>()
}

/**
 * recursive function that returns
 */
fn dfs<'a>(
    curr_workflow: &'a str,
    workflows: &BTreeMap<&str, Workflow<'_>>,
    x: BTreeSet<u32>,
    m: BTreeSet<u32>,
    a: BTreeSet<u32>,
    s: BTreeSet<u32>,
) -> (
    &'a str,
    BTreeSet<u32>,
    BTreeSet<u32>,
    BTreeSet<u32>,
    BTreeSet<u32>,
) {
    dbg!(curr_workflow);
    if curr_workflow == "A" || curr_workflow == "R" {
        return (curr_workflow, x, m, a, s);
    }
    let mut xres = x.clone();
    let mut mres = m.clone();
    let mut ares = a.clone();
    let mut sres = s.clone();
    for rule in workflows[curr_workflow].rules.iter() {
        let (dx, dm, da, ds) = rule.get_diffs();
        let (res, xx, mm, aa, ss) = dfs(
            rule.dest,
            workflows,
            x.intersection(&dx).copied().collect::<BTreeSet<_>>(),
            m.intersection(&dm).copied().collect::<BTreeSet<_>>(),
            a.intersection(&da).copied().collect::<BTreeSet<_>>(),
            s.intersection(&ds).copied().collect::<BTreeSet<_>>(),
        );
        match res {
            "A" => {
                // intersection
                xres = xres.intersection(&xx).copied().collect::<BTreeSet<_>>();
                mres = mres.intersection(&mm).copied().collect::<BTreeSet<_>>();
                ares = ares.intersection(&aa).copied().collect::<BTreeSet<_>>();
                sres = sres.intersection(&ss).copied().collect::<BTreeSet<_>>();
            }
            "R" => {
                // difference
                xres = xres.difference(&xx).copied().collect::<BTreeSet<_>>();
                mres = mres.difference(&mm).copied().collect::<BTreeSet<_>>();
                ares = ares.difference(&aa).copied().collect::<BTreeSet<_>>();
                sres = sres.difference(&ss).copied().collect::<BTreeSet<_>>();
            }
            _ => unreachable!(),
        }
    }
    // handle default case
    ("A", xres, mres, ares, sres)
}

fn process(input: &str) -> String {
    let (_, workflows) = get_parts(input).unwrap();

    // track set of xmas vals
    // filter out invalid vals
    // 1. start at in
    // 2. traverse each edge, updating set
    // 3. at end, if A - intersect, if R - difference
    // - approach: dfs
    // return lengths

    let x = (1..=4000).collect::<BTreeSet<_>>();
    let m = (1..=4000).collect::<BTreeSet<_>>();
    let a = (1..=4000).collect::<BTreeSet<_>>();
    let s = (1..=4000).collect::<BTreeSet<_>>();

    let (_, x, m, a, s) = dfs("in", &workflows, x, m, a, s);

    let res = x.len() * m.len() * a.len() * s.len();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("167409079868000", process(input));
    }
}
