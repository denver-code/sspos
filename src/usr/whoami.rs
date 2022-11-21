use crate::{sys};
use crate::api::console::Style;
use crate::api::process::ExitCode;
use alloc::string::{String, ToString};

pub fn main() -> Result<(), ExitCode> {
    let csi_color = Style::color("LightBlue");
    let csi_reset = Style::reset();
    let mut username = "guest".to_string();
    if let Some(_username) = sys::process::env("USER") {
        username = _username;
    }
    println!("{}{}{}", csi_color, username, csi_reset);
    Ok(())
}
