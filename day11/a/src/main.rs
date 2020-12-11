#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for State {
    fn from(c: char) -> State {
        match c {
            '.' => State::Floor,
            'L' => State::Empty,
            '#' => State::Occupied,
            _ => panic!("bad map: {}", c),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Map(Vec<Vec<State>>);

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Map {
    fn size(&self) -> (usize, usize) {
        (self.0[0].len(), self.0.len())
    }

    fn at(&self, loc: (usize, usize), dir: (isize, isize)) -> State {
        let x: isize = loc.0 as isize + dir.0;
        let y: isize = loc.1 as isize + dir.1;
        let (w, h) = self.size();

        if x < 0 || x as usize >= w || y < 0 || y as usize >= h {
            State::Floor
        } else {
            self.0[y as usize][x as usize]
        }
    }

    fn count_neighours(&self, loc: (usize, usize)) -> usize {
        DIRECTIONS
            .iter()
            .filter(|&d| self.at(loc, *d) == State::Occupied)
            .count()
    }

    fn tick(&self) -> Map {
        let mut new = self.clone();
        let (w, h) = self.size();
        for y in 0..h {
            for x in 0..w {
                new.0[y][x] = match self.0[y][x] {
                    State::Floor => State::Floor,
                    State::Empty => match self.count_neighours((x, y)) {
                        0 => State::Occupied,
                        _ => State::Empty,
                    },
                    State::Occupied => match self.count_neighours((x, y)) {
                        0 | 1 | 2 | 3 => State::Occupied,
                        _ => State::Empty,
                    },
                }
            }
        }
        new
    }
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let map: Vec<Vec<State>> = input
        .split("\n")
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(l.chars().map(State::from).collect())
            }
        })
        .collect();
    let mut old_map = Map(map);

    loop {
        let map = old_map.tick();
        if map == old_map {
            println!(
                "{}",
                map.0
                    .into_iter()
                    .flatten()
                    .filter(|&s| s == State::Occupied)
                    .count()
            );
            return;
        }
        old_map = map;
    }
}
