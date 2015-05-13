
pub struct Field{
    state: u32,
    left: Option<Box<Field>>,
    pub right: Option<Box<Field>>,
    up: Option<Box<Field>>,
    down: Option<Box<Field>>,
}

impl Field{
    fn new(state: u32) -> Field {
        Field {
            state: state,
            left: None,
            right: None,
            up: None,
            down: None,
        }
    }
}


pub struct Map{
    first: Option<Field>,
}

impl Map{
    fn new(x: u32, y: u32) -> Map {
        let mut map = Map{first: Some(Field::new(0))};
        for i in 0..x {
            let mut field = map.first.unwrap();
            for j in 0..y {
                field.right = Some(Box::new(Field::new(0)));
                field = field.right.unwrap();
            }
        }
    }
}
