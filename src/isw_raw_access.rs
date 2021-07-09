use std::io::prelude::*;
use std::io::SeekFrom;

pub struct IswRawAccess {
    m_sys_fs_file : String,
}

impl IswRawAccess {
    pub fn new(sys_fs_file : String) -> IswRawAccess {
        let s = IswRawAccess {
            m_sys_fs_file: sys_fs_file
        };
        return s;
    }

    pub fn write_hw(&self, base_address: u64, value: u16) {
        match std::fs::OpenOptions::new().write(true).open(self.m_sys_fs_file.clone()) {
            Ok(mut f) => {
                f.seek(SeekFrom::Start(base_address)).expect("Address does not exist");
                f.write(&value.to_le_bytes()).expect("Could not write to file");
            }
            Err(e) => {
                panic!("Opening file <{}> failed with <{}>", self.m_sys_fs_file.clone(), e);
            }
        }
    }

    pub fn read_hw(&self, base_address: u64) -> u16{
        match std::fs::OpenOptions::new().read(true).open(self.m_sys_fs_file.clone()) {
            Ok(mut f) => {
                let mut buf = [0, 0];
                f.seek(SeekFrom::Start(base_address)).expect("Address does not exist");
                f.read(&mut buf).expect("Could not write to file");

                return u16::from_le_bytes(buf);
            }
            Err(e) => {
                panic!("Opening file <{}> failed with <{}>", self.m_sys_fs_file.clone(), e);
            }
        }
    }
}