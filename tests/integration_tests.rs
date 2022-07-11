use riimut::younger_futhark;
use riimut::elder_futhark;
use riimut::medieval_futhork;
use riimut::futhorc;
use riimut::staveless_futhark;

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

#[test]
fn parses_to_anglo_frisian_futhorc() {
    // From 8th century Franks Casket, in late West Saxon.
    let content = "fisc.flodu.ahofonferg | enberig |";
    let expected = "ᚠᛁᛋᚳ.ᚠᛚᚩᛞᚢ.ᚪᚻᚩᚠᚩᚾᚠᛖᚱᚷ:|:ᛖᚾᛒᛖᚱᛁᚷ:|";
    let result = futhorc::letters_to_runes(content);

    assert_eq!(result, expected);    
}

#[test]
fn parses_anglo_frisian_futhorc_to_text() {
    // From 8th century Franks Casket, in late West Saxon.
    let content = "ᚠᛁᛋᚳ.ᚠᛚᚩᛞᚢ.ᚪᚻᚩᚠᚩᚾᚠᛖᚱᚷ:|:ᛖᚾᛒᛖᚱᛁᚷ:|";
    let expected = "fisc.flodu.ahofonferg | enberig |";
    let result = futhorc::runes_to_letters(content);

    assert_eq!(result, expected);    
}

#[test]
fn parses_to_staveless_futhark() {
  // From Old Norse text in Hög runestone.
  let content = "kuþniutr þru sun lit rita stin þina ak bru kirþi aftiʀ bruþr sina asbiurn ak at kuþlaf";
  let expected = "ᛍ╮ו⸜ᛁ╮⸍◟:ו◟╮:╵╮⸜:⸌ᛁ⸍:◟ᛁ⸍⸝:╵⸍ᛁ⸜:וᛁ⸜⸝:⸝ᛍ:ˏ◟╮:ᛍᛁ◟וᛁ:⸝ᛙ⸍ᛁʀ:ˏ◟╮ו◟:╵ᛁ⸜⸝:⸝╵ˏᛁ╮◟⸜:⸝ᛍ:⸝⸍:ᛍ╮ו⸌⸝ᛙ";
  let result = staveless_futhark::letters_to_runes(content);
  
  assert_eq!(result, expected);
}

#[test]
fn parses_staveless_futhark_to_test() {
  // From Old Norse text in Hög runestone.
  let content = "ᛍ╮ו⸜ᛁ╮⸍◟:ו◟╮:╵╮⸜:⸌ᛁ⸍:◟ᛁ⸍⸝:╵⸍ᛁ⸜:וᛁ⸜⸝:⸝ᛍ:ˏ◟╮:ᛍᛁ◟וᛁ:⸝ᛙ⸍ᛁʀ:ˏ◟╮ו◟:╵ᛁ⸜⸝:⸝╵ˏᛁ╮◟⸜:⸝ᛍ:⸝⸍:ᛍ╮ו⸌⸝ᛙ";
  let expected = "kuþniutr þru sun lit rita stin þina ak bru kirþi aftiʀ bruþr sina asbiurn ak at kuþlaf";
  let result = staveless_futhark::runes_to_letters(content);
  
  assert_eq!(result, expected);
}