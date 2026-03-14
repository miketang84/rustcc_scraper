【Rust日报】2026-03-13


### TITLE

## Fork - 微控制器固件构建工具

Fork 是一个命令行工具，用于为任何微控制器(MCU)构建固件，无需构建工具抽象或额外配置。目前仍处于早期开发阶段(Beta版本)。

### 核心特点

- **简化构建流程**：通过简单命令 `fork build -m stm32f405` 即可构建固件
- **自动检测工具链**：从配置文件自动检测项目的工具链，构建Dockerfile并在容器内运行构建
- **无需额外配置**：项目中不需要超出构建系统本身的配置文件
- **基于Docker**：利用Docker容器实现隔离构建环境

### 主要优势

相比直接使用Docker，Fork简化了多MCU项目的构建流程：
- 传统Docker方式需要为不同MCU编写复杂的命令（包含UID映射、路径配置等）
- Fork统一为简单命令：`fork build -m rp2040` 和 `fork build -m esp32`

### 使用方法

- **构建固件**：`fork build` (自动检测MCU) 或 `fork build --mcu rp2040 --tool embassy-rp`
- **烧录固件**：`fork flash` 或 `fork flash --file ./my-firmware.uf2`

### 扩展支持

添加新的开发板支持很简单，只需在 `/boards` 目录创建TOML配置文件并提交PR

### 系统要求

需要安装Docker或Podman

### 许可证

MIT开源许可

[https://github.com/TareqRafed/fork
](https://github.com/TareqRafed/fork
)
    


### TITLE

## Burn 框架通信层优化：速度提升 5 倍超越 Rust 标准 Channel

### 背景与动机
- Burn 框架为支持复杂递归状态（如自动微分图、融合张量编译器和 CubeCL 服务器）依赖可重入互斥锁
- 锁开销在多 GPU 和多线程环境下成为性能瓶颈
- 尝试使用标准 `std::sync::mpsc` channel 替代互斥锁，但性能反而下降

### 优化成果
- **单线程场景**：互斥锁仍是最快方案（无数据复制，无竞争）
- **多线程场景**：
  - 自定义 channel 比标准 channel 快 **10 倍**
  - 比互斥锁快约 **2 倍**
  - 8 线程测试中任务执行数量提升近 **2 倍**

### 核心技术实现

#### 1. **零分配任务入队**
- 问题：闭包（closure）通常超过 1000 字节，频繁的 `Box<dyn FnOnce()>` 分配造成严重竞争
- 解决方案：采用分层双缓冲内存池（Double-Buffer Arena）
  - 小闭包（≤48 字节）：直接内联到 64 字节 Task 结构体中（CPU 缓存行对齐）
  - 大闭包（≤4KB）：使用预分配内存池，绕过全局分配器
  - 超大闭包（>4KB）：回退到标准 Box（占比极少）

#### 2. **无锁双缓冲机制**
- 采用双缓冲交换策略消除传统环形缓冲区的竞争
- 生产者使用原子操作写入客户端缓冲区
- 运行线程通过单次原子交换批量获取任务到私有服务端缓冲区
- 实现零干扰的顺序任务执行

#### 3. **线程本地存储（TLS）保证递归安全**
- 运行线程通过 TLS 检测是否向自身提交任务
- 如果是，立即同步执行而非入队，避免死锁
- 无需可重入锁的高开销

### 结论
- 标准库适用于大多数场景，但在深度了解传输对象特性时，专用实现可大幅提升性能
- 新的设备句柄（device handle）为独立设备服务提供了优雅的零开销通信通道
- 保持 `FnOnce` 接口，确保灵活性和易用性

[
https://burn.dev/blog/faster-channel/
](
https://burn.dev/blog/faster-channel/
)
    


### TITLE

## Orhun Parmaksız (orhun) - GitHub 个人主页

