# Web Cacher 🚀

[![Crates.io](https://img.shields.io/crates/v/web-cacher)](https://crates.io/crates/web-cacher)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

一个高性能的网页监控与内容存档工具，支持智能缓存、变化检测和可视化报告。适用于开发者、内容维护者和数字存档场景。

## ✨ 核心特性

- **智能监控**：CSS选择器定位+元素过滤，精准捕获目标内容变化
- **多级缓存**：内存+磁盘双缓存，支持版本回溯与差异对比
- **可视化报告**：自动生成HTML仪表盘，含变化时间线统计图表
- **企业级功能**：
  - 代理支持 (HTTP/SOCKS5)
  - 请求头定制
  - 触发脚本执行
  - Prometheus监控指标
- **高性能**：基于Tokio的异步运行时，实测可稳定监控1000+个页面

## 📦 安装方式

### 二进制安装 (Linux/macOS)
```bash
curl -fsSL https://raw.githubusercontent.com/yourname/web-cacher/install.sh | bash
```

### Cargo 安装
```bash
cargo install web-cacher --features "full"
```

### Docker 运行
```bash
docker run -v $(pwd)/config.toml:/app/config.toml ghcr.io/yourname/web-cacher:latest
```

## 🛠️ 快速开始

1. 创建配置文件：
```bash
web-cacher init > config.toml
```

2. 编辑配置文件：
```toml
[global]
cache_dir = "~/.web-cacher"
log_level = "info"

[[targets]]
url = "https://example.com/blog"
check_interval = 300  # 5分钟检查一次
css_selector = "article"  # 只监控文章区域
ignore_hash = [".sidebar", "footer"]  # 忽略侧边栏和页脚
```

3. 启动监控服务：
```bash
web-cacher run --daemon
```

4. 查看变化报告：
```bash
web-cacher report --output report.html
```

## 📝 命令大全

| 命令                     | 描述                       |
| ------------------------ | -------------------------- |
| `run [--daemon]`         | 启动监控服务（可后台运行） |
| `check <URL> [--json]`   | 手动检查指定URL            |
| `cache clean [--keep=5]` | 清理旧缓存版本             |
| `cache list [--filter]`  | 查看缓存内容               |
| `report [--output]`      | 生成监控报告               |
| `completions <SHELL>`    | 生成shell自动补全          |

## 🧩 高级功能

### 触发脚本示例
当检测到变化时执行自定义脚本：
```toml
[[targets]]
url = "https://example.com"
trigger_script = """
#!/bin/sh
echo "Change detected at $(date)" | mail -s "Web Update" admin@example.com
"""
```

### Prometheus监控
启动指标端点：
```bash
web-cacher run --metrics-addr 0.0.0.0:9090
```

示例查询：
```
web_cacher_changes_total{url="https://example.com"}
```

## 🏗️ 开发指南

### 构建要求
- Rust 1.70+
- OpenSSL/LibreSSL开发库

### 特性开关
| 特性         | 描述                 |
| ------------ | -------------------- |
| `full`       | 启用所有功能（默认） |
| `minimal`    | 仅基础监控功能       |
| `prometheus` | 监控指标支持         |
| `docker`     | 容器化构建支持       |

### 测试运行
```bash
cargo test --all-features
cargo run --example demo
```

## 🌍 应用场景

- **博客监控**：追踪技术博客更新，自动生成RSS
- **文档存档**：定期保存项目文档版本
- **竞品分析**：监控竞品网站内容变化
- **数字取证**：司法场景下的网页证据保存

## 🤝 贡献指南

欢迎提交PR！请遵循以下流程：
1. Fork 仓库
2. 创建特性分支 (`git checkout -b feat/awesome-feature`)
3. 提交更改 (`git commit -am 'Add awesome feature'`)
4. 推送到分支 (`git push origin feat/awesome-feature`)
6. 创建Pull Request

