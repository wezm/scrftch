//! Information parser for unix-like systems

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::env;

use sys_info;
use ini::Ini;

/// Unix information
pub fn info() -> Vec<Vec<String>>{
    let mut info = Vec::new();

    info.push(vec!("".to_owned(), format!("{}@{}", get_username(), get_hostname())));

    let os_release = get_os_release();
    info.push(vec!("OS".to_owned(), os_release.get_from(None::<String>, "PRETTY_NAME").unwrap().to_string()));

    info.push(vec!("Kernel".to_owned(), format!("{} {}", sys_info::os_type().unwrap(), sys_info::os_release().unwrap())));

    let uptime = get_uptime();
    info.push(vec!("Uptime".to_owned(), format!("{}d {}h {}m", uptime/86400, uptime%86400/3600, uptime%3600/60)));

    info.push(vec!("Shell".to_owned(), get_shell()));    

    info.push(vec!("DE".to_owned(), get_desktop()));

    info
}

/// Parse computer hostname from /etc/hosts
fn get_hostname() -> String {
    let mut hostname = String::new();
    match File::open("/etc/hostname") {
        Ok(mut f) => { f.read_to_string(&mut hostname).unwrap(); },
        Err(_) => { hostname.push_str("unknown"); }
    }

    hostname.replace("\n", "")
}

/// Get current username
fn get_username() -> String {
    match env::var("USER") {
        Ok(username) => username,
        Err(_) => "unknown".to_owned()
    }

    /*
    match Command::new("whoami").output() {
        Ok(username) => String::from_utf8(username.stdout).unwrap().replace("\n", ""),
        Err(_) => "unknown".to_owned()
    }
    */
}

/// Get distro info
pub fn get_os_release() -> Ini {
    match Ini::load_from_file("/etc/os-release") {
        Ok(i) => i,
        Err(_) => Ini::new()
    }
}

/// Get the system uptime from /proc/uptime
pub fn get_uptime() -> usize {
    match File::open("/proc/uptime") {
        Ok(mut f) => {
            let mut proc_uptime = String::new();
            f.read_to_string(&mut proc_uptime).unwrap();

            let a: Vec<&str> = proc_uptime.split(".").collect();

            a[0].parse::<usize>().unwrap_or(0)
        },
        Err(_) => 0
    }
}

/// Get memory info from /proc/meminfo
fn get_memory() -> HashMap<String, String> {
    HashMap::new()
}

/// Get default shell path
fn get_shell() -> String {
    match env::var("SHELL") {
        Ok(shell_name) => shell_name,
        Err(_) => "unknown".to_owned()
    }
}

/// Get current desktop environment name
fn get_desktop() -> String {
    env::var("XDG_DESKTOP_SESSION")
        .or_else(|_| env::var("XDG_CURRENT_DESKTOP"))
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .ok()
        .unwrap_or_else(|| "unknown".to_owned())
}
