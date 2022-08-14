use crate::isw_raw_access::IswRawAccess;
use crate::isw_config_ops::IswConfigOps;

pub enum UsbBacklightKind {
    Off,
    Half,
    Full,
    None,
}

#[derive(Clone)]
pub struct IswRsBase {
    pub raw_access: IswRawAccess,
    m_config_ops: IswConfigOps,
}

impl IswRsBase {
    const MSI_ADDRESS_DEFAULT: &'static str = "MSI_ADDRESS_DEFAULT";
    const COOLER_BOOST: &'static str = "COOLER_BOOST";
    const COOLER_BOOST_OFF: &'static str = "cooler_boost_off";
    const COOLER_BOOST_ON: &'static str = "cooler_boost_on";
    const COOLER_BOOST_ADDRESS_IDENTIFIER: &'static str = "cooler_boost_address";
    const BATTERY_CHARGING_THRESHOLD_ADDRESS_IDENTIFIER: &'static str = "battery_charging_threshold_address";
    const USB_BACKLIGHT: &'static str = "USB_BACKLIGHT";
    const USB_BACKLIGHT_ADDRESS_IDENTIFIER: &'static str = "usb_backlight_address";
    const USB_BACKLIGHT_OFF: &'static str = "usb_backlight_off";
    const USB_BACKLIGHT_HALF: &'static str = "usb_backlight_half";
    const USB_BACKLIGHT_FULL: &'static str = "usb_backlight_full";
    const GPU_TEMP_ADDRESS_IDENTIFIER: &'static str = "realtime_gpu_temp_address";
    const CPU_TEMP_ADDRESS_IDENTIFIER: &'static str = "realtime_cpu_temp_address";
    const GPU_FAN_SPEED_ADDRESS_IDENTIFIER: &'static str = "realtime_gpu_fan_speed_address";
    const CPU_FAN_SPEED_ADDRESS_IDENTIFIER: &'static str = "realtime_cpu_fan_speed_address";
    const GPU_FAN_RPM_ADDRESS_IDENTIFIER: &'static str = "realtime_gpu_fan_rpm_address";
    const CPU_FAN_RPM_ADDRESS_IDENTIFIER: &'static str = "realtime_cpu_fan_rpm_address";
    const IO_FILE: &'static str = "dump.sys2";

    const FAN_DIVISOR_CONSTANT: u32 = 478000;

    pub fn new(cfg_file: String) -> Result<IswRsBase, String> {
        let mut s = IswRsBase {
            raw_access: IswRawAccess::new(IswRsBase::IO_FILE.to_string()),
            m_config_ops: IswConfigOps::new(cfg_file),
        };

        s.m_config_ops.load_config()?;

        return Ok(s);
    }
    /// set USB Backlight
    pub fn set_usb_backlight(&mut self, state: UsbBacklightKind) -> Result<(), String> {
        let value: u64;
        let base_address = self.m_config_ops.get_base_address(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_ADDRESS_IDENTIFIER.to_string())?;

        match state {
            UsbBacklightKind::Off => {
                value = self.m_config_ops.get_numeric_property(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_OFF.to_string())?;
            }
            UsbBacklightKind::Half => {
                value = self.m_config_ops.get_numeric_property(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_HALF.to_string())?;
            }
            UsbBacklightKind::Full => {
                value = self.m_config_ops.get_numeric_property(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_FULL.to_string())?;
            }
            _ => {
                return Result::Err("No viable option provided".to_string());
            }
        }

        self.raw_access.write_hw(base_address, value as u16)?;
        Ok(())
    }
    pub fn get_usb_backlight(&mut self) -> Result<UsbBacklightKind, String> {
        let base_address = self.m_config_ops.get_base_address(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_ADDRESS_IDENTIFIER.to_string())?;

        let half = self.m_config_ops.get_numeric_property(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_HALF.to_string())? as u16;
        let full = self.m_config_ops.get_numeric_property(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_FULL.to_string())? as u16;
        let off = self.m_config_ops.get_numeric_property(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_OFF.to_string())? as u16;

        let value = self.raw_access.read_hw(base_address)?;

        if value == half {
            return Ok(UsbBacklightKind::Half);
        } else if value == full {
            return Ok(UsbBacklightKind::Full);
        } else if value == off {
            return Ok(UsbBacklightKind::Off);
        }
        Ok(UsbBacklightKind::None)
    }

