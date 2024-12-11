use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    let re = Regex::new(r"do(n't)?\(\)|mul\(\d+,\d+\)").unwrap();

    re.find_iter(input)
        .fold((true, 0), |(enabled, acc), capture| {
            match (enabled, capture.as_str()) {
                (_, "do()") => (true, acc),
                (_, "don't()") => (false, acc),
                (true, mul) => {
                    let (a, b) = mul[4..mul.len() - 1].split_once(",").unwrap();
                    let a = a.parse::<u32>().unwrap();
                    let b = b.parse::<u32>().unwrap();
                    (true, acc + a * b)
                }
                _ => (enabled, acc),
            }
        })
        .1
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input));
    }
}
