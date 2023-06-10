use display::DisplayControl;
use std::process::ExitCode;

mod display;
mod print;

pub const APP_TITLE: &str = "ChangeDisplayInputSource";
pub const RELEASE_DATE: &str = "05/2023";

fn run() -> Result<bool, String> {
    let mut dc = DisplayControl::new();
    dc.init()?;
    println!("Detected displays:");
    let list = dc.get_display_list()?;
    for l in list {
        println!("  {}", l);
    }
    let multi = dc.has_multiple_displays()?;
    println!(
        "Has multiple displays: {}",
        if multi { "Yes" } else { "No" }
    );
    let exists = dc.display_id_exists(0)?;
    println!(
        "Display with ID=0 exists: {}",
        if exists { "Yes" } else { "No" }
    );
    let desc = dc.get_display_description(0)?;
    println!("Description of display with ID=0: {}", desc);
    println!("Capabilities of display with ID=0:");
    let caps = dc.get_display_capabilities(0)?;
    println!("  {}", caps);
    let supports = dc.display_supports_vcp60(0)?;
    println!(
        "Display with ID=0 has VCP 60 support: {}",
        if supports { "Yes" } else { "No" }
    );
    let value = dc.get_vcp60_value(0)?;
    println!("VCP 60 value of display with ID=0: {:02X}", value);
    Ok(false)
}

fn main() -> ExitCode {
    print::show_title();
    // I know i could also return Result<> from main() function, since version 1.26 of Rust.
    // But this will be more clear to me in the future, since i code in Rust just at times.
    match run() {
        Ok(b) => {
            if b {
                println!("Have a nice day.");
            }
            ExitCode::SUCCESS
        }
        Err(s) => {
            println!("Error: {}", s);
            ExitCode::FAILURE
        }
    }
}