    /// set Battery Threshold
    pub fn set_battery_threshold(&mut self, t: u8) -> Result<(), String> {
        if t < 20 && t > 100 {
            return Err("No viable threshold provided".to_string());
        }
        let base_address = self.m_config_ops.get_numeric_property(IswRsBase::MSI_ADDRESS_DEFAULT.to_string(), IswRsBase::BATTERY_CHARGING_THRESHOLD_ADDRESS_IDENTIFIER.to_string())?;
        self.raw_access.write_hw(base_address, (t as u16) + 128)?;
        Ok(())
    }
    pub fn get_battery_threshold(&mut self) -> Result<u8, String> {
        let base_address = self.m_config_ops.get_numeric_property(IswRsBase::MSI_ADDRESS_DEFAULT.to_string(), IswRsBase::BATTERY_CHARGING_THRESHOLD_ADDRESS_IDENTIFIER.to_string())?;
        let read = self.raw_access.read_hw(base_address)? - 128;
        Ok(read as u8)
    }

    /// Set Coolerboost
    pub fn set_cooler_boost(&mut self, on: bool) -> Result<(), String> {
        let value: u64;
        let base_address = self.m_config_ops.get_base_address(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_ADDRESS_IDENTIFIER.to_string());

        if on {
            value = self.m_config_ops.get_numeric_property(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_ON.to_string())?;
        } else {
            value = self.m_config_ops.get_numeric_property(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_OFF.to_string())?;
        }
        self.raw_access.write_hw(base_address.unwrap(), value as u16)?;
        Ok(())
    }
    pub fn get_cooler_boost(&mut self) -> Result<bool, String> {
        let base_address = self.m_config_ops.get_base_address(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_ADDRESS_IDENTIFIER.to_string())?;
        let is_on = self.m_config_ops.get_numeric_property(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_ON.to_string())?;

        if self.raw_access.read_hw(base_address)? == (is_on as u16) {
            return Ok(true);
        }
        Ok(false)
    }

    fn get_data<T: num::NumCast>(&self, address_of: String) -> Result<T, String> {
        let base_address = self.m_config_ops.get_base_address(IswRsBase::MSI_ADDRESS_DEFAULT.to_string(), address_of)?;
        let read = self.raw_access.read_hw(base_address)?;
        Ok(num::cast(read).unwrap())
    }

    fn get_temp(&mut self, address_of: String) -> Result<f64, String> {
        let temp = self.get_data(address_of)?;
        Ok(temp)
    }

    pub fn get_gpu_temp(&mut self) -> Result<f64, String> {
        let temp = self.get_temp(IswRsBase::GPU_TEMP_ADDRESS_IDENTIFIER.to_string())?;
        Ok(temp)
    }

    pub fn get_cpu_temp(&mut self) -> Result<f64, String> {
        let temp = self.get_temp(IswRsBase::CPU_TEMP_ADDRESS_IDENTIFIER.to_string())?;
        Ok(temp)
    }

    fn get_fan_speed(&mut self, address_of: String) -> Result<u16, String> {
        let speed: u16 = self.get_data(address_of)?;
        // We are reading two bytes, but only need the last nibble...
        let corrected = speed & 0x000000FF;
        Ok(corrected)
    }

    pub fn get_gpu_fan_speed(&mut self) -> Result<u16, String> {
        let speed = self.get_fan_speed(IswRsBase::GPU_FAN_SPEED_ADDRESS_IDENTIFIER.to_string())?;
        Ok(speed)
    }

    pub fn get_cpu_fan_speed(&mut self) -> Result<u16, String> {
        let speed = self.get_fan_speed(IswRsBase::CPU_FAN_SPEED_ADDRESS_IDENTIFIER.to_string())?;
        Ok(speed)
    }

    fn get_fan_rpm(&mut self, address_of: String) -> Result<u16, String> {
        let value: u32 = self.get_data(address_of)?;
        if value == 0 {
            return Ok(0);
        }
        Ok((IswRsBase::FAN_DIVISOR_CONSTANT / value) as u16)
    }

    pub fn get_gpu_fan_rpm(&mut self) -> Result<u16, String> {
        let rpm = self.get_fan_rpm(IswRsBase::GPU_FAN_RPM_ADDRESS_IDENTIFIER.to_string())?;
        Ok(rpm)
    }

    pub fn get_cpu_fan_rpm(&mut self) -> Result<u16, String> {
        let rpm = self.get_fan_rpm(IswRsBase::CPU_FAN_RPM_ADDRESS_IDENTIFIER.to_string())?;
        Ok(rpm)
    }
}