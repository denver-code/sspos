use crate::api::console::Style;
use crate::api::process::ExitCode;

pub fn main() -> Result<(), ExitCode> {
    let csi_color = Style::color("LightBlue");
    let csi_reset = Style::reset();
    println!("{}sspos v{}{}", csi_color, env!("CARGO_PKG_VERSION"), csi_reset);
    Ok(())
}
