//! Attept to write screenfetch in Rust
#[macro_use] extern crate prettytable;
#[macro_use] extern crate lazy_static;
extern crate ini;
extern crate sys_info;
extern crate psutil;

use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::collections::HashMap;

use prettytable::Table;
use prettytable::format;
use ini::Ini;

static DISTROS: &'static [&str] = &["linux", "debian", "ubuntu", "manjaro"];

lazy_static! {
    static ref LOGOS: HashMap <&'static str, &'static str> = {
        let mut logos: HashMap<&'static str, &'static str> = HashMap::new();

        logos.insert("linux", include_str!("logos/linux.txt"));
        logos.insert("debian", include_str!("logos/debian.txt"));
        logos.insert("ubuntu", include_str!("logos/ubuntu.txt"));
        logos.insert("manjaro", include_str!("logos/manjaro.txt"));

        /*
        for distro in DISTROS {
            let f: str = &format!("logos/{}.txt", distro).as_ref();
            logos.insert(distro, include_str!(f));
        }
        */

        logos
    };
}

fn get_logo(distro: &str) -> String {
    LOGOS.get(distro).unwrap().to_string()
}


fn print_output(logo: &str, info_s: &str) {    
    println!();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(row![&logo, &info_s]);
    table.printstd();

    println!();
}


fn main() {
    let mut info = Vec::new();

    match env::var("USERNAME") {
        Ok(username) => { info.push(vec!("".to_owned(), format!("{}@{}", username, sys_info::hostname().unwrap()))); },
        Err(e) => println!("{}", e),
    }

    let os_release = Ini::load_from_file("/etc/os-release").unwrap();

    info.push(vec!("OS".to_owned(), os_release.get_from(None::<String>, "PRETTY_NAME").unwrap().to_string()));
    info.push(vec!("Kernel".to_owned(), format!("{} {}", sys_info::os_type().unwrap(), sys_info::os_release().unwrap())));
    info.push(vec!("Uptime".to_owned(), format!("{} sec", psutil::system::uptime().to_string())));

    match env::var("XDG_CURRENT_DESKTOP") {
        Ok(name) => { info.push(vec!("DE".to_owned(), name))},
        Err(e) => println!("{}", e),
    }

    let mut info_s = String::new();

    for i in &info {
        if i[0].is_empty() {
            info_s.push_str(&format!("{}\n", i[1]));
        } else {
            info_s.push_str(&format!("{}: {}\n", i[0], i[1]));
        }
    }

    print_output(&get_logo(os_release.get_from(None::<String>, "ID").unwrap()), &info_s);
}
