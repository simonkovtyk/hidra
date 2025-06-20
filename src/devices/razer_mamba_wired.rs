use std::{thread::sleep, time::Duration};

use crate::utils;

const VID: u16 = 0x1532;
const PID: u16 = 0x006c;
const WAIT_MIN_MICRO: u16 = 31000;
const WAIT_MAX_MICRO: u16 = 31100;

/* Color-related */
const COLOR_START_SEQUENCE: [u8; 10] = [
  0x00, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x09, 0x0f, 0x02, 0x01
];
const COLOR_MID_SEQUENCE: [u8; 4] = [
  0x01, 0x00, 0x00, 0x01
];
const SCROLL_WHEEL_IDENTIFIER: u8 = 0x1;
const LOGO_IDENTIFIER: u8 = 0x4;
const RIGHT_IDENTIFIER: u8 = 0x10;


fn get_sequence (identifier: u8, color: [u8; 3]) -> Vec<u8> {
  let mut data = Vec::new();

  data.append(
    &mut COLOR_START_SEQUENCE.to_vec()
  );
  data.push(
    identifier
  );
  data.append(
    &mut COLOR_MID_SEQUENCE.to_vec()
  );
  data.append(
    &mut color.to_vec()
  );

  return data;
}


pub struct RazerMambaWiredBuilder {
  sequences: Option<Vec<Vec<u8>>>
}

impl RazerMambaWiredBuilder {
  pub fn new () -> Self {
    return Self {
      sequences: None
    }
  }

  fn add_sequence (&mut self, value: Vec<u8>) -> () {
    if self.sequences.is_some() {
      self.sequences.as_mut().unwrap().push(
        value
      );

      return;
    }

    let mut sequences = Vec::new();

    sequences.push(
      value
    );

    self.sequences = Some(sequences);
  }

  pub fn scroll_wheel_color (&mut self, color: [u8; 3]) -> &mut Self {
    let mut value = get_sequence(SCROLL_WHEEL_IDENTIFIER, color);

    value.append(
      &mut vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfb,
        0x00
      ]
    );

    self.add_sequence(value);

    return self;
  }

  pub fn logo_color (&mut self, color: [u8; 3]) -> &mut Self {
    let mut value = get_sequence(LOGO_IDENTIFIER, color);

    value.append(
      &mut vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfe,
        0x00
      ]
    );

    self.add_sequence(value);

    return self;
  }

  pub fn write (&mut self) -> () {
    let api = hidapi::HidApi::new().expect("Could not create new HID-API");

    let device = api.open(VID, PID).expect("Device not found");

    println!("Connected to {}", device.get_product_string().unwrap().unwrap());

    for sequence in self.sequences.as_mut().expect("No sequences defined").into_iter() {
      device.send_feature_report(&sequence).expect("Could not write data");

      let duration = utils::time::rand_u16_range(WAIT_MIN_MICRO, WAIT_MAX_MICRO);

      sleep(Duration::from_micros(duration as u64));
    }
  }
}
