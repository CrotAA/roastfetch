// src/main.rs
mod roasts;

use sysinfo::System;
use rand::seq::SliceRandom;
use std::process::Command;
use crate::roasts::Roasts;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    
let title = [
    "\x1b[38;5;196m██████╗ \x1b[38;5;202m ██████╗ \x1b[38;5;208m █████╗ \x1b[38;5;214m███████╗\x1b[38;5;220m████████╗\x1b[0m",
    "\x1b[38;5;196m██║  ██║\x1b[38;5;202m██╔═══██╗\x1b[38;5;208m██╔══██╗\x1b[38;5;214m██╔════╝\x1b[38;5;220m╚══██╔══╝\x1b[0m",
    "\x1b[38;5;196m██████╔╝\x1b[38;5;202m██║   ██║\x1b[38;5;208m███████║\x1b[38;5;214m███████╗  \x1b[38;5;220m██║   \x1b[0m",
    "\x1b[38;5;196m██╔══██╗\x1b[38;5;202m██║   ██║\x1b[38;5;208m██╔══██║\x1b[38;5;214m╚════██║  \x1b[38;5;220m██║   \x1b[0m",
    "\x1b[38;5;196m██║  ██║\x1b[38;5;202m╚██████╔╝\x1b[38;5;208m██║  ██║\x1b[38;5;214m███████║  \x1b[38;5;220m██║   \x1b[0m",
    "\x1b[38;5;196m╚═╝  ╚═╝\x1b[38;5;202m ╚═════╝ \x1b[38;5;208m╚═╝  ╚═╝\x1b[38;5;214m╚══════╝  \x1b[38;5;220m╚═╝   \x1b[0m",
    "               \x1b[1;38;5;226mROASTFETCH v1.1\x1b[0m \x1b[91m- 嘴臭你的破系统\x1b[0m",
];
    for line in title.iter() {
        println!("{}", line);
    }

    // 经典 neofetch 彩条（每次运行顺序随机）
    let mut palette = vec![
        "\x1b[41m  ", "\x1b[42m  ", "\x1b[43m  ", "\x1b[44m  ",
        "\x1b[45m  ", "\x1b[46m  ", "\x1b[47m  ", "\x1b[101m  ",
    ];
    palette.shuffle(&mut rand::thread_rng());
    println!();
    for block in &palette {
        print!("{}", block);
    }
    println!("\x1b[0m\n");

    let username = whoami();
    let os_info = os_pretty_name();
    let de = desktop_environment();

    let total_mem = sys.total_memory() / 1024 / 1024;
    let used_mem = sys.used_memory() / 1024 / 1024;
    let mem_percent = used_mem as f64 / total_mem as f64 * 100.0;
    let mem_str = format!("{:.1}% ({}/{} GB)", mem_percent, used_mem, total_mem);

    let (disk_str, disk_avail_gb) = parse_disk_gb();
    let pkgs = package_count();

    // 每行颜色随机
    let colors = ["\x1b[91m", "\x1b[92m", "\x1b[93m", "\x1b[94m", "\x1b[95m", "\x1b[96m"];
    let mut rng = rand::thread_rng();

    println!("  {}{}\x1b[0m@\x1b[91m废物认证通过\x1b[0m", colors.choose(&mut rng).unwrap(), username);
    println!("  {}OS\x1b[0m → {} \x1b[90m(2025年了，还在用？丢人)\x1b[0m", colors.choose(&mut rng).unwrap(), os_info);
    println!("  {}DE/WM\x1b[0m → {} \x1b[90m(审美被狗吃了)\x1b[0m", colors.choose(&mut rng).unwrap(), de);
    println!("  {}内存\x1b[0m → {} \x1b[90m(电脑快炸了)\x1b[0m", colors.choose(&mut rng).unwrap(), mem_str);
    println!("  {}硬盘\x1b[0m → {} \x1b[90m(该删资源了兄弟)\x1b[0m", colors.choose(&mut rng).unwrap(), disk_str);
    println!("  {}软件包\x1b[0m → {} 个 \x1b[90m(你这是电脑还是仓库？)\x1b[0m", colors.choose(&mut rng).unwrap(), pkgs);

    // 动态精准毒舌（根据真实数据变化）
    let roasts = Roasts::new();
    let dynamic_roasts = roasts.get_roast(
        mem_percent,
        disk_avail_gb,
        pkgs,
        &de,
        &os_info,
        &username,
    );

    println!();
    for roast in dynamic_roasts {
        println!("   \x1b[91m▶ {}\x1b[0m", roast);
    }

    println!("\n   \x1b[1;38;5;196mR\x1b[38;5;202mO\x1b[38;5;208mA\x1b[38;5;214mS\x1b[38;5;220mT\x1b[38;5;226mE\x1b[38;5;190mD \x1b[91m成功！明天继续来送\x1b[0m");
}

// ================== 以下函数一个都不能少 ==================

fn whoami() -> String {
    Command::new("whoami")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
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
            if let Some(line) = s.lines().nth(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let total = parts[1].trim_end_matches('G').parse::<u64>().unwrap_or(0);
                    let avail = parts[3].trim_end_matches('G').parse::<u64>().unwrap_or(0);
                    return (format!("{}G 可用 / {}G 总", avail, total), avail);
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

    let mut total = 0usize;
    for (cmd, count_cmd) in managers {
        if Command::new("which").arg(cmd).status().map_or(false, |s| s.success()) {
            if let Ok(out) = Command::new("sh").arg("-c").arg(count_cmd).output() {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    if let Ok(n) = s.trim().parse::<usize>() {
                        total += n;
                    }
                }
            }
        }
    }
    total
}