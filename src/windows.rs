//! Information parser for windows systems

use std::env;

/// Windows information
pub fn info() -> Vec<Vec<String>>{
    let mut info = Vec::new();

    info.push(vec!("".to_owned(), format!("{}@{}", get_username(), get_hostname())));

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
    match env::var("COMPUTERNAME") {
        Ok(desktop_name) => desktop_name,
        Err(_) => "unknown".to_owned()
    }
}
