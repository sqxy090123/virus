use sys_info;
use raw_cpuid::CpuId;
use raw_cpuid::CpuIdResult;
use std::fs;
use std::process::Command;
use std::sync::LazyLock;
use crate::sys;


pub fn is_virtual_machine() -> bool {
    if let Ok(b) = std::fs::read_to_string("/sys/class/dmi/id/bios_version") {
        if b.contains("VirtualBox") || b.contains("VMware") {
            println!("检测到虚拟机 BIOS: {}", b.trim());
            return true;
        }
        println!("BIOS版本: {}", b.trim());
    } else {
        println!("无法读取 BIOS 信息");
    }

    if let Ok(p) = std::fs::read_to_string("/sys/class/dmi/id/product_name") {
        if p.contains("VirtualBox") || p.contains("VMware") {
            println!("检测到虚拟机设备: {}", p.trim());
            return true;
        }
        println!("设备名称: {}", p.trim());
    } else {
        println!("无法读取设备信息");
    }

    if let Ok(m) = sys_info::mem_info() {
        if m.total < 2048 {
            println!("检测到内存过小，可能运行在虚拟机或沙盒中: {} MB", m.total);
            return true;
        }
        println!("内存大小: {} MB", m.total);
    } else {
        println!("无法获取内存信息");
    }

    if sys::DEVICE_MODEL.as_str().contains("VirtualBox") || sys::DEVICE_MODEL.as_str().contains("VMware") || sys::DEVICE_MODEL.as_str().contains("QEMU") || sys::DEVICE_MODEL.as_str().contains("System manufacturer") {
        println!("检测到虚拟机设备名称: {}", sys::DEVICE_MODEL.trim());
        return true;
    }
    println!("系统制造商: {}", sys::DEVICE_MODEL.trim());

    println!("未检测到虚拟机");
    false
}

fn main() {
    if is_virtual_machine() {
        println!("程序运行在虚拟机中！");
    } else {
        println!("程序运行在物理机中！");
    }
}