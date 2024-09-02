// References
// http://www.unicode.org/versions/Unicode9.0.0/ch03.pdf#M9.32468.Heading.310.Combining.Jamo.Behavior

/// 초성 개수
const L_COUNT: u32 = 0x13;
/// 중성 개수
const V_COUNT: u32 = 0x15;
/// 종성 개수
const T_COUNT: u32 = 0x1C;

/// 중성과 종성의 조합 개수
const N_COUNT: u32 = V_COUNT * T_COUNT;

/// 초성과 중성, 종성의 조합 개수
///
/// 가능한 한글 음절의 경우의 수
const S_COUNT: u32 = L_COUNT * N_COUNT;

/// 완성형 한글 시작
///
/// '가'
const S_BASE: u32 = 0xAC00;
/// 완성형 한글 끝
///
/// '힣'
const S_LAST: u32 = S_BASE + S_COUNT - 1;

/// 초성 시작
///
/// 'ㄱ'
const L_BASE: u32 = 0x1100;
/// 초성 끝
///
/// 'ㅎ'
const L_LAST: u32 = L_BASE + L_COUNT - 1;

/// 중성 시작
const V_BASE: u32 = 0x1161;
/// 중성 끝
const V_LAST: u32 = V_BASE + V_COUNT - 1;

/// 종성 시작
const T_BASE: u32 = 0x11A7;
/// 종성 끝
const T_LAST: u32 = T_BASE + T_COUNT - 1;

const CHOSEONGS: [char; L_COUNT as usize] = [
  'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
  'ㅌ', 'ㅍ', 'ㅎ',
];

pub struct Decompose;

impl Decompose {
  pub fn decompose(string: String) -> String {
    let decomposed_hanguls = Self::decompose_to_groups(string);

    decomposed_hanguls
      .into_iter()
      .fold("".to_string(), |hanguls, decomposed_hangul| {
        return hanguls + &decomposed_hangul.join("");
      })
  }

  pub fn decompose_to_groups(string: String) -> Vec<Vec<String>> {
    string
      .chars()
      .map(|letter| {
        let decomposed_hangul = DecomposedHangul::new(letter).unwrap();
        let mut group = vec![
          decomposed_hangul.choseong.to_string(),
          decomposed_hangul.jungseong.to_string(),
        ];
        if let Some(jongseong) = decomposed_hangul.jongseong {
          group.push(jongseong.to_string())
        }
        return group;
      })
      .collect()
  }
}

#[derive(Debug)]
pub struct DecomposedHangul {
  choseong: String,
  jungseong: String,
  jongseong: Option<String>,
}

impl DecomposedHangul {
  fn new(letter: char) -> Option<DecomposedHangul> {
    Self::new_inner(letter)
  }

  fn new_inner(letter: char) -> Option<DecomposedHangul> {
    if !Self::is_hangul(letter) {
      return None;
    }

    let (choseong, jungseong, jongseong) = Self::nfd_normalize(letter);

    let choseong = std::char::from_u32(choseong).unwrap().to_string();

    let jungseong = Self::decompose_vowel_from_u32(&jungseong).to_string();

    let jongseong = jongseong.map(|consonant| {
      return Self::decompose_consonant_from_u32(&consonant).to_string();
    });

    Some(Self {
      choseong,
      jungseong,
      jongseong,
    })
  }

  fn nfd_normalize(letter: char) -> (u32, u32, Option<u32>) {
    let letter = letter as u32;

    let s_index = letter - S_BASE;
    let choseong_index = s_index / N_COUNT;
    let jungseong_index = (s_index % N_COUNT) / T_COUNT;
    let jongseong_index = s_index % T_COUNT;

    let choseong = L_BASE + choseong_index;
    let jungseong = V_BASE + jungseong_index;
    let jongseong = if jongseong_index > 0 {
      Some(T_BASE + jongseong_index)
    } else {
      None
    };

    (choseong, jungseong, jongseong)
  }

  fn is_hangul(letter: char) -> bool {
    let letter = letter as u32;
    S_BASE <= letter && letter <= S_LAST
  }

  fn decompose_consonant_from_u32(consonant: &u32) -> &str {
    match consonant {
      0x20 => " ",      // ' ' (공백)
      0x3131 => "ㄱ",   // 'ㄱ'
      0x3132 => "ㄲ",   // 'ㄲ'
      0x3133 => "ㄱㅅ", // 'ㄳ'
      0x3134 => "ㄴ",   // 'ㄴ'
      0x3135 => "ㄴㅈ", // 'ㄵ'
      0x3136 => "ㄴㅎ", // 'ㄶ'
      0x3137 => "ㄷ",   // 'ㄷ'
      0x3138 => "ㄸ",   // 'ㄸ'
      0x3139 => "ㄹ",   // 'ㄹ'
      0x313A => "ㄹㄱ", // 'ㄺ'
      0x313B => "ㄹㅁ", // 'ㄻ'
      0x313C => "ㄹㅂ", // 'ㄼ'
      0x313D => "ㄹㅅ", // 'ㄽ'
      0x313E => "ㄹㅌ", // 'ㄾ'
      0x313F => "ㄹㅍ", // 'ㄿ'
      0x3140 => "ㄹㅎ", // 'ㅀ'
      0x3141 => "ㅁ",   // 'ㅁ'
      0x3142 => "ㅂ",   // 'ㅂ'
      0x3143 => "ㅃ",   // 'ㅃ'
      0x3144 => "ㅂㅅ", // 'ㅄ'
      0x3145 => "ㅅ",   // 'ㅅ'
      0x3146 => "ㅆ",   // 'ㅆ'
      0x3147 => "ㅇ",   // 'ㅇ'
      0x3148 => "ㅈ",   // 'ㅈ'
      0x3149 => "ㅉ",   // 'ㅉ'
      0x314A => "ㅊ",   // 'ㅊ'
      0x314B => "ㅋ",   // 'ㅋ'
      0x314C => "ㅌ",   // 'ㅌ'
      0x314D => "ㅍ",   // 'ㅍ'
      0x314E => "ㅎ",   // 'ㅎ'
      _ => unreachable!(),
    }
  }

