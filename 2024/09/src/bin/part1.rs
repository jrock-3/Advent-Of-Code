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

    let mut idx = 0;
    while idx < disk.len() {
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
