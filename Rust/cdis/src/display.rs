use std::os::windows::prelude::OsStringExt;

use ddc::Ddc;
use ddc_winapi::Monitor;
use winapi::shared::windef::HMONITOR;

use crate::print;

#[derive(Debug)]
pub struct DisplayControl {
    is_initialized: bool,
    available_monitors: Vec<Monitor>,
}

impl DisplayControl {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            available_monitors: vec![],
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        if !self.is_initialized {
            self.is_initialized = true;
            self.available_monitors = Monitor::enumerate()
                .map_err(|_| "Underlying DDC Monitor API failed while enumerating monitors.")?;
            if self.available_monitors.is_empty() {
                return Err(String::from("No connected display(s) found."));
            }
        }
        Ok(())
    }

    pub fn get_display_list(&self) -> Result<Vec<String>, String> {
        self.check_init()?;
        let displays = self
            .available_monitors
            .iter()
            .enumerate()
            .map(|(index, monitor)| format!("[{}] {}", index, monitor.description()))
            .collect::<Vec<_>>();
        Ok(displays)
    }

    pub fn has_multiple_displays(&self) -> Result<bool, String> {
        self.check_init()?;
        Ok(self.available_monitors.len() > 1)
    }

    pub fn display_id_exists(&self, display_id: u8) -> Result<bool, String> {
        self.check_init()?;
        let index = usize::from(display_id);
        Ok(index < self.available_monitors.len())
    }

    pub fn get_display_description(&self, display_id: u8) -> Result<String, String> {
        self.check_init()?;
        let index = self.check_display_id(display_id)?;
        let description = self.available_monitors[index].description();
        Ok(description)
    }

    pub fn get_display_capabilities(&mut self, display_id: u8) -> Result<String, String> {
        self.check_init()?;
        let index = self.check_display_id(display_id)?;
        let bytes = self.available_monitors[index]
            .capabilities_string()
            .map_err(|_| "Could not determine the display´s DDC capabilities.")?;
        let capabilities = String::from_utf8(bytes)
            .map_err(|_| "Could not convert the displays´s DDC capabilities to string.")?;
        Ok(capabilities)
    }

    pub fn display_supports_vcp60(&mut self, display_id: u8) -> Result<bool, String> {
        let supports60 = self.check_vcp60_support(display_id, false)?;
        Ok(supports60)
    }

    pub fn get_vcp60_value(&mut self, display_id: u8) -> Result<u16, String> {
        self.check_init()?;
        let index = self.check_display_id(display_id)?;
        self.check_vcp60_support(display_id, true)?;
        let vcp_value = self.available_monitors[index]
            .get_vcp_feature(0x60)
            .map_err(|_| "Could not get DDC VCP 60 value.")?;
        let value = vcp_value.value();
        Ok(value)
    }

    pub fn set_vcp60_value(&mut self, display_id: u8, value: u16) -> Result<(), String> {
        self.check_init()?;
        let index = self.check_display_id(display_id)?;
        self.check_vcp60_support(display_id, true)?;
        self.available_monitors[index]
            .set_vcp_feature(0x60, value)
            .map_err(|_| "Could not set DDC VCP 60 value.")?;
        Ok(())
    }

    pub fn test(&mut self) {
        let window_handle: winapi::shared::windef::HWND =
            unsafe { winapi::um::winuser::GetDesktopWindow() };
        println!(
            "TEST: GetDesktopWindow()  --> window handle  (HWND)     --> {:?}",
            window_handle
        );
        let monitor_handle: HMONITOR = unsafe {
            winapi::um::winuser::MonitorFromWindow(
                window_handle,
                winapi::um::winuser::MONITOR_DEFAULTTOPRIMARY,
            )
        };
        println!(
            "TEST: MonitorFromWindow() --> monitor handle (HMONITOR) --> {:?}",
            monitor_handle
        );
        let hmonitors = ddc_winapi::enumerate_monitors().unwrap();
        println!("TEST: ddc_winapi::enumerate_monitors() --> List of all HMONITORs:");
        for (i, hmonitor) in hmonitors.iter().enumerate() {
            println!("TEST:   [{}] {:?}", i, hmonitor);
            let mut monitor_info: winapi::um::winuser::MONITORINFOEXW =
                unsafe { std::mem::zeroed() };
            monitor_info.cbSize = std::mem::size_of::<winapi::um::winuser::MONITORINFOEXW>() as u32;
            let monitor_info_ptr = <*mut _>::cast(&mut monitor_info);
            let result =
                unsafe { winapi::um::winuser::GetMonitorInfoW(hmonitor.clone(), monitor_info_ptr) };
            if result == winapi::shared::minwindef::TRUE {
                println!(
                    "TEST:   GetMonitorInfo().dwFlags for {:?} HMONITOR: {:?}",
                    hmonitor, monitor_info.dwFlags
                );
                if monitor_info.dwFlags == winapi::um::winuser::MONITORINFOF_PRIMARY {
                    println!("TEST:   List of PHYSICAL_MONITOR for {:?} HMONITOR:", hmonitor);
                    let phys_mon_vec = ddc_winapi::get_physical_monitors_from_hmonitor(hmonitor.clone()).unwrap();
                    for (num, phy_mon) in phys_mon_vec.iter().enumerate() {
                        let m = unsafe { Monitor::new(*phy_mon) };
                        println!("TEST:     [{}] PHYSICAL_MONITOR.hPhysicalMonitor (HANDLE): {:?}", num, m.handle())
                    }

                }
            }
        }
        // let phys_mons = ddc_winapi::get_physical_monitors_from_hmonitor(first_enum_mon).unwrap();

        // println!("WUFF - The physical monitor list:");
        // for (pos, _) in phys_mons.iter().enumerate() {
        //     println!("  - Position: {}", pos);
        //     let fuzz = phys_mons[pos].hPhysicalMonitor;
        //     println!("  - Element at Position {}: {:?}", pos, fuzz);
        // }

        // let first_phy_mon = phys_mons[0];
        // let str_ptr = std::ptr::addr_of!(first_phy_mon.szPhysicalMonitorDescription);
        // let desc = match (str_ptr as usize) & (std::mem::align_of::<u16>() - 1) {
        //     0 => std::borrow::Cow::Borrowed(unsafe { &*str_ptr }),
        //     _ => std::borrow::Cow::Owned(first_phy_mon.szPhysicalMonitorDescription),
        // };
        // let desc = match widestring::WideCStr::from_slice_truncate(&desc[..]) {
        //     Ok(cstr) => cstr.to_string_lossy(),
        //     Err(_) => widestring::WideStr::from_slice(&desc[..]).to_string_lossy(),
        // };
        // println!("MY hMonitor (HMONITOR): {:?}", hmonitors[0]);
        // //print!("MY hPhysicalMonitor (HANDLE): {:?}", physmon);
        // println!("MY szPhysicalMonitorDescription: {}", desc);
    }

    fn check_init(&self) -> Result<(), String> {
        match self.is_initialized {
            true => Ok(()),
            false => Err(String::from(
                "Not initialized (please call 'DisplayControl::init()' first).",
            )),
        }
    }

    fn check_display_id(&self, display_id: u8) -> Result<usize, String> {
        let index = usize::from(display_id);
        if index >= self.available_monitors.len() {
            return Err(String::from("Display with given ID not exists."));
        }
        Ok(index)
    }

    fn check_vcp60_support(&mut self, display_id: u8, show_error: bool) -> Result<bool, String> {
        let capabilities = self.get_display_capabilities(display_id)?;
        let supports60 = capabilities.contains("60(");
        if !supports60 && show_error {
            return Err(String::from("Display with given ID has no VCP 60 support."));
        }
        Ok(supports60)
    }
}
