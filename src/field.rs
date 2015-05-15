extern crate rand;
use std::collections::HashMap;
use rand::random;

pub struct Field{
    state: u32,
}

impl Field{
    pub fn new(state: u32) -> Field{
        Field{
            state: state,
        }
    }

    pub fn set(&mut self, newstate: u32){
        self.state = newstate;
    }
}

pub struct Map{
    height: u32,
    width: u32,
    fields: HashMap<(u32,u32),Field>,
}

impl Map {
    pub fn new(x: u32, y: u32) -> Map{
        let mut map = Map{
            height: y,
            width: x,
            fields: HashMap::new(),
        };

        for i in 0..y {
            for j in 0..x{
                map.fields.insert((j,i),Field::new(0));
            }
        }        
        map
    }

    pub fn print(&self){
        for i in 0..self.height {
            for j in 0..self.width {
                let sym = if self.fields.get(&(j,i)).unwrap().state > 0
                    { '*' } else { ' ' };
                print!(" {}", sym);
            }
            println!("");
        }
    }


    fn calc_weight(&self, p: (u32,u32)) -> u32{
        let mut res = self.fields.get(&p).unwrap().state;
        
        // right cell
        if p.0 < self.width -1 {
            res += self.fields.get(&(p.0 + 1 , p.1)).unwrap().state;
        }
        // left cell
        if p.0 > 0 {
            res += self.fields.get(&(p.0 - 1, p.1)).unwrap().state;
        }
        // upper cell
        if p.1 > 0 {
            res += self.fields.get(&(p.0, p.1 - 1)).unwrap().state;
        }
        // down cell
        if p.1 < self.height -1 {
            res += self.fields.get(&(p.0, p.1 + 1)).unwrap().state;
        }
        res
    }
    
    fn calc_neighbours(&self,pre_map :&mut HashMap<(u32,u32),u32>){
        for i in 0..self.height {
            for j in 0..self.width {
                let w = self.calc_weight((j,i));
                pre_map.insert((j,i),w);
            }
        }        
    }
    
    pub fn generate(&mut self){
        // seed map
        for (_,field) in self.fields.iter_mut() {
            if random::<u32>() % 32 == 1 {
                field.state = 1;
            }
        }
        
        let mut neighbours = HashMap::<(u32,u32),u32>::new();

        for _ in 0..10 {
            self.calc_neighbours(&mut neighbours);

            for (p, field) in self.fields.iter_mut() {
                if field.state > 0 {
                    continue;
                }

                let weight = neighbours.get(&p).unwrap();
                let chance = random::<u32>() % 100;

                match *weight {
                    0 => if chance < 3 { field.state = 1 },
                    1 => if chance < 10 { field.state = 1 },
                    2 => if chance < 20 { field.state = 1 },
                    3 => if chance < 30 { field.state = 1 },
                    4 => if chance < 80 { field.state = 1 },
                    _ => {},
                }
            }
        }            
    }
}
