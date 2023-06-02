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
    println!("This is a pre-release version for testing and the features are not documented.");
    println!();
}
