use crate::models::note::Note;

pub struct RootViewModel {
    note: Option<Note>,
    notes: Vec<Note>,
}

impl RootViewModel {
    pub fn new(note: Option<Note>, notes: Vec<Note>) -> Self {
        Self { note, notes }
    }

    pub fn get_note(&self) -> &Option<Note> {
        &self.note
    }

    pub fn get_notes(&self) -> &Vec<Note> {
        &self.notes
    }
}
