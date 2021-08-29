use crate::isw_rs_base::IswRsBase;

mod isw_rs_base;
mod isw_raw_access;
mod isw_config_ops;

fn main() {
    let mut test = IswRsBase::new("isw.conf".to_string());

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    if args.get(0).unwrap() == "on" {
        test.set_cooler_boost(true);
    }
    else if args.get(0).unwrap() == "off" {
        test.set_cooler_boost(false);
    }


    print!("{}\n", test.raw_access.read_hw(0x98))
}
