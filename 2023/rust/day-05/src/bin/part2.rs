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
struct SeedRange {
    start: u128,
    range_len: u128,
}

#[derive(Debug)]
struct MapEntry {
    dest: u128,
    src: u128,
    range_len: u128,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<SeedRange>,
    map_entries: Vec<Vec<MapEntry>>,
}

impl Almanac {
    fn reverse_search(&self) -> u128 {
        let mut res = 0;
        loop {
            // check if (after reverse transforming res--a location--to seed) is within input range
            // TODO: potential optimization - start from smallest seed range (transformed) - i think this is part1 solution
            let mut num = res;
            for maps in self.map_entries.iter().rev() {
                for map_entry in maps {
                    if map_entry.dest <= num && num < map_entry.dest + map_entry.range_len {
                        num = map_entry.src + (num - map_entry.dest);
                        break;
                    }
                }
            }
            for seed_range in self.seeds.iter() {
                if seed_range.start <= num && num < seed_range.start + seed_range.range_len {
                    return res;
                }
            }
            res += 1;
        }
    }
}

fn get_seeds(input: &str) -> IResult<&str, Vec<SeedRange>> {
    let (input, nums) = delimited(
        tuple((tag("seeds:"), space1)),
        many1(count(terminated(complete::u128, opt(space1)), 2)),
        opt(count(line_ending, 2)),
    )(input)?;

    let seeds = nums
        .iter()
        .map(|nums| SeedRange {
            start: nums[0],
            range_len: nums[1],
        })
        .collect::<Vec<_>>();

    Ok((input, seeds))
}

fn get_map_entry(input: &str) -> IResult<&str, Vec<MapEntry>> {
    let (input, _) = many_till(anychar, line_ending)(input)?;
    let (input, nums) = many1(count(terminated(complete::u128, multispace1), 3))(input)?;

    let map_entries = nums
        .iter()
        .map(|n| MapEntry {
            dest: n[0],
            src: n[1],
            range_len: n[2],
        })
        .collect::<Vec<_>>();

    Ok((input, map_entries))
}

fn get_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = get_seeds(input)?;
    let (input, map_entries) = many1(get_map_entry)(input)?;

    Ok((input, Almanac { seeds, map_entries }))
}

fn process(input: &str) -> String {
    let (_, almanac) = get_almanac(input).unwrap();

    let res = almanac.reverse_search();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in2() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("46", process(input));
    }
}
