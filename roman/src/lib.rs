fn add(x:String, y:String) -> String {
  x + &y
}

#[test]
fn it_works() {
  let foo = add("I".to_string(), "I".to_string());
  assert_eq!(&foo, "II");
}
