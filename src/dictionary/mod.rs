use std::collections::HashMap;

pub enum DictionaryMapping {
    LetterDefinitions(HashMap<char, char>),
    MultipleLetterDefinitions(HashMap<char, &'static str>)
} 