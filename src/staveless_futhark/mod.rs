pub mod mapping;
use crate::transform;

/// Transform latin text to runes
///
/// 
/// # Examples
/// 
/// ```
/// use riimut::staveless_futhark;
/// 
/// let result = staveless_futhark::letters_to_runes("And if the day would only come");
/// let expected = "⸝⸜⸍:ᛁᛙ:⸍ᚽᛁ:⸍⸝╮:╮ˎ╮⸌⸍:ˎ⸜⸌╮:╵ˎ⠃ᛁ";
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
/// use riimut::staveless_futhark;
/// 
/// let result = staveless_futhark::runes_to_letters("⸝⸜⸍:ᛁᛙ:⸍ᚽᛁ:⸍⸝╮:╮ˎ╮⸌⸍:ˎ⸜⸌╮:╵ˎ⠃ᛁ");
/// let expected = "ant if thi tau uoult onlu somi";
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
    fn transforms_letters_to_staveless_futhark() {
        let content = "aábcdðeéfghiíjklmnoópqrRstþuúvwxyýzåäæöøǫþ ";
        let expected = "⸝⸝ˏ╵⸍וᛁᛁᛙᛍᚽᛁᛁᛁᛍ⸌⠃⸜ˎˎˏᛍ◟◟╵⸍ו╮╮╮╮╵╮╮╵ˎ⸝⸝ˎˎˎו:";
        let result = letters_to_runes(content);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_runes_to_letters() {
        let content = "ᛙ╮וˎ⡄ᛍᚽ⸜ᛁ⸝╵⸍ˏ⠃⸌⡄:";
        let expected = "fuþoRkhniastbmlR ";
        let result = runes_to_letters(content);

        assert_eq!(result, expected);
    }
}

