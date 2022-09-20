use crate::isw_rs_base::{IswRsBase, UsbBacklightKind};
use serde::{Deserialize, Serialize};

mod isw_rs_base;
mod isw_raw_access;
mod isw_config_ops;
mod online;

use clap::{AppSettings, Clap};
use crate::online::Online;

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

/// ISW-clone written in Rust
#[derive(Clap, Clone)]
#[clap(version = "0.1", author = "Tobias Egger")]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
struct Opts {
    /// Use custom isw-config file
    #[clap(short, long, default_value = "/home/tobi/CLionProjects/isw-rs/isw.conf")]
    config: String,
    /// Raw Access(Manually Reading and Writing values from/to the Controller)
    #[clap(subcommand)]
    raw: Raw,
}

#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
enum Raw {
    /// Common Functions (Coolerboost etc.)
    #[clap(version = "1.3", author = "Tobias Egger")]
    Common(CommonHandler),
    /// Write to Controller
    #[clap(version = "1.3", author = "Tobias Egger")]
    Write(WriteHandler),
    /// Read from Controller
    #[clap(version = "1.3", author = "Tobias Egger")]
    Read(ReadHandler),
    /// Read from Controller
    #[clap(version = "1.3", author = "Tobias Egger")]
    Get(StateGetter),
    /// Read CPU-Data
    #[clap(version = "1.3", author = "Tobias Egger")]
    CPU(CPUHandler),
    /// Read GPU-Data
    #[clap(version = "1.3", author = "Tobias Egger")]
    GPU(GPUHandler),
}

/// Subcommand for Writing to Controller
#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
struct CommonHandler {
    /// Enables Coolerboost with 'on', disables Coolerboost with 'off'
    #[clap(short, long)]
    boost: Option<String>,
    /// Enables a websocket to listen to at 127.0.0.1:6799 & 6800
    #[clap(short, long)]
    socket: Option<bool>,
    /// Sets USB-backlight; 'off' for off, 'half' for half-strength, 'full' for full-strength
    #[clap(short, long)]
    usb_backlight: Option<String>,
    /// Sets Battery-Charging threshold; Accepts any value between 20 and 100
    #[clap(long)]
    battery: Option<u8>,
}

/// Subcommand for Writing to Controller
#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
struct WriteHandler {
    /// Address where value will be written to
    #[clap(short)]
    address: u64,
    /// Value to be written
    #[clap(long)]
    value: u16,
}

/// Subcommand for Reading from Controller
#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
struct ReadHandler {
    /// Address to read from
    #[clap(short)]
    address: u64,
}

/// Subcommand for getting States from Controller
#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
struct StateGetter {
    /// Gets state of Coolerboost
    #[clap(long)]
    boost: bool,
    /// SeGets state of ts USB-backlight
    #[clap(short, long)]
    usb_backlight: bool,
    /// Gets Battery-Charging threshold
    #[clap(long)]
    battery: bool,
}

/// Subcommand for Reading CPU-Data
#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
struct CPUHandler {
    /// CPU-Temperature
    #[clap(short, long)]
    temperature: bool,
    /// CPU-Fan RPM
    #[clap(short, long)]
    rpm: bool,
    /// CPU-Fan Speed
    #[clap(short, long)]
    speed: bool,
}

/// Subcommand for Reading CPU-Data
#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
struct GPUHandler {
    /// CPU-Temperature
    #[clap(short, long)]
    temperature: bool,
    /// CPU-Fan RPM
    #[clap(short, long)]
    rpm: bool,
    /// CPU-Fan Speed
    #[clap(short, long)]
    speed: bool,
}

