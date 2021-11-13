use clap::{AppSettings, Clap};

/// ISW-clone written in Rust
#[derive(Clap, Clone)]
#[clap(version = "0.1", author = "Tobias Egger")]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
pub struct Opts {
    /// Use custom isw-config file
    #[clap(short, long, default_value = "isw.conf")]
    pub(crate) config: String,
    /// Raw Access(Manually Reading and Writing values from/to the Controller)
    #[clap(subcommand)]
    pub(crate) raw: Raw,
}

#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
pub enum Raw {
    /// Write to Controller
    #[clap(version = "1.3", author = "Tobias Egger")]
    Write(WriteHandler),
    /// Read from Controller
    #[clap(version = "0.1", author = "Tobias Egger")]
    Read(ReadHandler),
    /// Read from Controller
    #[clap(version = "0.1", author = "Tobias Egger")]
    Get(StateGetter),
    /// Read CPU-Data
    #[clap(version = "0.1", author = "Tobias Egger")]
    CPU(CPUHandler),
    /// Read GPU-Data
    #[clap(version = "0.1", author = "Tobias Egger")]
    GPU(GPUHandler),
    /// Misc Functions(e.g Coolerboost)
    #[clap(version = "0.1", author = "Tobias Egger")]
    MISC(MiscHandler),
}

/// Subcommand for Writing to Controller
#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
pub struct WriteHandler {
    /// Address where value will be written to
    #[clap(short)]
    pub(crate) address: u64,
    /// Value to be written
    #[clap(long)]
    pub(crate) value: u16,
}

/// Subcommand for Misc-Fucntions like Coolerboost
#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
pub struct MiscHandler {
    /// Enables Coolerboost with 'on', disables Coolerboost with 'off'
    #[clap(short, long)]
    pub(crate) boost: Option<String>,
    /// Sets USB-backlight; 'off' for off, 'half' for half-strength, 'full' for full-strength
    #[clap(short, long)]
    pub(crate) usb_backlight: Option<String>,
    /// Sets Battery-Charging threshold; Accepts any value between 20 and 100
    #[clap(long)]
    pub(crate) battery: Option<u8>,
}

/// Subcommand for Reading from Controller
#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
pub struct ReadHandler {
    /// Address to read from
    #[clap(short)]
    pub(crate) address: u64,
}

/// Subcommand for getting States from Controller
#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
pub struct StateGetter {
    /// Gets state of Coolerboost
    #[clap(long)]
    pub(crate) boost: bool,
    /// SeGets state of ts USB-backlight
    #[clap(short, long)]
    pub(crate) usb_backlight: bool,
    /// Gets Battery-Charging threshold
    #[clap(long)]
    pub(crate) battery: bool,
}

/// Subcommand for Reading CPU-Data
#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
pub struct CPUHandler {
    /// CPU-Temperature
    #[clap(short, long)]
    pub(crate) temperature: bool,
    /// CPU-Fan RPM
    #[clap(short, long)]
    pub(crate) rpm: bool,
    /// CPU-Fan Speed
    #[clap(short, long)]
    pub(crate) speed: bool,
}

/// Subcommand for Reading CPU-Data
#[derive(Clap, Clone)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
pub struct GPUHandler {
    /// CPU-Temperature
    #[clap(short, long)]
    pub(crate) temperature: bool,
    /// CPU-Fan RPM
    #[clap(short, long)]
    pub(crate) rpm: bool,
    /// CPU-Fan Speed
    #[clap(short, long)]
    pub(crate) speed: bool,
}