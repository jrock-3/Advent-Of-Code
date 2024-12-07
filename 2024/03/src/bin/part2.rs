use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    let mut res = 0;
    let re = Regex::new(r"do(n't)?\(\)|mul\(\d+,\d+\)").unwrap();
    let mut enabled = true;
    for capture in re.find_iter(input) {
        match capture.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            mul => {
                if !enabled {
                    continue;
                }
                let (a, b) = mul[4..mul.len() - 1].split_once(",").unwrap();
                let a = a.parse::<u32>().unwrap();
                let b = b.parse::<u32>().unwrap();
                res += a * b;
            }
        }
    }
    res.to_string()
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
