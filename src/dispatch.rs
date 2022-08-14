use crate::IswRsBase;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct ReceivedOption {
    pub cmd: String,
    pub option: String
}
#[derive(Serialize, Deserialize, Clone)]
struct Response {
    pub id: String,
    pub value: String
}

pub struct Dispatch {
    isw: IswRsBase
}

impl Dispatch {
    const GPU_TEMP: &'static str = "address_profile";
    const CPU_TEMP: &'static str = "address_profile";

    pub fn new(isw: &mut IswRsBase) -> Dispatch {
        Dispatch {
            isw: isw.clone()
        }
    }
    pub fn dispatch(&mut self, payload: String) -> String {
        match serde_json::from_str::<ReceivedOption>(payload.as_str()) {
            Ok(val) => {
                let cmd = val.cmd;
                let mut response = Response {
                    id: "".to_string(),
                    value: "".to_string()
                };

                if cmd == Dispatch::CPU_TEMP {
                    match self.isw.get_cpu_temp() {
                        Ok(val) => {
                            response.id = "cpu_temp".to_string();
                            response.value = val.to_string();
                            return serde_json::to_string(&response).expect("Could not Serialize Response")
                        }
                        Err(_) => {}
                    }
                } else if cmd == Dispatch::GPU_TEMP {
                    match self.isw.get_gpu_temp() {
                        Ok(val) => {
                            response.id = "gpu_temp".to_string();
                            response.value = val.to_string();
                            return serde_json::to_string(&response).expect("Could not Serialize Response")
                        }
                        Err(_) => {}
                    }
                }
                "".to_string()
            }
            Err(_) => {
                "".to_string()
            }
        }
    }
}
