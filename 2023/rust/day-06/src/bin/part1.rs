use std::iter::zip;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::opt,
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

#[derive(Debug)]
struct Race {
    time: u32,
    dist: u32,
}

impl Race {
    fn get_combinations(&self) -> u32 {
        /*
        Example:
        - time: 7
        - dist: 9
        ---
        Num ways to win:
        1. hold for 2ms, coast at 2mm/ms for 7ms to get 14mm
        2. hold for 3ms, coast at 3mm/ms for 6ms to get 18mm
        3. hold for 4ms, coast at 4mm/ms for 5ms to get 20mm
        4. hold for 5ms, coast at 5mm/ms for 4ms to get 20mm
        ---
        Observations:
        - need at least [dist] to win
        - if hold for [X] ms, get [X] mm/ms of speed for [time - X] ms, total dist of [X * (time - X)] mm
        - goal: count all [X] where [X * (time - X)] > [dist]
        */
        (0..self.time)
            .filter(|hold| hold * (self.time - hold) > self.dist)
            .count() as u32
    }
}

fn get_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = delimited(
        tuple((tag("Time:"), space1)),
        many1(terminated(complete::u32, opt(space1))),
        line_ending,
    )(input)?;

    let (input, distances) = delimited(
        tuple((tag("Distance:"), space1)),
        many1(terminated(complete::u32, opt(space1))),
        opt(line_ending),
    )(input)?;

    Ok((
        input,
        zip(times, distances)
            .map(|(time, dist)| Race { time, dist })
            .collect(),
    ))
}

fn process(input: &str) -> String {
    let (_, races) = get_races(input).unwrap();
    let res = races
        .iter()
        .map(|race| race.get_combinations())
        .product::<u32>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("288", process(input));
    }
}
