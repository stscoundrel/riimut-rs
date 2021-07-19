use std::collections::HashMap;

pub fn with_char_to_char_map(content: &str, dictionary: &HashMap<char, char>) -> String {
    let mut result: String = String::new();
    let characters: Vec<char> = content.chars().collect();

    for character in characters {
        let lower_case: char = character.to_lowercase().next().unwrap();
        if dictionary.contains_key(&lower_case) {
            result.push(*dictionary.get(&lower_case).unwrap());
        } else {
            result.push(character);
        }
    }    

    result
}

// Some runes transform to combination of latin letters, like "ea".
// As char is invalid type for that, use alternative transform for those runesets.
pub fn with_char_to_str_map(content: &str, dictionary: &HashMap<char, &'static str>) -> String {
    let mut result: String = String::new();
    let characters: Vec<char> = content.chars().collect();

    for character in characters {
        let lower_case: char = character.to_lowercase().next().unwrap();
        if dictionary.contains_key(&lower_case) {
            result.push_str(*dictionary.get(&lower_case).unwrap());
        } else {
            result.push(character);
        }
    }    

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::younger_futhark;
    use crate::futhorc;

    #[test]
    fn transforms_letters_to_runes() {
        let runemap = younger_futhark::mapping::get_letters_to_runes_map();
        let content = "auk tani karþi kristna";
        let expected = "ᛅᚢᚴ:ᛏᛅᚾᛁ:ᚴᛅᚱᚦᛁ:ᚴᚱᛁᛋᛏᚾᛅ";
        let result = with_char_to_char_map(content, &runemap);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_upper_and_lower_to_same() {
        let runemap = younger_futhark::mapping::get_letters_to_runes_map();
        let content = "AUK tani Karþi kriSTnA";
        let expected = "ᛅᚢᚴ:ᛏᛅᚾᛁ:ᚴᛅᚱᚦᛁ:ᚴᚱᛁᛋᛏᚾᛅ";
        let result = with_char_to_char_map(content, &runemap);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_char_to_string_mapped_runes() {
        let runemap = futhorc::mapping::get_runes_to_letters_map();
        let content = "ᚪᛒᚳᛞᛖᚠᚷᛠ";
        let expected = "abcdefgea";
        let result = with_char_to_str_map(content, &runemap);

        assert_eq!(result, expected);
    }
}