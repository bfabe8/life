#[derive(Debug, Clone, Default)]
pub struct Cell {
    state: u8,
}

#[derive(Debug)]
pub struct Grid {
    data: Vec<Cell>,
    bounds: (usize, usize)
}

impl Cell {
    pub fn new() -> Cell {
        Default::default()
    }
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let dat = vec![Cell::new(); width * height];
        Grid { data: dat, bounds: (width, height) }
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        let (width, _) = self.bounds;
        &self.data[y * width + x]
    }
}