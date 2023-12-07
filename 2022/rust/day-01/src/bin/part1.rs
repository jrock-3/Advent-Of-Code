use std::cmp;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines = input.split('\n');
    let mut max = 0;
    let mut curr = 0;
    for line in lines {
        match line {
            "" => {
                max = cmp::max(max, curr);
                curr = 0;
            }
            s => curr += s.parse::<i32>().unwrap(),
        }
    }
    max = cmp::max(max, curr);
    max.to_string()
}

// solution from chris biscardi: https://www.youtube.com/watch?v=bkvSRfgDG-E&list=PLWtPciJ1UMuBNTifxm5ADY65SkAdwoQiL&index=1
#[allow(dead_code)]
fn process_sol(input: &str) -> String {
    let res = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|food| food.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input1() {
        let input = include_str!("./test_input1.txt");
        let result = process(input);
        assert_eq!(result, "24000");
    }
}
