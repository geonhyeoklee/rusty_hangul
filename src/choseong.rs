#[derive(Debug)]
pub struct Choseong {
  pub value: char,
  pub code: u32,
  pub decomposed: Vec<u32>,
  pub decomposed_string: String,
}

impl Choseong {
  pub fn new_from_u32(code: u32) -> Option<Self> {
    if !Self::is_choseong_from_u32(code) {
      return None;
    }

    let value = unsafe { std::char::from_u32_unchecked(code) };

    Some(Self {
      value,
      code,
      decomposed: vec![code],
      decomposed_string: value.to_string(),
    })
  }

  fn is_choseong_from_u32(choseong_code: u32) -> bool {
    const CHOSEONG_BASE: u32 = 0x1100;
    const CHOSEONG_LAST: u32 = 0x1112;
    CHOSEONG_BASE <= choseong_code && choseong_code <= CHOSEONG_LAST
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_choseong() {
    use crate::nfd::Nfd;

    let letter = '궑';
    let Nfd(choseong_code, _, _) = Nfd::normalize_from_u32(letter as u32).unwrap();
    let choseong = Choseong::new_from_u32(choseong_code).unwrap();

    assert_eq!(choseong.code, 0x1100);
    assert_eq!(
      choseong
        .decomposed_string
        .chars()
        .map(|c| c as u32)
        .collect::<Vec<u32>>(),
      vec![0x1100]
    )
  }
}
