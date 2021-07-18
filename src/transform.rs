use std::collections::HashMap;

pub fn transform(content: &str, dictionary: &HashMap<char, char>) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::younger_futhark;

    #[test]
    fn transforms_letters_to_runes() {
        let runemap = younger_futhark::mapping::get_letters_to_runes_map();
        let content = "auk tani karþi kristna";
        let expected = "ᛅᚢᚴ:ᛏᛅᚾᛁ:ᚴᛅᚱᚦᛁ:ᚴᚱᛁᛋᛏᚾᛅ";
        let result = transform(content, &runemap);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_upper_and_lower_to_same() {
        let runemap = younger_futhark::mapping::get_letters_to_runes_map();
        let content = "AUK tani Karþi kriSTnA";
        let expected = "ᛅᚢᚴ:ᛏᛅᚾᛁ:ᚴᛅᚱᚦᛁ:ᚴᚱᛁᛋᛏᚾᛅ";
        let result = transform(content, &runemap);

        assert_eq!(result, expected);
    }
}