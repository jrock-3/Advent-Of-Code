use regex::Regex;

fn main() {
    let input = include_str!("../in/in1.txt");
    let res = process(input);
    dbg!("{:?}", res);
}

fn match_to_val(input: &str) -> u32 {
    match input {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n @ ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9") => n.parse::<u32>().unwrap(),
        _ => panic!(),
    }
}

fn process(input: &str) -> String {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_rev = Regex::new(r"(enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|\d)").unwrap();

    let res = input.split('\n').map(|line| {
        let first = line[re.find(line).unwrap().range()].to_string();
        let first_val = match_to_val(first.as_str());

        let rev_line = line.chars().rev().collect::<String>();
        let last = rev_line[re_rev.find(rev_line.as_str()).unwrap().range()].to_string();
        let last_val = match_to_val(last.chars().rev().collect::<String>().as_str());

        first_val * 10 + last_val
    });
    // dbg!("{}", res.collect::<Vec<_>>());
    res.sum::<u32>().to_string()
    // input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample2.txt");
        let res = process(input);
        assert_eq!(res, "281")
    }
}
