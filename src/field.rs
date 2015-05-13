pub struct Field{
    state: u32,
}

impl Field{
    pub fn new(state: u32) -> Field{
        Field{
            state: state,
        }
    }
}

pub struct Map{
    lines: Vec<Vec<Field>>,
}

impl Map {
    pub fn new(x: u32, y: u32) -> Map{
        let mut map = Map{
            lines: Vec::new(),
        };
        for i in 0..x {
            let mut line = Vec::new();
            for j in 0..y {
                line.push(Field::new(0));
            }
            map.lines.push(line);
        }
        map
    }
}
