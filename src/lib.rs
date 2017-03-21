#[derive(Debug, Clone, Default, Copy)]
pub struct Cell {
    alive: bool,
}

#[derive(Debug)]
pub struct Grid {
    data: Vec<Cell>,
    bounds: (usize, usize),
}

impl Cell {
    pub fn new() -> Cell {
        Default::default()
    }

    pub fn with_value(val: bool) -> Cell {
        Cell { alive: val}
    }

    pub fn update(&mut self, neighbors: &[Option<Cell>]) {
        let count = neighbors.into_iter().fold(0, |acc, &op| match op {
            Some(c) => if c.is_alive() { acc + 1 } else { acc },
            None => acc,
        });

        self.alive = match (self.alive, count) {
            (true, 2) | (true, 3) | (false, 3) => true,
            _ => false,
        };
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let dat = vec![Cell::new(); width * height];
        Grid {
            data: dat,
            bounds: (width, height),
        }
    }

    pub fn with_data(data: Vec<bool>, bounds: (usize, usize)) -> Grid {
        let (width, height) = bounds;
        assert_eq!(data.len(), width * height);

        let data = data.into_iter().map(Cell::with_value).collect();
        Grid{
            data: data,
            bounds: bounds
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        let (width, _) = self.bounds;
        &self.data[y * width + x]
    }

    pub fn update(&mut self) {

        let (width, height) = self.bounds;
        let curstate = self.data.clone();

        for idx in 0..self.data.len() {
            let mut cell = &mut self.data[idx];

            let neighbors: Vec<Option<Cell>> = get_neighbors(idx_to_pos(idx, width), width, height)
                .into_iter()
                .map(|op| op.map(|pos| curstate[pos_to_idx(pos, width)]))
                .collect();

            cell.update(neighbors.as_slice());
        }
    }
}

fn idx_to_pos(idx: usize, width: usize) -> (usize, usize) {
    let y = idx / width;
    let x = idx - y * width;
    (x, y)
}

fn pos_to_idx(pos: (usize, usize), width: usize) -> usize {
    let (x, y) = pos;
    y * width + x
}

fn get_neighbors(pos: (usize, usize), width: usize, height: usize) -> [Option<(usize, usize)>; 8] {
    let mut arr = [None; 8];

    let (x, y) = pos;


    let bottom = y != 0;
    let top = y != height - 1;
    let left = x != 0;
    let right = x != width - 1;
    let top_left = top && left;
    let top_right = top && right;
    let bottom_left = bottom && left;
    let bottom_right = bottom && right;

    if bottom {
        arr[0] = Some((x, y - 1));
    }
    if top {
        arr[1] = Some((x, y + 1));
    }
    if left {
        arr[2] = Some((x - 1, y));
    }
    if right {
        arr[3] = Some((x + 1, y));
    }
    if top_left {
        arr[4] = Some((x - 1, y + 1));
    }
    if top_right {
        arr[5] = Some((x + 1, y + 1));
    }
    if bottom_left {
        arr[6] = Some((x - 1, y - 1));
    }
    if bottom_right {
        arr[7] = Some((x + 1, y - 1));
    }

    arr
}

