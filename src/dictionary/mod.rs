use std::collections::HashMap;

pub struct Dictionary {
    pub letters: Vec<char>,
    pub mapping: DictionaryMapping,
}

impl Dictionary {
    pub fn new(letters: Vec<char>, mapping: DictionaryMapping) -> Self {
        Self {
            letters,
            mapping
        }
    }
}

pub enum DictionaryMapping {
    LetterDefinitions(HashMap<char, char>),
    MultipleLetterDefinitions(HashMap<char, &'static str>)
} 