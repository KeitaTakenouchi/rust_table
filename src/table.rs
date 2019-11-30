#[derive(Debug)]
pub struct Table<'a> {
    name: &'a str,
}

impl<'a> Table<'a> {
    pub fn new(name: &str) -> Table {
        return Table { name: name };
    }

    pub fn print_name(&self) {
        println!("hello {}", self.name);
    }

    pub fn update_name(&mut self, name: &'a str) {
        self.name = name;
    }
}
