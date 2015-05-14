extern crate earthinator;

use earthinator::field::Map;

fn main(){
    let mut map = Map::new(10,10);
    map.generate();
    map.print();
}
