use crate::api::console::Style;
use crate::api::fs;
use crate::api::process::ExitCode;

use crate::usr;
use object::{Object, ObjectSection};

pub fn main(args: &[&str]) -> Result<(), ExitCode> {
    if args.len() != 2 {
        return Err(ExitCode::UsageError);
    }

    let color = Style::color("Yellow");
    let reset = Style::reset();

    let pathname = args[1];
    if let Ok(buf) = fs::read_to_bytes(pathname) {
        let bin = buf.as_slice();
        if let Ok(obj) = object::File::parse(bin) {
            println!("ELF entry address: {:#x}", obj.entry());
            for section in obj.sections() {
                if let Ok(name) = section.name() {
                    if name.is_empty() {
                        continue;
                    }
                    let addr = section.address() as usize;
                    let size = section.size();
                    let align = section.align();
                    println!();
                    println!("{}{}{} (addr: {:#x}, size: {}, align: {})", color, name, reset, addr, size, align);
                    if let Ok(data) = section.data() {
                        usr::hex::print_hex(data);
                    }
                }
            }
            Ok(())
        } else {
            println!("Could not parse ELF");
            Err(ExitCode::Failure)
        }
    } else {
        println!("File not found '{}'", pathname);
        Err(ExitCode::Failure)
    }
}
