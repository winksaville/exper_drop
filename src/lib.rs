#[derive(Debug)]
pub struct HasDrop {
    pub name: String,
}

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

#[derive(Debug)]
pub struct HasTwoDrops {
    pub name: String,
    pub one: HasDrop,
    pub two: HasDrop,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = HasTwoDrops {
            name: "HasTwoDrops".to_owned(),
            one: HasDrop {
                name: "one".to_owned(),
            },
            two: HasDrop {
                name: "two".to_owned(),
            },
        };
        assert_eq!(x.name, "HasTwoDrops");
        assert_eq!(x.one.name, "one");
        assert_eq!(x.two.name, "two");
    }
}
