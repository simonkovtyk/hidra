use std::{thread::sleep, time::Duration};

use crate::{devices::razer::utils::{calculate_checksum_from_payload}, utils};

const VID: u16 = 0x1532;
const PID: u16 = 0x006c;
const WAIT_MIN_MICRO: u16 = 31000;
const WAIT_MAX_MICRO: u16 = 31100;

enum MambaWiredLed {
  ScrolWheel = 0x1,
  Logo = 0x4,
  Right = 0x10,
  Left = 0x11
}

pub struct MambaWiredBuilder {
  sequences: Option<Vec<Vec<u8>>>
}

impl MambaWiredBuilder {
  pub fn new () -> Self {
    return Self {
      sequences: None
    }
  }

  fn prepare_payload (mamba_wired_led: MambaWiredLed, color: &[u8; 3]) -> Vec<u8> {
    let mut data = Vec::<u8>::new();

    data.append(&mut vec![0x00, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x09, 0x0f, 0x02, 0x01]);
    data.push(mamba_wired_led as u8);
    data.append(&mut vec![ 0x01, 0x00, 0x00, 0x01 ]);
    data.append(&mut color.to_vec());
    data.append(
      &mut vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
      ]
    );
    data.push(
      calculate_checksum_from_payload(&data)
    );
    data.push(0x00);

    return data;
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
    let value = MambaWiredBuilder::prepare_payload(MambaWiredLed::ScrolWheel, &color);
    self.add_sequence(value);

    return self;
  }

  pub fn logo_color (&mut self, color: [u8; 3]) -> &mut Self {
    let value = MambaWiredBuilder::prepare_payload(MambaWiredLed::Logo, &color);
    self.add_sequence(value);

    return self;
  }

  pub fn right_color (&mut self, color: [u8; 3]) -> &mut Self {
    let value = MambaWiredBuilder::prepare_payload(MambaWiredLed::Right, &color);
    self.add_sequence(value);

    return self;
  }

  pub fn left_color (&mut self, color: [u8; 3]) -> &mut Self {
    let value = MambaWiredBuilder::prepare_payload(MambaWiredLed::Left, &color);
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
