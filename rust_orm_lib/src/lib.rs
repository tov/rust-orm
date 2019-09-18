extern crate syn;

use::syn::{Type};

pub struct Column {
    name: String,
    typ: Type
}

pub struct Table {
    name: String,
    columns: Vec<Column>
}

pub trait Relation {
    fn get_table(&self) -> Table;
}
