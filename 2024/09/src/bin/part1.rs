use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

fn process(input: &str) -> String {
    let mut disk = VecDeque::new();
    for (i, num) in input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
    {
        if i % 2 == 0 {
            disk.extend((0..num).map(|_| (i / 2).to_string()));
        } else {
            disk.extend((0..num).map(|_| ".".to_string()));
        }
    }
    // dbg!(&disk);

    let mut idx = 0;
    while idx < disk.len() {
        // dbg!(&idx, &disk[idx]);
        if disk[idx].as_str() != "." {
            idx += 1;
            continue;
        }

        let mut block = disk.pop_back().unwrap();
        while idx + 1 < disk.len() && block.as_str() == "." {
            block = disk.pop_back().unwrap();
        }

        disk[idx] = block;
        idx += 1;
    }
    // dbg!(&disk);

    let checksum = disk
        .into_iter()
        .enumerate()
        .filter(|(_, block)| block != ".")
        .map(|(i, block)| i * block.parse::<usize>().unwrap())
        .sum::<usize>();

    checksum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input));
    }
}

/*
0 1 2 3 4 5 6 7 8 9
2333133121414131402

        0123456789
files:  2313244342
spaces: 333111110

        0123456789
files:  2313244340
spaces: 133111110

let idx = files.len() - 1
let block = 0
while idx > 0 and !spaces.is_empty
    cnt = min(spaces[0],files[-1])
    spaces[0] -= cnt
    res += idx * ( (block+
    // 4 * 5 + 4 * 6 + 4 * 7 = 4 * (5 + 6 + 7) = 4 *
    files[spaces.len()-1] -= cnt
    if files[spaces.len()-1] == 0:
        files.pop_back()
        idx -= 1
    if spaces[0] == 0: spaces.pop_back()
res += idx * files[0] * (files[0] + 1) / 2

n -> n + k
( (n+k)(n+k+1) - (n)(n+1) ) / 2

*/
