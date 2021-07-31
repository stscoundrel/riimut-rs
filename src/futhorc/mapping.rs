use std::collections::HashMap;
use crate::dictionary::DictionaryMapping;

pub fn get_letters_to_runes_map() -> DictionaryMapping {
    let mut letter_mapping = HashMap::new();
    
    letter_mapping.insert('a', 'ᚪ');
    letter_mapping.insert('á', 'ᚪ');
    letter_mapping.insert('b', 'ᛒ');
    letter_mapping.insert('c', 'ᚳ');
    letter_mapping.insert('d', 'ᛞ');
    letter_mapping.insert('ð', 'ᛞ');
    letter_mapping.insert('e', 'ᛖ');
    letter_mapping.insert('é', 'ᛖ');
    letter_mapping.insert('f', 'ᚠ');
    letter_mapping.insert('g', 'ᚷ');
    letter_mapping.insert('h', 'ᚻ');
    letter_mapping.insert('i', 'ᛁ');
    letter_mapping.insert('í', 'ᛇ');
    letter_mapping.insert('ï', 'ᛇ');
    letter_mapping.insert('ʒ', 'ᛇ');
    letter_mapping.insert('j', 'ᛡ');
    letter_mapping.insert('k', 'ᚳ');
    letter_mapping.insert('l', 'ᛚ');
    letter_mapping.insert('m', 'ᛗ');
    letter_mapping.insert('n', 'ᚾ');
    letter_mapping.insert('ŋ', 'ᛝ');
    letter_mapping.insert('o', 'ᚩ');
    letter_mapping.insert('ó', 'ᚩ');
    letter_mapping.insert('ǫ', 'ᚩ');
    letter_mapping.insert('p', 'ᛈ');
    letter_mapping.insert('q', 'ᚳ');
    letter_mapping.insert('r', 'ᚱ');
    letter_mapping.insert('s', 'ᛋ');
    letter_mapping.insert('t', 'ᛏ');
    letter_mapping.insert('u', 'ᚢ');
    letter_mapping.insert('ú', 'ᚢ');
    letter_mapping.insert('v', 'ᚹ');
    letter_mapping.insert('w', 'ᚹ');
    letter_mapping.insert('x', 'ᛉ');
    letter_mapping.insert('y', 'ᚣ');
    letter_mapping.insert('ý', 'ᚣ');
    letter_mapping.insert('z', 'ᛉ');
    letter_mapping.insert('å', 'ᚩ');
    letter_mapping.insert('ä', 'ᚫ');
    letter_mapping.insert('æ', 'ᚫ');
    letter_mapping.insert('œ', 'ᛟ');
    letter_mapping.insert('ö', 'ᛟ');
    letter_mapping.insert('ø', 'ᛟ');
    letter_mapping.insert('þ', 'ᚦ');
    letter_mapping.insert(' ', ':');

    
    DictionaryMapping::LetterDefinitions(letter_mapping)
}

pub fn get_runes_to_letters_map() -> DictionaryMapping {
    let mut rune_mapping = HashMap::new();

    rune_mapping.insert('ᚠ', "f");
    rune_mapping.insert('ᚢ', "u");
    rune_mapping.insert('ᚦ', "þ");
    rune_mapping.insert('ᚩ', "o");
    rune_mapping.insert('ᚱ', "r");
    rune_mapping.insert('ᚳ', "c");
    rune_mapping.insert('ᚷ', "g");
    rune_mapping.insert('ᚹ', "w");
    rune_mapping.insert('ᚻ', "h");
    rune_mapping.insert('ᚾ', "n");
    rune_mapping.insert('ᛁ', "i");
    rune_mapping.insert('ᛡ', "j");
    rune_mapping.insert('ᛄ', "j");
    rune_mapping.insert('ᛇ', "ï");
    rune_mapping.insert('ᛈ', "p");
    rune_mapping.insert('ᛉ', "x");
    rune_mapping.insert('ᛋ', "s");
    rune_mapping.insert('ᚴ', "s");
    rune_mapping.insert('ᛏ', "t");
    rune_mapping.insert('ᛒ', "b");
    rune_mapping.insert('ᛖ', "e");
    rune_mapping.insert('ᛗ', "m");
    rune_mapping.insert('ᛚ', "l");
    rune_mapping.insert('ᛝ', "ŋ");
    rune_mapping.insert('ᛟ', "œ");
    rune_mapping.insert('ᛞ', "d");
    rune_mapping.insert('ᚪ', "a");
    rune_mapping.insert('ᚫ', "æ");
    rune_mapping.insert('ᚣ', "y");
    rune_mapping.insert(':', " ");
    rune_mapping.insert('ᛠ', "ea");

    DictionaryMapping::MultipleLetterDefinitions(rune_mapping)
}