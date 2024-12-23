use std::collections::BTreeMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn run_program(regs: &mut BTreeMap<&str, u32>, program: &Vec<u32>) -> Vec<u32> {
    let mut res = Vec::new();
    let mut idx = 0;
    while idx + 1 < program.len() {
        let (opcode, operand) = (program[idx], program[idx + 1]);
        let combo = match operand {
            0..=3 => operand,
            4 => regs["A"],
            5 => regs["B"],
            6 => regs["C"],
            _ => unreachable!(),
        };

        match opcode {
            // A = A / 1 << combo
            0 => {
                *regs.get_mut("A").unwrap() = regs["A"] / (1 << combo);
            }
            // B = B ^ operand
            1 => {
                *regs.get_mut("B").unwrap() = regs["B"] ^ operand;
            }
            // B = combo & 0b111
            2 => {
                *regs.get_mut("B").unwrap() = combo & 7u32;
            }
            // jnz
            3 => {
                if regs["A"] != 0 {
                    idx = operand as usize;
                    continue;
                }
            }
            // B = B ^ C
            4 => {
                *regs.get_mut("B").unwrap() = regs["B"] ^ regs["C"];
            }
            // print combo & 0b111
            5 => {
                res.push(combo & 7u32);
            }
            // B = A / 1 << combo
            6 => {
                *regs.get_mut("B").unwrap() = regs["A"] / (1 << combo);
            }
            // C = A / 1 << combo
            7 => {
                *regs.get_mut("C").unwrap() = regs["A"] / (1 << combo);
            }
            _ => unreachable!(),
        }

        idx += 2;
    }
    res
}

fn process(input: &str) -> String {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let mut regs = registers
        .lines()
        .map(|line| {
            let (name, val) = line
                .strip_prefix("Register ")
                .unwrap()
                .split_once(": ")
                .unwrap();
            let val = val.parse::<u32>().unwrap();
            (name, val)
        })
        .collect::<BTreeMap<_, _>>();

    let program = program
        .trim()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|val| val.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let res = run_program(&mut regs, &program);

    res.into_iter().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input));
    }

    #[test]
    fn test_in2() {
        let input = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";
        assert_eq!("0,1,2", process(input));
    }
}
