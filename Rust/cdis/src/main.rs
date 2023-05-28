use std::{
    env::args,
    process::{Command, ExitCode},
};

// No need for some code here, to verify name and version from cargo.toml file,
// since cargo will show an error, if name or version contains an empty string.
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RELEASE_DATE: &str = "2023-05-28";

fn main() -> ExitCode {
    println!();
    println!("{} {} (by MBODM {})", APP_NAME, APP_VERSION, RELEASE_DATE);
    println!();
    println!("A tiny Windows command line tool, pinging Telekom´s main DNS server.");
    println!();
    if args().len() > 1 {
        println!("Error: Arguments are not supported.");
        println!();
        println!("Please restart the executable without any command line arguments.");
        return ExitCode::FAILURE;
    }
    // No validation here if cmd, start and ping really exist.
    // If not, you have a more serious problem than this, imo.
    // But i tested those scenarios and error-behavior was ok.
    match Command::new("cmd")
        .args("/c start ping -t 194.25.2.129".split_whitespace())
        .status()
    {
        Ok(exit_status) => match exit_status.success() {
            true => {
                println!(" - Successfully started ping command (ping -t 194.25.2.129) in a separate window.");
                println!(" - The ping command is running until you press CTRL+C there or close that window.");
                println!();
                println!("Take a look at https://github.com/mbodm/pingalive for more information.");
                println!();
                println!("Have a nice day.");
                ExitCode::SUCCESS
            }
            false => {
                // No evaluation of failure exit code here, since the tool has some weird exit code behaviour.
                println!("Error: Exit code of ping´s environment was not SUCCESS.");
                ExitCode::FAILURE
            }
        },
        Err(error) => {
            println!("Error: Starting ping´s environment failed ({}).", error);
            ExitCode::FAILURE
        }
    }
}
