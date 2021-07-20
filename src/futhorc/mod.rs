pub mod mapping;
use crate::transform;

/// Transform latin text to runes
///
/// 
/// # Examples
/// 
/// ```
/// use riimut::futhorc;
/// 
/// let result = futhorc::letters_to_runes("Then you might just appear");
/// let expected = "ᛏᚻᛖᚾ:ᚣᚩᚢ:ᛗᛁᚷᚻᛏ:ᛡᚢᛋᛏ:ᚪᛈᛈᛖᚪᚱ";
/// 
/// assert_eq!(result, expected);
/// ```
pub fn letters_to_runes(content: &str) -> String {
    let letter_map = mapping::get_letters_to_runes_map();
    
    transform::with_dictionary(&content, &letter_map)
}

/// Transform runes to latin letters
///
/// 
/// # Examples
/// 
/// ```
/// use riimut::futhorc;
/// 
/// let result = futhorc::runes_to_letters("ᛏᚻᛖᚾ:ᚣᚩᚢ:ᛗᛁᚷᚻᛏ:ᛡᚢᛋᛏ:ᚪᛈᛈᛖᚪᚱ");
/// let expected = "then you might just appear";
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
    fn transforms_letters_to_futhorc() {
        let content = "aábcdðeéfghiíïʒjklmnŋoóǫpqrstuúvwxyýzåäæœöøþ ";
        let expected = "ᚪᚪᛒᚳᛞᛞᛖᛖᚠᚷᚻᛁᛇᛇᛇᛡᚳᛚᛗᚾᛝᚩᚩᚩᛈᚳᚱᛋᛏᚢᚢᚹᚹᛉᚣᚣᛉᚩᚫᚫᛟᛟᛟᚦ:";
        let result = letters_to_runes(content);

        println!("{}", result);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_runes_to_letters() {
        let content = "ᚠᚢᚦᚩᚱᚳᚷᚹᚻᚾᛁᛡᛄᛇᛈᛉᛋᚴᛏᛒᛖᛗᛚᛝᛟᛞᚪᚫᚣᛠ:";
        let expected = "fuþorcgwhnijjïpxsstbemlŋœdaæyea ";
        let result = runes_to_letters(content);

        assert_eq!(result, expected);
    }
}