### 个人简介
- **身份**：开源开发者，对 Rust 语言和终端有深厚热情
- **职位**：@ratatui 的终端开发者（Terminal Chef）| @archlinux 维护者
- **地理位置**：土耳其安卡拉
- **社交媒体**：活跃于 YouTube、Mastodon、Bluesky 等平台

### GitHub 统计数据
- **加入时间**：9年前
- **提交记录**：30,827 次提交
- **持续贡献**：保持 2,590 天的连续提交记录
- **项目影响力**：
  - 开设 763 个议题
  - 提交 1,685 个拉取请求
  - 获得 21,934 个星标
  - 拥有 148 个个人项目
  - 贡献于 152 个公共仓库
- **社区影响**：4,500+ 关注者，关注 582 人

### 主要开源项目（Rust）
- **git-cliff**：高度可定制的变更日志生成器
- **binsider**：二进制文件分析的终端用户界面
- **ratzilla**：构建终端主题 Web 应用的库
- **kmon**：Linux 内核管理器和活动监视器
- **systeroid**：sysctl 的强大替代工具
- **gpg-tui**：GnuPG 的终端用户界面
- **rustypaste**：极简文件上传/粘贴服务
- **tuitar**：便携式吉他训练工具和 DIY 套件
- **menyoki**：命令行图像操作工具
- **runst**：Linux 的简单通知守护程序
- **halp**：CLI 工具帮助工具
- **daktilo**：将键盘变成打字机
- **arch-repro-status**：检查 Arch Linux 软件包可重现性状态的工具

[
https://github.com/orhun
](
https://github.com/orhun
)
    


### TITLE

## RAVEN — RISC-V 模拟器与集成开发环境

RAVEN 是一个免费开源的 RISC-V 模拟器和终端 IDE，专为学生和汇编学习者设计。它支持 RV32IMAF 指令集（基础整数集、乘除法、原子操作和单精度浮点运算），并在程序运行时可视化机器的每个部分。

### 核心功能

**编辑器（标签页 1）**
- 语法高亮显示指令、寄存器、标签、指令和字符串
- 输入时显示操作数提示
- Ctrl+R 快速汇编，错误提示显示行号和原因
- 支持撤销/重做（50级）、注释切换、行复制
- 转到定义（F12）、标签高亮、地址边栏（F2）

**调试器（运行标签页 2）**
- 自由运行、暂停（Space/F5）或单步执行（n/F10）
- 断点设置（b/F9）、跳转到地址（g）、执行跟踪（t）
- 显示全部32个整数寄存器和浮点寄存器（带ABI名称）
- RAM视图、栈视图、断点列表
- 指令内存面板显示指令类型、执行热度和分支结果
- 指令解码器：完整字段分解

**缓存模拟器（标签页 3）**
- 独立的指令缓存和数据缓存，支持多级缓存（L2、L3等）
- 可配置：组数、路数、块大小、写策略、包含策略
- 六种替换策略：LRU、FIFO、LFU、Clock、MRU、随机
- 实时统计：命中率、MPKI、RAM流量、热点未命中地址
- 学术指标：AMAT、IPC、每类指令的CPI
- 可视化矩阵视图，支持导出结果为CSV格式

**文档标签页（标签页 4）**
- 支持指令的完整参考文档
- 运行标签页快捷键指南

### ISA 覆盖范围

- **RV32I**: 基础整数指令集（算术、逻辑、加载/存储、分支、跳转等）
- **RV32M**: 乘法和除法扩展
- **RV32A**: 原子操作扩展
- **RV32F**: 单精度浮点扩展
- 支持伪指令和系统调用（ecall）

### ELF 二进制文件加载

完全兼容 riscv32im/ima-unknown-none-elf 目标编译的 ELF32 LE RISC-V 二进制文件。可以直接加载并执行由标准工具链（如Rust）编译的程序。

### 汇编器特性

- 支持 .text、.data、.bss 段
- 多种汇编指令：.byte、.word、.float、.string 等
- 支持块注释和内联注释
- 清晰的错误提示信息

