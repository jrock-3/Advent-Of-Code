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
enum Digit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

impl Digit {
    fn from(c: char) -> Digit {
        match c {
            '2' => Digit::Two,
            '1' => Digit::One,
            '0' => Digit::Zero,
            '-' => Digit::Minus,
            '=' => Digit::DoubleMinus,
            _ => unreachable!(),
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                Digit::Two => '2',
                Digit::One => '1',
                Digit::Zero => '0',
                Digit::Minus => '-',
                Digit::DoubleMinus => '=',
            }
            .to_string()
            .as_str(),
        )
    }
}

#[derive(Debug)]
struct SnafuDigit {
    digit: Digit,
    carry: u128,
}

impl SnafuDigit {
    fn new(digit: Digit, carry: u128) -> SnafuDigit {
        SnafuDigit { digit, carry }
    }

    fn from(digit: u32) -> SnafuDigit {
        match digit {
            0 => SnafuDigit::new(Digit::Zero, 0),
            1 => SnafuDigit::new(Digit::One, 0),
            2 => SnafuDigit::new(Digit::Two, 0),
            3 => SnafuDigit::new(Digit::DoubleMinus, 1),
            4 => SnafuDigit::new(Digit::Minus, 1),
            _ => unreachable!(),
        }
    }

    fn to_base5(&self) -> i128 {
        match self.digit {
            Digit::Two => 2,
            Digit::One => 1,
            Digit::Zero => 0,
            Digit::Minus => -1,
            Digit::DoubleMinus => -2,
        }
    }

    fn next(&mut self) {
        match self.digit {
            Digit::DoubleMinus => self.digit = Digit::Minus,
            Digit::Minus => self.digit = Digit::Zero,
            Digit::Zero => self.digit = Digit::One,
            Digit::One => self.digit = Digit::Two,
            Digit::Two => {
                self.digit = Digit::DoubleMinus;
                self.carry += 1;
            }
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
         * Example:
         * * 124030
         * * 2=-1=0
         * Approach:
         * * convert to base 5, then convert
         */
        let mut base = Convert::new(10, 5);
        let base5_digits = base.convert::<u32, u32>(
            &decimal
                .to_string()
                .chars()
                .rev()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );

        let digits = base5_digits
            .into_iter()
            .fold(vec![], |mut acc: Vec<SnafuDigit>, digit| {
                let mut carry = SnafuDigit::from(digit);

                if acc.is_empty() {
                    acc.push(carry);
                    return acc;
                }

                match acc[acc.len() - 1].carry.cmp(&1) {
                    Ordering::Equal => carry.next(),
                    Ordering::Greater => unreachable!(),
                    Ordering::Less => (),
                }
                acc.push(carry);

                acc
            })
            .into_iter()
            .rev()
            .collect::<Vec<_>>();

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
        for digit in self
            .digits
            .iter()
            .map(|SnafuDigit { digit, carry: _ }| digit)
        {
            f.write_str(digit.to_string().as_str())?;
        }
        Ok(())
    }
}

fn get_snafu_nums(input: &str) -> IResult<&str, Vec<SnafuNum>> {
    let (input, words) = many1(terminated(many1(one_of("210-=")), opt(line_ending)))(input)?;

    let snafu_nums = words
        .into_iter()
        .map(|chars| SnafuNum {
            digits: chars
                .into_iter()
                .map(|char| SnafuDigit::new(Digit::from(char), 0))
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();

    Ok((input, snafu_nums))
}

fn process(input: &str) -> String {
    let (_, snafu_nums) = get_snafu_nums(input).unwrap();

    let res = SnafuNum::from(
        snafu_nums
            .into_iter()
            .map(|snafu_num| snafu_num.to_decimal())
            .sum(),
    );

    res.to_string()
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
