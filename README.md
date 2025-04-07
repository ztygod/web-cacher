# 🌐 web-cacher

> 异步并发的网页监控与内容缓存工具 - 用 Rust 打造你的第一个实用 CLI 项目！

![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)
![Status](https://img.shields.io/badge/status-active-success?style=flat-square)

---

## ✨ 项目简介

`web-cacher` 是一个基于 Rust 的异步命令行工具，用于定时监控多个网页的可访问性、内容更新，并将其缓存到本地。它适用于开发者对文档、博客、服务状态的轻量级监控场景。

这个项目旨在帮助你掌握 Rust 的以下核心能力：

- 异步编程（`async` / `await`）
- 多线程与并发
- 智能指针（`Arc`, `Mutex`, `Box` 等）
- 生命周期管理
- CLI 构建、配置解析与项目结构化设计

---

## 🚀 功能特性

- ✅ 异步并发抓取网页内容
- ✅ 定时任务轮询监控网页
- ✅ 网页内容变化检测与缓存
- ✅ 支持配置文件（TOML）指定目标和间隔
- ✅ 健壮的错误处理与日志记录
- ✅ 内容保存本地，可作为简易网页备份工具

---

## 🛠️ 使用方法

### 1. 克隆项目

```bash
git clone https://github.com/your-username/web-cacher.git
cd web-cacher
```

### 2. 编辑配置文件

编辑项目根目录下的 `config.toml`：

```toml
[[targets]]
url = "https://example.com"
interval_secs = 60

[[targets]]
url = "https://docs.rs"
interval_secs = 120
```

### 3. 构建并运行

```bash
cargo run --release
```

你也可以使用命令行参数覆盖配置文件：

```bash
cargo run -- --config config.toml
```

---

## 📦 依赖技术栈

| 分类 | 使用库 | 说明 |
|------|--------|------|
| 异步运行时 | [tokio](https://crates.io/crates/tokio) | 主流异步执行器 |
| HTTP 客户端 | [reqwest](https://crates.io/crates/reqwest) | 异步网页请求 |
| CLI 解析 | [clap](https://crates.io/crates/clap) | 命令行参数处理 |
| 配置解析 | [serde](https://crates.io/crates/serde), [toml](https://crates.io/crates/toml) | 结构化配置加载 |
| 并发缓存 | [dashmap](https://crates.io/crates/dashmap) | 并发哈希表 |
| 日志追踪 | [tracing](https://crates.io/crates/tracing) | 日志与跟踪 |
| 错误处理 | [anyhow](https://crates.io/crates/anyhow) | 错误处理统一封装 |

---

## 📁 项目结构

```bash
web-cacher/
├── src/
│   ├── main.rs           # 程序入口
│   ├── config.rs         # 配置加载
│   ├── fetcher.rs        # 网页抓取逻辑
│   ├── cache.rs          # 缓存逻辑
│   ├── scheduler.rs      # 任务调度器
│   └── utils.rs          # 工具模块
├── config.toml           # 目标网页配置
├── Cargo.toml
└── README.md
```

---

## 📸 项目
```
[12:00:00] ✅ https://example.com - 200 OK - 内容未更新
[12:00:01] 🔄 https://docs.rs - 内容有变动，已缓存更新
```

---

## 🧠 学到什么？

这个项目能让你：

- 学会在 Rust 中使用异步执行器和并发模型
- 实战使用智能指针和生命周期
- 掌握 Rust 项目模块化和结构设计
- 熟悉主流三方库的组合使用方式
- 编写实用的 CLI 工具

