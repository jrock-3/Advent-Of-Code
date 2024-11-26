use std::{char, collections::VecDeque};

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

/*
Example:
spring string: ???.###
nums: 1,1,3

---

Approach
1. pop state
2. base case - if end of string:
    a. if len(num)==0: increment count
    b. else: continue
3. if 1st num = 0, cases:
    a. '.': do nothing
    b. '#': invalid, continue
    c. '?': set as '.'
4. check curr char, cases:
    a. '.': do nothing
    b. '#': decrement 1st num
    c. '?': binary choice, enqueue 2 choices, continue
5. enqueue choices

State
- curr index in spring string
- nums

Problems
- duplicates

---

Approach: stars and bars (do for part 2)

---

#.?.### 0,1,3
#.#.### 0,0,0

.??.###
*/

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct State {
    springs: String,
    spring_idx: usize,
    nums: Vec<u32>,
    nums_idx: usize,
    changed: bool,
}

impl State {
    fn new(springs: String, nums: Vec<u32>) -> Self {
        State {
            springs,
            spring_idx: 0,
            nums,
            nums_idx: 0,
            changed: false,
        }
    }

    fn process(mut self) -> Option<Self> {
        // process everything until the next '?'
        loop {
            if !self.has_next() {
                if self.is_valid_string() {
                    dbg!(&self.springs);
                    return Some(self);
                }
                return None;
            }
            match self.get_curr_spring() {
                '#' => {
                    if self.nums[self.nums_idx] == 0 {
                        return None;
                    }
                    self.nums[self.nums_idx] -= 1;
                    self.changed = true;
                }
                '.' => {
                    if self.changed {
                        if self.get_curr_num() != 0 {
                            return None;
                        }
                        self.nums_idx += 1;
                    }
                    self.changed = false;
                }
                '?' => return Some(self),
                _ => unreachable!(),
            }
            self.spring_idx += 1;
            // dbg!(&self);
        }
    }

    fn is_valid_string(&self) -> bool {
        (self.spring_idx == self.springs.len()
            || self
                .springs
                .chars()
                .nth(self.spring_idx)
                .iter()
                .all(|c| *c == '.' || *c == '?'))
            && self.nums[self.nums.len() - 1] == 0
    }

    fn has_next(&self) -> bool {
        self.spring_idx < self.springs.len() && self.nums_idx < self.nums.len()
    }

    fn get_curr_spring(&self) -> char {
        self.springs.chars().nth(self.spring_idx).unwrap()
    }

    fn set_curr_spring(&mut self, spring: char) {
        self.springs
            .replace_range(self.spring_idx..self.spring_idx + 1, &spring.to_string());
    }

    fn get_curr_num(&self) -> u32 {
        self.nums[self.nums_idx]
    }
}

// ???.### 1,1,3
fn count_arrangements(spring_str: String, nums: Vec<u32>) -> u32 {
    dbg!(&spring_str, &nums);
    let mut count = 0;

    let mut states = VecDeque::new();
    let state = State::new(spring_str, nums);

    states.push_back(state.process().unwrap());

    while states.len() > 0 {
        let mut state = states.pop_back().unwrap();
        // dbg!(&state);

        if !state.has_next() {
            if state.is_valid_string() {
                count += 1;
            }
            continue;
        }

        // process '?'
        let mut state1 = state.clone();
        state1.set_curr_spring('#');
        // dbg!(&state1);
        let state1 = state1.process();
        // dbg!(&state1);
        if let Some(state1) = state1 {
            states.push_back(state1);
        }

        state.set_curr_spring('.');
        // dbg!(&state);
        let state = state.process();
        // dbg!(&state);
        if let Some(state) = state {
            states.push_back(state);
        }
    }

    count
}

fn process(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let (spring_str, num_str) = line.split_once(" ").unwrap();
        let nums = num_str
            .split(",")
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        total += dbg!(count_arrangements(spring_str.to_string(), nums));
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1_line1() {
        let input = include_str!("../in/sample1.txt").lines().nth(0).unwrap();
        assert_eq!("1", process(input));
    }

    #[test]
    fn test_in1_line2() {
        let input = include_str!("../in/sample1.txt").lines().nth(1).unwrap();
        assert_eq!("4", process(input));
    }

    #[test]
    fn test_in1_line3() {
        let input = include_str!("../in/sample1.txt").lines().nth(2).unwrap();
        assert_eq!("1", process(input));
    }

    #[test]
    fn test_in1_line4() {
        let input = include_str!("../in/sample1.txt").lines().nth(3).unwrap();
        assert_eq!("1", process(input));
    }

    #[test]
    fn test_in1_line5() {
        let input = include_str!("../in/sample1.txt").lines().nth(4).unwrap();
        assert_eq!("4", process(input));
    }

    #[test]
    fn test_in1_line6() {
        let input = include_str!("../in/sample1.txt").lines().nth(5).unwrap();
        assert_eq!("10", process(input));
    }

    #[test]
    fn test_in1_line4_state() {
        let state = State {
            springs: "####.#...#...".to_string(),
            spring_idx: 3,
            nums: vec![1, 1, 1],
            nums_idx: 0,
            changed: true,
        };
        dbg!(&state);

        assert!(matches!(dbg!(&state.process()), Some(_)));
    }

    #[test]
    fn test_in1() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("21", process(input));
    }

    // ?????????
    // ###.##.#.
    // ###.##..#
    // ###..##.#
    // .###.##.#
    #[test]
    fn test_in2() {
        let input = "????????? 3,2,1";
        assert_eq!("4", process(input));
    }
}
