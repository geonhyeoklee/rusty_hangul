pub struct Jungseong {
  pub value: char,
  pub code: u32,
  pub decomposed: Vec<u32>,
  pub decomposed_string: String,
}

impl Jungseong {
  pub fn new_from_u32(code: u32) -> Option<Self> {
    if !Self::is_jungseong_from_u32(code) {
      return None;
    }

    let value = unsafe { std::char::from_u32_unchecked(code) };
    let decomposed = Self::decompose_jungseong_from_u32(&code);

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

  fn decompose_jungseong_from_u32(jungseong_code: &u32) -> Vec<u32> {
    match jungseong_code {
      0x1161 => vec![0x1161],         // "ㅏ"
      0x1162 => vec![0x1162],         // "ㅐ"
      0x1163 => vec![0x1163],         // "ㅑ"
      0x1164 => vec![0x1164],         // "ㅒ"
      0x1165 => vec![0x1165],         // "ㅓ"
      0x1166 => vec![0x1166],         // "ㅔ"
      0x1167 => vec![0x1167],         // "ㅕ"
      0x1168 => vec![0x1168],         // "ㅖ"
      0x1169 => vec![0x1169],         // "ㅗ"
      0x116A => vec![0x1169, 0x1161], // "ㅗㅏ"
      0x116B => vec![0x1169, 0x1162], // "ㅗㅐ"
      0x116C => vec![0x1169, 0x1175], // "ㅗㅣ"
      0x116D => vec![0x116D],         // "ㅛ",
      0x116E => vec![0x116E],         // "ㅜ",
      0x116F => vec![0x116E, 0x1165], // "ㅜㅓ",
      0x1170 => vec![0x116E, 0x1166], // "ㅜㅔ",
      0x1171 => vec![0x116E, 0x1175], // "ㅜㅣ",
      0x1172 => vec![0x1172],         // "ㅠ",
      0x1173 => vec![0x1173],         // "ㅡ",
      0x1174 => vec![0x1173, 0x1175], // "ㅡㅣ",
      0x1175 => vec![0x1175],         // "ㅣ",
      _ => unreachable!(),
    }
  }

  fn is_jungseong_from_u32(jungseong_code: u32) -> bool {
    const JUNGSEONG_BASE: u32 = 0x1161;
    const JUNGSEONG_LAST: u32 = 0x1175;
    JUNGSEONG_BASE <= jungseong_code && jungseong_code <= JUNGSEONG_LAST
  }
}

#[test]
fn test_jungseong() {
  use crate::nfd::Nfd;

  let letter = '궐';
  let Nfd(_, jung, _) = Nfd::normalize_from_u32(letter as u32).unwrap();
  let jungseong = Jungseong::new_from_u32(jung).unwrap();

  assert_eq!(
    jungseong
      .decomposed_string
      .chars()
      .map(|c| c as u32)
      .collect::<Vec<u32>>(),
    "ㅜㅓ"
      .chars()
      // Mac OS에서 글자 하나의 자음마다 0x1FEE 값 만큼 더해지는 버그가 있다.
      .map(|c| if Jungseong::is_jungseong_from_u32(c as u32) {
        c as u32
      } else {
        c as u32 - 0x1FEE
      })
      .collect::<Vec<u32>>()
  )
}