### 快速开始

从 Releases 下载最新二进制文件，或使用源码构建（需要 Rust 1.75+）：
```bash
git clone https://github.com/Gaok1/Raven
cd Raven
cargo run
```

### 主要快捷键

- **Ctrl+R**: 汇编并加载
- **F5/Space**: 运行/暂停
- **F10/n**: 单步执行
- **F9/b**: 切换断点
- **1-4**: 切换标签页

[
https://github.com/Gaok1/Raven
](
https://github.com/Gaok1/Raven
)
    


### TITLE

## Rust 编程语言社区 (Reddit)

这是一个专注于 Rust 编程语言的 Reddit 社区页面，Rust 是一种强调**性能、可靠性和生产力**的开源系统编程语言。

### 社区规则

1. **遵守行为准则** - 尊重、耐心、善意和同理心对待他人，遵守 Rust 项目行为准则

2. **内容相关性** - 帖子必须与 Rust 相关，标题需包含有用上下文；Rust 问题请使用置顶的问答帖；周末允许发布艺术作品类帖子

3. **建设性批评** - 鼓励批评但必须具有建设性、有用且可操作；批评 GitHub 项目时不得直接链接问题追踪器

4. **保持理性** - 不要过度激动，避免狂热主义，以善意理解他人

5. **避免重复讨论** - 避免翻旧账或讨论已充分讨论过的话题；避免过度纠结细节

6. **禁止低质量内容** - 禁止表情包、图片宏等；使用格式化文本分享代码；疑似 AI 生成的内容可能被删除

### 有用资源

- **官方资源**：官方网站、博客、安装程序、源代码、问题追踪器
- **学习资源**：Rust 电子书、标准库 API 参考、示例代码、在线练习场
- **讨论平台**：官方用户论坛、Discord、Matrix 聊天、Stack Overflow

[
https://old.reddit.com/r/rust
](
https://old.reddit.com/r/rust
)
    


### TITLE

## Alakit - 混合桌面应用框架

Alakit 是一个将 Rust 的强大功能与 Web 技术的灵活性相结合的混合桌面应用程序框架。该项目旨在提供一个环境，让开发者可以使用 HTML 和 CSS 构建界面，而无需为应用程序逻辑编写 JavaScript。

### 项目优势

- **平衡性**：在快速原型开发的简便性和二进制文件大小之间找到平衡点
- **技术组合**：Rust 提供安全性和性能，Webview 处理渲染，声明式属性系统管理两者之间的桥接

### 核心特性

- **无 JS 方法**：按钮事件、表单提交和 UI 值更新通过 HTML 属性直接与 Rust 代码绑定
- **受保护的后端存储**：敏感数据在 Rust 端使用 AES-256-GCM 加密存储在内存中（注意：由于 JS 运行时限制，Webview 中显示的数据在 RAM 中以明文形式存在）
- **低资源占用**：专注于保持二进制文件小巧并优化运行时内存使用
- **灵活的控制器系统**：通过宏自动添加新功能（控制器），无需手动注册

### 工作原理

界面使用特殊属性与后端通信：
- **alakit-cmd**：指定要执行的 Rust 函数
- **alakit-bind**：将 HTML 元素连接到加密存储中的键
- **alakit-form**：以 JSON 格式为 Rust 端收集整个容器的数据

后台的 Rust 控制器处理消息并更新状态，立即反映在 UI 中。

### 安装和使用

- 项目目前处于开发阶段
- 需要 Rust 环境进行构建
- 可通过导航到相应文件夹并运行 'cargo run' 来启动示例应用程序
- 详细文档和示例位于 'doc' 文件夹中

[
https://github.com/fejestibi/alakit
](
https://github.com/fejestibi/alakit
)
    


### TITLE

## OCI生成器：适用于任何嵌入式工具链

该项目简化了针对不同目标平台的构建过程，提供了便捷的开发工具。

### 核心特点

