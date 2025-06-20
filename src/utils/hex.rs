pub fn parse_hex_code (value: &str) -> [u8; 3] {
  let red_str = &value[0..1];
  let green_str = &value[1..2];
  let blue_str = &value[2..3];

  let red = u8::from_str_radix(&red_str, 16).expect("Could not parse red");
  let green = u8::from_str_radix(&green_str, 16).expect("Could not parse green");
  let blue = u8::from_str_radix(&blue_str, 16).expect("Could not parse blue");

  return [
    red,
    green,
    blue
  ];
}
