// src/roasts.rs
use rand::seq::SliceRandom;
use regex::Regex;
use rand::Rng;

pub struct Roasts {
    pub memory_high: Vec<&'static str>,  // >80%
    pub memory_low: Vec<&'static str>,   // <20%
    pub disk_full: Vec<&'static str>,    // <50G 可用
    pub disk_empty: Vec<&'static str>,   // >1TB
    pub pkgs_hoarder: Vec<&'static str>, // >3000
    pub pkgs_minimal: Vec<&'static str>, // <500
    pub de_gnome: Vec<&'static str>,
    pub de_kde: Vec<&'static str>,
    pub de_i3: Vec<&'static str>,
    pub de_other: Vec<&'static str>,
    pub os_fedora: Vec<&'static str>,
    pub os_arch: Vec<&'static str>,
    pub os_ubuntu: Vec<&'static str>,
    pub os_other: Vec<&'static str>,
    pub username_loser: Vec<&'static str>, // 匹配 "crot" 等
    pub general_endings: Vec<&'static str>,
}

impl Roasts {
    pub fn new() -> Self {
        Self {
            memory_high: vec![
                "内存 90%+？你开 500 个 Chrome 标签在挖矿吗？",
                "电脑内存在尖叫：求求你关掉点东西！",
                "78% 内存占用：恭喜，你的机器快成电风扇了",
                "内存爆表，建议卖肾换 SSD",
            ],
            memory_low: vec![
                "内存才 10%？你在用它冥想吗？醒醒干活！",
                "低内存占用：因为你啥也没装，空壳子",
            ],
            disk_full: vec![
                "硬盘只剩 20G？该删种子/学习资料/游戏存档了兄弟",
                "硬盘报警：空间不足以保存你的摆烂记录",
                "10G 可用？直接格式化重装吧，省事",
            ],
            disk_empty: vec![
                "1TB+ 空闲？富哥，借我点空间存代码",
                "硬盘空荡荡：你用电脑干嘛？看风景？",
            ],
            pkgs_hoarder: vec![
                "5000+ 包？这是电脑还是软件博物馆？删点吧",
                "包多到爆：你囤货是为了开超市吗？",
                "3000 包，恭喜：系统复杂度世界第一",
            ],
            pkgs_minimal: vec![
                "才 200 包？极简主义？不，贫穷主义",
                "包少得可怜：连 vim 都没装？手写代码？",
            ],
            de_gnome: vec![
                "GNOME 用户：味儿真大，扩展装满还卡",
                "GNOME？2025 年了，还在追逐鼠标阴影？",
            ],
            de_kde: vec![
                "KDE：花里胡哨，配置一天崩溃一小时",
                "KDE 党：你的桌面主题值 5000 块，但代码值 0",
            ],
            de_i3: vec![
                "i3  tiling：硬核？不，懒癌晚期",
                "i3 用户：窗口管理大师，社交零分",
            ],
            de_other: vec![
                "你的 DE：未知，估计是自制，丑爆",
                "DE 品味：-100 分，建议重学设计",
            ],
            os_fedora: vec![
                "Fedora：Red Hat 的实验田，你是小白鼠？",
                "Fedora 41？升级啊兄弟，别卡在旧版摆烂",
            ],
            os_arch: vec![
                "Arch btw：我用 Arch（已扣 50 IQ）",
                "Arch 用户：pacman -Syu 后哭一晚",
            ],
            os_ubuntu: vec![
                "Ubuntu：新手福音，老鸟噩梦",
                "Ubuntu 还在用？snap 卡死你没？",
            ],
            os_other: vec![
                "你的 OS：小众到我都不认识，牛逼",
                "OS 太旧：建议 museum 展览",
            ],
            username_loser: vec![
                "用户名 'crot'？听起来像 'croissant' 的失败版",
                "你的名字：生成器随机？难怪代码也随机",
                "crot：缩写 'Can't Run On Time'？",
            ],
            general_endings: vec![
                "今天继续摆烂，明天再废",
                "你已入选 2025 最废 Top 10",
                "关机吧，世界不需要你的代码",
                "洗澡去！键盘味儿冲天",
                "你妈喊吃饭，别摸鱼了",
                "直接 rm -rf / 吧，反正你备份都没",
                "代码写得烂，系统用得更烂",
                "建议换 Windows，匹配你的水平",
                "你存在就是 bug",
                "♥ 爱你哦，但你还是废物",
                // 加到 100 条：下面复制 70+ 条通用/变体
                "内存/硬盘双爆：你的机器在求饶",
                "包多 DE 丑：全栈废物认证",
                "Fedora + GNOME：经典摆烂组合",
                "用户名太土：改成 'pro_grok' 吧",
                "2025 年了，还不学 Rust？哦你在学呢，但写得烂",
                "你的终端：neofetch 都不配用，直接 roast",
                "硬盘满：删掉你的浏览器历史先",
                "内存低：因为你浏览器只开一个标签？进步",
                "i3 + Arch：ricer 认证，但代码呢？",
                "Ubuntu snap 党：速度慢如龟，匹配你效率",
                "KDE 效果满分，生产力零分",
                "GNOME 扩展：装了 50 个，崩溃 100 次",
                "小众 OS：你以为酷？不，孤立",
                "用户名 root？脚本小子 2025 版",
                "包少： minimalist？不，啥也不会",
                "硬盘空：买不起游戏？穷鬼",
                "内存 50%：平衡废物",
                "DE 未知：自定义？丑到爆",
                "OS Debian：稳定如你的人生，停滞",
                "Arch rolling：更新如你心情，崩溃",
                "Fedora RPM hell：自找苦吃",
                "你的 setup：Reddit 贴不了，丑",
                "代码行数：0，roast 次数：∞",
                "建议：git commit --amend 'fix: stop being loser'",
                "你：full-time procraster",
                "硬盘：你的 'later' 文件夹满了",
                "内存：思考人生时占用的",
                "包：收藏的 'someday I'll use' 垃圾",
                "DE：反射你的混乱大脑",
                "OS：选择错误，从头来过",
                "用户名：匿名，因为丢人",
                "今天目标：跑 roastfetch 100 次",
                "明天目标：还是摆烂",
                "你代码：hello world 都错",
                "系统：比你稳定多了",
                "roast over：但你的人生继续",
                "♥ 别哭，明天再来",
                "Fedora：workstation？no, waste-station",
                "GNOME：wayland 卡死，x11 也卡",
                "i3：mod+enter，新窗口新借口",
                "KDE：plasma，but your productivity plasma-0",
                "Ubuntu：PPA 地狱欢迎你",
                "Arch：wiki 是你的圣经，但你不读",
                "硬盘 100G：够存失败日志",
                "内存 100%：swap 党，贫民",
                "包 10000：apt/rpm/pacman 战争",
                "用户名 admin：懒癌",
                "root 用户：危险+蠢",
                "guest：甚至不拥有你的机器",
                "DE cinnamon：中庸丑",
                "mate：老古董",
                "xfce：轻量，但你重",
                "lxde：极简穷",
                "OS mint：cinnamon 的赝品",
                "popos：cosmic 还没好",
                "nixos：配置如炼狱",
                "gentoo：编译一周",
                "void：musl？痛苦",
                "alpine：docker 专用，别装逼",
                "你的选择：全错",
                "roast 结束：但痛永存",
                "加油？不，躺平",
                "2025：AI 取代你了",
                "代码：AI 写的更好",
                "系统：neofetch 哭了",
                "你：roastfetch 的灵感来源",
            ],
        }
    }

