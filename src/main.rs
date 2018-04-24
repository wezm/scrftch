//! Attept to write screenfetch in Rust

#[macro_use] extern crate prettytable;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate ini;
extern crate sys_info;

mod unix;
mod windows;
mod logos;

use prettytable::Table;
use prettytable::format;

fn main() {
    let mut info = Vec::new();
    let mut os_name = String::new();

    if cfg!(unix) {
        info = unix::info();
        os_name = unix::get_os_release().get_from(None::<String>, "ID").unwrap_or("unix").to_owned();
    } else if cfg!(windows) {
        info = windows::info();
        os_name.push_str("windows");
    }

    let mut info_s = String::new();

    for i in &info {
        if i[0].is_empty() {
            info_s.push_str(&format!("{}\n", i[1]));
        } else {
            info_s.push_str(&format!("{}: {}\n", i[0], i[1]));
        }
    }

    print_output(&get_logo(&os_name), &info_s);
}

/// Get distro logo
fn get_logo(distro: &str) -> String {
    logos::LOGOS.get(distro).unwrap().to_string()
}

/// Print result to stdout
fn print_output(logo: &str, info_s: &str) {    
    println!();
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(row![&logo, &info_s]);
    table.printstd();
    println!();
}
