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

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        let (width, _) = self.bounds;
        &self.data[y * width + x]
    }

    pub fn update(&mut self) {

        let (width, height) = self.bounds;
        let curstate = self.data.clone();

        for idx in 0..self.data.len() {
            let mut cell = &mut self.data[idx];

            let neighbors: Vec<Option<Cell>> = get_neighbors(idx, width, height)
                .into_iter()
                .map(|op| op.map(|idx| curstate[idx]))
                .collect();

            cell.update(neighbors.as_slice());
        }
    }
}

fn get_neighbors(idx: usize, width: usize, height: usize) -> [Option<usize>; 8] {
    let mut arr = [None; 8];

    let bottom = || idx >= (width * (height - 1));
    let top = || idx < width;
    let left = || idx % width == 0;
    let right = || idx % (width - 1) == 0;
    let top_left = || top() || left();
    let top_right = || top() || right();
    let bottom_left = || bottom() || left();
    let bottom_right = || bottom() || right();

    if !bottom() {
        arr[0] = Some(idx - width);
    }
    if !top() {
        arr[1] = Some(idx + width);
    }
    if !left() {
        arr[2] = Some(idx - 1);
    }
    if !right() {
        arr[3] = Some(idx + 1);
    }
    if !top_left() {
        arr[4] = Some(idx + width - 1);
    }
    if !top_right() {
        arr[5] = Some(idx + width + 1);
    }
    if !bottom_left() {
        arr[6] = Some(idx - width - 1);
    }
    if !bottom_right() {
        arr[7] = Some(idx - width + 1);
    }

    arr
}