    pub fn get_roast(&self, mem_percent: f64, disk_avail_gb: u64, pkgs: usize, de: &str, os: &str, username: &str) -> Vec<String> {
        let mut roasts = Vec::new();

        // 内存
        if mem_percent > 80.0 {
            roasts.push(self.memory_high.choose(&mut rand::thread_rng()).unwrap().to_string());
        } else if mem_percent < 20.0 {
            roasts.push(self.memory_low.choose(&mut rand::thread_rng()).unwrap().to_string());
        }

        // 硬盘 (简单估算 GB)
        let avail_gb = if disk_avail_gb > 1000 { disk_avail_gb / 1000 } else { disk_avail_gb }; // TB to GB
        if avail_gb < 50 {
            roasts.push(self.disk_full.choose(&mut rand::thread_rng()).unwrap().to_string());
        } else if avail_gb > 1000 {
            roasts.push(self.disk_empty.choose(&mut rand::thread_rng()).unwrap().to_string());
        }

        // 包数
        if pkgs > 3000 {
            roasts.push(self.pkgs_hoarder.choose(&mut rand::thread_rng()).unwrap().to_string());
        } else if pkgs < 500 {
            roasts.push(self.pkgs_minimal.choose(&mut rand::thread_rng()).unwrap().to_string());
        }

        // DE
        let de_lower = de.to_lowercase();
        if de_lower.contains("gnome") {
            roasts.push(self.de_gnome.choose(&mut rand::thread_rng()).unwrap().to_string());
        } else if de_lower.contains("kde") || de_lower.contains("plasma") {
            roasts.push(self.de_kde.choose(&mut rand::thread_rng()).unwrap().to_string());
        } else if de_lower.contains("i3") {
            roasts.push(self.de_i3.choose(&mut rand::thread_rng()).unwrap().to_string());
        } else {
            roasts.push(self.de_other.choose(&mut rand::thread_rng()).unwrap().to_string());
        }

        // OS
        let os_lower = os.to_lowercase();
        if os_lower.contains("fedora") {
            roasts.push(self.os_fedora.choose(&mut rand::thread_rng()).unwrap().to_string());
        } else if os_lower.contains("arch") {
            roasts.push(self.os_arch.choose(&mut rand::thread_rng()).unwrap().to_string());
        } else if os_lower.contains("ubuntu") {
            roasts.push(self.os_ubuntu.choose(&mut rand::thread_rng()).unwrap().to_string());
        } else {
            roasts.push(self.os_other.choose(&mut rand::thread_rng()).unwrap().to_string());
        }

        // 用户名 (简单 regex 匹配 'crot' 或类似)
        let re = Regex::new(r"(?i)crot|loser|admin|root").unwrap();
        if re.is_match(username) {
            roasts.push(self.username_loser.choose(&mut rand::thread_rng()).unwrap().to_string());
        }

        // 随机 1-2 个通用结尾
        if roasts.len() < 3 {
            let extra = self.general_endings.choose_multiple(&mut rand::thread_rng(), rand::thread_rng().gen_range(1..=2));
            roasts.extend(extra.map(|s| s.to_string()));
        }

        roasts
    }
}