mod isw_rs_base;
mod isw_raw_access;
mod isw_config_ops;
mod isw_parse;

use crate::isw_rs_base::{IswRsBase, UsbBacklightKind};
use clap::{Clap};
use crate::isw_parse::{CPUHandler, GPUHandler, StateGetter, Raw, Opts, MiscHandler};

fn is_either(a : String, b : String, value : String) -> Result<bool, String> {
    if value == a {
        return Ok(true)
    } else if value == b {
        return Ok(false)
    }
    return Err("Unrecognized option <".to_string() + value.as_str() + ">")
}

fn run_boost(boost: String, isw: &mut IswRsBase) -> Result<bool, String> {
    match isw.set_cooler_boost(is_either("on".to_string(), "off".to_string(), boost)?) {
        Ok(_) => {return Ok(true);}
        Err(error) => {
            return Err("nn".to_string());
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

fn run_misc(misc: MiscHandler, isw: &mut IswRsBase){
    if misc.boost.is_some() {

    }
    match misc.boost {
        None => {}
        Some(boost) => {
            run_boost(boost, isw);
        }
    }
    match misc.usb_backlight {
        None => {}
        Some(backlight) => {
            run_backlight(backlight, isw);
        }
    }
    match misc.battery {
        None => {}
        Some(battery) => {
            run_battery(battery, isw);
        }
    }
}

fn run(isw: &mut IswRsBase, opts: Opts) {
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
        Raw::MISC(misc) => {
            run_misc(misc, isw);
        }
    }
}

fn parse() {
    let opts: isw_parse::Opts = isw_parse::Opts::parse();
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
