mod table;

fn main() {
    let mut t1 = table::Table::new("Genboy");
    t1.print_name();

    let t2 = &mut t1;
    t2.print_name();
    t2.update_name("Keita");
    t2.print_name();
    t2.print_name();
}
