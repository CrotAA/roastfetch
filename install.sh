#!/bin/bash
set -e

echo "┌────────────────────────────────────────────────────┐"
echo "│           Roastfetch 正在安装……                   │"
echo "│       你的系统即将迎来一场精准暴击洗礼             │"
echo "└────────────────────────────────────────────────────┘"

# 等你发了 v0.1.0 Release 后改成这个链接
curl -L -o roastfetch https://github.com/CrotAA/roastfetch/releases/download/v0.1.0/roastfetch
chmod +x roastfetch
sudo mv roastfetch /usr/local/bin/roastfetch

echo "安装完成！现在直接敲 roastfetch 享受被嘴臭的快感！"
echo
roastfetch || true
