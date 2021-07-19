use riimut::younger_futhark;
use riimut::elder_futhark;
use riimut::medieval_futhork;

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


#[test]
fn parses_to_elder_futhark() {
    // From 4th century axe in Jutland
    let content = "wagagastiz alu wihgu sikijaz aiþalataz";
    let expected = "ᚹᚨᚷᚨᚷᚨᛋᛏᛁᛉ:ᚨᛚᚢ:ᚹᛁᚻᚷᚢ:ᛋᛁᚲᛁᛃᚨᛉ:ᚨᛁᚦᚨᛚᚨᛏᚨᛉ";
    let result = elder_futhark::letters_to_runes(content);

    assert_eq!(result, expected);    
}

#[test]
fn parses_elder_futhark_to_text() {
    // From 4th century axe in Jutland
    let content = "ᚹᚨᚷᚨᚷᚨᛋᛏᛁᛉ:ᚨᛚᚢ:ᚹᛁᚻᚷᚢ:ᛋᛁᚲᛁᛃᚨᛉ:ᚨᛁᚦᚨᛚᚨᛏᚨᛉ";
    let expected = "wagagastiz alu wihgu sikijaz aiþalataz";
    let result = elder_futhark::runes_to_letters(content);

    assert_eq!(result, expected);    
}

#[test]
fn parses_to_medieval_futhork() {
    // From Lord's Prayer.
    let content = "Faðer uor som ast i himlüm, halgað warðe þit nama";
    let expected = "ᚠᛆᚦᚽᚱ:ᚢᚮᚱ:ᛋᚮᛘ:ᛆᛋᛏ:ᛁ:ᚼᛁᛘᛚᚢᛘ,:ᚼᛆᛚᚵᛆᚦ:ᚠᛆᚱᚦᚽ:ᚦᛁᛏ:ᚿᛆᛘᛆ";
    let result = medieval_futhork::letters_to_runes(content);

    assert_eq!(result, expected);    
}

#[test]
fn parses_medieval_futhork_to_text() {
    // From Lord's Prayer.
    let content = "ᚠᛆᚦᚽᚱ:ᚢᚮᚱ:ᛋᚮᛘ:ᛆᛋᛏ:ᛁ:ᚼᛁᛘᛚᚢᛘ:ᚼᛆᛚᚵᛆᚦ:ᚠᛆᚱᚦᚽ:ᚦᛁᛏ:ᚿᛆᛘᛆ";
    let expected = "faþer uor som ast i himlum halgaþ farþe þit nama"; // Wont tell apart eth & thorn in mid sentence.
    let result = medieval_futhork::runes_to_letters(content);

    assert_eq!(result, expected);    
}