use crate::api::console::Style;
use crate::api::process::ExitCode;

pub fn main() -> Result<(), ExitCode> {
    let csi_color = Style::color("LightBlue");
    let csi_reset = Style::reset();
    println!("{}Pong!{}", csi_color, csi_reset);
    Ok(())
}