- **简化构建命令**：只需使用简单的命令即可为任何目标平台构建，例如：`fork build -m esp32c3`

- **工具链集成**：可以与probe-rs配合使用，形成完整的开发周期闭环

- **项目地址**：https://github.com/TareqRafed/fork

### 主要优势

这个工具让嵌入式开发中的跨平台构建变得更加容易，特别适合需要频繁切换不同硬件目标的开发场景。

[
https://old.reddit.com/r/rust/comments/1rskv4t/oci_generator_for_any_embedded_toolchain/
](
https://old.reddit.com/r/rust/comments/1rskv4t/oci_generator_for_any_embedded_toolchain/
)
    


### TITLE

## Rust学习者询问：是否有类似Rust但带垃圾回收的语言？

### 问题背景
- 提问者正在学习Rust，喜欢其性能和安全模型
- 了解Rust不使用垃圾回收器（GC），而是依赖所有权和借用机制
- 希望找到类似Rust但使用垃圾回收的编程语言进行对比

### 核心诉求
- 寻找在设计理念上与Rust相似但采用GC的编程语言
- 想要比较两种不同的内存管理方法
- 希望理解各自的权衡取舍（trade-offs）

### 关键要点
该提问反映了开发者对不同内存管理策略的兴趣：
- **Rust方式**：所有权系统 + 借用检查器（无GC）
- **替代方式**：垃圾回收机制
- 目的是通过对比学习，深入理解两种方案的优缺点

[
https://old.reddit.com/r/rust/comments/1rrsj4k/is_there_a_language_similar_to_rust_but_with_a/
](
https://old.reddit.com/r/rust/comments/1rrsj4k/is_there_a_language_similar_to_rust_but_with_a/
)
    


### TITLE

## 比Rust标准通道(MPSC)快5倍的实现

一位开发者分享了他们为Burn框架开发的高性能自定义通道实现，在多线程场景下性能显著优于Rust标准库。

### 核心特点

- **不是通用替代品**：这是一个专门优化的通道，用于执行FnOnce闭包，而非功能完整的MPSC实现
- **针对性优化**：使用了专门的unsafe实现和内存arena策略，专为高性能异步任务执行而设计
- **有使用限制**：不支持自动刷新，有诸多前提假设，不能直接替换标准通道

### 性能测试结果

- **单线程场景**：互斥锁(Mutex)仍然最快，因为避免了数据复制且无竞争
- **多线程场景**：
  - 比标准通道快**10倍**
  - 比互斥锁快约**2倍**
  - 在8线程下，任务执行量是可重入互斥锁的**近2倍**

### 技术实现细节

1. **零分配任务入队 - 分层双缓冲Arena**
   - 小闭包（≤48字节）：直接内联到64字节的Task结构中，按CPU缓存行对齐以防止伪共享
   - 大闭包（≤4KB）：使用预分配内存arena，完全绕过全局分配器
   - 超大闭包（>4KB）：回退到标准Box（占比极少）

2. **无锁双缓冲机制**
   - 生产者写入客户端缓冲区（使用原子Acquire/Release语义）
   - 运行线程通过单次原子交换批量获取所有任务到私有服务端缓冲区
   - 运行线程顺序执行任务，零干扰

3. **线程本地存储(TLS)保证递归安全**
   - 运行线程通过TLS检测是否向自己提交任务
   - 如果是，立即执行任务而非入队，避免死锁

### 关键洞察

**问题根源**：最初使用标准通道比锁还慢的原因是**内存分配**。发送`Box<dyn FnOnce()>`闭包时，由于闭包常超过1000字节，多线程同时分配/释放造成的竞争比原始互斥锁更严重。

### 结论

**一般情况**：不应该自己实现自定义通道，应使用标准库

**特殊场景**：当你了解传输对象的特性时，确实可以显著超越通用实现的性能

完整博客：https://burn.dev/blog/faster-channel/

