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

            let you = u32::from(x.get(1).unwrap().chars().next().unwrap()) - u32::from('X');

            let round = if oppo == you {
                3
            } else if you > oppo && you - oppo == 1 || oppo > you && oppo - you == 2 {
                6
            } else {
                0
            };
            you + 1 + round
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
        assert_eq!(res, "15")
    }
}
