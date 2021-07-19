pub mod mapping;
use crate::transform;

pub fn letters_to_runes(content: &str) -> String {
    let letter_map = mapping::get_letters_to_runes_map();
    
    transform::transform(&content, &letter_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforms_letters_to_younger_futhark() {
        let content = "aábcdðeéfghiíjklmnoópqrstþuúvwxyýzåäæöøǫ";
        let expected = "ᛅᛅᛒᛋᛏᚦᛁᛁᚠᚴᚼᛁᛁᛁᚴᛚᛘᚾᚢᚢᛒᚴᚱᛋᛏᚦᚢᚢᚢᚢᛋᚢᚢᛋᚢᛅᛅᚢᚢᚢ";
        let result = letters_to_runes(content);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_full_sentence() {
        let content = "auk tani karþi kristna";
        let expected = "ᛅᚢᚴ:ᛏᛅᚾᛁ:ᚴᛅᚱᚦᛁ:ᚴᚱᛁᛋᛏᚾᛅ";
        let result = letters_to_runes(content);

        assert_eq!(result, expected);
    }
}
