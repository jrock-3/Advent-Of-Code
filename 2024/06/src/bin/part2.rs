use std::collections::BTreeSet;

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

    fn step(&mut self) -> Option<()> {
        loop {
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
                return Some(());
            }
            self.pos = next;
        }
    }

    fn did_loop(&mut self, turn_pts: &BTreeSet<(Pos, Dir)>) -> bool {
        for _ in 0..turn_pts.len() * 2 {
            if self.step().is_none() {
                return false;
            }
        }
        while self.step().is_some() {
            if turn_pts.contains(&(self.pos, self.dir)) {
                return true;
            }
        }
        false
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

    let mut turn_pts = BTreeSet::new();
    let mut patrols = Map::new(map.clone(), pos.clone(), obstructions.clone());
    // TODO: Decrease number of instances by only using visited nodes
    // let mut visited = BTreeSet::new();
    while patrols.step().is_some() {
        turn_pts.insert((patrols.pos, patrols.dir));
    }
    // dbg!(&turn_pts);

    (0..map.len())
        .flat_map(|i| (0..map[i].len()).map(move |j| (i, j)))
        .filter(|(i, j)| !(obstructions.contains(&(*i, *j)) || pos == (*i, *j)))
        .filter(|(i, j)| {
            let mut obstacles = obstructions.clone();
            obstacles.insert((*i, *j));
            Map::new(map.clone(), pos.clone(), obstacles).did_loop(&turn_pts)
        })
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
