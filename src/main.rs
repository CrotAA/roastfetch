// src/main.rs
mod roasts; // 声明模块（必须有这行！）

use sysinfo::System;
use rand::seq::SliceRandom;
use std::process::Command;
use std::str::FromStr;
use crate::roasts::Roasts; // 引入咱们的核弹库

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // 随机 ASCII art
    let arts = [
        r#"
      ██████╗  █████╗  █████╗ ███████╗████████╗
     ██╔══██╗██╔══██╗██╔══██╗██╔════╝╚══██╔══╝
     ██████╔╝███████║███████║███████╗   ██║   
     ██╔══██╗██╔══██║██╔══██║╚════██║   ██║   
     ██║  ██║██║  ██║██║  ██║███████║   ██║   
     ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝   ╚═╝   
             ROASTFETCH v0.1 - 嘴臭你的系统
    "#,
        r#"
    ⢀⣴⠾⠛⠛⠻⢿⣦⡀
   ⣠⠿⠶⠛⠛⠓⠂⠈⠙⠻⣦⡀
  ⣰⠏⠄⠄⠄⠄⠄⠄⠄⠄⠄⠈⠻⣦
 ⣴⠁⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠈⢿⣷
 ⢸⣿⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠸⣿
 ⢸⣿⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢠⣿
 ⠸⣿⣆⠄⠄⠄⠄⠄⠄⢀⣠⣼⣿
 ⠀⢻⢿⣷⣶⣤⣤⣶⣾⣿⡿⠟
        准备好被毒舌了吗？♡
    "#,
    ];
    println!("{}", arts.choose(&mut rand::thread_rng()).unwrap());

    let username = whoami();
    let os_info = os_pretty_name();
    let de = desktop_environment();

    // 内存信息 + 计算百分比
    let total_mem = sys.total_memory() / 1024 / 1024;
    let used_mem = sys.used_memory() / 1024 / 1024;
    let mem_percent = used_mem as f64 / total_mem as f64 * 100.0;
    let mem_str = format!("{:.1}% ({}/{} GB)", mem_percent, used_mem, total_mem);

    // 硬盘信息 + 解析可用 GB（用于动态 roast）
    let (disk_str, disk_avail_gb) = parse_disk_gb();

    let pkgs = package_count();

    // 基本信息打印
    println!("  \x1b[91m{}\x1b[0m@\x1b[91m废物认证通过\x1b[0m", username);
    println!("  \x1b[95mOS\x1b[0m → {} \x1b[90m(2025年了，还在用？丢人)\x1b[0m", os_info);
    println!("  \x1b[92mDE/WM\x1b[0m → {} \x1b[90m(审美水平：幼儿园)\x1b[0m", de);
    println!("  \x1b[94m内存\x1b[0m → {} \x1b[90m(电脑在求饶)\x1b[0m", mem_str);
    println!("  \x1b[96m硬盘\x1b[0m → {} \x1b[90m(该删资源了兄弟)\x1b[0m", disk_str);
    println!("  \x1b[93m软件包\x1b[0m → {} 个 \x1b[90m(你这是仓库还是电脑？)\x1b[0m", pkgs);

    // ==============================
    // 重头戏：动态精准毒舌启动！
    // ==============================
    let roasts = Roasts::new();
    let dynamic_roasts = roasts.get_roast(
        mem_percent,
        disk_avail_gb,
        pkgs,
        &de,
        &os_info,
        &username,
    );

    println!(); // 空行
    for roast in dynamic_roasts {
        println!("  \x1b[91m» {}\x1b[0m", roast);
    }

    println!("\n  \x1b[95m♥ \x1b[91m你已经成功被 roast 了，明天再来哦~\x1b[0m");
}

// ================== 下面这些函数全稳 ==================

fn whoami() -> String {
    Command::new("whoami")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_owned())
        .unwrap_or_else(|| "unknown_loser".to_string())
}

fn os_pretty_name() -> String {
    std::fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|content| {
            content.lines()
                .find(|l| l.starts_with("PRETTY_NAME="))
                .and_then(|l| l.split_once('='))
                .map(|(_, v)| v.trim_matches('"').to_string())
        })
        .unwrap_or_else(|| "Windows 11（赶紧换 Linux）".to_string())
}

fn desktop_environment() -> String {
    std::env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| std::env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "裸奔终端（硬核但孤独）".to_string())
}

fn parse_disk_gb() -> (String, u64) {
    if let Ok(output) = Command::new("df").arg("-BG").arg("/").output() {
        if let Ok(s) = String::from_utf8(output.stdout) {
            let lines: Vec<&str> = s.lines().collect();
            if lines.len() > 1 {
                let stats: Vec<&str> = lines[1].split_whitespace().collect();
                if stats.len() >= 4 {
                    let avail = stats[3].trim_end_matches('G').parse::<u64>().unwrap_or(0);
                    let total = stats[1].trim_end_matches('G').parse::<u64>().unwrap_or(0);
                    let str_info = format!("{}G 可用 / {}G 总", avail, total);
                    return (str_info, avail);
                }
            }
        }
    }
    ("未知（反正满了）".to_string(), 0)
}

fn package_count() -> usize {
    let managers = [
        ("dpkg", "dpkg --get-selections 2>/dev/null | grep -v deinstall | wc -l"),
        ("rpm", "rpm -qa 2>/dev/null | wc -l"),
        ("pacman", "pacman -Q 2>/dev/null | wc -l"),
        ("flatpak", "flatpak list --user 2>/dev/null | wc -l || echo 0"),
        ("snap", "snap list 2>/dev/null | tail -n +2 | wc -l || echo 0"),
    ];

    let mut total = 0;
    for (cmd, count_cmd) in managers {
        if Command::new("which").arg(cmd).status().ok().map_or(false, |s| s.success()) {
            if let Ok(out) = Command::new("sh").arg("-c").arg(count_cmd).output() {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    if let Ok(n) = usize::from_str(s.trim()) {
                        total += n;
                    }
                }
            }
        }
    }
    total
}