use lazy_static::lazy_static;

lazy_static! {
    pub static ref ADS: &'static [&'static str] = {
        let data = include_str!(concat!(env!("OUT_DIR"), "/ads"));
        data.lines().collect::<Vec<_>>().leak()
    };
}
