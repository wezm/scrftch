//! Information parser for windows systems

use std::env;
use std::process::Command;

/// Windows information
pub fn info() -> Vec<Vec<String>>{
    let mut info = Vec::new();

    info.push(vec!("".to_owned(), format!("{}@{}", get_username(), get_hostname())));
    info.push(vec!("OS".to_owned(), get_system_name()));

    info
}

/// Get pc name
fn get_hostname() -> String {
    match env::var("COMPUTERNAME") {
        Ok(desktop_name) => desktop_name,
        Err(_) => "unknown".to_owned()
    }
}

/// Get name of current user
fn get_username() -> String {
    match env::var("username") {
        Ok(desktop_name) => desktop_name,
        Err(_) => "unknown".to_owned()
    }
}

/// Get OS name
fn get_system_name() -> String {
    match Command::new("ver").output() {
        Ok(ver) => String::from_utf8(ver.stdout).unwrap(),
        Err(_) => "unknown".to_owned()
    }
}
