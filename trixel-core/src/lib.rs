pub struct Color {}

pub struct Index {}

pub struct Unit {
    pub Index: Index,
    pub Color: Color,
}

pub enum Shape {}

pub struct Layer {
    pub Hidden: bool,
    pub Shape: Shape,
    pub Scale: u16,
    pub Units: Vec<Unit>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
