use ddc::Ddc;
use ddc_winapi::Monitor;

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
