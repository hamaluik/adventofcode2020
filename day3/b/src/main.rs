struct Map {
    map: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Map {
    fn load(s: &str) -> Map {
        let mut rows: Vec<Vec<bool>> = Vec::default();
        for line in s.split('\n') {
            if line.is_empty() {
                continue;
            }
            let mut row: Vec<bool> = Vec::default();
            for c in line.chars() {
                match c {
                    '#' => row.push(true),
                    '.' => row.push(false),
                    _ => {}
                }
            }
            rows.push(row)
        }

        let height = rows.len();
        assert!(height > 0);
        let width = rows[0].len();
        assert!(width > 0);
        for row in rows.iter() {
            assert!(row.len() == width);
        }
        Map {
            map: rows,
            width,
            height,
        }
    }

    fn at(&self, x: usize, y: usize) -> Option<bool> {
        if y >= self.height {
            return None;
        }
        let x = x % self.width;
        Some(self.map[y][x])
    }

    fn slope_iter(&self, start_x: usize, dx: usize, dy: usize) -> SlopeIterator {
        SlopeIterator {
            map: self,
            x: start_x,
            y: 0,
            dx,
            dy,
        }
    }
}

struct SlopeIterator<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
    dx: usize,
    dy: usize,
}

impl<'a> Iterator for SlopeIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let tree = self.map.at(self.x, self.y);
        self.x += self.dx;
        self.y += self.dy;
        tree
    }
}

fn main() {
    let map = std::fs::read_to_string("../input.txt").unwrap();
    let map = Map::load(&map);

    let a = map.slope_iter(0, 1, 1).filter(|t| *t).count();
    let b = map.slope_iter(0, 3, 1).filter(|t| *t).count();
    let c = map.slope_iter(0, 5, 1).filter(|t| *t).count();
    let d = map.slope_iter(0, 7, 1).filter(|t| *t).count();
    let e = map.slope_iter(0, 1, 2).filter(|t| *t).count();

    let trees = a * b * c * d * e;
    println!("{}", trees);
}