[
https://old.reddit.com/r/rust/comments/1rrx1bx/5x_faster_than_rust_standard_channel_mpsc/
](
https://old.reddit.com/r/rust/comments/1rrx1bx/5x_faster_than_rust_standard_channel_mpsc/
)
    


### TITLE

## Rustup 1.29.0 发布公告

Rustup 团队于 2026 年 3 月 12 日发布了 rustup 1.29.0 版本。Rustup 是安装 Rust 编程语言的推荐工具。

### 主要新特性

**性能改进：**
- **并发下载和解压**：在执行 `rustup update` 或 `rustup toolchain` 等操作时，现在支持并发下载组件并在下载过程中同步解压
- **并发检查更新**：`rustup check` 命令现在支持并发检查更新
- 这些改进源自 GSoC 2025 项目，显著提升了工具链安装性能

**新增平台支持：**
- sparcv9-sun-solaris
- x86_64-pc-solaris

**扩展的 Shell 支持：**
- `rustup-init` 现在会自动为 tcsh 和 xonsh 添加正确的 $PATH 配置

### 其他改进

- **rust-analyzer 代理支持**：通过代理运行 rust-analyzer 时，如果 rustup 管理的版本不存在，会自动使用 PATH 中的版本，方便用户使用自定义的 rust-analyzer（适用于 Neovim、Helix 等编辑器用户）
- **环境变量处理**：空环境变量现在被视为未设置，便于重置配置值
- **退出码优化**：`rustup check` 会根据是否有更新返回不同的退出码（有更新返回 100，无更新返回 0）

### 团队动态

欢迎 @FranciscoTGouveia 加入 rustup 团队，他在并发功能开发中发挥了重要作用。

### 更新方法

已安装用户：
```bash
$ rustup self update
```
或通过常规工具链更新：
```bash
$ rustup update
```

### 注意事项

新版本发布后，杀毒软件可能会暂时阻止 rustup 或其文件操作，此问题通常会在几周内自动解决。

[
https://blog.rust-lang.org/2026/03/12/Rustup-1.29.0/
](
https://blog.rust-lang.org/2026/03/12/Rustup-1.29.0/
)
    


### TITLE

## ry(o3) - Python的Rust封装库

### 项目概述
**ry**是一个不断增长的Python库集合，围绕Rust crates构建，具有快速、异步优先和人性化的特点。这是一个进行中的项目，欢迎反馈和PR。

### 核心特性
- **异步优先的HTTP客户端**：基于reqwest构建，提供类似fetch的API，支持流式传输、零拷贝IO、超时、重定向和原生JSON序列化
- **日期时间库**：基于jiff的综合日期时间库，兼容pydantic
- **异步文件I/O**：基于tokio构建，API类似aiofiles和anyio，支持缓冲读写、截断、流式读取
- **(解)压缩工具**：支持zstd、brotli、gzip和bzip2
- **其他绑定**：包含globset、walkdir、sqlformat、unindent、twox-hash等crate
- **类型注解**：所有公共API都有完整的类型注解
- **Pydantic集成**：ry数据类型与pydantic良好兼容
- **高性能**：注重实际性能表现

### 安装方式
```
pip install ry
uv add ry
python -m ry  # 检查安装
```

### 项目结构
- **ry**：Python包
- **ryo3-***：Rust crate，可用于构建自己的pyo3 Python绑定

### 包含的Crate绑定
涵盖标准库、HTTP、JSON、压缩、哈希、正则表达式、文件系统操作等多个Rust crate的Python绑定

### 开发规范
- 禁止使用"blazing fast"短语或emoji
- 必须提供类型注解
- 使用ruff进行格式化和代码检查
- 由人类编写，非AI生成

[
https://ryo3.dev/
](
https://ryo3.dev/
)
    


### TITLE

## Rust在量子计算领域的应用探讨

### 主要内容

这是一个来自Reddit r/rust社区的讨论帖。发帖者想要了解：

