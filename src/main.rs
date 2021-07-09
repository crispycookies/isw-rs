use crate::isw_rs_base::IswRsBase;

mod isw_rs_base;

fn main() {
    let mut test = IswRsBase::new("isw.conf".to_string());
    test.load_config();
    test.set_cooler_boost(true);
}
