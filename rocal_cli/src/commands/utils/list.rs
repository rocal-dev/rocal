use std::fmt;

pub struct List {
    text: Option<String>,
    items: Vec<List>,
}

#[derive(Clone)]
enum BulletStyle {
    Dot,
    Asterisk,
    Plus,
    Dash,
}

impl BulletStyle {
    pub fn to_symbol(&self) -> char {
        match self {
            Self::Dot => '.',
            Self::Asterisk => '*',
            Self::Plus => '+',
            Self::Dash => '-',
        }
    }

    pub fn len() -> usize {
        4
    }

    pub fn list() -> Vec<Self> {
        vec![Self::Dot, Self::Asterisk, Self::Plus, Self::Dash]
    }
}

impl List {
    pub fn new() -> Self {
        Self {
            text: None,
            items: vec![],
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.text = Some(text.to_string());
    }

    pub fn add_list(&mut self, list: List) {
        self.items.push(list);
    }

    fn print(list: &List, indent: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut i = indent.clone();

        while 0 < i {
            write!(f, " ")?;
            i -= 1;
        }

        if let Some(text) = &list.text {
            let bullet_i: usize = indent % BulletStyle::len();

            let bullet = BulletStyle::list().get(bullet_i).unwrap().clone();

            writeln!(f, "{} {}", bullet.to_symbol(), text)?;
        }

        if list.items.len() < 1 {
            return Ok(());
        }

        for l in list.items.iter() {
            List::print(l, indent + 1, f)?;
        }

        Ok(())
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        List::print(self, 0, f)
    }
}
