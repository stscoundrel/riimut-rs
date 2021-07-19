pub mod mapping;
use crate::transform;

pub fn letters_to_runes(content: &str) -> String {
    let letter_map = mapping::get_letters_to_runes_map();
    
    transform::with_dictionary(&content, &letter_map)
}

pub fn runes_to_letters(content: &str) -> String {
    let rune_map = mapping::get_runes_to_letters_map();
    
    transform::with_dictionary(&content, &rune_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforms_letters_to_younger_futhark() {
        let content = "aábcdðeéfghiíjklmnoópqrstþuúvwxyýzåäæöøǫþ";
        let expected = "ᛅᛅᛒᛋᛏᚦᛁᛁᚠᚴᚼᛁᛁᛁᚴᛚᛘᚾᚢᚢᛒᚴᚱᛋᛏᚦᚢᚢᚢᚢᛋᚢᚢᛋᚢᛅᛅᚢᚢᚢᚦ";
        let result = letters_to_runes(content);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_runes_to_letters() {
        let content = "ᚠᚢᚦᚬᚱᚴᚼᚽᚾᚿᛁᛅᛆᛋᛌᛏᛐᛒᛘᛚᛦ:";
        let expected = "fuþorkhhnniaassttbmlR ";
        let result = runes_to_letters(content);

        assert_eq!(result, expected);
    }
}
