use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, one_of, space1},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

#[derive(Debug)]
enum Spring {
    Healthy,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Record {
    springs: Vec<Spring>,
    damaged_list: Vec<u32>,
}

impl Record {
    fn calculate_permuatations(&mut self) -> u32 {
        todo!()
    }
}

fn get_records(input: &str) -> IResult<&str, Vec<Record>> {
    separated_list1(
        newline,
        tuple((
            many1(one_of("#.?").map(|c| match c {
                '.' => Spring::Healthy,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => unreachable!(),
            })),
            space1,
            separated_list1(tag(","), complete::u32),
        ))
        .map(|(springs, _, damaged_list)| Record {
            springs,
            damaged_list,
        }),
    )(input)
}

fn process(input: &str) -> String {
    let (_, records) = get_records(input).unwrap();

    let res = records
        .into_iter()
        .map(|mut record| record.calculate_permuatations())
        .sum::<u32>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("21", process(input));
    }
}
