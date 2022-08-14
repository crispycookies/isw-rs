use regex::Regex;
use configparser::ini::Ini;

pub struct IswConfigOps {
    m_cfg_file: String,
    m_cfg_parser: Ini,
}

impl Clone for IswConfigOps {
    fn clone(&self) -> IswConfigOps {
      IswConfigOps::new(self.m_cfg_file.clone())
    }
}

impl IswConfigOps {
    const ADDRESS_PROFILE: &'static str = "address_profile";

    pub fn new(cfg_file: String) -> IswConfigOps {
        IswConfigOps {
            m_cfg_file: cfg_file,
            m_cfg_parser: Ini::new(),
        }
    }

    fn format_not_found_error(&self, x: String, y: String) -> String {
        "Could not get <".to_string() + y.as_str() + "> of <"
            + x.as_str() + ">"
    }

    pub fn load_config(&mut self) -> Result<(), String> {
        self.m_cfg_parser = Ini::new();
        self.m_cfg_parser.load(self.m_cfg_file.as_str())?;
        Ok(())
    }

    pub fn get_numeric_property(&self, section: String, key: String) -> Result<u64, String> {
        match self.m_cfg_parser.get(section.as_str(),
                                    key.as_str()) {
            None => Err(self.format_not_found_error(key, section)),
            Some(e) => {
                let re = Regex::new("0[xX][0-9a-fA-F]+").unwrap();
                return if re.is_match(e.as_str()) {
                    let wo_prefix = e.trim_start_matches("0x");
                    Ok(u64::from_str_radix(wo_prefix, 16).expect("Could not parse value"))
                } else {
                    Ok(u64::from_str_radix(e.as_str(), 10).expect("Could not parse value"))
                };
            }
        }
    }

    pub fn get_base_address(&self, section: String, address_of: String) -> Result<u64, String> {
        match self.m_cfg_parser.get(section.as_str(),
                                    &*IswConfigOps::ADDRESS_PROFILE.to_string()) {
            None => Err(self.format_not_found_error(address_of, section)),
            Some(val) => {
                match self.get_numeric_property(val, address_of) {
                    Ok(read) => Ok(read),
                    Err(error) => Err(error)
                }
            }
        }
    }
}