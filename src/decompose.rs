use crate::{choseong::Choseong, jongseong::Jongseong, jungseong::Jungseong};

pub struct Nfd(pub u32, pub u32, pub Option<u32>);

impl Nfd {
  const HANGUL_BASE: u32 = 0xAC00;
  const HANGUL_LAST: u32 = 0xD7A3;

  const CHOSEONG_COUNT: u32 = 0x13;
  const JUNGSEONG_COUNT: u32 = 0x15;
  const JONGSEONG_COUNT: u32 = 0x1C;

  const CHOSEONG_BASE: u32 = 0x1100;
  const JUNGSEONG_BASE: u32 = 0x1161;
  const JONGSEONG_BASE: u32 = 0x11A8;

  pub fn normalize_from_u32(letter_code: u32) -> Option<Self> {
    if !Self::is_complete_hangul_from_u32(letter_code) {
      return None;
    }

    let hangul_code = letter_code - Self::HANGUL_BASE;

    let choseong_index = hangul_code / (Self::JUNGSEONG_COUNT + Self::JONGSEONG_COUNT);
    let jungseong_index =
      (hangul_code % (Self::JUNGSEONG_COUNT + Self::JONGSEONG_COUNT)) / Self::JONGSEONG_COUNT;
    let jongseong_index = hangul_code % Self::JONGSEONG_COUNT;

    let choseong = Self::CHOSEONG_BASE + choseong_index;
    let jungseong = Self::JUNGSEONG_BASE + jungseong_index;
    let jongseong = if jongseong_index > 0 {
      Some(Self::JONGSEONG_BASE + jongseong_index - 1)
    } else {
      None
    };

    Some(Self(choseong, jungseong, jongseong))
  }

  fn is_complete_hangul_from_u32(letter_code: u32) -> bool {
    Self::HANGUL_BASE <= letter_code && letter_code <= Self::HANGUL_LAST
  }
}

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
    let letter = letter as u32;
    let Nfd(choseong_code, jungseong_code, jongseong_code) =
      Nfd::normalize_from_u32(letter).unwrap();
    let choseong = Choseong::new_from_u32(choseong_code).unwrap();
    let jungseong = Jungseong::new_from_u32(jungseong_code).unwrap();
    let jongseong = Jongseong::new_from_u32(jongseong_code.unwrap());

    Some(Self {
      choseong: choseong.decomposed_string,
      jungseong: jungseong.decomposed_string,
      jongseong: if let Some(jongseong) = jongseong {
        Some(jongseong.decomposed_string)
      } else {
        None
      },
    })
  }
}

#[test]
fn test_decomposed_hangul() {
  let result = DecomposedHangul::new('각');
  if let Some(decomposed_hangul) = result {
    assert_eq!(decomposed_hangul.choseong, "ㄱ".to_string());
    assert_eq!(decomposed_hangul.jungseong, "ㅏ".to_string());
    assert_eq!(decomposed_hangul.jongseong, Some("ㄱ".to_string()));
  }
}
