extern crate rand;
use std::collections::HashMap;
use rand::random;

pub struct Field{
    state: u8,
}

impl Field{
    pub fn new(state: u8) -> Field{
        Field{
            state: state,
        }
    }

    pub fn set(&mut self, newstate: u8){
        self.state = newstate;
    }
}

#[derive(Hash,Eq, PartialEq)]
struct Point(u32,u32);

pub struct Map{
    height: u32,
    width: u32,
    fields: HashMap<Point,Field>,
}

impl Map {
    pub fn new(x: u32, y: u32) -> Map{
        let mut map = Map{
            height: y,
            width: x,
            fields: HashMap::new(),
        };

        for i in 0..x {
            for j in 0..y{
                map.fields.insert(Point(i,j),Field::new(0));
            }
        }        
        map
    }

    pub fn print(&self){
        for i in 0..self.height {
            for j in 0..self.width {
                print!(" {}",self.fields.get(&Point(i,j)).unwrap().state);
            }
            println!("");
        }
    }

    pub fn generate(&mut self){
        for (_,field) in self.fields.iter_mut() {
            if random::<u32>() % 2 == 1 {
                field.state = 1;
            }
        }
    }
}