- **核心问题**：是否有使用Rust在量子计算领域完成的重要工作？
- **具体需求**：希望探索相关项目，寻求GitHub仓库或博客资源的分享

### 背景说明

这是一个开放性的提问帖，发帖者对Rust语言在量子计算这一前沿科技领域的应用感兴趣，希望社区成员能够分享相关的项目和资源。

[
https://old.reddit.com/r/rust/comments/1rsr873/rust_in_quantum_computing/
](
https://old.reddit.com/r/rust/comments/1rsr873/rust_in_quantum_computing/
)
    


### TITLE

## RAVEN：用 Rust 编写的 RISC-V 模拟器和 TUI IDE

一位开发者分享了他们的项目 **RAVEN**，这是一个用 Rust 编写的 RISC-V 模拟器和终端用户界面（TUI）集成开发环境。该项目最初只是一个学习性质的业余项目，但已发展成为功能强大的工具。

### 主要特性

- **Rust 程序支持**：现在可以编写 Rust 程序，将其编译为 RISC-V 架构，并在模拟器内运行
- **调试功能**：支持逐条指令单步执行、实时查看寄存器变化、检查内存状态，帮助理解代码在机器层面的实际运行情况

### 开箱即用的 no_std 环境

项目包含 `rust-to-raven/` 目录，提供了预配置的 `no_std` 启动项目，已经配置好：
- `_start` 入口点
- panic 处理器
- 全局内存分配器
- `print!` / `println!` 宏
- `read_line!` 宏

### 技术实现

- 支持 `Vec`、`BTreeMap` 等需要堆分配的数据结构
- 目前使用基于 `sbrk` 调用的简单 bump 分配器（尚未实现 `free` 功能）
- 可配置的缓存层次结构，用于深入分析内存行为和性能剖析
- UI 基于 ratatui 框架构建

### 项目价值

该工具提供了一种直观的方式来观察"高级 Rust 代码"与"实际机器执行指令"之间的差距，适合用作学习和调试工具。

**GitHub 地址**：https://github.com/Gaok1/Raven

[
https://old.reddit.com/r/rust/comments/1rswz5v/riscv_simulator_in_rust_tui_you_can_now_write/
](
https://old.reddit.com/r/rust/comments/1rswz5v/riscv_simulator_in_rust_tui_you_can_now_write/
)
    


### TITLE

## Rust中的持久化作业队列选项

### 问题背景
用户在寻找Rust语言中的持久化作业队列解决方案。他们发现很多讨论只是简单建议"启动一个tokio线程"，但这忽略了作业队列的一个重要方面：**持久化**。

### 核心需求
- 服务器重启后作业仍然存在
- 重启后能够继续处理作业
- 类似于BullMQ和Celery这类工具提供的持久化作业功能

### 关键问题
现有的常见建议（使用tokio线程）无法满足持久化需求，用户希望了解在Rust生态系统中有哪些可以实现作业持久化的选项。

[
https://old.reddit.com/r/rust/comments/1rsjip6/persistent_job_queues/
](
https://old.reddit.com/r/rust/comments/1rsjip6/persistent_job_queues/
)
    


### TITLE

## Alakit：基于 WebView 的无 JavaScript Rust GUI 框架

一位开发者分享了他正在开发的 GUI 框架 Alakit，现已发布 v0.1 版本。该项目旨在让 Rust 开发者能够构建 GUI 应用，同时避免 JavaScript 的复杂性。

### 核心特性

- **零 JS 逻辑**：所有业务逻辑 100% 使用 Rust 编写，HTML/CSS 仅作为界面"外观"
- **自动发现**：通过简单的宏自动注册控制器，无需手动配置
- **加密后端存储（开发中）**：敏感数据在 Rust 端内存中加密存储（注意：当前发送到 WebView 显示的数据仍以明文形式存在于 JS 运行时，作者正在改进这一边界）
- **单一二进制文件**：HTML/CSS/Rust 代码全部嵌入到可执行文件中

