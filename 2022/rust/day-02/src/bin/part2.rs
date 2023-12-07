fn main() {
    let input = include_str!("../in/in1.txt");
    let res = process(input);
    dbg!("{:?}", res);
}

fn process(input: &str) -> String {
    let res = input
        .split('\n')
        .map(|line| {
            let x = line.split(' ').collect::<Vec<_>>();

            let oppo = u32::from(x.first().unwrap().chars().next().unwrap()) - u32::from('A');

            match *x.get(1).unwrap() {
                "X" => (oppo + 2) % 3 + 1,
                "Y" => oppo + 4,
                "Z" => (oppo + 1) % 3 + 7,
                _ => panic!(),
            }
        })
        // .collect::<Vec<_>>();
        // dbg!("{}", res);
        .sum::<u32>();
    res.to_string()
    // input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        let res = process(input);
        assert_eq!(res, "12")
    }
}
