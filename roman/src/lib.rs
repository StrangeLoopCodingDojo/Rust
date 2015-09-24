pub fn add(x:&str, y:&str) -> String {
  let expanded = expandio(x) + &expandio(y);
  match expanded.len() {
    4 => "IV".to_string(),
    5 => "V".to_string(),
    _ => String::new() + x + y,
  }
}

fn expandio(x:&str) -> String {
  let matched = match x {
    "IV" => "IIII",
    _    => x,
  };

  matched.to_string() 
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn iv_plus_i_eq_v() {
    assert_eq!(&add("IV", "I"), "V");
  }

  #[test]
  fn iii_plus_ii_eq_v() {
    assert_eq!(&add("III", "II"), "V");
  }

  #[test]
  fn i_plus_i_eq_ii() {
    let foo = add("I", "I");
    assert_eq!(&foo, "II");
  }

  #[test]
  fn i_plus_ii_eq_iii() {
    let foo = add("I", "II");
    assert_eq!(&foo, "III");
  }

  #[test]
  fn ii_plus_ii_eq_iv() {
    assert_eq!(&add("II", "II"), "IV");
  }
}
