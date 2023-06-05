use std::process::ExitCode;

mod display;
mod print;

pub const APP_TITLE: &str = "ChangeDisplayInputSource";
pub const RELEASE_DATE: &str = "05/2023";

fn run() -> Result<bool, String> {
    display::init()?;
    let display_names = display::get_displays_names()?;
    for display_name in display_names {
        println!("{}", display_name);
    }
    Ok(false)
}

fn main() -> ExitCode {
    print::show_title();
    // I know i could also return Result<> from main() function, since version 1.26 of Rust.
    // But this will be more clear to me in the future, since i just code in Rust at times.
    match run() {
        Ok(b) => {
            if b {
                print!("Have a nice day.");
            }
            ExitCode::SUCCESS
        }
        Err(s) => {
            print!("Error: {}", s);
            ExitCode::FAILURE
        }
    }
}

/*



// display::get_detected_displays(&monitors)?




// if !display::print_detected_displays(&monitors) {
//     return ExitCode::FAILURE;
// }

// if display::is_one_display_only(monitors)
// {

// }

// return ExitCode::SUCCESS;

// let params = env::args().collect::<Vec<String>>();
// let params_count = env::args().count();
// if params_count == 1 {
//     //print::show_usage();
//     //return ExitCode::SUCCESS;
// }

// let prim = &mut monitors[0];
// //prim.set_vcp_feature(0, 0);

// match display::set_vcp_60_code(prim, 0x03) {
//     Ok(_) => println!("hurra"),
//     Err(e) => eprintln!("{:?}", e),
// }
// return ExitCode::SUCCESS;
//









let prim = &mut monitors[0];
if monitors.is_empty() {
    println!("Error: Could not found any connected displays.");
    return ExitCode::FAILURE;
}
// Scan
if params_count == 2 && params[1] == "/scan" {
    println!("Count of detected displays:       {}", monitors.len());

    // Todo: if 1 sonst error
    println!("Primary display is:               Detected display #1");

    // Description
    let desc = monitors[0].description();
    println!("Display description:              {}", desc);
    // Capabilities
    let caps_raw = monitors[0]
        .capabilities_string()
        .expect("Error: Could not determine DDC capabilities.");
    let caps = String::from_utf8(caps_raw)
        .expect("Error: Could not convert determined capabilities to string.");
    //println!("Display capabilities:");
    //println!("{}", caps);
    //println!();
    // VCP 60 support
    print!("Display has VCP 60 support:       ");
    if !caps.contains("60(") {
        println!("No");
        println!();
        println!("Error: Display not supports VCP 60 feature.");
        return ExitCode::FAILURE;
    }
    println!("Yes");
    // VCP 60 supported values
    print!("Supported VCP 60 feature codes:   ");
    let parts1 = caps.split("60(").collect::<Vec<&str>>();
    let parts1_last = parts1.last().unwrap();
    let parts2 = parts1_last.split(")").collect::<Vec<&str>>();
    let parts2_first = parts2.first().unwrap();
    println!("{}", parts2_first);
    // VCP 60 active value

    let code = vcp60::get_vcp_60_code(prim);
    print!("Active VCP 60 feature code:       {}", code);

    // let vcp_value = monitors[0]
    //     .get_vcp_feature(0x60)
    //     .expect("Error: Could not determine VCP 60 feature.");
    // let code = vcp_value.value();
    // println!("{:02X}", code);
    return ExitCode::SUCCESS;
}
// Get
if params_count == 2 && params[1] == "/get" {
    let value = monitors[0].get_vcp_feature(0x60).unwrap();
    let code = value.value();
    println!("Actual active VCP 60 feature code is: {:02X}", code);
    return ExitCode::SUCCESS;
}
// Set
if params_count > 2 && params[1] == "/set" {
    if params_count < 3 {
        println!("Error: Missing code value of /set parameter.");
        println!();
        print::show_usage_error();
        return ExitCode::FAILURE;
    }
    if params_count == 3 {
        let code_string = &params[2];
        let code = code_string.parse::<u16>().unwrap();
        monitors[0].set_vcp_feature(0x60, code).unwrap();
        return ExitCode::SUCCESS;
    }
}
println!("Error: Incorrect parameters.");
print::show_usage();
return ExitCode::FAILURE;
 */
