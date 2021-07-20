# Riimut

Transform latin letters to runes &amp; vice versa.

Includes transformers for four main runic alphabets:

- Elder Futhark
- Younger Futhark
- Medieval Futhork
- Futhorc (Anglo-Frisian runes)

### Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
riimut = "1.0.0"
```

### Usage

Text to runes:
```rust
// Ships four dialects in separate modules.
use riimut::younger_futhark;
use riimut::elder_futhark;
use riimut::medieval_futhork;
use riimut::futhorc;

// From Old Groms runestone.
let younger_futhark = younger_futhark::letters_to_runes("auk tani karþi kristna");
println!(younger_futhark); // "ᛅᚢᚴ:ᛏᛅᚾᛁ:ᚴᛅᚱᚦᛁ:ᚴᚱᛁᛋᛏᚾᛅ"

// From 4th century axe in Jutland
let elder_futhark = elder_futhark::letters_to_runes("wagagastiz alu wihgu sikijaz aiþalataz");
println!(elder_futhark); // "ᚹᚨᚷᚨᚷᚨᛋᛏᛁᛉ:ᚨᛚᚢ:ᚹᛁᚻᚷᚢ:ᛋᛁᚲᛁᛃᚨᛉ:ᚨᛁᚦᚨᛚᚨᛏᚨᛉ"

// From Lord's Prayer, in Old Norse.
let medieval_futhork = medieval_futhork::letters_to_runes("Faðer uor som ast i himlüm, halgað warðe þit nama");
println!(medieval_futhork); // "ᚠᛆᚦᚽᚱ:ᚢᚮᚱ:ᛋᚮᛘ:ᛆᛋᛏ:ᛁ:ᚼᛁᛘᛚᚢᛘ,:ᚼᛆᛚᚵᛆᚦ:ᚠᛆᚱᚦᚽ:ᚦᛁᛏ:ᚿᛆᛘᛆ"

// From 8th century Franks Casket, in late West Saxon.
let futhorc = futhorc::letters_to_runes("fisc.flodu.ahofonferg | enberig |");
println!(futhorc); // "ᚠᛁᛋᚳ.ᚠᛚᚩᛞᚢ.ᚪᚻᚩᚠᚩᚾᚠᛖᚱᚷ:|:ᛖᚾᛒᛖᚱᛁᚷ:|"

```

Runes to text:
```rust

// All four dialects contain runes_to_letters function.
use riimut::younger_futhark;

let runic_text = "ᛅᚢᚴ:ᛏᛅᚾᛁ:ᚴᛅᚱᚦᛁ:ᚴᚱᛁᛋᛏᚾᛅ";
let latin_text = younger_futhark::runes_to_letters(runic_text);

println!(latin_text); // "auk tani karþi kristna"

```

#### What's in the name?

"Riimut" is the Finnish word for "runes". Most rune related cratenames were already taken, so just added some Finnish flavor to it.