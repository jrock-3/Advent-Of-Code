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
    // dbg!(&disk);

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
        // dbg!(disk[l], disk[r]);

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
    // dbg!(disk
    //     .iter()
    //     .map(|&(num, cnt)| match num {
    //         Some(file) => file.to_string().repeat(cnt),
    //         None => ".".to_string().repeat(cnt),
    //     })
    //     .collect::<Vec<_>>()
    //     .join(""));

    let mut checksum = 0;
    let mut idx = 0;
    for (block, cnt) in disk.into_iter() {
        match block {
            Some(file) => {
                checksum += file * (idx..idx + cnt).sum::<usize>();
                idx += cnt;
            }
            None => {
                idx += cnt;
            }
        }
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
