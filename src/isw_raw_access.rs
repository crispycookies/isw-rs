use std::io::prelude::*;
use std::io::SeekFrom;

#[derive(Clone)]
pub struct IswRawAccess {
    m_sys_fs_file: String,
}

impl IswRawAccess {
    pub fn new(sys_fs_file: String) -> IswRawAccess {
        let s = IswRawAccess {
            m_sys_fs_file: sys_fs_file
        };
        return s;
    }

    fn format_opening_error(&self, error: String) -> String {
        return "Opening file <".to_string() + self.m_sys_fs_file.clone().as_str()
            + "> failed with <" + error.as_str() + ">";
    }

    fn format_seek_error(&self, base_address: u64, error: String) -> String {
        return "Seeking pos <".to_string() + base_address.to_string().as_str() + "> of file <"
            + self.m_sys_fs_file.clone().as_str() + "> failed with <" + error.as_str() + ">";
    }

    fn format_rw_error(&self, read: bool) -> String {
        if read {
            return "Could not read from file <".to_string() + self.m_sys_fs_file.clone().as_str()
                + ">";
        }
        return "Could not write from file <".to_string() + self.m_sys_fs_file.clone().as_str()
            + ">";
    }

    pub fn write_hw(&self, base_address: u64, value: u16) -> Result<(), String> {
        match std::fs::OpenOptions::new().write(true).open(self.m_sys_fs_file.clone()) {
            Ok(mut f) => {
                match f.seek(SeekFrom::Start(base_address)) {
                    Ok(_) => {
                        match f.write(&value.to_le_bytes()) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(self.format_rw_error(false))
                        }
                    }
                    Err(error) => Err(self.format_seek_error(base_address.clone(),
                                                             error.to_string()))
                }
            }
            Err(error) => {
                Err(self.format_opening_error(error.to_string()))
            }
        }
    }

    pub fn read_hw(&self, base_address: u64) -> Result<u16, String> {
        match std::fs::OpenOptions::new().read(true).open(self.m_sys_fs_file.clone()) {
            Ok(mut f) => {
                match f.seek(SeekFrom::Start(base_address)) {
                    Ok(_) => {
                        let mut buf = [0, 0];
                        match f.read(&mut buf) {
                            Ok(_) => Ok(u16::from_le_bytes(buf)),
                            Err(_) => Err(self.format_rw_error(true))
                        }
                    }
                    Err(error) => Err(self.format_seek_error(base_address.clone(),
                                                             error.to_string()))
                }
            }
            Err(error) => {
                Err(self.format_opening_error(error.to_string()))
            }
        }
    }
}