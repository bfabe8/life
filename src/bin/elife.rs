extern crate life;
use life::Grid;

fn main() {
    println!("Hello, world!");
    let grid = Grid::new(10, 10);
    println!("{:?}", grid);
}
