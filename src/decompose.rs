use crate::choseong::Choseong;
use crate::jongseong::Jongseong;
use crate::jungseong::Jungseong;
use crate::nfd::Nfd;

// pub struct Decompose;

// impl Decompose {
//   pub fn decompose(string: String) -> String {
//     let decomposed_hanguls = Self::decompose_to_groups(string);

//     decomposed_hanguls
//       .into_iter()
//       .fold("".to_string(), |hanguls, decomposed_hangul| {
//         return hanguls + &decomposed_hangul.join("");
//       })
//   }

//   pub fn decompose_to_groups(string: String) -> Vec<Vec<String>> {
//     string
//       .chars()
//       .map(|letter| {
//         let decomposed_hangul = DecomposedHangul::new(letter).unwrap();
//         let mut group = vec![
//           decomposed_hangul.choseong.to_string(),
//           decomposed_hangul.jungseong.to_string(),
//         ];
//         if let Some(jongseong) = decomposed_hangul.jongseong {
//           group.push(jongseong.to_string())
//         }
//         return group;
//       })
//       .collect()
//   }
// }

#[derive(Debug)]
pub struct DecomposedHangul {
  choseong: Choseong,
  jungseong: Jungseong,
  jongseong: Option<Jongseong>,
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
      choseong,
      jungseong,
      jongseong: if let Some(jongseong) = jongseong {
        Some(jongseong)
      } else {
        None
      },
    })
  }
}

#[cfg(test)]
mod tests {
  use super::DecomposedHangul;

  fn create_test_char_for_mac(test_char: char) -> u32 {
    let test_char = test_char as u32;
    if test_char > 10000 {
      test_char - 0x1FEE
    } else {
      test_char
    }
  }

  #[test]
  fn test_decomposed_hangul() {
    let result = DecomposedHangul::new('각');
    if let Some(decomposed_hangul) = result {}
  }
}
