extern crate ini;

use ini::Ini;
use std::collections::HashMap;

pub fn parse() -> Result<String, &'static str> {
    let i = Ini::load_from_file("src/connect_database/config.ini").map_err(|_| "Failed")?;
    let mut parse_map = HashMap::new();

    if let Some(section) = i.section(Some("postgresql")) {
        for (key, value) in section.iter() {
            parse_map.insert(key, value);
        }
    } else {
        return Err("Section ini file not find");
    }

    let host = parse_map.get("host").ok_or("Host not found in config file")?;
    let user = parse_map.get("username").ok_or("Username not found in config file")?;
    let password = parse_map.get("password").ok_or("Password not found in config file")?;
    let dbname = parse_map.get("database").ok_or("Database name not found in config file")?;

    Ok(format!("host={} user={} password={} dbname={}", host, user, password, dbname))
}