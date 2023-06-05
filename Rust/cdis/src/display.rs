use ddc::Ddc;
use ddc_winapi::Monitor;
use std::usize;

static mut MONITORS: Option<Vec<Monitor>> = None;

pub fn init() -> Result<(), String> {
    let monitors = Monitor::enumerate().map_err(|_| "Could not enumerate displays.")?;
    if monitors.is_empty() {
        return Err(String::from("No display(s) found."));
    }
    unsafe {
        MONITORS = Some(monitors);
    }
    Ok(())
}

pub fn has_multiple_displays() -> Result<bool, String> {
    let monitors = get_monitors()?;
    let has_multiple_displays = monitors.len() > 1;
    Ok(has_multiple_displays)
}

pub fn get_displays_names() -> Result<Vec<String>, String> {
    let monitors = get_monitors()?;
    let descriptions = monitors
        .iter()
        .enumerate()
        .map(|(index, monitor)| format!("[{}] {}", index, monitor.description()))
        .collect::<Vec<_>>();
    Ok(descriptions)
}

pub fn get_display_name_by_id(display_id: u8) -> Result<String, String> {
    let monitor = get_monitor_by_id(display_id)?;
    let description = monitor.description();
    Ok(description)
}

pub fn display_supports_vcp60(display_id: u8) -> Result<bool, String> {
    let capabilities = get_display_capabilities(display_id)?;
    let is_supported = capabilities.contains("60(");
    Ok(is_supported)
}

pub fn set_vcp60_value(display_id: u8, value: u16) -> Result<(), String> {
    let mut monitor = get_monitor_by_id(display_id)?;
    monitor
        .set_vcp_feature(0x60, value)
        .map_err(|_| "Could not set DDC VCP 60 value.")?;
    Ok(())
}

pub fn get_vcp60_value(display_id: u8) -> Result<u16, String> {
    let monitor = get_monitor_by_id(display_id)?;
    let value = monitor
        .get_vcp_feature(0x06)
        .map_err(|_| "Could not get DDC VCP 60 value.")?;
    let vcp60_value = value.value();
    Ok(vcp60_value)
}

pub fn get_vcp60_value_as_string(display_id: u8) -> Result<String, String> {
    let vcp60_value = get_vcp60_value(display_id)?;
    let s = format!("{:02X}", vcp60_value);
    Ok(s)
}

fn get_monitors() -> Result<Vec<Monitor>, String> {
    match unsafe { MONITORS } {
        Some(monitors) => Ok(monitors),
        None => Err(String::from("Make sure you called the init() function.")),
    }
}

fn get_monitor_by_id(display_id: u8) -> Result<&'static mut Monitor, String> {
    let monitors = get_monitors()?;
    let index = usize::from(display_id);
    if index >= monitors.len() {
        return Err(String::from("Display with given ID not exists."));
    }
    let monitor = &mut monitors[index];
    Ok(monitor)
}

fn get_display_capabilities(display_id: u8) -> Result<String, String> {
    let monitor = get_monitor_by_id(display_id)?;
    let bytes = monitor
        .capabilities_string()
        .map_err(|_| "Could not determine the display´s DDC capabilities.")?;
    let capabilities = String::from_utf8(bytes)
        .map_err(|_| "Could not convert the displays´s DDC capabilities to string.")?;
    Ok(capabilities)
}