fn run_boost(boost: String, isw: &mut IswRsBase) {
    let status: bool;
    match boost.as_ref() {
        "off" => status = false,
        "on" => status = true,
        _ => {
            panic!("Unrecognized option {}", boost);
        }
    }
    match isw.set_cooler_boost(status) {
        Ok(_) => {}
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_backlight(backlight: String, isw: &mut IswRsBase) {
    let status: UsbBacklightKind;
    match backlight.as_ref() {
        "off" => status = UsbBacklightKind::Off,
        "half" => status = UsbBacklightKind::Half,
        "full" => status = UsbBacklightKind::Full,
        _ => {
            panic!("Unrecognized option {}", backlight);
        }
    }
    match isw.set_usb_backlight(status) {
        Ok(_) => {}
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_battery(battery: u8, isw: &mut IswRsBase) {
    match isw.set_battery_threshold(battery) {
        Ok(_) => {}
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_socket(enable: bool, isw: &mut IswRsBase) {
    if enable {
        let mut sock = Online::new("127.0.0.1".to_string(), 6800, 6799).expect("Cannot open Socket");
        loop {
            match sock.receive() {
                Ok(value) => {
                    if !value.is_empty() {
                        match serde_json::from_str::<ReceivedOption>(value.as_str()) {
                            Ok(received) => {
                                if received.cmd == "cpu_temp"{
                                    match isw.get_cpu_temp() {
                                        Ok(value) => {
                                            let json = Response {
                                                id: "cpu_temp".to_string(),
                                                value: value.to_string()
                                            };
                                            match serde_json::to_string(&json) {
                                                Ok(to_be_sent) => {
                                                    let trimmed = to_be_sent.trim().to_string();
                                                    sock.send(trimmed);
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                                if received.cmd == "gpu_temp"{
                                    match isw.get_gpu_temp() {
                                        Ok(value) => {
                                            let json = Response {
                                                id: "gpu_temp".to_string(),
                                                value: value.to_string()
                                            };
                                            match serde_json::to_string(&json) {
                                                Ok(to_be_sent) => {
                                                    let trimmed = to_be_sent.trim().to_string();
                                                    sock.send(trimmed);
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                                if received.cmd == "cpu_fan_speed"{
                                    match isw.get_gpu_fan_speed() {
                                        Ok(value) => {
                                            let json = Response {
                                                id: "cpu_fan_speed".to_string(),
                                                value: value.to_string()
                                            };
                                            match serde_json::to_string(&json) {
                                                Ok(to_be_sent) => {
                                                    let trimmed = to_be_sent.trim().to_string();
                                                    sock.send(trimmed);
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                                if received.cmd == "gpu_fan_speed"{
                                    match isw.get_cpu_fan_speed() {
                                        Ok(value) => {
                                            let json = Response {
                                                id: "gpu_fan_speed".to_string(),
                                                value: value.to_string()
                                            };
                                            match serde_json::to_string(&json) {
                                                Ok(to_be_sent) => {
                                                    let trimmed = to_be_sent.trim().to_string();
                                                    sock.send(trimmed);
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                                if received.cmd == "cpu_fan_rpm"{
                                    match isw.get_cpu_fan_rpm() {
                                        Ok(value) => {
                                            let json = Response {
                                                id: "cpu_fan_rpm".to_string(),
                                                value: value.to_string()
                                            };
                                            match serde_json::to_string(&json) {
                                                Ok(to_be_sent) => {
                                                    let trimmed = to_be_sent.trim().to_string();
                                                    sock.send(trimmed);
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                                if received.cmd == "gpu_fan_rpm"{
                                    match isw.get_gpu_fan_rpm() {
                                        Ok(value) => {
                                            let json = Response {
                                                id: "gpu_fan_rpm".to_string(),
                                                value: value.to_string()
                                            };
                                            match serde_json::to_string(&json) {
                                                Ok(to_be_sent) => {
                                                    let trimmed = to_be_sent.trim().to_string();
                                                    sock.send(trimmed);
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                            }
                            Err(_) => {}
                        };
                    } else {
                       println!("Receiving data via Websocket failed");
                    }
                }
                Err(_) => {}
            }
        }
    }
}

fn run_read(address: u64, isw: &mut IswRsBase) {
    match isw.raw_access.read_hw(address) {
        Ok(val) => {
            panic!("Value: {}", val);
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_write(address: u64, value: u16, isw: &mut IswRsBase) {
    match isw.raw_access.write_hw(address, value) {
        Ok(_) => {}
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_get_battery(isw: &mut IswRsBase) {
    match isw.get_battery_threshold() {
        Ok(val) => {
            println!("Battery threshold: {}", val);
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_get_backlight(isw: &mut IswRsBase) {
    match isw.get_usb_backlight() {
        Ok(val) => {
            match val {
                UsbBacklightKind::Off => {
                    println!("USB-Backlight is off")
                }
                UsbBacklightKind::Half => {
                    println!("USB-Backlight is at half-strength")
                }
                UsbBacklightKind::Full => {
                    println!("USB-Backlight is at full-strength")
                }
                UsbBacklightKind::None => {
                    println!("No USB-Backlight detected")
                }
            }
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_get_boost(isw: &mut IswRsBase) {
    match isw.get_cooler_boost().unwrap() {
        true => {
            println!("Coolerboost is on")
        }
        false => {
            println!("Coolerboost is off")
        }
    }
}

fn run_getters(getter: StateGetter, isw: &mut IswRsBase) {
    if getter.battery {
        run_get_battery(isw);
    }
    if getter.usb_backlight {
        run_get_backlight(isw);
    }
    if getter.boost {
        run_get_boost(isw);
    }
}

fn run_get_cpu_rpm(isw: &mut IswRsBase) {
    match isw.get_cpu_fan_rpm() {
        Ok(val) => {
            println!("CPU-Fan rpm: {}", val)
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_get_cpu_speed(isw: &mut IswRsBase) {
    match isw.get_cpu_fan_speed() {
        Ok(val) => {
            println!("CPU-Fan speed: {}", val)
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_get_cpu_temp(isw: &mut IswRsBase) {
    match isw.get_cpu_temp() {
        Ok(val) => {
            println!("CPU temperature: {}", val)
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_get_gpu_rpm(isw: &mut IswRsBase) {
    match isw.get_gpu_fan_rpm() {
        Ok(val) => {
            println!("GPU-Fan rpm: {}", val)
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_get_gpu_speed(isw: &mut IswRsBase) {
    match isw.get_gpu_fan_speed() {
        Ok(val) => {
            println!("GPU-Fan speed: {}", val)
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_get_gpu_temp(isw: &mut IswRsBase) {
    match isw.get_gpu_temp() {
        Ok(val) => {
            println!("GPU temperature: {}", val)
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn run_cpu(cpu: CPUHandler, isw: &mut IswRsBase) {
    if cpu.rpm {
        run_get_cpu_rpm(isw);
    }
    if cpu.speed {
        run_get_cpu_speed(isw);
    }
    if cpu.temperature {
        run_get_cpu_temp(isw);
    }
}

fn run_common(common: CommonHandler, isw: &mut IswRsBase) {
    match common.boost {
        None => {}
        Some(boost) => {
            run_boost(boost, isw);
        }
    }
    match common.usb_backlight {
        None => {}
        Some(backlight) => {
            run_backlight(backlight, isw);
        }
    }
    match common.battery {
        None => {}
        Some(battery) => {
            run_battery(battery, isw);
        }
    }
    match common.socket {
        None => {}
        Some(enable) => {
            run_socket(enable, isw);
        }
    }
}

fn run_gpu(gpu: GPUHandler, isw: &mut IswRsBase) {
    if gpu.rpm {
        run_get_gpu_rpm(isw);
    }
    if gpu.speed {
        run_get_gpu_speed(isw);
    }
    if gpu.temperature {
        run_get_gpu_temp(isw);
    }
}

fn run(isw: &mut IswRsBase, opts: Opts) {
    /*
    match opts.boost {
        None => {}
        Some(boost) => {
            run_boost(boost, isw);
        }
    }
    match opts.usb_backlight {
        None => {}
        Some(backlight) => {
            run_backlight(backlight, isw);
        }
    }
    match opts.battery {
        None => {}
        Some(battery) => {
            run_battery(battery, isw);
        }
    }*/
    match opts.raw {
        Raw::Write(write) => {
            run_write(write.address, write.value, isw);
        }
        Raw::Read(read) => {
            run_read(read.address, isw);
        }
        Raw::Get(getter) => {
            run_getters(getter, isw);
        }
        Raw::CPU(cpu) => {
            run_cpu(cpu, isw);
        }
        Raw::GPU(gpu) => {
            run_gpu(gpu, isw);
        }
        Raw::Common(common) => {
            run_common(common, isw);
        }
    }
}

fn parse() {
    let opts: Opts = Opts::parse();
    match IswRsBase::new(opts.clone().config) {
        Ok(mut isw) => {
            run(&mut isw, opts.clone())
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn main() {
    parse();
}
