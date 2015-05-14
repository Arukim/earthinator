use std::collections::HashMap;

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

#[derive(Hash,Eq, PartialEq)]
struct Point(u32,u32);

pub struct Map{
    height: u32,
    width: u32,
    field: HashMap<Point,Field>,
}

impl Map {
    pub fn new(x: u32, y: u32) -> Map{
        let mut map = Map{
            height: y,
            width: x,
            field: HashMap::new(),
        };

        for i in 0..x {
            for j in 0..y{
                map.field.insert(Point(i,j), Field::new(0));
            }
        }        
        map
    }

    pub fn print(self){
        for i in 0..self.height {
            for j in 0..self.width {
                print!(" {}",self.field.get(&Point(i,j)).unwrap().state);
            }
            println!("");
        }
    }
}
