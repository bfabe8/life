#[derive(Debug, Clone, Default)]
pub struct Cell {
    state: u8,
}

#[derive(Debug)]
pub struct Grid {
    data: Vec<Vec<Cell>>,
}

impl Cell {
    pub fn new() -> Cell {
        Default::default()
    }
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let dat = vec![vec![Cell::new(); height]; width];
        Grid { data: dat }
    }
}