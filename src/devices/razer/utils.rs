pub fn calculate_checksum_from_payload (bytes: &[u8]) -> u8 {
  let byte_offset = &bytes[3..88];
  let mut checksum: u8 = 0;
  
  for byte in byte_offset {
    checksum ^= byte;
  }

  return checksum;
}

pub fn extended_static_matrix_effect () -> () {

}
