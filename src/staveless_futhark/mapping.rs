use crate::dictionary::DictionaryMapping;
use std::collections::HashMap;

pub fn get_letters_to_runes_map() -> DictionaryMapping {
    let mut letter_mapping = HashMap::new();

    letter_mapping.insert('a', '⸝');
    letter_mapping.insert('á', '⸝');
    letter_mapping.insert('b', 'ˏ');
    letter_mapping.insert('c', '╵');
    letter_mapping.insert('d', '⸍');
    letter_mapping.insert('ð', 'ו');
    letter_mapping.insert('e', 'ᛁ');
    letter_mapping.insert('é', 'ᛁ');
    letter_mapping.insert('f', 'ᛙ');
    letter_mapping.insert('g', 'ᛍ');
    letter_mapping.insert('h', 'ᚽ');
    letter_mapping.insert('i', 'ᛁ');
    letter_mapping.insert('í', 'ᛁ');
    letter_mapping.insert('j', 'ᛁ');
    letter_mapping.insert('k', 'ᛍ');
    letter_mapping.insert('l', '⸌');
    letter_mapping.insert('m', '⠃');
    letter_mapping.insert('n', '⸜');
    letter_mapping.insert('o', 'ˎ');
    letter_mapping.insert('ó', 'ˎ');
    letter_mapping.insert('p', 'ˏ');
    letter_mapping.insert('q', 'ᛍ');
    letter_mapping.insert('r', '◟');
    letter_mapping.insert('R', '⡄');
    letter_mapping.insert('s', '╵');
    letter_mapping.insert('t', '⸍');
    letter_mapping.insert('þ', 'ו');
    letter_mapping.insert('u', '╮');
    letter_mapping.insert('ú', '╮');
    letter_mapping.insert('v', '╮');
    letter_mapping.insert('w', '╮');
    letter_mapping.insert('x', '╵');
    letter_mapping.insert('y', '╮');
    letter_mapping.insert('ý', '╮');
    letter_mapping.insert('z', '╵');
    letter_mapping.insert('å', 'ˎ');
    letter_mapping.insert('ä', '⸝');
    letter_mapping.insert('æ', '⸝');
    letter_mapping.insert('œ', 'ˎ');
    letter_mapping.insert('ö', 'ˎ');
    letter_mapping.insert('ø', 'ˎ');
    letter_mapping.insert('ǫ', 'ˎ');
    letter_mapping.insert(' ', ':');

    DictionaryMapping::LetterDefinitions(letter_mapping)
}

pub fn get_runes_to_letters_map() -> DictionaryMapping {
    let mut rune_mapping = HashMap::new();

    rune_mapping.insert('ᛙ', 'f');
    rune_mapping.insert('╮', 'u');
    rune_mapping.insert('ו', 'þ');
    rune_mapping.insert('ˎ', 'o');
    rune_mapping.insert('◟', 'r');
    rune_mapping.insert('ᛍ', 'k');
    rune_mapping.insert('ᚽ', 'h');
    rune_mapping.insert('⸜', 'n');
    rune_mapping.insert('ᛁ', 'i');
    rune_mapping.insert('⸝', 'a');
    rune_mapping.insert('╵', 's');
    rune_mapping.insert('⸍', 't');
    rune_mapping.insert('ˏ', 'b');
    rune_mapping.insert('⠃', 'm');
    rune_mapping.insert('⸌', 'l');
    rune_mapping.insert('⡄', 'R');
    rune_mapping.insert(':', ' ');

    DictionaryMapping::LetterDefinitions(rune_mapping)
}
