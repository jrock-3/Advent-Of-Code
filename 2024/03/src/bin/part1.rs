use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    let mut res = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();
        res += a * b;
    }
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input));
    }
}
