pub mod devices;
pub mod utils;

fn main() -> () {
  println!("Started");

  let builder = devices::razer_mamba_wired::RazerMambaWiredBuilder::new()
    .scroll_wheel_color([ 0x00, 0xFF, 0xFF ])
    .logo_color([ 0xFF, 0x00, 0x00 ])
    .write();

  println!("Finished");
}
