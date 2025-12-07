fn main() {
    let input = include_str!("../input.txt");
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
        let input = "";
        assert_eq!("", process(input));
    }
}
