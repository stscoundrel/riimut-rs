use std::collections::HashMap;

pub enum Dictionary {
    LetterDefinitions(HashMap<char, char>),
    MultipleLetterDefinitions(HashMap<char, &'static str>)
} 