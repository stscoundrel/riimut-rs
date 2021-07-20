pub mod mapping;
use crate::transform;

/// Transform latin text to runes
///
/// 
/// # Examples
/// 
/// ```
/// use riimut::medieval_futhork;
/// 
/// let result = medieval_futhork::letters_to_runes("When I reach out my hand");
/// let expected = "ᚠᚼᚽᚿ:ᛁ:ᚱᚽᛆᚴᚼ:ᚮᚢᛏ:ᛘᛦ:ᚼᛆᚿᚦ";
/// 
/// assert_eq!(result, expected);
/// ```
pub fn letters_to_runes(content: &str) -> String {
    let letter_map = mapping::get_letters_to_runes_map();
    
    transform::with_dictionary(&content, &letter_map)
}

/// Transform latin text to runes
///
/// 
/// # Examples
/// 
/// ```
/// use riimut::medieval_futhork;
/// 
/// let result = medieval_futhork::runes_to_letters("ᚠᚼᚽᚿ:ᛁ:ᚱᚽᛆᚴᚼ:ᚮᚢᛏ:ᛘᛦ:ᚼᛆᚿᚦ");
/// let expected = "fhen i reakh out my hanþ";
/// 
/// assert_eq!(result, expected);
/// ```
pub fn runes_to_letters(content: &str) -> String {
    let rune_map = mapping::get_runes_to_letters_map();
    
    transform::with_dictionary(&content, &rune_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforms_letters_to_medieval_futhork() {
        let content = "aábcdðeéfghiíjklmnoóǫpqrstuúvwxyýzåäæœöøþ ";
        let expected = "ᛆᛆᛒᚴᚦᚦᚽᚽᚠᚵᚼᛁᛁᛁᚴᛚᛘᚿᚮᚮᚰᛕᚴᚱᛋᛏᚢᚢᚠᚠᛋᛦᛦᛋᚮᛅᛅᚯᚯᚯᚦ:";
        let result = letters_to_runes(content);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_runes_to_letters() {
        let content = "ᚠᚢᚦᚮᚱᚴᚼᚿᛁᛆᛌᛋᛐᛏᛒᛘᛚᛦᚯᛅᚰᛕᚽ:";
        let expected = "fuþorkhniassttbmlyøæǫᴘe ";
        let result = runes_to_letters(content);

        assert_eq!(result, expected);
    }
}


