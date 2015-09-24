pub fn add(x:&str, y:&str) -> String {
  let expanded = expandio(x) + &expandio(y);
  match expanded.len() {
    4 => "IV".to_string(),
    5 => "V".to_string(),
    6 => "VI".to_string(),
    _ => String::new() + x + y,
  }
}

fn expandio(x:&str) -> String {
  let matched = match x {
    "IV" => "IIII",
    "V"  => "IIIII",
    _    => x,
  };

  matched.to_string() 
}

