use exper_drop::{HasTwoDrops, HasDrop};

fn main() {
    println!("main:+");
    let outer_h2d = HasTwoDrops {
        name: "outer_h2d".to_owned(),
        one: HasDrop {
            name: "outer_h2d.one".to_owned(),
        },
        two: HasDrop {
            name: "outer_h2d.two".to_owned(),
        },
    };
    println!("outer_h2d: {outer_h2d:?}");

    println!();
    for i in 1..=3 {
        println!("TOL 1.{i}");

        #[allow(unused_labels)]
        'inner: {
            let inner_h2d= HasTwoDrops {
                name: "inner_h2d".to_owned(),
                one: HasDrop {
                    name: "inner_h2d.one".to_owned(),
                },
                two: HasDrop {
                    name: "inner_h2d.two".to_owned(),
                },
            };
            println!("inner_h2d: {inner_h2d:?}");
            // inner_h2d dropped here
        }

        println!("BOL 1.{i}");
    }

    println!();
    for i in 1..=3 {
        println!("TOL 2.{i}");

        let inner_h2d= HasTwoDrops {
            name: "inner_h2d".to_owned(),
            one: HasDrop {
                name: "inner_h2d.one".to_owned(),
            },
            two: HasDrop {
                name: "inner_h2d.two".to_owned(),
            },
        };
        println!("inner_h2d: {inner_h2d:?}");

        println!("BOL 2.{i}");
        // inner_h2d dropped here
    }

    println!("main:-");
}
