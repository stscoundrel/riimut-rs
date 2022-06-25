pub mod mapping;
use crate::transform;

/// Transform latin text to runes.
/// Defaults to long branch runeset.
/// 
/// # Examples
/// 
/// ```
/// use riimut::younger_futhark;
/// 
/// let result = younger_futhark::letters_to_runes("Even though you'd soon be gone");
/// let expected = "ᛁᚢᛁᚾ:ᛏᚼᚢᚢᚴᚼ:ᚢᚢᚢ'ᛏ:ᛋᚢᚢᚾ:ᛒᛁ:ᚴᚢᚾᛁ";
/// 
/// assert_eq!(result, expected);
/// ```
pub fn letters_to_runes(content: &str) -> String {
    letters_to_long_branch_runes(content)
}

/// Transform latin text to long branch runes.
/// Long branch is the Danish rune variant of Younger Futhark.
/// 
/// # Examples
/// 
/// ```
/// use riimut::younger_futhark;
/// 
/// let result = younger_futhark::letters_to_long_branch_runes("Even though you'd soon be gone");
/// let expected = "ᛁᚢᛁᚾ:ᛏᚼᚢᚢᚴᚼ:ᚢᚢᚢ'ᛏ:ᛋᚢᚢᚾ:ᛒᛁ:ᚴᚢᚾᛁ";
/// 
/// assert_eq!(result, expected);
/// ```
pub fn letters_to_long_branch_runes(content: &str) -> String {
    let letter_map = mapping::get_letters_to_long_branch_runes_map();
    
    transform::with_dictionary(content, &letter_map)
}

/// Transform latin text to short_twig runes.
/// Short twig is the Swedish / Norwegian rune variant of Younger Futhark.
/// 
/// # Examples
/// 
/// ```
/// use riimut::younger_futhark;
/// 
/// let result = younger_futhark::letters_to_short_twig_runes("Even though you'd soon be gone");
/// let expected = "ᛁᚢᛁᚿ:ᛐᚽᚢᚢᚴᚽ:ᚢᚢᚢ'ᛐ:ᛌᚢᚢᚿ:ᛒᛁ:ᚴᚢᚿᛁ";
/// 
/// assert_eq!(result, expected);
/// ```
pub fn letters_to_short_twig_runes(content: &str) -> String {
    let letter_map = mapping::get_letters_to_short_twig_runes_map();
    
    transform::with_dictionary(content, &letter_map)
}

/// Transform latin text to runes.
///
/// 
/// # Examples
/// 
/// ```
/// use riimut::younger_futhark;
/// 
/// let result = younger_futhark::runes_to_letters("ᛁᚢᛁᚾ:ᛏᚼᚢᚢᚴᚼ:ᚢᚢᚢ'ᛏ:ᛋᚢᚢᚾ:ᛒᛁ:ᚴᚢᚾᛁ");
/// let expected = "iuin thuukh uuu't suun bi kuni";
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
    fn transforms_letters_to_younger_futhark_default_function() {
        let content = "aábcdðeéfghiíjklmnoópqrstþuúvwxyýzåäæöøǫþ";
        let expected = "ᛅᛅᛒᛋᛏᚦᛁᛁᚠᚴᚼᛁᛁᛁᚴᛚᛘᚾᚢᚢᛒᚴᚱᛋᛏᚦᚢᚢᚢᚢᛋᚢᚢᛋᚢᛅᛅᚢᚢᚢᚦ";
        let result = letters_to_runes(content);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_letters_to_long_branch_runes() {
        let content = "aábcdðeéfghiíjklmnoópqrstþuúvwxyýzåäæöøǫþ";
        let expected = "ᛅᛅᛒᛋᛏᚦᛁᛁᚠᚴᚼᛁᛁᛁᚴᛚᛘᚾᚢᚢᛒᚴᚱᛋᛏᚦᚢᚢᚢᚢᛋᚢᚢᛋᚢᛅᛅᚢᚢᚢᚦ";
        let result = letters_to_long_branch_runes(content);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_letters_to_short_twig_runes() {
        let content = "aábcdðeéfghiíjklmnoópqrstþuúvwxyýzåäæöøǫþ";
        let expected = "ᛆᛆᛒᛌᛐᚦᛁᛁᚠᚴᚽᛁᛁᛁᚴᛚᛘᚿᚢᚢᛒᚴᚱᛌᛐᚦᚢᚢᚢᚢᛌᚢᚢᛌᚢᛆᛆᚢᚢᚢᚦ";
        let result = letters_to_short_twig_runes(content);

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_runes_to_letters() {
        let long_branch = "ᚠᚢᚦᚬᚱᚴᚼᚽᚾᚿᛁᛅᛆᛋᛌᛏᛐᛒᛘᛚᛦ:";
        let short_twig = "ᚠᚢᚦᚬᚱᚴᚽᚽᚿᚿᛁᛆᛆᛌᛌᛐᛐᛒᛘᛚᛦ:";
        let expected = "fuþorkhhnniaassttbmlR ";
        let long_branch_result = runes_to_letters(long_branch);
        let short_twig_result = runes_to_letters(short_twig);

        // Both runesets should transform to same result.
        assert_eq!(long_branch_result, expected);
        assert_eq!(short_twig_result, expected);
    }
}
