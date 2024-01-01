fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    let res: u32 = input[..input.len() - 1]
        .split(',')
        .map(|step: &str| {
            let mut count: u32 = 0;
            for c in step.chars() {
                count += c as u32;
                count *= 17;
                count %= 256;
            }
            dbg!(step, &count);
            count
        })
        .sum();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("1320", process(input));
    }
}
