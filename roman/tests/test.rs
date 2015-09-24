  extern crate roman;
  use roman::*;

  #[test]
  fn i_plus_v_eq_vi() {
    assert_eq!(&add("I", "V"), "VI");
  }

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
