use crate::api::fs;
use crate::api::process::ExitCode;

pub fn main(args: &[&str]) -> Result<(), ExitCode> {
    if args.len() != 2 {
        return Err(ExitCode::UsageError);
    }

    let pathname = args[1];

    let pathname = pathname.trim_end_matches('/');
    fs::create_dir(pathname);

    if fs::exists(pathname){
        // syscall::close(pathname);
        Ok(())
    } else {
        error!("Could not create '{}'", pathname);
        Err(ExitCode::Failure)
    }

}
