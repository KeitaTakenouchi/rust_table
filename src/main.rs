mod table;

use table::CellType::*;
use table::*;

fn main() {
    let mut table = Table::new(vec![
        ColSchema::new("c1", Str),
        ColSchema::new("c2", Int),
        ColSchema::new("c3", Str),
    ]);
    table.add_row(&mut vec![
        Cell::new("A", Str),
        Cell::new("12", Str),
        Cell::new("XX", Str),
    ]);
    table.add_row(&mut vec![
        Cell::new("B", Str),
        Cell::new("10", Str),
        Cell::new("XX", Str),
    ]);
    table.add_row(&mut vec![
        Cell::new("A", Str),
        Cell::new("21", Str),
        Cell::new("YY", Str),
    ]);

    println!("{}", table);
}
