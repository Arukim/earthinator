
pub struct Field{
    state: u32,
    right: Option<Box<Field>>,
    down: Option<Box<Field>>,
}


pub struct Map{
    first: Option<Field>,
}
