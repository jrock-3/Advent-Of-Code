use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar, line_ending, multispace1, space1},
    combinator::opt,
    multi::{count, many1, many_till},
    sequence::{delimited, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

#[derive(Debug)]
struct MapEntry {
    dest: u128,
    src: u128,
    range_len: u128,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u128>,
    map_entries: Vec<Vec<MapEntry>>,
}

impl Almanac {
    fn get_min_seed_location(&self) -> u128 {
        self.seeds
            .iter()
            .map(|&seed| {
                self.map_entries.iter().fold(seed, |acc, map| {
                    map.iter()
                        .find(|&map_entry| {
                            map_entry.src <= acc && acc < map_entry.src + map_entry.range_len
                        })
                        .map(|entry| (entry.dest + (acc - entry.src)))
                        .unwrap_or(acc)
                })
            })
            .min()
            .unwrap()
    }
}

fn get_seeds(input: &str) -> IResult<&str, Vec<u128>> {
    let (input, seeds) = delimited(
        tuple((tag("seeds:"), space1)),
        many1(terminated(complete::u128, opt(space1))),
        opt(count(line_ending, 2)),
    )(input)?;

    Ok((input, seeds))
}

fn get_map_entry(input: &str) -> IResult<&str, Vec<MapEntry>> {
    let (input, _) = many_till(anychar, line_ending)(input)?;
    let (input, nums) = many1(count(terminated(complete::u128, multispace1), 3))(input)?;
    Ok((
        input,
        nums.iter()
            .map(|n| MapEntry {
                dest: n[0],
                src: n[1],
                range_len: n[2],
            })
            .collect(),
    ))
}

fn get_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = get_seeds(input)?;
    let (input, map_entries) = many1(get_map_entry)(input)?;

    Ok((input, Almanac { seeds, map_entries }))
}

fn process(input: &str) -> String {
    let (_, almanac) = get_almanac(input).unwrap();

    let res = almanac.get_min_seed_location();
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("35", process(input));
    }
}
