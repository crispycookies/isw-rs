use regex::Regex;
use configparser::ini::Ini;

pub struct IswConfigOps {
    m_cfg_file: String,
    m_cfg_parser: Ini,
}

impl IswConfigOps {
    const ADDRESS_PROFILE: &'static str = "address_profile";

    pub fn new(cfg_file: String) -> IswConfigOps {
        let s = IswConfigOps {
            m_cfg_file: cfg_file,
            m_cfg_parser: Ini::new()
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

    pub fn get_numeric_property(&self, section: String, key: String) -> u64
    {
        match self.m_cfg_parser.get(section.as_str(),
                                    key.as_str()) {
            None => {
                panic!("Could not get <{}> of <{}>",
                       key, section)
            }
            Some(e) => {
                let re = Regex::new("0[xX][0-9a-fA-F]+").unwrap();
                return if re.is_match(e.as_str()) {
                    let wo_prefix = e.trim_start_matches("0x");
                    u64::from_str_radix(wo_prefix, 16).expect("Could not parse value")
                } else {
                    u64::from_str_radix(e.as_str(), 10).expect("Could not parse value")
                }
            }
        }
    }

    pub fn get_base_address(&self, section: String, address_of: String) -> u64
    {
        match self.m_cfg_parser.get(section.as_str(),
                                    &*IswConfigOps::ADDRESS_PROFILE.to_string()) {
            None => {
                panic!("Could not get <{}> of <{}>",
                       address_of, section)
            }
            Some(e) => {
                return self.get_numeric_property(e, address_of);
            }
        }
    }

}