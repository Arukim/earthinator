extern crate rand;
extern crate image;

use std::collections::HashMap;
use rand::random;

use std::fs::File;
use std::path::Path;

/// Single map point
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

/// Whole map
pub struct Map{
    height: u32,
    width: u32,
    fields: HashMap<(u32,u32),Field>,
}

/// Params for algo func
pub struct Params{
    /// algo iteration count
    iterations: u32,
    /// chances (0..100) to fill tile as earth
    /// for tiles with 0,1,2,3,4 neighbours
    chances: (u32,u32,u32,u32,u32),
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

    /// Print map into stdout
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

    pub fn export(&self){
        
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let color = if self.fields.get(&(x,y)).unwrap().state > 0
            { [0,125,0] } else { [0,0,125] };

            *pixel = image::Rgb(color);
        }

        let ref mut fout = File::create(&Path::new("out.png")).unwrap();

        image::ImageRgb8(imgbuf).save(fout, image::PNG);

    }
    

    /// Find neighbours for single map point
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

    /// Create neighbours map
    fn calc_neighbours(&self,pre_map: &mut HashMap<(u32,u32),u32>){
        for i in 0..self.height {
            for j in 0..self.width {
                let w = self.calc_weight((j,i));
                pre_map.insert((j,i),w);
            }
        }        
    }

    fn algo(&mut self, params: Params){
        let mut neighbours = HashMap::<(u32,u32),u32>::new();

        for _ in 0..params.iterations {
            self.calc_neighbours(&mut neighbours);

            for (p, field) in self.fields.iter_mut() {
                if field.state > 0 {
                    continue;
                }

                let weight = neighbours.get(&p).unwrap();
                let chance = random::<u32>() % 100;

                match *weight {
                    0 => if chance < params.chances.0 { field.state = 1 },
                    1 => if chance < params.chances.1 { field.state = 1 },
                    2 => if chance < params.chances.2 { field.state = 1 },
                    3 => if chance < params.chances.3 { field.state = 1 },
                    4 => if chance < params.chances.4 { field.state = 1 },
                    _ => {},
                }
            }
        }
    }

    fn seed(&mut self, chance: u32){
        // seed map
        for (_,field) in self.fields.iter_mut() {
            if random::<u32>() % 100 == chance {
                field.state = 1;
            }
        }
    }
    
    //TODO add target fill rate
    /// Generate map
    pub fn generate(&mut self){

        self.seed(1);

        self.algo(Params{iterations: 25, chances: (0,20,40,50,80)});       
       
    }
}
