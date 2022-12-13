use exper_drop::{HasTwoDrops, HasDrop};

fn main() {
    let x = HasTwoDrops {
        name: "HasTwoDrops".to_owned(),
        one: HasDrop {
            name: "one".to_owned(),
        },
        two: HasDrop {
            name: "two".to_owned(),
        },
    };
    println!("x: {:?}", x);
    println!("Running!");
}
