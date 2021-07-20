use std::collections::HashMap;
use crate::dictionary::{Dictionary, DictionaryMapping};

pub fn get_letters_to_runes_map() -> Dictionary {
    let mut letter_mapping = HashMap::new();

    letter_mapping.insert('a', 'ᚨ');
    letter_mapping.insert('á', 'ᚨ');
    letter_mapping.insert('b', 'ᛒ');
    letter_mapping.insert('c', 'ᚲ');
    letter_mapping.insert('d', 'ᛞ');
    letter_mapping.insert('ð', 'ᚦ');
    letter_mapping.insert('e', 'ᛖ');
    letter_mapping.insert('é', 'ᛖ');
    letter_mapping.insert('f', 'ᚠ');
    letter_mapping.insert('g', 'ᚷ');
    letter_mapping.insert('h', 'ᚻ');
    letter_mapping.insert('i', 'ᛁ');
    letter_mapping.insert('í', 'ᛁ');
    letter_mapping.insert('j', 'ᛃ');
    letter_mapping.insert('k', 'ᚲ');
    letter_mapping.insert('l', 'ᛚ');
    letter_mapping.insert('m', 'ᛗ');
    letter_mapping.insert('n', 'ᚾ');
    letter_mapping.insert('ŋ', 'ᛜ');
    letter_mapping.insert('o', 'ᛟ');
    letter_mapping.insert('ó', 'ᛟ');
    letter_mapping.insert('ǫ', 'ᛟ');
    letter_mapping.insert('p', 'ᛈ');
    letter_mapping.insert('q', 'ᚲ');
    letter_mapping.insert('r', 'ᚱ');
    letter_mapping.insert('s', 'ᛋ');
    letter_mapping.insert('t', 'ᛏ');
    letter_mapping.insert('u', 'ᚢ');
    letter_mapping.insert('ú', 'ᚢ');
    letter_mapping.insert('v', 'ᚹ');
    letter_mapping.insert('w', 'ᚹ');
    letter_mapping.insert('x', 'ᛋ');
    letter_mapping.insert('y', 'ᛁ');
    letter_mapping.insert('ý', 'ᛁ');
    letter_mapping.insert('z', 'ᛉ');
    letter_mapping.insert('å', 'ᛟ');
    letter_mapping.insert('ä', 'ᛇ');
    letter_mapping.insert('æ', 'ᛇ');
    letter_mapping.insert('ö', 'ᚢ');
    letter_mapping.insert('ø', 'ᚢ');
    letter_mapping.insert('þ', 'ᚦ');
    letter_mapping.insert(' ', ':');

    Dictionary::new(
        letter_mapping.keys().copied().collect(),
        DictionaryMapping::LetterDefinitions(letter_mapping)
    )
}

pub fn get_runes_to_letters_map() -> Dictionary {
    let mut rune_mapping = HashMap::new();

    rune_mapping.insert('ᚠ', 'f');
    rune_mapping.insert('ᚢ', 'u');
    rune_mapping.insert('ᚦ', 'þ');
    rune_mapping.insert('ᚨ', 'a');
    rune_mapping.insert('ᚱ', 'r');
    rune_mapping.insert('ᚲ', 'k');
    rune_mapping.insert('ᚷ', 'g');
    rune_mapping.insert('ᚹ', 'w');
    rune_mapping.insert('ᚺ', 'h');
    rune_mapping.insert('ᚻ', 'h');
    rune_mapping.insert('ᚾ', 'n');
    rune_mapping.insert('ᛁ', 'i');
    rune_mapping.insert('ᛃ', 'j');
    rune_mapping.insert('ᛇ', 'ï');
    rune_mapping.insert('ᛈ', 'p');
    rune_mapping.insert('ᛉ', 'z');
    rune_mapping.insert('ᛊ', 's');
    rune_mapping.insert('ᛋ', 's');
    rune_mapping.insert('ᛏ', 't');
    rune_mapping.insert('ᛒ', 'b');
    rune_mapping.insert('ᛖ', 'e');
    rune_mapping.insert('ᛗ', 'm');
    rune_mapping.insert('ᛚ', 'l');
    rune_mapping.insert('ᛜ', 'ŋ');
    rune_mapping.insert('ᛝ', 'ŋ');
    rune_mapping.insert('ᛟ', 'o');
    rune_mapping.insert('ᛞ', 'd');
    rune_mapping.insert(':', ' ');

    Dictionary::new(
        rune_mapping.keys().copied().collect(),
        DictionaryMapping::LetterDefinitions(rune_mapping)
    )
}