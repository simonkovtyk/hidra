pub mod devices;
pub mod utils;

fn main() -> () {
  let color = [ 0xFF, 0x00, 0x00 ];

  println!("Started");

  let builder = devices::razer_mamba_wired::RazerMambaWiredBuilder::new()
    .scroll_wheel_color(color)
    .logo_color(color)
    .write();

  println!("Finished");
}
