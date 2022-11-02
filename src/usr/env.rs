use crate::api::process::ExitCode;
use crate::sys;

pub fn main(args: &[&str]) -> Result<(), ExitCode> {
    match args.len() {
        1 => {
            for (key, val) in sys::process::envs() {
                println!("{:10} \"{}\"", key, val);
            }
            Ok(())
        }
        2 => {
            let key = args[1];
            if let Some(val) = sys::process::env(key) {
                println!("{}", val);
                Ok(())
            } else {
                error!("Could not get '{}'", key);
                Err(ExitCode::Failure)
            }
        }
        3 => {
            sys::process::set_env(args[1], args[2]);
            Ok(())
        }
        _ => {
            error!("Invalid number of arguments");
            Err(ExitCode::UsageError)
        }
    }
}
