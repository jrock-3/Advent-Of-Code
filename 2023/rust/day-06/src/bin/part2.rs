use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::opt,
    sequence::delimited,
    IResult,
};

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

#[derive(Debug)]
struct Race {
    time: u128,
    dist: u128,
}

impl Race {
    fn get_combinations(&self) -> u128 {
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
            .count() as u128
    }
}

fn get_races(input: &str) -> IResult<&str, Race> {
    let (input, time) = delimited(tag("Time:"), complete::u128, opt(line_ending))(input)?;

    let (input, dist) = delimited(tag("Distance:"), complete::u128, opt(line_ending))(input)?;

    Ok((input, Race { time, dist }))
}

fn process(input: &str) -> String {
    let (_, races) = get_races(
        input
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .as_str(),
    )
    .unwrap();

    races.get_combinations().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in2() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("71503", process(input));
    }
}
