use riimut::younger_futhark;

#[test]
fn parses_to_younger_futhark() {
    // From Old Groms runestone.
    let content = "auk tani karþi kristna";
    let expected = "ᛅᚢᚴ:ᛏᛅᚾᛁ:ᚴᛅᚱᚦᛁ:ᚴᚱᛁᛋᛏᚾᛅ";
    let result = younger_futhark::letters_to_runes(content);

    assert_eq!(result, expected);    
}

#[test]
fn parses_younger_futhark_to_text() {
    // From Old Groms runestone.
    let content = "ᛅᚢᚴ:ᛏᛅᚾᛁ:ᚴᛅᚱᚦᛁ:ᚴᚱᛁᛋᛏᚾᛅ";
    let expected = "auk tani karþi kristna";
    let result = younger_futhark::runes_to_letters(content);

    assert_eq!(result, expected);    
}