### 项目状态

- 目前处于早期 alpha 阶段，可能存在一些 bug
- 文档正在完善中，部分代码注释仍在从作者母语翻译为英文
- 作者根据社区反馈，已将安全特性重命名为"加密后端存储"并更新了相关说明

**项目地址**：https://github.com/fejestibi/alakit

[
https://old.reddit.com/r/rust/comments/1rsqstz/jsfree_rust_gui_using_webview/
](
https://old.reddit.com/r/rust/comments/1rsqstz/jsfree_rust_gui_using_webview/
)
    


### TITLE

## Vite 8.0 正式发布

Vite 8.0 于2026年3月12日正式发布，这是自Vite 2以来最重大的架构变更。

### 核心变化

- **统一构建工具**：Vite 8采用Rolldown作为唯一的、统一的、基于Rust的构建工具，取代了之前esbuild和Rollup的双构建器架构
- **性能提升**：构建速度提升10-30倍，同时保持完全的插件兼容性
- **市场表现**：Vite目前每周下载量达6500万次

### 问题背景

早期版本的Vite依赖两个独立的构建工具：
- **esbuild**：处理开发环境的快速编译（依赖预构建和TypeScript/JSX转换）
- **Rollup**：处理生产环境的打包、代码分割和优化

这种双构建器方案存在的问题：
- 需要维护两个独立的转换管道和插件系统
- 需要大量粘合代码保持两个管道同步
- 模块处理不一致的边缘情况逐渐累积

### Rolldown解决方案

Rolldown的三大设计目标：
1. **性能**：使用Rust编写，速度比Rollup快10-30倍，达到esbuild性能水平
2. **兼容性**：支持与Rollup和Vite相同的插件API，大多数现有插件可直接使用
3. **高级特性**：支持完整打包模式、更灵活的代码分割、模块级持久化缓存和模块联邦

### 实际性能表现

多家公司报告的生产构建时间改进：
- **Linear**：从46秒降至6秒
- **Ramp**：构建时间减少57%
- **Mercedes-Benz.io**：构建时间减少最多38%
- **Beehiiv**：构建时间减少64%

### 其他改进

- 推出了**registry.vite.dev**：可搜索的Vite、Rolldown和Rollup插件目录
- 提供多语言文档翻译
- 超过1200名贡献者参与Vite核心开发

[
https://vite.dev/blog/announcing-vite8
](
https://vite.dev/blog/announcing-vite8
)
    


### TITLE

## 如何使用"讲故事"方法将内联汇编融入 Rust

这篇文章讨论了内联汇编在 Rust 抽象机（Abstract Machine）中的语义问题。

### 核心问题
- Rust 抽象机包含许多实际硬件中不存在的概念（如 provenance、未初始化内存、Tree Borrows 等）
- 当使用内联汇编时，这些抽象概念如何应用成为一个重要问题
- 文中的讨论同样适用于 FFI 调用

### 为什么内联汇编不能为所欲为？

文章通过一个例子说明：
- 一个函数接收 `&i32`（共享引用）参数，但通过内联汇编修改了该值
- 编译器优化时假设共享引用不会改变值，导致优化前后程序行为不一致
- 这说明即使使用内联汇编，也必须遵守某些规则，否则会产生未定义行为（UB）

### "讲故事"方法

作者提出的解决方案：
- **不需要为汇编代码定义完整的 Tree Borrows 规则**（这几乎不可能实现）
- 相反，要求程序员为每个内联汇编块**提供一个等效的 Rust 代码"故事"**
- 这个"故事代码"必须在纯 Rust 代码可观察的状态方面与汇编代码做相同的事情
- 在推理程序行为时，用这个"故事代码"替代内联汇编块进行分析

### 实际应用
- 不需要真正编写故事代码，但这样的代码必须在概念上存在
- 通过故事代码可以立即判断内联汇编是否违反了 Rust 的规则
- 例如上述例子中，故事代码会是 `(x as *const i32 as *mut i32).write(0)`，这明显违反了共享引用的规则

