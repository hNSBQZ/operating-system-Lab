use alloc::vec::Vec;
use lazy_static::*;

lazy_static! {
        static ref APP_NAMES: Vec<&'static str> = {
                    let num_app = get_num_app();
                            extern "C" { fn _app_names(); }
                                    let mut start = _app_names as usize as *const u8;
                                            let mut v = Vec::new();
                                                    unsafe {
                                                                    for _ in 0..num_app {
                                                                                        let mut end = start;
                                                                                                        while end.read_volatile() != '\0' as u8 {
                                                                                                                                end = end.add(1);
                                                                                                                                                }
                                                                                                                        let slice = core::slice::from_raw_parts(start, end as usize - start as usize);
                                                                                                                                        let str = core::str::from_utf8(slice).unwrap();
                                                                                                                                                        v.push(str);
                                                                                                                                                                        start = end.add(1);
                                                                                                                                                                                    }
                                                                            }
                                                            v
                                                                    };
}

#[allow(unused)]
pub fn get_app_data_by_name(name: &str) -> Option<&'static [u8]> {
        let num_app = get_num_app();
            (0..num_app)
                        .find(|&i| APP_NAMES[i] == name)
                                .map(|i| get_app_data(i))
}

pub fn list_apps() {
        println!("/**** APPS ****");
            for app in APP_NAMES.iter() {
                        println!("{}", app);
                            }
                println!("**************/");
}
