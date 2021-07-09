use std::io::prelude::*;
use std::io::SeekFrom;
use regex::Regex;

use configparser::ini::Ini;

pub struct IswRsBase {
    m_cfg_file: String,
    m_cfg_parser: Ini,
}

impl IswRsBase {
    const COOLER_BOOST: &'static str = "COOLER_BOOST";
    const COOLER_BOOST_OFF: &'static str = "cooler_boost_off";
    const COOLER_BOOST_ON: &'static str = "cooler_boost_on";
    const COOLER_BOOST_ADDRESS_IDENTIFIER: &'static str = "cooler_boost_address";
    const ADDRESS_PROFILE: &'static str = "address_profile";
    const IO_FILE: &'static str = "/sys/kernel/debug/ec/ec0/io";
    //const IO_FILE: &'static str = "dump.sys";

    pub fn new(cfg_file: String) -> IswRsBase {
        let s = IswRsBase {
            m_cfg_file: cfg_file,
            m_cfg_parser: Ini::new(),
        };
        return s;
    }

    pub fn load_config(&mut self) {
        self.m_cfg_parser = Ini::new();
        match self.m_cfg_parser.load(self.m_cfg_file.as_str()) {
            Ok(_) => {}
            Err(e) => {
                panic!("Opening config failed with: {}", e.as_str());
            }
        }
    }
    fn get_numeric_property(&self, section: String, key: String) -> i64
    {
        match self.m_cfg_parser.get(section.as_str(),
                                    key.as_str()) {
            None => {
                panic!("Could not get <{}> of <{}>",
                       key, section)
            }
            Some(e) => {
                let re = Regex::new("0[xX][0-9a-fA-F]+").unwrap();
                if re.is_match(e.as_str()) {
                    let wo_prefix = e.trim_start_matches("0x");
                    return i64::from_str_radix(wo_prefix, 16).expect("Could not parse value");
                } else {
                    return i64::from_str_radix(e.as_str(), 10).expect("Could not parse value");
                }
            }
        }
    }

    fn get_base_address(&self, section: String, address_of: String) -> i64
    {
        match self.m_cfg_parser.get(section.as_str(),
                                    &*IswRsBase::ADDRESS_PROFILE.to_string()) {
            None => {
                panic!("Could not get <{}> of <{}>",
                       address_of, section)
            }
            Some(e) => {
                return self.get_numeric_property(e, address_of);
            }
        }
    }
    pub fn set_cooler_boost(&mut self, on: bool) {
        let value: i64;
        let base_address = self.get_base_address(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_ADDRESS_IDENTIFIER.to_string());

        if on {
            value = self.get_numeric_property(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_ON.to_string());
        } else {
            value = self.get_numeric_property(IswRsBase::COOLER_BOOST.to_string(), IswRsBase::COOLER_BOOST_OFF.to_string());
        }
        self.write_chunk(IswRsBase::IO_FILE.to_string(), base_address as u64, value as u16);
    }
    pub fn write_chunk(&self, file: String, base_address: u64, value: u16) {
        match std::fs::OpenOptions::new().write(true).open(file.clone()) {
            Ok(mut f) => {
                f.seek(SeekFrom::Start(base_address)).expect("Address does not exist");
                f.write(&value.to_le_bytes()).expect("Could not write to file");
            }
            Err(e) => {
                panic!("Opening file <{}> failed with <{}>", file.clone(), e);
            }
        }
    }
    pub fn read_chunk(&self, file: String, base_address: u64) -> i16{
        match std::fs::OpenOptions::new().read(true).open(file.clone()) {
            Ok(mut f) => {
                let mut buf = [0, 0];
                f.seek(SeekFrom::Start(base_address)).expect("Address does not exist");
                f.read(&mut buf).expect("Could not write to file");

                return i16::from_le_bytes(buf);
            }
            Err(e) => {
                panic!("Opening file <{}> failed with <{}>", file.clone(), e);
            }
        }
    }
}