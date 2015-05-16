extern crate earthinator;

use earthinator::field::Map;

fn main(){
    let mut map = Map::new(200,150);
    map.generate();
    //map.print();
    map.export();
}
