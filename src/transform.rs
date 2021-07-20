use crate::dictionary::{Dictionary, DictionaryMapping};

pub fn with_dictionary(content: &str, dictionary: &Dictionary) -> String {
    let mut result: String = String::new();
    let characters: Vec<char> = content.chars().collect();

    for character in characters {
        let lower_case: char = character.to_lowercase().next().unwrap();

        if dictionary.letters.contains(&lower_case) {
            match &dictionary.mapping {
                DictionaryMapping::LetterDefinitions(mapping) => {
                    result.push(*mapping.get(&lower_case).unwrap());
                    continue;
                }
                DictionaryMapping::MultipleLetterDefinitions(mapping) => {
                    result.push_str(*mapping.get(&lower_case).unwrap());
                    continue;
                }
            }
        }

        result.push(character);
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
        let result = with_dictionary(content, &runemap);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_upper_and_lower_to_same() {
        let runemap = younger_futhark::mapping::get_letters_to_runes_map();
        let content = "AUK tani Karþi kriSTnA";
        let expected = "ᛅᚢᚴ:ᛏᛅᚾᛁ:ᚴᛅᚱᚦᛁ:ᚴᚱᛁᛋᛏᚾᛅ";
        let result = with_dictionary(content, &runemap);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_char_to_string_mapped_runes() {
        let runemap = futhorc::mapping::get_runes_to_letters_map();
        let content = "ᚪᛒᚳᛞᛖᚠᚷᛠ";
        let expected = "abcdefgea";
        let result = with_dictionary(content, &runemap);

        assert_eq!(result, expected);
    }
}