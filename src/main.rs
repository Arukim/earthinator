extern crate earthinator;

use earthinator::field::Map;

fn main(){
    let mut map = Map::new(45,20);
    map.generate();
    map.print();
}
