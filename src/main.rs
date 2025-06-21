pub mod devices;
pub mod utils;

fn main() -> () {
  let color = [ 0x73, 0x00, 0xFF ];

  println!("Started");

  let builder = devices::razer::mamba_wired::MambaWiredBuilder::new()
    .scroll_wheel_color(color)
    .logo_color(color)
    .right_color(color)
    .left_color(color)
    .write();

  println!("Finished");
}
