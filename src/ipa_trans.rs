use regex::Regex;
use lazy_static::lazy_static;

pub struct TransPair(pub Regex, pub &'static str);

macro_rules! trans_pair_group {
    ($($r:literal => $i:literal;)*) => {
        vec![$(TransPair(Regex::new($r).unwrap(), $i),)*]
    }
}

pub trait IpaTranslate {
    fn translate(&self, text: &str) -> String;
}

impl IpaTranslate for Vec<TransPair> {
    fn translate(&self, text: &str) -> String {
        let mut text = text.to_string();
        for trans_pair in self.iter() {
            text = trans_pair.0.replace_all(&text, trans_pair.1).to_string();
        }

        text
    }
}

lazy_static! {
    pub static ref XSAMPA_TRANS: Vec<TransPair> = trans_pair_group! {
        // diacritics part 1.
        r"_A" => "̘";
        r"_a" => "̺";
        r"_B_L" => "᷅";
        r"_B" => " ̏";
        r"_c" => "̜";
        r"_d" => "̪";
        r"_e" => "̴";
        r"<F>" => "↘";
        r"_F" => "̂";
        r"_G" => "ˠ";
        r"_H_T" => "᷄";
        r"_H" => " ́";
        r"_h" => "ʰ";
        r"_j" => "ʲ";
        r"_k" => "̰";
        r"_L" => " ̀";
        r"_l" => "ˡ";
        r"_M" => "̄";
        r"_m" => "̻";
        r"_N" => "̼";
        r"_n" => "ⁿ";
        r"_O" => "̹";
        r"_o" => "̞";
        r"_q" => "̙";
        r"<R>" => "↗";
        r"_R_F" => "᷈";
        r"_R" => "̌";
        r"_r" => "̝";
        r"_T" => "̋";
        r"_t" => "̤";
        r"_v" => "̬";
        r"_w" => "ʷ";
        r"_X" => "̆";
        r"_x" => "̽";
        
        // lowercase symbols.
        r"a" => "a";
        r"b_<" => "ɓ";
        r"b" => "b";
        r"c" => "c";
        r"d_<" => "ɗ";
        r"d`" => "ɖ";
        r"d" => "d";
        r"e" => "e";
        r"f" => "f";
        r"g_<" => "ɠ";
        r"g" => "ɡ";
        r"h\\" => "ɦ";
        r"h" => "h";
        r"i" => "i";
        r"j\\" => "ʝ";
        r"j" => "j";
        r"k" => "k";
        r"l`" => "ɭ";
        r"l\\" => "ɺ";
        r"l" => "l";
        r"m" => "m";
        r"n`" => "ɳ";
        r"n" => "n";
        r"o" => "o";
        r"p\\" => "ɸ";
        r"p" => "p";
        r"q" => "q";
        r"r\\`" => "ɻ";
        r"r\\" => "ɹ";
        r"r`" => "ɽ";
        r"r" => "r";
        r"s\\" => "ɕ";
        r"s`" => "ʂ";
        r"s" => "s";
        r"t`" => "ʈ";
        r"t" => "t";
        r"u" => "u";
        r"v\\" => "ʋ";
        r"v" => "v";
        r"w" => "w";
        r"x\\" => "ɧ";
        r"x" => "x";
        r"y" => "y";
        r"z\\" => "ʑ";
        r"z`" => "ʐ";
        r"z" => "z";

        // uppercase symbols.
        r"A" => "ɑ";
        r"B\\" => "ʙ";
        r"B" => "β";
        r"C" => "ç";
        r"D" => "ð";
        r"E" => "ɛ";
        r"F" => "ɱ";
        r"G\\_<" => "ʛ";
        r"G\\" => "ɢ";
        r"G" => "ɣ";
        r"H\\" => "ʜ";
        r"H" => "ɥ";
        r"I\\" => "ᵻ";
        r"I" => "ɪ";
        r"J\\_<" => "ʄ";
        r"J\\" => "ɟ";
        r"J" => "ɲ";
        r"K\\" => "ɮ";
        r"K" => "ɬ";
        r"L\\" => "ʟ";
        r"L" => "ʎ";
        r"M\\" => "ɰ";
        r"M" => "ɯ";
        r"N\\" => "ɴ";
        r"N" => "ŋ";
        r"O\\" => "ʘ";
        r"O" => "ɔ";
        r"P" => "ʋ";
        r"Q" => "ɒ";
        r"R\\" => "ʀ";
        r"R" => "ʁ";
        r"S" => "ʃ";
        r"T" => "θ";
        r"U\\" => "ᵿ";
        r"U" => "ʊ";
        r"V" => "ʌ";
        r"W" => "ʍ";
        r"X\\" => "ħ";
        r"X" => "χ";
        r"Y" => "ʏ";
        r"Z" => "ʒ";

        // diacritics part 2.
        r#"_""# => "̈";
        r"_\+" => "̟";
        r"_\-" => "̠";
        r"_/" => "̌";
        r"_0" => "̥";
        r"_=" => "̩";
        r"=" => "̩";
        r"_>" => "ʼ";
        r"_\?\\" => "ˤ";
        r"_\\" => "̂";
        r"_\^" => "̯";
        r"_\}" => "̚";
        r"`" => "˞";
        r"_~" => " ̃";
        r"~" => " ̃";

        // other symbols.
        r"\." => ".";
        r#"""# => "ˈ";
        r"%" => "ˌ";
        r"'" => "ʲ";
        r":\\" => "ˑ";
        r":" => "ː";
        r"@\\" => "ɘ";
        r"@`" => "ɚ";
        r"@" => "ə";
        r"\{" => "æ";
        r"\}" => "ʉ";
        r"1" => "ɨ";
        r"2" => "ø";
        r"3\\" => "ɞ";
        r"3" => "ɜ";
        r"4" => "ɾ";
        r"5" => "ɫ";
        r"6" => "ɐ";
        r"7" => "ɤ";
        r"8" => "ɵ";
        r"9" => "œ";
        r"\&" => "ɶ";
        r"\?\\" => "ʕ";
        r"\?" => "ʔ";
        r"<\\" => "ʢ";
        r">\\" => "ʡ";
        r"\^" => "ꜛ";
        r"!\\" => "ǃ";
        r"!" => "ꜜ";
        r"\|\\\|\\" => "ǁ";
        r"\|\|" => "‖";
        r"\|\\" => "ǀ";
        r"\|" => "|";
        r"=\\" => "ǂ";
        r"\-\\" => "‿";
    };
}
