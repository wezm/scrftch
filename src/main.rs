//! Attept to write screenfetch in Rust
#![feature(attr_literals)]

#[macro_use] extern crate prettytable;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rust_embed;
#[macro_use] extern crate log;
extern crate regex;
extern crate ini;
extern crate sys_info;

mod unix;
mod windows;

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
    } else {
        info.push(vec!("Unexpected Error".to_owned(), "Unknown OS".to_owned()));
        os_name = "unknown".to_owned();
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

#[derive(RustEmbed)]
#[folder("res/logos/")]
struct Logo;

/// Get distro logo
fn get_logo(distro: &str) -> String {
    String::from_utf8(Logo::get(distro).unwrap_or(Logo::get("unknown").unwrap_or("UNKNOWN".as_bytes().to_vec()))).unwrap()
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
