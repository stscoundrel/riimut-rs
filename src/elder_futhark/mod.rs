pub mod mapping;
use crate::transform;

/// Transform latin text to runes
///
/// 
/// # Examples
/// 
/// ```
/// use riimut::elder_futhark;
/// 
/// let result = elder_futhark::letters_to_runes("And if the day would only come");
/// let expected = "ᚨᚾᛞ:ᛁᚠ:ᛏᚻᛖ:ᛞᚨᛁ:ᚹᛟᚢᛚᛞ:ᛟᚾᛚᛁ:ᚲᛟᛗᛖ";
/// 
/// assert_eq!(result, expected);
/// ```
pub fn letters_to_runes(content: &str) -> String {
    let letter_map = mapping::get_letters_to_runes_map();
    
    transform::with_dictionary(content, &letter_map)
}

/// Transform runes to latin text
///
/// 
/// # Examples
/// 
/// ```
/// use riimut::elder_futhark;
/// 
/// let result = elder_futhark::runes_to_letters("ᚨᚾᛞ:ᛁᚠ:ᛏᚻᛖ:ᛞᚨᛁ:ᚹᛟᚢᛚᛞ:ᛟᚾᛚᛁ:ᚲᛟᛗᛖ");
/// let expected = "and if the dai would onli kome";
/// 
/// assert_eq!(result, expected);
/// ```
pub fn runes_to_letters(content: &str) -> String {
    let rune_map = mapping::get_runes_to_letters_map();
    
    transform::with_dictionary(content, &rune_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforms_letters_to_elder_futhark() {
        let content = "aábcdðeéfghiíjklmnŋoópqrstþuúvwxyýzåäæöøǫþ";
        let expected = "ᚨᚨᛒᚲᛞᚦᛖᛖᚠᚷᚻᛁᛁᛃᚲᛚᛗᚾᛜᛟᛟᛈᚲᚱᛋᛏᚦᚢᚢᚹᚹᛋᛁᛁᛉᛟᛇᛇᚢᚢᛟᚦ";
        let result = letters_to_runes(content);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_runes_to_letters() {
        let content = "ᚠᚢᚦᚨᚱᚲᚷᚹᚺᚻᚾᛁᛃᛇᛈᛉᛊᛋᛏᛒᛖᛗᛚᛜᛝᛟᛞ:";
        let expected = "fuþarkgwhhnijïpzsstbemlŋŋod ";
        let result = runes_to_letters(content);

        assert_eq!(result, expected);
    }
}