[
https://www.ralfj.de/blog/2026/03/13/inline-asm.html
](
https://www.ralfj.de/blog/2026/03/13/inline-asm.html
)
    


### TITLE

## Rust中通过模拟高阶类型（HKTs）导致编译器崩溃

### 前言
作者在开发一个函数式编程脚本语言时，遇到了Rust不支持高阶类型的问题。

### Rust不完全支持高阶类型（HKTs）

**什么是高阶类型？**
- 高阶类型是指泛型可以拥有泛型的概念
- 例如 `struct Foo<T>(T<i32>)`，其中 `T` 是一个可以接受泛型类型的类型构造器
- 类型系统存在于自己的"宇宙"中，与值的世界相对应

**类型构造器的概念：**
- `Vec` 本身不是一个类型，而是类型构造器
- `Vec<i32>` 才是完整的类型
- 类型构造器可以理解为 `fn(type) -> type`，接受一个类型参数并返回新类型

**对比表格：**
| 层级 | 函数 | 类型构造器 |
|------|------|------------|
| 功能 | `fn(bool) -> bool` | `fn(type) -> type` |
| 示例 | 逻辑非运算 `!` | `Vec` |
| 参数 | 值（如 `true`） | 类型（如 `i32`） |
| 输出 | 值（布尔值） | 类型（如 `Vec<i32>`） |

### 关键问题
作者在尝试抽象代码时发现需要写 `Ast<W: ???>`，但Rust无法直接表达"W是一个接受类型参数的类型构造器"这个约束，这正是缺少高阶类型支持的体现。

[
https://www.harudagondi.space/blog/torturing-rustc-by-emulating-hkts
](
https://www.harudagondi.space/blog/torturing-rustc-by-emulating-hkts
)
    


### TITLE

## Cargo 构建目录布局 v2 测试征集

Rust 官方博客发布于 2026 年 3 月 13 日，作者 Ed Page 呼吁社区测试 Cargo 的新构建目录布局。

### 核心要点

**测试方法：**
- 使用 nightly 2026-03-10 或更高版本
- 在测试、发布流程等场景中添加 `-Zbuild-dir-new-layout` 标志
- 示例：`cargo test -Zbuild-dir-new-layout`

**背景说明：**
- 从 Cargo 1.91 开始，用户可以分离中间构建产物（build-dir）和最终产物（target-dir）的存储位置
- 虽然构建目录布局属于内部细节，但许多项目因 Cargo 功能缺失而依赖这些未规范的细节

### 已知失败场景

1. 从 `[[test]]` 路径推断 `[[bin]]` 路径
   - 解决方案：使用 `std::env::var_os("CARGO_BIN_EXE_*")`（Cargo 1.94+）
2. 构建脚本从二进制或 OUT_DIR 查找 target-dir（Issue #13663）
3. 从 rustc 查找用户请求的产物（Issue #13672）

**库支持状态：**
- 已修复：assert_cmd、executable-path、snapbox、trycmd
- 待处理：cli_test_dir、compiletest_rs、term-transcript、test_bin

### 主要变化

**不变内容：**
- target 目录中最终产物的布局
- 构建产物在配置文件和目标元组下的嵌套结构

**变化内容：**
- 从**按内容类型组织**切换为**按包名和构建单元哈希值划分**
- 旧布局：按 `.fingerprint/`、`build/`、`deps/` 等类型目录组织
- 新布局：按包名分组，每个包下包含不同哈希值的构建单元

### 实施原因

这项改进是实现跨工作空间缓存的基础步骤，由贡献者 ranger-ross 主导开发。

[
https://blog.rust-lang.org/2026/03/13/call-for-testing-build-dir-layout-v2/
](
https://blog.rust-lang.org/2026/03/13/call-for-testing-build-dir-layout-v2/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

