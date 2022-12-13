use exper_drop::{HasTwoDrops, HasDrop};

fn main() {
    let _x = HasTwoDrops { one: HasDrop, two: HasDrop };
    println!("Running!");
}
