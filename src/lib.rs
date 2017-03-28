mod life;
pub use life::*;

#[cfg(test)]
mod tests {
    use super::life;

    #[test]
    fn block_test() {
        let mut grid = life::Grid::with_data(vec![false, false, false, false, false, true, true,
                                                  false, false, true, true, false, false, false,
                                                  false, false],
                                             (4, 4));

        let grid2 = grid.clone();
        grid.update();
        assert_eq!(grid, grid2);

    }

    #[test]
    fn blinker_test() {
        let mut grid = life::Grid::with_data(vec![false, false, false, false, false, false,
                                                  false, true, false, false, false, false, true,
                                                  false, false, false, false, true, false, false,
                                                  false, false, false, false, false],
                                             (5, 5));
        let grid2 = life::Grid::with_data(vec![false, false, false, false, false, false, false,
                                               false, false, false, false, true, true, true,
                                               false, false, false, false, false, false, false,
                                               false, false, false, false],
                                          (5, 5));
        let grid3 = grid.clone();
        assert_eq!(grid, grid3);
        assert!(grid != grid2);
        grid.update();
        assert_eq!(grid, grid2);
        assert!(grid != grid3);
        grid.update();
        assert!(grid != grid2);
        assert_eq!(grid, grid3);
    }

}
