use ddc::Ddc;
use ddc_winapi::Monitor;

pub fn get_names_of_detected_displays(monitors: &[Monitor]) -> Result<Vec<String>, String> {
    if monitors.is_empty() {
        Err(String::from("No display(s) connected."))
    } else {
        let strings = monitors
            .iter()
            .enumerate()
            .map(|(index, mon)| format!("[{}] {}", index, mon.description()))
            .collect::<Vec<_>>();
        Ok(strings)
    }
}

pub fn is_one_display_only(monitors: Vec<Monitor>) -> bool {
    monitors.len() == 1
}

pub fn get_display_name(monitor: &mut Monitor) -> String {
    monitor.description()
}

pub fn supports_vcp60_code(monitor: &mut Monitor) -> Result<bool, String> {
    let caps_raw = monitor
        .capabilities_string()
        .map_err(|_| "Could not determine DDC capabilities.")?;
    let caps = String::from_utf8(caps_raw)
        .map_err(|_| "Could not convert determined capabilities to string.")?;
    let has60 = caps.contains("60(");
    Ok(has60)
}

pub fn set_vcp60_value(monitor: &mut Monitor, value: u16) -> Result<(), String> {
    monitor
        .set_vcp_feature(0x60, value)
        .map_err(|_| "Could not set DDC VCP 60 value.")?;
    Ok(())
}

pub fn get_vcp60_value(monitor: &mut Monitor) -> Result<String, String> {
    let vcp_value = monitor
        .get_vcp_feature(0x06)
        .map_err(|_| "Could not get DDC VCP 60 value.")?;
    let code = vcp_value.value();
    let s = format!("{:02X}", code);
    Ok(s)
}
