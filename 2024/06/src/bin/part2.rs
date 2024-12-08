use std::collections::BTreeSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(process(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn rotate(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

type Pos = (usize, usize);

#[derive(PartialEq)]
enum Step {
    Dir,
    Pos,
}

#[derive(Clone)]
struct Map {
    map: Vec<Vec<char>>,
    pos: Pos,
    dir: Dir,
    obstacles: BTreeSet<Pos>,
}

impl Map {
    fn new(map: Vec<Vec<char>>, pos: Pos, obstacles: BTreeSet<Pos>) -> Self {
        Self {
            map,
            pos,
            dir: Dir::Up,
            obstacles,
        }
    }

    fn get_pos(&self) -> Option<Pos> {
        Some(match self.dir {
            Dir::Up => (self.pos.0.checked_sub(1)?, self.pos.1),
            Dir::Down => (self.pos.0.checked_add(1)?, self.pos.1),
            Dir::Left => (self.pos.0, self.pos.1.checked_sub(1)?),
            Dir::Right => (self.pos.0, self.pos.1.checked_add(1)?),
        })
    }

    fn pos_in_bounds(&self, pos: &Pos) -> bool {
        (0..self.map.len()).contains(&pos.0) && (0..self.map[0].len()).contains(&pos.1)
    }

    fn step(&mut self) -> Option<Step> {
        // dbg!(&self.pos);
        let next = self.get_pos();
        // dbg!(&next);
        if next.is_none() {
            return None;
        }
        let next = next.unwrap();
        if !self.pos_in_bounds(&next) {
            return None;
        }
        if self.obstacles.contains(&next) {
            self.dir = self.dir.rotate();
            return Some(Step::Dir);
        }
        self.pos = next;
        return Some(Step::Pos);
    }

    fn step_turn(&mut self) -> Option<()> {
        loop {
            if self.step()? == Step::Dir {
                return Some(());
            }
        }
    }

    fn did_loop(mut self) -> bool {
        let mut fast = self.clone();
        loop {
            let slow_step = self.step_turn();
            let fast_step = fast.step_turn();
            if fast_step.is_none() {
                return false;
            }
            let fast_step = fast.step_turn();

            if slow_step.is_none() || fast_step.is_none() {
                return false;
            }

            if (self.pos, self.dir) == (fast.pos, fast.dir) {
                return true;
            }
        }
    }
}

// NOTE: Idea
// Use turn points as points on a graph (point, direction)
// if an obstacle ever matches one of those points, it is a loop
// try placing an obstacle in every part of the grid (minus start + preexisting obstacles)
// if a guard leaves the area, there is no loop
fn process(input: &str) -> String {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // dbg!(&map);

    let obstructions = (0..map.len())
        .flat_map(|i| {
            (0..map[i].len())
                .map(move |j| (i, j))
                .filter(|(i, j)| map[*i][*j] == '#')
        })
        .collect::<BTreeSet<_>>();
    // dbg!(&obstructions);

    let pos = (0..map.len())
        .flat_map(|i| (0..map[i].len()).map(move |j| (i, j)))
        .filter(|(i, j)| map[*i][*j] == '^')
        .collect::<Vec<_>>()[0];
    // dbg!(&pos);

    let mut visited = BTreeSet::new();
    visited.insert(pos);
    let mut patrols = Map::new(map.clone(), pos.clone(), obstructions.clone());
    loop {
        match patrols.step() {
            Some(Step::Dir) => (),
            Some(Step::Pos) => {
                visited.insert(patrols.pos);
            }
            None => break,
        }
    }
    // dbg!(&turn_pts);

    visited
        .par_iter()
        // .inspect(|x| {
        //     dbg!(x);
        // })
        .map(|(i, j)| {
            let mut obstacles = obstructions.clone();
            obstacles.insert((*i, *j));
            Map::new(map.clone(), pos.clone(), obstacles).did_loop()
        })
        .filter(|x| *x)
        // .inspect(|x| {
        //     dbg!(x);
        // })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        assert_eq!("6", process(input));
    }
}
