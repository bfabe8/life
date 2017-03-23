extern crate life;
use life::Grid;

fn main() {
    println!("Hello, world!");
    let grid = Grid::with_data(vec![true, true, true, true, false, false, true, false],
                               (4, 2));
    println!("{}", grid);
}
