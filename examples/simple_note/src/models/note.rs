use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Note {
    pub id: i64,
    pub title: Option<String>,
    pub body: Option<String>,
}

impl Note {
    pub fn get_title(&self) -> &Option<String> {
        if let Some(title) = &self.title {
            if title.is_empty() {
                &None
            } else {
                &self.title
            }
        } else {
            &self.title
        }
    }

    pub fn get_body(&self) -> &Option<String> {
        &self.body
    }
}
