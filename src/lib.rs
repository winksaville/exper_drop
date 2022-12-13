#[derive(Debug, PartialEq)]
pub struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
    }
}

pub struct HasTwoDrops {
    pub one: HasDrop,
    pub two: HasDrop,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = HasTwoDrops { one: HasDrop, two: HasDrop };
        assert_eq!(x.one, x.two);
    }
}
