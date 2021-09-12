use crate::isw_rs_base::{IswRsBase, UsbBacklightKind};

mod isw_rs_base;
mod isw_raw_access;
mod isw_config_ops;

use clap::{AppSettings, Clap};

/// ISW-clone written in Rust
#[derive(Clap)]
#[clap(version = "0.1", author = "Tobias Egger")]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
struct Opts {
    /// Use custom isw-config file
    #[clap(short, long, default_value="isw.conf")]
    config: String,
    /// Enables Coolerboost with 'on', disables Coolerboost with 'off'
    #[clap(short, long)]
    boost: Option<String>,
    /// Sets USB-backlight; 'off' for off, 'half' for half-strength, 'full' for full-strength
    #[clap(short, long)]
    usb_backlight: Option<String>,
    /// Sets Battery-Charging threshold; Accepts any value between 20 and 100
    #[clap(long)]
    battery: Option<u8>,
    /// Raw Access(Manually Reading and Writing values from/to the Controller)
    #[clap(subcommand)]
    raw: Raw,
}

#[derive(Clap)]
enum Raw {
    /// Write to Controller
    #[clap(version = "1.3", author = "Someone E. <someone_else@other.com>")]
    Write(WriteHandler),
    /// Read from Controller
    #[clap(version = "1.3", author = "Someone E. <someone_else@other.com>")]
    Read(ReadHandler),
    /// Read from Controller
    #[clap(version = "1.3", author = "Someone E. <someone_else@other.com>")]
    Get(StateGetter)

}

/// Subcommand for Writing to Controller
#[derive(Clap)]
struct WriteHandler {
    /// Address where value will be written to
    #[clap(short)]
    address: u64,
    /// Value to be written
    #[clap(long)]
    value: u16
}

/// Subcommand for Reading from Controller
#[derive(Clap)]
struct ReadHandler {
    /// Address to read from
    #[clap(short)]
    address: u64,
}

/// Subcommand for getting States from Controller
#[derive(Clap)]
struct StateGetter {
    /// Gets state of Coolerboost
    #[clap(short, long)]
    boost: bool,
    /// SeGets state of ts USB-backlight
    #[clap(short, long)]
    usb_backlight: bool,
    /// Gets Battery-Charging threshold
    #[clap(long)]
    battery: bool
}

fn run_boost(boost: String, isw : & mut IswRsBase) {
    match boost.as_ref() {
        "off" => {
            isw.set_cooler_boost(false);
        },
        "on" => {
            isw.set_cooler_boost(true);
        }
        _ => {
            panic!("Unrecognized option {}", boost);
        }
    }
}

fn run_backlight(backlight: String, isw : & mut IswRsBase) {
    match backlight.as_ref() {
        "off" => {
            isw.set_usb_backlight(UsbBacklightKind::Off);
        },
        "half" => {
            isw.set_usb_backlight(UsbBacklightKind::Half);
        }
        "full" => {
            isw.set_usb_backlight(UsbBacklightKind::Full);
        }
        _ => {
            panic!("Unrecognized option {}", backlight);
        }
    }
}

fn run_battery(battery: u8, isw : & mut IswRsBase) {
    if battery >= 20 && battery <= 100 {
        isw.set_battery_threshold(battery);
    }else {
        panic!("Cannot set value {}", battery);
    }
}

fn run_read(address: u64, isw : & mut IswRsBase) {
    println!("Value: {}", isw.raw_access.read_hw(address));
}

fn run_write(address: u64, value : u16, isw : & mut IswRsBase) {
    isw.raw_access.write_hw(address, value);
}

fn run_get_battery(isw : & mut IswRsBase) {
    println!("Battery threshold: {}", isw.get_battery_threshold());
}

fn run_get_backlight(isw : & mut IswRsBase) {
    match isw.get_usb_backlight() {
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

fn run_get_boost(isw : & mut IswRsBase) {
    match isw.get_cooler_boost() {
        true => {
            println!("Coolerboost is on")
        }
        false => {
            println!("Coolerboost is off")
        }
    }
}

fn run_getters(getter: StateGetter,  isw : & mut IswRsBase) {
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

fn parse() {
    let opts: Opts = Opts::parse();

    let mut isw = IswRsBase::new(opts.config);
    match opts.boost {
        None => {}
        Some(boost) => {
            run_boost(boost, &mut isw);
        }
    }
    match opts.usb_backlight {
        None => {}
        Some(backlight) => {
            run_backlight(backlight, &mut isw);
        }
    }
    match opts.battery {
        None => {}
        Some(battery) => {
            run_battery(battery, &mut isw);
        }
    }
    match opts.raw {
        Raw::Write(write) => {
            run_write(write.address, write.value, &mut isw);
        }
        Raw::Read(read) => {
            run_read(read.address, &mut isw);
        }
        Raw::Get(getter) => {
            run_getters(getter, &mut isw);
        }
    }
}

fn main() {
    parse();
}
