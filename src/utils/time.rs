use rand::Rng;

pub fn rand_u16_range (min: u16, max: u16) -> u16 {
  return rand::rng()
    .random_range(min..max);
}
