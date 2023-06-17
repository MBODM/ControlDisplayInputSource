use std::mem;

use windows::{
    core::*, Win32::Devices::Display::*, Win32::Foundation::*, Win32::Graphics::Gdi::*,
    Win32::UI::WindowsAndMessaging::*, s, w,
};

fn main() -> Result<()> {
    println!("----------------------------------------------------------------------");
    println!("Hello!");
    println!("----------------------------------------------------------------------");
    unsafe {
        let mut api_fn_name: String;
        let mut api_fn_bret: BOOL;
        // GetDesktopWindow
        let h_window = GetDesktopWindow();
        println!("GetDesktopWindow() return value:     {:?}", h_window);
        println!("GetLastError() return value:         {:?}", GetLastError());
        println!("----------------------------------------------------------------------");
        // MonitorFromWindow
        let h_monitor = MonitorFromWindow(h_window, MONITOR_DEFAULTTOPRIMARY);
        println!("MonitorFromWindow() return value:    {:?}", h_monitor);
        println!("GetLastError() return value:         {:?}", GetLastError());
        println!("----------------------------------------------------------------------");
        // GetMonitorInfoW
        api_fn_name = String::from("GetMonitorInfoW()");
        let mut mon_info = MONITORINFOEXW::default();
        mon_info.monitorInfo.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
        let p_to_mon_info = &mut mon_info as *mut MONITORINFOEXW;
        api_fn_bret = GetMonitorInfoW(h_monitor, p_to_mon_info as *mut MONITORINFO);
        println!("{} return value:      {:?}", api_fn_name, api_fn_bret);
        let mi = mon_info.monitorInfo;
        println!("{} result.cbSize:     {:?}", api_fn_name, mi.cbSize);
        println!("{} result.rcMonitor:  {:?}", api_fn_name, mi.rcMonitor);
        println!("{} result.rcWork:     {:?}", api_fn_name, mi.rcWork);
        println!("{} result.dwFlags:    {:?}", api_fn_name, mi.dwFlags);
        let sz_device = String::from_utf16_lossy(mon_info.szDevice.as_slice());
        println!("{} result.szDevice:   {}", api_fn_name, sz_device);
        let is_primary = mon_info.monitorInfo.dwFlags == MONITORINFOF_PRIMARY;
        let is_primary_string = is_primary.to_string().to_uppercase();
        println!("Is primary monitor:                  {}", is_primary_string);
        println!("GetLastError() return value:         {:?}", GetLastError());
        println!("----------------------------------------------------------------------");
        // GetNumberOfPhysicalMonitorsFromHMONITOR
        api_fn_name = String::from("GetNumberOfPhysicalMonitorsFromHMONITOR()");
        let mut physmons_num: u32 = 0;
        let p_physmons_num = &mut physmons_num as *mut u32;
        api_fn_bret = GetNumberOfPhysicalMonitorsFromHMONITOR(h_monitor, p_physmons_num);
        println!("{} return value:  {:?}", api_fn_name, api_fn_bret);
        println!("{} result number: {:?}", api_fn_name, physmons_num);
        println!("GetLastError() return value:         {:?}", GetLastError());
        println!("----------------------------------------------------------------------");
        // GetPhysicalMonitorsFromHMONITOR
        api_fn_name = String::from("GetPhysicalMonitorsFromHMONITOR()");
        let physmons_vec_cap = physmons_num as usize;
        let mut physmons_vec: Vec<PHYSICAL_MONITOR> = Vec::with_capacity(physmons_vec_cap);
        for _ in 0..physmons_num {
            physmons_vec.push(PHYSICAL_MONITOR::default());
        }
        let physmons_slice = physmons_vec.as_mut_slice();
        api_fn_bret = GetPhysicalMonitorsFromHMONITOR(h_monitor, physmons_slice);
        println!("{} return value:          {:?}", api_fn_name, api_fn_bret);
        println!(
            "{} result array length:   {:?}",
            api_fn_name,
            physmons_slice.len()
        );
        println!("GetLastError() return value:         {:?}", GetLastError());
        println!("----------------------------------------------------------------------");
        // Result
        let mut prim_physmon = physmons_slice[0];
        let prim_physmon_handle = prim_physmon.hPhysicalMonitor;
        //let mutter = prim_physmon.szPhysicalMonitorDescription.align_to();
        //let hs = String::from(prim_physmon.szPhysicalMonitorDescription);
        //let hs = HSTRING::from();
        //let prim_physmon_desc = String::from_utf16_lossy(mutter.0);
        println!("Primary physical monitor handle:  {:?}", prim_physmon_handle);
        println!("Primary physical monitor desc:    {:?}", prim_physmon_desc);
    }
    println!("Have a nice day.");
    Ok(())
}
