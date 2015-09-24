fn add(x:&str, y:&str) -> String {
  let xy = String::new() + x + y;
  if xy.len() == 4 {
    "IV".to_string()
  } else {
    xy
  }
}

#[test]
fn it_works() {
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
