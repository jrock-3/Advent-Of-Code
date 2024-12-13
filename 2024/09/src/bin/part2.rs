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
            disk.push_back((Some(i / 2), num));
        } else {
            disk.push_back((None, num));
        }
    }

    let mut r = disk.len() - 1;
    while r > 0 {
        if disk[r].0.is_none() {
            r -= 1;
            continue;
        }

        let mut l = 0;
        while l < r && (disk[l].0.is_some() || disk[l].0.is_none() && disk[r].1 > disk[l].1) {
            l += 1;
        }

        if disk[l].0.is_some() || disk[r].1 > disk[l].1 {
            r -= 1;
            continue;
        }

        let extra = disk[l].1 - disk[r].1;
        disk[l] = disk[r];
        disk[r].0 = None;
        if extra != 0 {
            disk.insert(l + 1, (None, extra));
            r += 1;
        }
    }

    let mut checksum = 0;
    let mut idx = 0;
    for (block, cnt) in disk.into_iter() {
        if let Some(file) = block {
            checksum += file * (idx..idx + cnt).sum::<usize>();
        }
        idx += cnt;
    }

    checksum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input));
    }
}
