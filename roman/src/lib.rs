fn add(x:&str, y:&str) -> String {
  String::new() + x + y 
}

#[test]
fn it_works() {
  let foo = add("I", "I");
  assert_eq!(&foo, "II");
}
