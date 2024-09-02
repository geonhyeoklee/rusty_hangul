pub struct Jongseong {
  pub value: char,
  pub code: u32,
  pub decomposed: Vec<u32>,
  pub decomposed_string: String,
}

impl Jongseong {
  pub fn new_from_u32(code: u32) -> Option<Self> {
    if Self::is_jongseong_from_u32(code) {
      return None;
    }

    let value = unsafe { std::char::from_u32_unchecked(code) };
    let decomposed = Self::decompose_jongseong_from_u32(&code);

    Some(Self {
      value,
      code,
      decomposed: decomposed.clone(),
      decomposed_string: decomposed
        .into_iter()
        .map(|code| {
          let character = char::from_u32(code).unwrap();
          character.to_string()
        })
        .collect::<Vec<String>>()
        .join(""),
    })
  }

  fn decompose_jongseong_from_u32(jongseong_code: &u32) -> Vec<u32> {
    match jongseong_code {
      0x20 => vec![0x20],             // ' '
      0x11A8 => vec![0x11A8],         //"ㄱ",
      0x11A9 => vec![0x11A9],         //"ㄲ",
      0x11AA => vec![0x11A8, 0x11BA], // "ㄱㅅ",
      0x11AB => vec![0x11AB],         // "ㄴ",
      0x11AC => vec![0x11AB, 0x11BD], // "ㄴㅈ",
      0x11AD => vec![0x11AB, 0x11C2], // "ㄴㅎ",
      0x11AE => vec![0x11AE],         //"ㄷ",
      0x11AF => vec![0x11AF],         // "ㄹ",
      0x11B0 => vec![0x11AF, 0x11A8], // "ㄹㄱ",
      0x11B1 => vec![0x11AF, 0x11B7], // "ㄹㅁ",
      0x11B2 => vec![0x11AF, 0x11B8], // "ㄹㅂ",
      0x11B3 => vec![0x11AF, 0x11BA], // "ㄹㅅ",
      0x11B4 => vec![0x11AF, 0x11C0], // "ㄹㅌ",
      0x11B5 => vec![0x11AF, 0x11C1], // "ㄹㅍ",
      0x11B6 => vec![0x11AF, 0x11C2], // "ㄹㅎ",
      0x11B7 => vec![0x11B7],         // "ㅁ",
      0x11B8 => vec![0x11B8],         // "ㅂ",
      0x11B9 => vec![0x11B8, 0x11BA], // "ㅂㅅ",
      0x11BA => vec![0x11BA],         // "ㅅ",
      0x11BB => vec![0x11BB],         // "ㅆ",
      0x11BC => vec![0x11BC],         // "ㅇ",
      0x11BD => vec![0x11BD],         // "ㅈ",
      0x11BE => vec![0x11BE],         // "ㅊ",
      0x11BF => vec![0x11BF],         // "ㅋ",
      0x11C0 => vec![0x11C0],         // "ㅌ",
      0x11C1 => vec![0x11C1],         // "ㅍ",
      0x11C2 => vec![0x11C2],         // "ㅎ",
      _ => unreachable!(),
    }
  }

  fn is_jongseong_from_u32(jongseong_code: u32) -> bool {
    const JONGSEONG_BASE: u32 = 0x11A8;
    const JONGSEONG_LAST: u32 = 0x11C2;
    JONGSEONG_BASE <= jongseong_code && jongseong_code <= JONGSEONG_LAST
  }
}