  fn decompose_consonant(consonant: &char) -> &str {
    match consonant {
      ' ' => " ",
      'ㄱ' => "ㄱ",
      'ㄲ' => "ㄲ",
      'ㄳ' => "ㄱㅅ",
      'ㄴ' => "ㄴ",
      'ㄵ' => "ㄴㅈ",
      'ㄶ' => "ㄴㅎ",
      'ㄷ' => "ㄷ",
      'ㄸ' => "ㄸ",
      'ㄹ' => "ㄹ",
      'ㄺ' => "ㄹㄱ",
      'ㄻ' => "ㄹㅁ",
      'ㄼ' => "ㄹㅂ",
      'ㄽ' => "ㄹㅅ",
      'ㄾ' => "ㄹㅌ",
      'ㄿ' => "ㄹㅍ",
      'ㅀ' => "ㄹㅎ",
      'ㅁ' => "ㅁ",
      'ㅂ' => "ㅂ",
      'ㅃ' => "ㅃ",
      'ㅄ' => "ㅂㅅ",
      'ㅅ' => "ㅅ",
      'ㅆ' => "ㅆ",
      'ㅇ' => "ㅇ",
      'ㅈ' => "ㅈ",
      'ㅉ' => "ㅉ",
      'ㅊ' => "ㅊ",
      'ㅋ' => "ㅋ",
      'ㅌ' => "ㅌ",
      'ㅍ' => "ㅍ",
      'ㅎ' => "ㅎ",
      _ => unreachable!(),
    }
  }

  fn decompose_vowel_from_u32(vowel: &u32) -> &str {
    match vowel {
      0x20 => " ",
      0x314F => "ㅏ",
      0x3150 => "ㅐ",
      0x3151 => "ㅑ",
      0x3152 => "ㅒ",
      0x3153 => "ㅓ",
      0x3154 => "ㅔ",
      0x3155 => "ㅕ",
      0x3156 => "ㅖ",
      0x3157 => "ㅗ",
      0x3158 => "ㅗㅏ",
      0x3159 => "ㅗㅐ",
      0x315A => "ㅗㅣ",
      0x315B => "ㅛ",
      0x315C => "ㅜ",
      0x315D => "ㅜㅓ",
      0x315E => "ㅜㅔ",
      0x315F => "ㅜㅣ",
      0x3160 => "ㅠ",
      0x3161 => "ㅡ",
      0x3162 => "ㅡㅣ",
      0x3163 => "ㅣ",
      _ => unreachable!(),
    }
  }

  fn decompose_vowel(vowel: &char) -> &str {
    match vowel {
      'ㅏ' => "ㅏ",
      'ㅐ' => "ㅐ",
      'ㅑ' => "ㅑ",
      'ㅒ' => "ㅒ",
      'ㅓ' => "ㅓ",
      'ㅔ' => "ㅔ",
      'ㅕ' => "ㅕ",
      'ㅖ' => "ㅖ",
      'ㅗ' => "ㅗ",
      'ㅘ' => "ㅗㅏ",
      'ㅙ' => "ㅗㅐ",
      'ㅚ' => "ㅗㅣ",
      'ㅛ' => "ㅛ",
      'ㅜ' => "ㅜ",
      'ㅝ' => "ㅜㅓ",
      'ㅞ' => "ㅜㅔ",
      'ㅟ' => "ㅜㅣ",
      'ㅠ' => "ㅠ",
      'ㅡ' => "ㅡ",
      'ㅢ' => "ㅡㅣ",
      'ㅣ' => "ㅣ",
      ' ' => " ",
      _ => unreachable!(),
    }
  }
}

#[test]
fn test_decompose_to_groups() {
  let result = Decompose::decompose_to_groups("값이 비싸다".to_string());
  println!("{:?}", result);

  // assert_eq!(decompose_to_groups("값이 비싸다".to_string()), vec![
  //     vec!["ㄱ", "ㅏ", "ㅂ", "ㅅ"],
  //     vec!["ㅇ", "ㅣ"],
  //     vec![" "],
  //     vec!["ㅂ", "ㅣ"],
  //     vec!["ㅆ", "ㅏ"],
  //     vec!["ㄷ", "ㅏ"],
  // ])
}

#[test]
fn test_is_hangul() {
  assert_eq!(true, DecomposedHangul::is_hangul('가'));
  assert_eq!(true, DecomposedHangul::is_hangul('홓'));
  assert_eq!(false, DecomposedHangul::is_hangul('A'));
  assert_eq!(false, DecomposedHangul::is_hangul('Z'));
}

#[test]
fn test_decomposed_consonant() {
  let result = DecomposedHangul::decompose_consonant(&'ㄳ');
  assert_eq!(result, "ㄱㅅ");
}

#[test]
fn test_decomposed_vowel() {
  let result = DecomposedHangul::decompose_vowel(&'ㅝ');
  assert_eq!(result, "ㅜㅓ");
}

#[test]
fn test_decompose_complete_character() {
  let result = DecomposedHangul::new('각');
  if let Some(decomposed_hangul) = result {
    assert_eq!(decomposed_hangul.choseong, "ᄀ".to_string()); // U+1100
    assert_eq!(decomposed_hangul.jungseong, "ᅡ".to_string()); // U+1161
    assert_eq!(decomposed_hangul.jongseong, Some('ᆨ'.to_string())); // U+11A8
  }
}
