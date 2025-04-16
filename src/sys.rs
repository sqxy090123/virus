
//mod sys;

use std::process::Command;

use std::sync::LazyLock;
use winapi::um::winuser::{SetWindowsHookExW, WH_CALLWNDPROC, CallNextHookEx, UnhookWindowsHookEx, CWPSTRUCT};
use winapi::shared::windef::HHOOK;
use winapi::shared::minwindef::{LRESULT, WPARAM, LPARAM, DWORD};
use std::ptr::null_mut;
use std::sync::Mutex;

pub static DEVICE_MODEL: LazyLock<String> = LazyLock::new(|| {
    match std::env::consts::OS {
        "windows" => {
            let output = Command::new("wmic")
                .arg("csproduct")
                .arg("get")
                .arg("name")
                .output()
                .expect("Failed to execute command");
            let output_str = String::from_utf8_lossy(&output.stdout);
            let result = output_str
                .lines() // 按行分割输出
                .skip(1) // 跳过第一行（列名）
                .find(|line| !line.trim().is_empty()) // 找到第一行非空内容
                .unwrap_or("未知设备"); // 如果没有有效行，返回默认值
            result.trim().to_string()
        }
        "linux" => {
            let output = Command::new("uname")
                .arg("-n")
                .output()
                .expect("Failed to execute command");
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => "未知设备".to_string(),
    }
});


#[cfg(target_os = "windows")]
pub fn install_message_hook() -> Result<(), String> {

    static HOOK: LazyLock<Mutex<Option<std::sync::Arc<std::sync::atomic::AtomicPtr<HHOOK>>>>> = LazyLock::new(|| Mutex::new(None));

    unsafe extern "system" fn hook_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        if code >= 0 {
            let cwp = &*(l_param as *const CWPSTRUCT);
            println!("Message: {}, HWND: {:?}", cwp.message, cwp.hwnd);
        }
        CallNextHookEx(
            HOOK.lock()
                .unwrap()
                .as_ref()
                .map(|arc| *arc.load(std::sync::atomic::Ordering::SeqCst))
                .unwrap_or(null_mut()),
            code,
            w_param,
            l_param,
        )
    }

    pub fn set_hook() -> Result<(), String> {
        let hook = unsafe { SetWindowsHookExW(WH_CALLWNDPROC, Some(hook_proc), null_mut(), 0) };
        if hook.is_null() {
            Err("Failed to install hook".to_string())
        } else {
            *HOOK.lock().unwrap() = Some(std::sync::Arc::new(std::sync::atomic::AtomicPtr::new(Box::into_raw(Box::new(hook)))));
            Ok(())
        }
    }

    pub fn remove_hook() {
        if let Some(hook) = HOOK.lock().unwrap().take() {
            unsafe { UnhookWindowsHookEx(*hook.load(std::sync::atomic::Ordering::SeqCst)) };
        }
    }

    set_hook()
}

#[cfg(not(target_os = "windows"))]
pub fn install_message_hook() -> Result<(), String> {
    Err("Message hooks are only supported on Windows.".to_string())
}


pub fn installed_applications() -> Vec<String> {
    match std::env::consts::OS {
        "windows" => {
            let output = Command::new("wmic")
                .arg("product")
                .arg("get")
                .arg("name")
                .output()
                .expect("Failed to execute command");
            let output_str = String::from_utf8_lossy(&output.stdout);
            output_str
                .lines() // 按行分割输出
                .skip(1) // 跳过第一行（列名）
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        Some(trimmed.to_string())
                    } else {
                        None
                    }
                }) // 过滤空行并转换为字符串
                .collect()
        }
        "linux" => {
            // Linux 上的实现可以根据需求调整
            vec!["功能未实现".to_string()]
        }
        _ => vec!["未知应用".to_string()],
    }
}


pub fn is_superuser() -> bool {
    match std::env::consts::OS {
        "windows" => {
            // Windows 上通过检查是否以管理员权限运行
            let output = Command::new("net")
                .arg("session")
                .output();
            if let Ok(output) = output {
                output.status.success()
            } else {
                // 如果命令失败，说明没有管理员权限
                false
            }
        }
        "linux" => {
            // Linux 上通过检查是否为 root 用户
            if let Ok(output) = Command::new("id")
                .arg("-u")
                .output()
            {
                let uid = String::from_utf8_lossy(&output.stdout).trim().to_string();
                uid == "0" // root 用户的 UID 为 0
            } else {
                false
            }
        }
        _ => false, // 其他系统暂不支持
    }
}

pub fn get_superuser() -> String {
    match std::env::consts::OS {
        "windows" => {
            // 尝试以管理员权限重新启动程序
            let result = Command::new("powershell")
                .arg("-Command")
                .arg("Start-Process")
                .arg("-FilePath")
                .arg(std::env::current_exe().unwrap().to_str().unwrap())
                .arg("-Verb")
                .arg("runAs")
                .status();

            if let Ok(status) = result {
                if status.success() {
                    "程序已以管理员权限重新启动".to_string()
                } else {
                    "无法申请管理员权限，请手动以管理员权限运行程序".to_string()
                }
            } else {
                "无法申请管理员权限，请手动以管理员权限运行程序".to_string()
            }
        }
        "linux" => {
            // 提示用户使用 sudo 重新运行程序
            "请使用 sudo 重新运行此程序".to_string()
        }
        _ => "当前系统不支持申请超级用户权限".to_string(),
    }
}
