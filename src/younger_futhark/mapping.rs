use std::collections::HashMap;
use crate::dictionary::Dictionary;

pub fn get_runes_list() -> Vec<char> {
    vec!('ᚠ','ᚢ','ᚦ','ᚬ','ᚱ','ᚴ','ᚼ','ᚽ','ᚾ','ᚿ','ᛁ','ᛅ','ᛆ','ᛋ','ᛌ', 'ᛏ','ᛐ','ᛒ','ᛘ', 'ᛚ','ᛦ',':')
}

pub fn get_letters_to_runes_map() -> Dictionary {
    let mut letter_mapping = HashMap::new();

    letter_mapping.insert('a', 'ᛅ');
    letter_mapping.insert('á', 'ᛅ');
    letter_mapping.insert('b', 'ᛒ');
    letter_mapping.insert('c', 'ᛋ');
    letter_mapping.insert('d', 'ᛏ');
    letter_mapping.insert('ð', 'ᚦ');
    letter_mapping.insert('e', 'ᛁ');
    letter_mapping.insert('é', 'ᛁ');
    letter_mapping.insert('f', 'ᚠ');
    letter_mapping.insert('g', 'ᚴ');
    letter_mapping.insert('h', 'ᚼ');
    letter_mapping.insert('i', 'ᛁ');
    letter_mapping.insert('í', 'ᛁ');
    letter_mapping.insert('j', 'ᛁ');
    letter_mapping.insert('k', 'ᚴ');
    letter_mapping.insert('l', 'ᛚ');
    letter_mapping.insert('m', 'ᛘ');
    letter_mapping.insert('n', 'ᚾ');
    letter_mapping.insert('o', 'ᚢ');
    letter_mapping.insert('ó', 'ᚢ');
    letter_mapping.insert('p', 'ᛒ');
    letter_mapping.insert('q', 'ᚴ');
    letter_mapping.insert('r', 'ᚱ');
    letter_mapping.insert('s', 'ᛋ');
    letter_mapping.insert('t', 'ᛏ');
    letter_mapping.insert('þ', 'ᚦ');
    letter_mapping.insert('u', 'ᚢ');
    letter_mapping.insert('ú', 'ᚢ');
    letter_mapping.insert('v', 'ᚢ');
    letter_mapping.insert('w', 'ᚢ');
    letter_mapping.insert('x', 'ᛋ');
    letter_mapping.insert('y', 'ᚢ');
    letter_mapping.insert('ý', 'ᚢ');
    letter_mapping.insert('z', 'ᛋ');
    letter_mapping.insert('å', 'ᚢ');
    letter_mapping.insert('ä', 'ᛅ');
    letter_mapping.insert('æ', 'ᛅ');
    letter_mapping.insert('ö', 'ᚢ');
    letter_mapping.insert('ø', 'ᚢ');
    letter_mapping.insert('ǫ', 'ᚢ');
    letter_mapping.insert(' ', ':');

    Dictionary::LetterDefinitions(letter_mapping)
}

pub fn get_runes_to_letters_map() -> Dictionary {
    let mut rune_mapping = HashMap::new();

    rune_mapping.insert('ᚠ', 'f');
    rune_mapping.insert('ᚢ', 'u');
    rune_mapping.insert('ᚦ', 'þ');
    rune_mapping.insert('ᚬ', 'o');
    rune_mapping.insert('ᚱ', 'r');
    rune_mapping.insert('ᚴ', 'k');
    rune_mapping.insert('ᚼ', 'h');
    rune_mapping.insert('ᚽ', 'h');
    rune_mapping.insert('ᚾ', 'n');
    rune_mapping.insert('ᚿ', 'n');
    rune_mapping.insert('ᛁ', 'i');
    rune_mapping.insert('ᛅ', 'a');
    rune_mapping.insert('ᛆ', 'a');
    rune_mapping.insert('ᛋ', 's');
    rune_mapping.insert('ᛌ', 's');
    rune_mapping.insert('ᛏ', 't');
    rune_mapping.insert('ᛐ', 't');
    rune_mapping.insert('ᛒ', 'b');
    rune_mapping.insert('ᛘ', 'm');
    rune_mapping.insert('ᛚ', 'l');
    rune_mapping.insert('ᛦ', 'R');
    rune_mapping.insert(':', ' ');

    Dictionary::LetterDefinitions(rune_mapping)
}