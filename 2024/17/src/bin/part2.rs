use std::collections::{BTreeMap, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn run_program(regs: &mut BTreeMap<&str, u128>, program: &Vec<u128>) -> Vec<u128> {
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
                *regs.get_mut("B").unwrap() = combo & 7u128;
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
                res.push(combo & 7u128);
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

fn to_bit_string(num: u128, length: usize) -> String {
    let mut res = VecDeque::new();
    for n in 0..length {
        res.push_front((num >> n & 1).to_string());
    }
    res.into_iter()
        .reduce(|acc, curr| format!("{}{}", acc, curr))
        .unwrap()
}

fn process(input: &str) -> String {
    let program = input
        .split_once("\n\n")
        .unwrap()
        .1
        .trim()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|val| val.parse::<u128>().unwrap())
        .collect::<Vec<_>>();
    // dbg!(&program);

    // XYZ = ABC ^ 101 ^ NXT
    // offset = ABC ^ 001
    // verify NXT = (ABC ^ 101 ^ XYZ)

    // NOTE:
    // This looks like a search problem
    // where each node is (res,i)
    // each edge is a different res where you add valid abc 0..=7
    // you fill in abc when conditions are met
    // termination is when i == 0 and program == run_program() output

    let mut regs = BTreeMap::new();
    // regs.insert("A", res);
    regs.insert("B", 0);
    regs.insert("C", 0);

    let mut stack = VecDeque::new();
    stack.push_back((0u128, Some(program.len() - 1)));
    while !stack.is_empty() {
        let (res, i) = stack.pop_back().unwrap();
        if i.is_none() {
            let mut regs = regs.clone();
            regs.insert("A", res);
            if run_program(&mut regs.clone(), &program) == program {
                return res.to_string();
            }
            continue;
        }
        dbg!(to_bit_string(res, i.unwrap() * 3 + 5), i);

        let new_i = i.unwrap().checked_sub(1);

        let i = i.unwrap();
        let shift = (i * 3) as u128;
        let nxt = program[i];
        for abc in 0..=7 {
            let xyz = abc ^ 5 ^ nxt;
            let offset = abc ^ 1;
            println!(
                "{} {} {}",
                to_bit_string(abc, 3),
                to_bit_string(xyz, 3),
                offset
            );
            let curr = res >> (shift + offset) & 7;
            if nxt == abc ^ 5 ^ xyz && curr == xyz {
                stack.push_back((res | abc << shift, new_i));
            }
        }
    }

    // let mut res = 0;
    // println!("{}", to_bit_string(res, 128));
    // for (i, &nxt) in program.iter().enumerate().rev().take(20) {
    //     let shift = i as u128 * 3;
    //     dbg!(&shift);
    //
    //     println!("abc xyz offset");
    //     let abc = (0..=7)
    //         .find(|&abc| {
    //             let xyz = abc ^ 5 ^ nxt;
    //             let offset = abc ^ 1;
    //             println!(
    //                 "{} {} {}",
    //                 to_bit_string(abc, 3),
    //                 to_bit_string(xyz, 3),
    //                 offset
    //             );
    //             let curr = res >> (shift + offset) & 7;
    //             nxt == abc ^ 5 ^ xyz && (curr == xyz)
    //         })
    //         .unwrap();
    //
    //     res |= abc << shift;
    //
    //     println!("{}", to_bit_string(res, 128));
    // }
    //
    // let mut regs = BTreeMap::new();
    // regs.insert("A", res);
    // regs.insert("B", 0);
    // regs.insert("C", 0);
    //
    // let output = run_program(&mut regs, &program);
    // println!(
    //     "{}",
    //     program
    //         .into_iter()
    //         .map(|num| num.to_string())
    //         .collect::<Vec<_>>()
    //         .join(",")
    // );
    // println!(
    //     "{}",
    //     output
    //         .into_iter()
    //         .map(|num| num.to_string())
    //         .collect::<Vec<_>>()
    //         .join(",")
    // );
    //
    // res.to_string()

    ".".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_in1() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!("117440", process(input));
    }

    #[test]
    fn test_in2() {
        let input = "Register A: 46337277
Register B: 0
Register C: 0

Program: 2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0";
        assert_eq!("", process(input));
    }
}

/*

Register A: 46337277
Register B: 0
Register C: 0

Program: 2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0

Intructions:
2,4 => B = A & 0b111
1,1 => B = B ^ 0b001
7,5 => C = A >> B
4,4 => B = B ^ C
1,4 => B = B ^ 0b100
0,3 => A = A >> 3
5,5 => print B & 0b111
3,0 => if A != 0: jmp 0

print (((A & 111 ^ 001) ^ (A >> (A & 111 ^ 001))) ^ 0b100) & 111

solve NXT = (ABC ^ 101 ^ (TUVWXYZABC >> (ABC ^ 001))) & 111
- try every ABC 0..=7
XYZ = ABC ^ 101 ^ NXT
offset = ABC ^ 001
verify XYZ == num >> shift + offset

*/
