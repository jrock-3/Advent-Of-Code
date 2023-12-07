use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

use convert_base::Convert;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

#[derive(Debug)]
struct SnafuDigit {
    digit: char,
    carry: u128,
}

impl SnafuDigit {
    fn new(digit: char, carry: u128) -> SnafuDigit {
        SnafuDigit { digit, carry }
    }

    fn to_base5(&self) -> i128 {
        match self.digit {
            n @ ('2' | '1' | '0') => n.to_digit(10).unwrap() as i128,
            '-' => -1,
            '=' => -2,
            _ => unreachable!(),
        }
    }

    fn next(&mut self) {
        match self.digit {
            '=' => self.digit = '-',
            '-' => self.digit = '0',
            '0' => self.digit = '1',
            '1' => self.digit = '2',
            '2' => {
                self.digit = '=';
                self.carry += 1;
            }
            _ => unreachable!(),
        };
    }
}

#[derive(Debug)]
struct SnafuNum {
    digits: Vec<SnafuDigit>,
}

impl SnafuNum {
    fn from(decimal: u128) -> SnafuNum {
        /*
         * 124030
         * 2=-1=0
         * >> convert to base 5, then convert
         */
        let mut base = Convert::new(10, 5);
        let digits = base
            .convert::<u32, u32>(
                &decimal
                    .to_string()
                    .chars()
                    .rev()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect::<Vec<_>>(),
            )
            .into_iter()
            .fold(vec![], |mut acc: Vec<SnafuDigit>, digit| {
                let mut carry = match digit {
                    0 => SnafuDigit::new('0', 0),
                    1 => SnafuDigit::new('1', 0),
                    2 => SnafuDigit::new('2', 0),
                    3 => SnafuDigit::new('=', 1),
                    4 => SnafuDigit::new('-', 1),
                    _ => unreachable!(),
                };

                if acc.is_empty() {
                    acc.push(carry);
                    return acc;
                }

                match acc[0].carry.cmp(&1) {
                    Ordering::Equal => carry.next(),
                    Ordering::Greater => unreachable!(),
                    _ => (),
                }

                acc.insert(0, carry);
                acc
            });
        SnafuNum { digits }
    }

    fn to_decimal(&self) -> u128 {
        self.digits
            .iter()
            .fold(0i128, |acc, digit| acc * 5 + digit.to_base5()) as u128
    }
}

impl Display for SnafuNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for digit in &self.digits {
            f.write_str(digit.digit.to_string().as_str())?;
        }
        Ok(())
    }
}

fn get_snafu_nums(input: &str) -> IResult<&str, Vec<SnafuNum>> {
    let (input, words) = many1(terminated(many1(one_of("210-=")), opt(line_ending)))(input)?;
    let snafu_nums = words
        .iter()
        .map(|chars| SnafuNum {
            digits: chars
                .iter()
                .map(|&char| SnafuDigit::new(char, 0))
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();
    Ok((input, snafu_nums))
}

fn process(input: &str) -> String {
    let (_, snafu_nums) = get_snafu_nums(input).unwrap();
    SnafuNum::from(
        snafu_nums
            .into_iter()
            .map(|snafu_num| snafu_num.to_decimal())
            .sum(),
    )
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in2() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("2=-1=0", process(input));
    }
}
