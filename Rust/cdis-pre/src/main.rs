use ddc::Ddc;
use ddc_winapi::Monitor;
use std::{env, process::ExitCode};

mod print;

pub const APP_TITLE: &str = "ChangeDisplayInputSource (Beta)";
pub const RELEASE_DATE: &str = "05/2023";

fn main() -> ExitCode {
    print::show_title();
    let params = env::args().collect::<Vec<String>>();
    let params_count = env::args().count();
    if params_count == 3 && params[1] == "/set" {
        let mut monitors = Monitor::enumerate().unwrap();
        if !monitors.is_empty() {
            let code_string = &params[2];
            let code = code_string.parse::<u16>().unwrap();
            println!("Display input source should change now to {:02X}", code);
            monitors[0].set_vcp_feature(0x60, code).unwrap();
            println!();
            println!("Success.");
            return ExitCode::SUCCESS;
        }
    }
    println!("Error.");
    return ExitCode::FAILURE;
}
