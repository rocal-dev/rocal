use std::fmt;

pub struct Table {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            header: vec![],
            rows: vec![],
        }
    }

    pub fn add_header(&mut self, header: Vec<String>) {
        self.header = header;
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut col_widths: Vec<usize> = self.header.iter().map(|h| h.len()).collect();
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if cell.len() > col_widths[i] {
                    col_widths[i] = cell.len();
                }
            }
        }

        let mut separator = String::new();
        separator.push('+');
        for &width in &col_widths {
            separator.push_str(&format!("{:-<width$}+", "", width = width + 2));
        }

        writeln!(f, "{}", separator)?;

        write!(f, "|")?;
        for (i, cell) in self.header.iter().enumerate() {
            write!(f, " {:<width$} |", cell, width = col_widths[i])?;
        }
        writeln!(f)?;

        writeln!(f, "{}", separator)?;

        for row in &self.rows {
            write!(f, "|")?;
            for (i, cell) in row.iter().enumerate() {
                write!(f, " {:<width$} |", cell, width = col_widths[i])?;
            }
            writeln!(f)?;
        }

        writeln!(f, "{}", separator)
    }
}
