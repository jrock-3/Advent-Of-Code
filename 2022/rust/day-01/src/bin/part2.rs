fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines = input.split('\n');
    let mut sums: Vec<i32> = vec![];
    let mut curr = 0;
    for line in lines {
        match line {
            "" => {
                sums.push(curr);
                curr = 0;
            }
            s => curr += s.parse::<i32>().unwrap(),
        }
    }
    sums.push(curr);
    sums.sort();
    sums[sums.len() - 3..sums.len()]
        .iter()
        .sum::<i32>()
        .to_string()
}

// solution from chris biscardi: https://www.youtube.com/watch?v=bkvSRfgDG-E&list=PLWtPciJ1UMuBNTifxm5ADY65SkAdwoQiL&index=1
#[allow(dead_code)]
fn process_sol(input: &str) -> String {
    let mut res = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|food| food.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    res.sort();
    let sum = res.iter().take(3).sum::<u32>();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input1() {
        let input = include_str!("./test_input1.txt");
        let result = process(input);
        assert_eq!(result, "45000");
    }
}
