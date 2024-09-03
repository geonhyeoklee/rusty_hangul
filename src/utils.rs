pub fn is_complete_hangul_from_u32(letter_unicode: u32) -> bool {
  const HANGUL_BASE: u32 = 0xAC00;
  const HANGUL_LAST: u32 = 0xD7A3;
  HANGUL_BASE <= letter_unicode && letter_unicode <= HANGUL_LAST
}

pub fn is_choseong_from_u32(choseong_code: u32) -> bool {
  const CHOSEONG_BASE: u32 = 0x1100;
  const CHOSEONG_LAST: u32 = 0x1112;
  CHOSEONG_BASE <= choseong_code && choseong_code <= CHOSEONG_LAST
}

pub fn is_jungseong_from_u32(jungseong_code: u32) -> bool {
  const JUNGSEONG_BASE: u32 = 0x1161;
  const JUNGSEONG_LAST: u32 = 0x1175;
  JUNGSEONG_BASE <= jungseong_code && jungseong_code <= JUNGSEONG_LAST
}
