// No need for some code here, to verify name and version from cargo.toml file,
// since cargo will show an error, if name or version contains an empty string.

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn show_title() {
    println!();
    println!(
        "{} {} ({}.exe - by MBODM {})",
        crate::APP_TITLE,
        APP_VERSION,
        APP_NAME,
        crate::RELEASE_DATE
    );
    println!();
    println!("A tiny Windows command line tool to change a display´s active input source (by using a DDC command)");
    println!();
}

pub fn show_usage() {
    println!("Usage: cdis.exe [/scan] [/set <code>] [/get]");
    println!();
    println!("Notes:");
    println!("- This tool solely deals with the DDC VCP 60 feature (used to control a display´s active input source)");
    println!("- Use /scan to test if your display supports the VCP 60 feature");
    println!("- All the available VCP 60 feature codes of your display (when supported) will be listed there too");
    println!("- Every listed code corresponds to one input source of your display (like HDMI, DP, DVI and so on)");
    println!("- Use /set (followed by one of the listed codes) to change your display´s active input source");
    println!("- Use /get to show the code of your display´s active input source");
    println!("- The VCP 60 feature codes are hexadecimal numbers (without the 0x prefix)");
    println!();
    println!("Parameters:");
    println!("  /scan        Scans the connected primary display and shows DDC VCP 60 infos");
    println!("  /set <code>  Sets the active VCP 60 feature code (active display input source)");
    println!("  /get         Gets the active VCP 60 feature code (active display input source)");
    println!();
    println!("Examples:");
    println!("  cdis.exe /scan");
    println!("  cdis.exe /set 0A");
    println!("  cdis.exe /get");
}

pub fn show_usage_error() {
    println!("Usage: cdis.exe [/scan] [/set <code>] [/get]");
    println!();
    println!("Please try again.");
}
