use std::collections::HashMap;

pub fn get_letters_to_runes_map() -> HashMap<char, char> {
    let mut letter_mapping = HashMap::new();

    letter_mapping.insert('a', 'ᛆ');
    letter_mapping.insert('á', 'ᛆ');
    letter_mapping.insert('b', 'ᛒ');
    letter_mapping.insert('c', 'ᚴ');
    letter_mapping.insert('d', 'ᚦ');
    letter_mapping.insert('ð', 'ᚦ');
    letter_mapping.insert('e', 'ᛁ');
    letter_mapping.insert('é', 'ᛁ');
    letter_mapping.insert('f', 'ᚠ');
    letter_mapping.insert('g', 'ᚵ');
    letter_mapping.insert('h', 'ᚼ');
    letter_mapping.insert('i', 'ᛁ');
    letter_mapping.insert('í', 'ᛁ');
    letter_mapping.insert('j', 'ᛁ');
    letter_mapping.insert('k', 'ᚴ');
    letter_mapping.insert('l', 'ᛚ');
    letter_mapping.insert('m', 'ᛘ');
    letter_mapping.insert('n', 'ᚿ');
    letter_mapping.insert('o', 'ᚮ');
    letter_mapping.insert('ó', 'ᚮ');
    letter_mapping.insert('ǫ', 'ᚰ');
    letter_mapping.insert('p', 'ᛕ');
    letter_mapping.insert('q', 'ᚴ');
    letter_mapping.insert('r', 'ᚱ');
    letter_mapping.insert('s', 'ᛋ');
    letter_mapping.insert('t', 'ᛏ');
    letter_mapping.insert('u', 'ᚢ');
    letter_mapping.insert('ú', 'ᚢ');
    letter_mapping.insert('v', 'ᚠ');
    letter_mapping.insert('w', 'ᚠ');
    letter_mapping.insert('x', 'ᛋ');
    letter_mapping.insert('y', 'ᛦ');
    letter_mapping.insert('ý', 'ᛦ');
    letter_mapping.insert('z', 'ᛋ');
    letter_mapping.insert('å', 'ᚮ');
    letter_mapping.insert('ä', 'ᛅ');
    letter_mapping.insert('æ', 'ᛅ');
    letter_mapping.insert('œ', 'ᚯ');
    letter_mapping.insert('ö', 'ᚯ');
    letter_mapping.insert('ø', 'ᚯ');
    letter_mapping.insert('þ', 'ᚦ');
    letter_mapping.insert(' ', ':');

    letter_mapping
}

pub fn get_runes_to_letters_map() -> HashMap<char, char> {
    let mut rune_mapping = HashMap::new();

    rune_mapping.insert('ᚠ', 'f');
    rune_mapping.insert('ᚢ', 'u');
    rune_mapping.insert('ᚦ', 'þ');
    rune_mapping.insert('ᚮ', 'o');
    rune_mapping.insert('ᚱ', 'r');
    rune_mapping.insert('ᚴ', 'k');
    rune_mapping.insert('ᚼ', 'h');
    rune_mapping.insert('ᚿ', 'n');
    rune_mapping.insert('ᛁ', 'i');
    rune_mapping.insert('ᛆ', 'a');
    rune_mapping.insert('ᛌ', 's');
    rune_mapping.insert('ᛋ', 's');
    rune_mapping.insert('ᛐ', 't');
    rune_mapping.insert('ᛏ', 't');
    rune_mapping.insert('ᛒ', 'b');
    rune_mapping.insert('ᛘ', 'm');
    rune_mapping.insert('ᛚ', 'l');
    rune_mapping.insert('ᛦ', 'y');
    rune_mapping.insert(':', ' ');

    // Sting diacritic secondary sounds.
    rune_mapping.insert('ᚯ', 'ø');
    rune_mapping.insert('ᛅ', 'æ');
    rune_mapping.insert('ᚰ', 'ǫ');
    rune_mapping.insert('ᛕ', 'ᴘ');

    rune_mapping
}