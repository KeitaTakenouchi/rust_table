use std::fmt;

#[derive(Debug)]
pub enum CellType {
    Str,
    Int,
    Dbl,
    Date,
}

#[derive(Debug)]
pub struct Table {
    columns: Vec<Column>,
}

impl Table {
    pub fn new(schema: Vec<ColSchema>) -> Table {
        Table {
            columns: schema.into_iter().map(|e| Column::new(e)).collect(),
        }
    }

    pub fn height(&self) -> usize {
        if self.columns.len() == 0 {
            return 0;
        }
        self.columns[0].cells.len()
    }

    pub fn width(&self) -> usize {
        self.columns.len()
    }

    pub fn add_row(&mut self, cells: &mut Vec<Cell>) {
        if self.columns.len() != cells.len() {
            panic!("invalid row added");
        }
        for i in 0..self.width() {
            let c = cells.remove(0);
            self.columns[i].add_cell(c);
        }
    }
}

#[derive(Debug)]
pub struct Column {
    schema: ColSchema,
    cells: Vec<Cell>,
}

impl Column {
    pub fn new(schema: ColSchema) -> Column {
        Column {
            schema: schema,
            cells: Vec::new(),
        }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }
}

#[derive(Debug)]
pub struct ColSchema {
    name: String,
    col_type: CellType,
}

impl ColSchema {
    pub fn new(name: &str, col_type: CellType) -> ColSchema {
        ColSchema {
            name: String::from(name),
            col_type: col_type,
        }
    }
}

#[derive(Debug)]
pub struct Cell {
    value: String,
    cell_type: CellType,
}

impl Cell {
    pub fn new(value: &str, cell_type: CellType) -> Cell {
        let val = match cell_type {
            CellType::Str => String::from(value),
            CellType::Int => {
                let _: i64 = value.parse().unwrap();
                String::from(value)
            }
            CellType::Dbl => {
                let s: f32 = value.parse().unwrap();
                format!("{0:.3}", s)
            }
            CellType::Date => String::from(value),
        };
        Cell {
            value: val,
            cell_type: cell_type,
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sb = String::new();

        // append the header
        sb.push_str("|");
        for c in self.columns.iter() {
            sb.push_str(&format!("{}", c.schema));
            sb.push_str(" | ");
        }
        sb.push_str("\n");

        // append the rows
        for i in 0..self.height() {
            sb.push_str("|");
            for c in self.columns.iter() {
                let cell = c.cells.get(i).unwrap();
                sb.push_str(&format!("{}", cell));
                sb.push_str(" | ");
            }
            sb.push_str("\n");
        }
        write!(f, "{}", sb)
    }
}

impl fmt::Display for ColSchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.col_type)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_01() {
        let c = Cell::new("hello", CellType::Str);
        assert_eq!(c.value, "hello");
    }

    #[test]
    fn test_cell_02() {
        let c = Cell::new("123", CellType::Int);
        assert_eq!(c.value, "123");
    }

    #[should_panic]
    #[test]
    fn test_cell_03() {
        let c = Cell::new("hello", CellType::Int);
        assert_eq!(c.value, "hello");
    }

    #[test]
    fn test_cell_04() {
        let c = Cell::new("123.4", CellType::Dbl);
        assert_eq!(c.value, "123.400");
    }

    #[test]
    fn test_cell_05() {
        let c = Cell::new("123", CellType::Dbl);
        assert_eq!(c.value, "123.000");
    }

    #[test]
    fn test_cell_06() {
        let c = Cell::new("123.0004", CellType::Dbl);
        assert_eq!(c.value, "123.000");
    }

    #[test]
    fn test_cell_07() {
        let c = Cell::new("3", CellType::Dbl);
        assert_eq!(c.value, "3.000");
    }

}
