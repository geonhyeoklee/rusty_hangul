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
