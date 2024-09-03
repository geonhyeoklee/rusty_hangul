const HANGUL_BASE: u32 = 0xAC00;
const HANGUL_LAST: u32 = 0xD7A3;

const CHOSEONG_COUNT: u32 = 0x13;
const JUNGSEONG_COUNT: u32 = 0x15;
const JONGSEONG_COUNT: u32 = 0x1C;

const CHOSEONG_BASE: u32 = 0x1100;
const JUNGSEONG_BASE: u32 = 0x1161;
const JONGSEONG_BASE: u32 = 0x11A8;

const JUNGSEONG_AND_JONGSEONG_NUMBER_OF_CASES: u32 = JUNGSEONG_COUNT * JONGSEONG_COUNT;

pub struct Nfd(pub u32, pub u32, pub Option<u32>);

impl Nfd {
  pub fn normalize_from_u32(letter_code: u32) -> Option<Self> {
    if !Self::is_complete_hangul_from_u32(letter_code) {
      return None;
    }

    let hangul_code = letter_code - HANGUL_BASE;

    let choseong_index = hangul_code / (JUNGSEONG_AND_JONGSEONG_NUMBER_OF_CASES);
    let jungseong_index = (hangul_code % JUNGSEONG_AND_JONGSEONG_NUMBER_OF_CASES) / JONGSEONG_COUNT;
    let jongseong_index = hangul_code % JONGSEONG_COUNT;

    let choseong = CHOSEONG_BASE + choseong_index;
    let jungseong = JUNGSEONG_BASE + jungseong_index;
    let jongseong = if jongseong_index > 0 {
      Some(JONGSEONG_BASE + jongseong_index - 1)
    } else {
      None
    };

    Some(Self(choseong, jungseong, jongseong))
  }

  fn is_complete_hangul_from_u32(letter_code: u32) -> bool {
    HANGUL_BASE <= letter_code && letter_code <= HANGUL_LAST
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_normalize_from_u32() {
    let test_cases = ['릴'];
    let Nfd(choseong_code, jungseong_code, jongseong_code) =
      Nfd::normalize_from_u32(test_cases[0] as u32).unwrap();

    assert_eq!(choseong_code, 4357);
    assert_eq!(jungseong_code, 4469);
    assert_eq!(jongseong_code, Some(4527));
  }

  #[test]
  fn test_is_complete_hangul_from_u32() {
    let test_cases = ['릴', 'H', 'ㅎ'];
    assert_eq!(Nfd::is_complete_hangul_from_u32(test_cases[0] as u32), true);
    assert_eq!(
      Nfd::is_complete_hangul_from_u32(test_cases[1] as u32),
      false
    );
    assert_eq!(
      Nfd::is_complete_hangul_from_u32(test_cases[2] as u32),
      false
    );
  }
}
