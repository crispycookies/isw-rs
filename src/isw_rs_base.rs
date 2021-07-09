use crate::isw_raw_access::IswRawAccess;
use crate::isw_config_ops::IswConfigOps;

pub struct IswRsBase {
    pub raw_access: IswRawAccess,
    m_config_ops : IswConfigOps
}

impl IswRsBase {
    const COOLER_BOOST: &'static str = "COOLER_BOOST";
    const COOLER_BOOST_OFF: &'static str = "cooler_boost_off";
    const COOLER_BOOST_ON: &'static str = "cooler_boost_on";
    const COOLER_BOOST_ADDRESS_IDENTIFIER: &'static str = "cooler_boost_address";
    const BATTERY_CHARGING_THRESHOLD_ADDRESS_IDENTIFIER: &'static str = "battery_charging_threshold_address";
    const IO_FILE: &'static str = "/sys/kernel/debug/ec/ec0/io";
    //const IO_FILE: &'static str = "dump.sys";

    pub fn new(cfg_file: String) -> IswRsBase {
        let mut s = IswRsBase {
            raw_access: IswRawAccess::new(IswRsBase::IO_FILE.to_string()),
            m_config_ops: IswConfigOps::new(cfg_file)
        };

        s.m_config_ops.load_config();

        return s;
    }

    pub fn set_battery_threshold(&mut self, t : u8) -> bool{
        if t >= 20 && t <= 100 {
            let base_address = self.m_config_ops.get_base_address(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::BATTERY_CHARGING_THRESHOLD_ADDRESS_IDENTIFIER.to_string());
            self.raw_access.write_hw(base_address, (t as u16) + 128);
            return true;
        }
        return false;
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
}