fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("", process(input));
    }
}
