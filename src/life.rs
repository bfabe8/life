use std::fmt;

pub type Vec2 = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    alive: bool,
}

#[derive(Debug)]
pub struct Grid {
    data: Vec<Cell>,
    bounds: Vec2,
}

impl Cell {
    pub fn new() -> Cell {
        Cell { alive: false }
    }

    pub fn with_value(val: bool) -> Cell {
        Cell { alive: val }
    }

    fn update(&mut self, neighbors: &[Cell]) {
        let count = neighbors.into_iter().filter(|n| n.is_alive()).count();

        self.alive = match (self.alive, count) {
            (true, 2) | (true, 3) | (false, 3) => true,
            _ => false,
        };
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }
}

impl From<bool> for Cell {
    fn from(b: bool) -> Cell {
        Cell::with_value(b)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if self.is_alive() { "+" } else { " " })
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::new()
    }
}

impl Grid {
    pub fn new((width, height): Vec2) -> Grid {
        let data = vec![Cell::new(); width * height];
        Grid {
            data: data,
            bounds: (width, height),
        }
    }

    pub fn with_data(data: Vec<bool>, bounds: Vec2) -> Grid {
        let (width, height) = bounds;
        assert_eq!(data.len(), width * height);

        let data = data.iter().map(|&b| Cell::from(b)).collect();
        Grid {
            data: data,
            bounds: bounds,
        }
    }

    pub fn get(&self, pos: Vec2) -> &Cell {
        let (width, _) = self.bounds;
        &self.data[pos_to_idx(pos, width)]
    }

    pub fn update(&mut self) {

        let (width, _) = self.bounds;
        let curstate = self.data.clone();

        for idx in 0..self.data.len() {
            let mut cell = &mut self.data[idx];

            let neighbors: Vec<Cell> = get_neighbors(idx_to_pos(idx, width), self.bounds)
                .iter()
                .filter_map(|op| op.map(|pos| curstate[pos_to_idx(pos, width)]))
                .collect();

            cell.update(&neighbors);
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.bounds.1 {
            for x in 0..self.bounds.0 {
                write!(f, "{}", self.get((x, y)))?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}


fn idx_to_pos(idx: usize, width: usize) -> Vec2 {
    let y = idx / width;
    let x = idx - y * width;
    (x, y)
}

fn pos_to_idx((x, y): Vec2, width: usize) -> usize {
    y * width + x
}

fn get_neighbors((x, y): Vec2, (width, height): Vec2) -> Vec<Option<Vec2>> {
    let mut arr = Vec::with_capacity(8);

    let bottom = y != 0;
    let top = y != height - 1;
    let left = x != 0;
    let right = x != width - 1;
    let top_left = top && left;
    let top_right = top && right;
    let bottom_left = bottom && left;
    let bottom_right = bottom && right;

    if bottom {
        arr.push(Some((x, y - 1)));
    }
    if top {
        arr.push(Some((x, y + 1)));
    }
    if left {
        arr.push(Some((x - 1, y)));
    }
    if right {
        arr.push(Some((x + 1, y)));
    }
    if top_left {
        arr.push(Some((x - 1, y + 1)));
    }
    if top_right {
        arr.push(Some((x + 1, y + 1)));
    }
    if bottom_left {
        arr.push(Some((x - 1, y - 1)));
    }
    if bottom_right {
        arr.push(Some((x + 1, y - 1)));
    }

    arr
}

