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

impl Clone for Field{
    fn clone(&self) -> Field {
        Field::new(self.state)
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

        let mut line = Vec::new();
        for _ in 0..y {
            line.push(Field::new(0));
        }

        for _ in 0..x{
            map.lines.push(line.to_vec());
        }
        map
    }

    pub fn print(self){
        for line in self.lines {
            for field in line {
                print!(" {}",field.state);
            }
            println!("");
        }
    }
}
