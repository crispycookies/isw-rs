use crate::isw_raw_access::IswRawAccess;
use crate::isw_config_ops::IswConfigOps;

pub enum UsbBacklightKind {
    Off,
    Half,
    Full,
    None
}

pub struct IswRsBase {
    pub raw_access: IswRawAccess,
    m_config_ops: IswConfigOps,
}

impl IswRsBase {
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
    const IO_FILE: &'static str = "/sys/kernel/debug/ec/ec0/io";

    pub fn new(cfg_file: String) -> IswRsBase {
        let mut s = IswRsBase {
            raw_access: IswRawAccess::new(IswRsBase::IO_FILE.to_string()),
            m_config_ops: IswConfigOps::new(cfg_file),
        };

        s.m_config_ops.load_config();

        return s;
    }

    pub fn set_usb_backlight(&mut self, state: UsbBacklightKind) {
        let value: u64;
        let base_address = self.m_config_ops.get_base_address(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_ADDRESS_IDENTIFIER.to_string());

        match state {
            UsbBacklightKind::Off => {
                value = self.m_config_ops.get_numeric_property(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_OFF.to_string());
            }
            UsbBacklightKind::Half => {
                value = self.m_config_ops.get_numeric_property(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_HALF.to_string());
            }
            UsbBacklightKind::Full => {
                value = self.m_config_ops.get_numeric_property(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_FULL.to_string());
            }
            _ => {
                return;
            }
        }

        self.raw_access.write_hw(base_address, value as u16);
    }
    pub fn get_usb_backlight(&mut self) -> UsbBacklightKind {
        let base_address = self.m_config_ops.get_base_address(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_ADDRESS_IDENTIFIER.to_string());

        let half = self.m_config_ops.get_numeric_property(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_HALF.to_string()) as u16;
        let full = self.m_config_ops.get_numeric_property(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_FULL.to_string()) as u16;
        let off  = self.m_config_ops.get_numeric_property(IswRsBase::USB_BACKLIGHT.to_string(), IswRsBase::USB_BACKLIGHT_OFF.to_string()) as u16;

        let value = self.raw_access.read_hw(base_address);

        if value == half {
            return UsbBacklightKind::Half
        } else if value == full {
            return UsbBacklightKind::Full
        } else if value == off {
            return UsbBacklightKind::Off
        }
        return UsbBacklightKind::None
    }

    pub fn set_battery_threshold(&mut self, t: u8) -> bool {
        if t >= 20 && t <= 100 {
            let base_address = self.m_config_ops.get_base_address(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::BATTERY_CHARGING_THRESHOLD_ADDRESS_IDENTIFIER.to_string());
            self.raw_access.write_hw(base_address, (t as u16) + 128);
            return true;
        }
        return false;
    }

    pub fn get_battery_threshold(&mut self) -> u8 {
        let base_address = self.m_config_ops.get_base_address(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::BATTERY_CHARGING_THRESHOLD_ADDRESS_IDENTIFIER.to_string());
        let read = self.raw_access.read_hw(base_address) - 128;
        return read as u8;
    }

    pub fn set_cooler_boost(&mut self, on: bool) {
        let value: u64;
        let base_address = self.m_config_ops.get_base_address(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_ADDRESS_IDENTIFIER.to_string());

        if on {
            value = self.m_config_ops.get_numeric_property(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_ON.to_string());
        } else {
            value = self.m_config_ops.get_numeric_property(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_OFF.to_string());
        }
        self.raw_access.write_hw(base_address, value as u16);
    }

    pub fn get_cooler_boost(&mut self) -> bool {
        let base_address = self.m_config_ops.get_base_address(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_ADDRESS_IDENTIFIER.to_string());
        let is_on = self.m_config_ops.get_numeric_property(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_ON.to_string());

        if self.raw_access.read_hw(base_address) == (is_on as u16) {
            return true;
        }
        return false;
    }
}