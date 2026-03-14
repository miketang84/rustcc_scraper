【Rust日报】2026-02-28


### TITLE

## Rust In Paris 2026 - 会议

**时间：** 2026年3月27日，上午8:30 - 下午5:30
**地点：** 法国巴黎
**形式：** 现场 + 在线

### 主要信息

Rust In Paris 是一个面向开发者的社区会议，专为那些热衷于使用 Rust 构建安全、并发、高性能软件的开发者而设。无论你是经验丰富的 Rust 开发者（Rustacean），还是刚刚开始学习 Rust 的新手，Rust In Paris 都能为每个人提供有价值的内容。

[https://ti.to/xperhub/rust-in-paris-2026
](https://ti.to/xperhub/rust-in-paris-2026
)
    


### TITLE

## Rust in Paris 会议日程

这是一场关于 Rust 编程语言的技术会议，包含多个精彩演讲。

### 主要议题

**上午场次：**

- **Rust 的小惊喜** (waffle)
  - 介绍 Rust 的高级特性和技巧，帮助初学者编写更好的代码
  - 内容包括：数组重复表达式、Result 收集、循环返回值、const 函数等

- **Rust GCC 后端：原因与方法** (Guillaume Gomez)
  - 讲解 Rust 编译器中代码生成后端的实现方式
  - 如何优化生成更好的汇编代码

- **如何将 Clippy 运行时间减半** (Alejandra González)

- **从 NAND 门开始构建 CPU** (Simon Sapin)
  - 从基础物理和半导体开始，逐步构建可用 Rust 编程的系统
  - 深入探讨计算机底层工作原理

- **AAA 多人游戏模式遇见 Rust、Tokio 和 WebAssembly** (Christoph Lürig)
  - 使用 Rust 为浏览器端棋盘游戏实现客户端托管中继架构
  - 讲解如何避免异步 Rust 中复杂的所有权模式

**下午场次：**

- **统一消息总线：模块化单体的异步 Rust** (Bertrand Darbon)
  - 介绍消息总线模式连接独立模块的架构
  - 提升性能和开发体验

- **与 Panic 处理器玩烫手山芋** (Jana Dönszelmann)
  - 讨论 Rust 编译器的最新和未来变化
  - 介绍"外部可实现项"新特性

- **维护 Rust** (Oli Scherer)

- **在 Thunderbird 中实现 Rust 异步操作队列**
  - Thunderbird 添加 Microsoft Exchange 支持的技术实现

### 会议信息
- 时间：全天活动（8:00 AM - 4:00 PM+）
- 包含茶歇、午餐和社交时间

[
https://www.rustinparis.com/schedule
](
https://www.rustinparis.com/schedule
)
    


### TITLE

## Reddit 搜索参数指南

### 搜索限定条件

以下是在 r/Zig 子版块中可使用的搜索参数，用于精确筛选内容：

### 主要搜索功能

- **subreddit:** 在指定子版块中查找帖子
- **author:** 查找特定用户发布的内容
- **site:** 查找来自特定网站的提交内容（如 example.com）
- **url:** 在网址中搜索特定文本
- **selftext:** 在自发帖正文中搜索特定文本
- **self:yes/no** 包含或排除自发帖
- **nsfw:yes/no** 包含或排除标记为成人内容的结果

### 使用示例

```
subreddit:aww site:imgur.com dog
```

### 更多信息

可查看搜索常见问题解答了解详情，或使用高级搜索功能按作者、子版块等条件进行筛选。

[
https://old.reddit.com/r/zig
](
https://old.reddit.com/r/zig
)
    


### TITLE

## numpy-ts - 完整的 TypeScript/JavaScript NumPy 实现

### 核心特性
- **📊 广泛的 API 覆盖**：实现了 507 个 NumPy 函数中的 476 个（覆盖率 93.9%）
- **✅ NumPy 验证**：通过 6,000+ 个测试用例与 Python NumPy 进行对比验证
- **🔒 类型安全**：提供完整的 TypeScript 类型定义
- **🌳 支持树摇优化**：按需导入所需功能
- **🌐 通用性强**：同时支持 Node.js 和浏览器环境

### 主要功能
- 支持数据类型（dtype）的数组创建
- 广播机制和链式操作
- 线性代数运算（矩阵乘法、迹等）
- 支持轴参数的归约操作
- NumPy 风格的字符串切片语法

### 安装与使用
```bash
npm install numpy-ts
```

### 相关资源
- 文档：https://numpyts.dev
- 在线演练场：https://numpyts.dev/playground
- 使用示例：https://numpyts.dev/examples
- API 覆盖报告：https://numpyts.dev/coverage
- 性能基准测试：https://numpyts.dev/performance

### 开源信息
- **许可证**：MIT
- **作者**：Nicolas Dupont
- **贡献**：欢迎提交 Issue 和 PR

[
https://github.com/dupontcyborg/numpy-ts
](
https://github.com/dupontcyborg/numpy-ts
)
    


### TITLE

## Concryptor - Rust多线程加密引擎

### 项目概述
Concryptor是一个用Rust构建的多线程AEAD加密引擎，通过三重缓冲io_uring管道、Rayon并行块处理和汇编优化密码算法实现千兆字节级的文件加密/解密吞吐量。

### ⚠️ 重要警告
- **实验性软件**，目前不建议用于生产环境或关键任务
- 尚未经过正式安全审计或广泛实际测试
- 推荐使用GnuPG、age或OpenSSL等成熟工具保护敏感数据

### 核心特性
- **双密码支持**：AES-256-GCM（硬件AES-NI加速）和ChaCha20-Poly1305
- **并行加密**：基于Rayon的多线程处理，利用所有CPU核心
- **三重缓冲io_uring管道**：三个缓冲池轮转，同时处理内核I/O和CPU加密
- **Argon2id密钥派生**：行业标准密码拉伸（默认256 MiB内存，3次迭代）
- **自描述KDF参数**：加密文件头中存储内存成本、迭代次数和并行度
- **块索引nonce**：TLS 1.3风格的XOR nonce派生，防止块重排攻击
- **头部认证AAD**：完整序列化头部包含在每个块的AAD中
- **原地加密**：最小化热循环中的内存分配
- **密码清零**：使用后安全擦除密钥和密码
- **O_DIRECT + 扇区对齐**：4 KiB对齐启用直接I/O，绕过内核页缓存

### 性能基准
**测试环境**：AMD Ryzen 5 5600X (6核/12线程)，16 GiB DDR4-2666

| 文件大小 | AES-256-GCM加密 | ChaCha20加密 | AES-256-GCM解密 | ChaCha20解密 |
|---------|----------------|-------------|----------------|-------------|
| 64 KiB  | 244 MiB/s     | 233 MiB/s   | 233 MiB/s     | 234 MiB/s   |
| 1 MiB   | 1.08 GiB/s    | 882 MiB/s   | 1010 MiB/s    | 876 MiB/s   |
| 16 MiB  | 1.10 GiB/s    | 923 MiB/s   | 1.06 GiB/s    | 988 MiB/s   |
| 256 MiB | 1.00 GiB/s    | 1015 MiB/s  | 1.01 GiB/s    | 1.02 GiB/s  |

### 性能特点
- AES-256-GCM在小文件上更快（利用AES-NI硬件指令）
- 峰值吞吐量出现在1-16 MiB文件（Rayon并行效率最高）
- 大文件时两种密码都收敛到~1.0 GiB/s（瓶颈转向I/O提交）
- 在真实NVMe驱动器上，O_DIRECT可实现更高吞吐量

[
https://github.com/FrogSnot/Concryptor
](
https://github.com/FrogSnot/Concryptor
)
    


### TITLE

## Rust in Paris 2026 会议即将召开

### 关键信息

- **时间**：会议将在一个月后举行
- **演讲嘉宾**：拥有出色的演讲嘉宾阵容
- **日程安排**：可在官网查看完整日程 (https://www.rustinparis.com/schedule)
- **购票方式**：可通过 https://ti.to/xperhub/rust-in-paris-2026 购买门票

这是一个关于 Rust 编程语言的技术会议，将在巴黎举办。

[
https://old.reddit.com/r/rust/comments/1rg5rwb/rust_in_paris_2026_conference_is_in_one_month/
](
https://old.reddit.com/r/rust/comments/1rg5rwb/rust_in_paris_2026_conference_is_in_one_month/
)
    


### TITLE

## Rust中处理部分借用的最佳实践困境

### 核心问题
开发者在使用Rust时遇到一个持续性的难题：当结构体包含多个相关数据字段时，由于借用检查器不能理解跨函数边界的部分借用，导致常见的代码模式无法通过编译。

### 具体案例
```rust
struct Data {
    stuff: Vec<u32>,
    queue: Vec<u32>,
}
```

在 `process_all` 方法中，当试图遍历 `self.stuff` 的同时调用修改 `self.queue` 的 `process` 方法时，编译器报错：无法借用 `self`，因为 `self.stuff` 已被借用。

### 开发者的困惑
- 是否应该放弃使用结构体，改为分别传递各个成员？
- 是否需要根据具体情况手动拆分结构体成员？
- 如何有效地处理这个问题？

### 情绪表达
这位有2年以上Rust开发经验的程序员表示：
- 这个问题严重影响开发体验，甚至考虑放弃使用Rust
- 感到非常沮丧，因为结构体本应是基础的抽象单元和数据分组方式
- 只想"把事情做完"，但似乎必须在性能和代码组织之间做出妥协（如使用中间Vec来传递值或按需拆分数据）

[
https://old.reddit.com/r/rust/comments/1rg3ftw/whats_the_most_idiomatic_way_to_deal_with_partial/
](
https://old.reddit.com/r/rust/comments/1rg3ftw/whats_the_most_idiomatic_way_to_deal_with_partial/
)
    


### TITLE

## Apache Iggy 迁移到基于 io_uring 的线程每核心架构

### 背景
Apache Iggy 以性能为核心原则，在原有架构达到硬件极限后，需要寻求新的方法来突破性能瓶颈。

### 迁移原因
- **原架构问题**：Apache Iggy 使用 tokio 作为异步运行时，采用多线程工作窃取执行器
  - 缺乏精细控制
  - 任务在工作线程间迁移导致缓存失效
  - 执行路径不可预测
  
- **关键痛点**：tokio 处理块设备 I/O 的方式
  - 基于通知机制（epoll）不适合块设备
  - Linux 内核将常规文件视为始终"就绪"，导致 I/O 操作仍会阻塞线程
  - 依赖线程池（默认最多 512 个线程）处理阻塞 I/O
  - 高性能系统会快速耗尽线程池能力

### 新架构：线程每核心无共享架构
**核心理念**：
- 将单个线程固定到每个 CPU 核心
- 基于启发式方法（通常是哈希）对资源进行分区
- 消除共享状态，减少锁竞争
- 提高缓存局部性
- 使用消息传递在线程（分片）之间通信

**关键改进**：
- 从"工作窃取"转向"工作引导"
- 采用 **io_uring** 实现真正的异步 I/O

### io_uring 的优势
- **基于完成而非就绪通知**：提交操作后由内核驱动至完成
- **核心机制**：用户空间和内核之间共享两个无锁环形缓冲区
  - 提交队列（SQ）：应用程序排队 I/O 请求
  - 完成队列（CQ）：内核放置操作结果
- 虽然与 Rust Future 的轮询模型不完全兼容，但性能开销可忽略不计

[
https://iggy.apache.org/blogs/2026/02/27/thread-per-core-io_uring/
](
https://iggy.apache.org/blogs/2026/02/27/thread-per-core-io_uring/
)
    


### TITLE

## 让你爱上Rust的第一个项目是什么？

### 核心观点

这是一个Reddit上的讨论话题，探讨了开发者第一次真正感受到Rust语言魅力的项目经历。

### 关键要点

- **项目类型**：对许多人来说，让他们爱上Rust的往往是一些小型项目，例如：
  - CLI（命令行）工具
  - 微服务
  - 系统实用程序

- **Rust的独特感受**：这些项目让开发者突然意识到Rust的特点：
  - **可靠性**高
  - **性能**快
  - 代码**简洁**清晰

- **核心问题**：讨论的焦点是"哪个项目让你产生了'哇，这门语言真的不一样'的时刻？"

这个话题旨在收集社区成员的真实经验，了解是什么样的实践场景让开发者真正体会到Rust的价值。

[
https://old.reddit.com/r/rust/comments/1rg3lby/whats_the_first_rust_project_that_made_you_fall/
](
https://old.reddit.com/r/rust/comments/1rg3lby/whats_the_first_rust_project_that_made_you_fall/
)
    


### TITLE

## Rust学习路径咨询：应该深入了解哪个crate？

### 学习背景
- 学习Rust已有一个月
- 刚完成借用(borrowing)和函数部分的学习
- 下一步计划学习生命周期(lifetimes)

### 主要问题

#### 1. 如何巩固Rust基础？
寻求建议以扎实掌握和理解Rust基础知识

#### 2. 异步编程学习规划
- Rust异步编程是接下来的学习路径
- 询问除了Rust默认的async之外，是否有特定的crate需要学习
- 何时学习这些crate：在学习async之前还是之后？

### 补充说明
作者后续补充说明：
- 这是新账号，请求不要踩帖
- Reddit有严格的垃圾信息检测系统，担心账号被封
- 自己是在校大学生，只是寻求学习帮助
- 希望得到指正和帮助

[
https://old.reddit.com/r/rust/comments/1rgc5ww/what_crate_in_rust_should_i_understand_the_most/
](
https://old.reddit.com/r/rust/comments/1rgc5ww/what_crate_in_rust_should_i_understand_the_most/
)
    


### TITLE

## Rust 还是 Zig 用于小型 WASM 数值计算？

一位开发者正在构建 **numpy-ts**（TypeScript 版的 NumPy 数值库），已完成 1.0 版本并覆盖了 94% 的 NumPy API。现在需要为计算密集型操作（如线性代数、排序等）选择 WASM 加速方案。

### 测试结果

- **性能和体积几乎相同**：两种语言编译出的 WASM（启用 SIMD128）性能相当，压缩后体积均为 ~7.5 KB（5个内核文件）
- 都通过 LLVM 编译，WASM 输出基本一致

### Rust 的优势

- **生态系统更丰富**：如需特殊数学函数（erf、gamma 等）支持更好
- **开发者采用度更广**：降低项目风险，社区支持更强

### Zig 的优势

- **`@setFloatMode(.optimized)`**：让 LLVM 自动向量化归约操作，无需手写 SIMD
- **向量类型更符合人体工程学**：`@Vector(4, f64)` 比 Rust 的 `core::arch::wasm32` 内置函数更易用
- **无需 unsafe 包装**：对于本质上是原始指针运算的代码，不需要 unsafe 块（在这种场景下 Rust 的借用检查器显得多余）

### 问题

开发者在两个社区同时提问，希望了解选择 Rust 用于 WASM 应用时还应考虑哪些因素。

[
https://old.reddit.com/r/rust/comments/1rgi2mh/rust_or_zig_for_small_wasm_numerical_compute/
](
https://old.reddit.com/r/rust/comments/1rgi2mh/rust_or_zig_for_small_wasm_numerical_compute/
)
    


### TITLE

## context-logger - Rust 结构化日志上下文库

### 项目简介
context-logger 是一个轻量级、符合人体工程学的 Rust 库，用于为日志添加结构化上下文信息。它增强了标准的 Rust log crate 生态系统，允许在不改变现有日志模式的情况下，为日志消息附加丰富的上下文信息。

### 主要特性

- **无缝集成**：与现有的 Rust log crate 生态系统兼容
- **上下文传播**：自动将上下文信息附加到日志消息中
- **异步支持**：支持异步函数并能在 `.await` 点之间传播日志上下文

### 基本使用方法

1. **添加依赖**：在 `Cargo.toml` 中添加 context-logger、log 和 env_logger
2. **创建包装器**：使用 `ContextLogger` 包装现有的日志记录器
3. **添加默认记录**：可以设置默认的上下文字段（如版本号）
4. **创建上下文**：使用 `LogContext::new()` 创建上下文并添加键值对
5. **进入上下文**：调用 `.enter()` 激活上下文
6. **记录日志**：正常使用 `info!` 等宏，上下文信息会自动附加

### 异步上下文传播

- 使用 `FutureExt` trait 的 `.in_log_context()` 方法
- 上下文会自动在异步调用链中传播
- 可以在任何位置使用 `LogContext::add_record()` 添加额外的上下文信息

### 许可证
MIT 许可证

[
https://github.com/alekseysidorov/context-logger
](
https://github.com/alekseysidorov/context-logger
)
    


### TITLE

## oken - 更智能的 SSH 命令行工具

oken 是一个完全向后兼容、无需配置的 SSH CLI 增强工具，可直接替代原生 ssh 命令。

### 核心特点

- **完全兼容**：所有 ssh 命令的参数和选项都原样传递给系统 SSH，无需迁移或学习成本
- **智能功能增强**：
  - 模糊主机选择器
  - 连接历史记录
  - 自动重连
  - 保持连接活跃
  - 生产环境警告
  - 命名隧道配置
  - 审计日志

### 解决的痛点

- 无需记忆主机地址 - 通过选择器快速查找连接
- 网络中断自动重连 - 不再丢失会话
- 防止空闲超时 - 默认注入保持活跃机制
- 避免误操作生产环境 - 标记的生产主机需要确认
- 简化隧道管理 - 保存配置文件，一键启动

### 安装方式

**macOS/Linux:**
```bash
curl -LsSf https://github.com/linkwithjoydeep/oken/releases/latest/download/oken-installer.sh | sh
```

**Windows:**
```powershell
irm https://github.com/linkwithjoydeep/oken/releases/latest/download/oken-installer.ps1 | iex
```

**从源码安装:**
```bash
cargo install --git https://github.com/linkwithjoydeep/oken
```

### 快速使用

```bash
# 打开交互式选择器
oken

# 按名称连接主机
oken prod-web

# 像使用 ssh 一样使用（所有参数透传）
oken -p 2222 -i ~/.ssh/deploy_key ubuntu@10.0.1.50

# 保存主机到选择器
oken host add prod-web ubuntu@10.0.1.50 --tag prod

# 按标签连接
oken --tag prod
```

### 主要功能

1. **模糊搜索选择器** - 实时过滤主机别名、主机名、用户名和标签，按最近使用排序
2. **标签过滤** - 使用 `#` 前缀按标签搜索
3. **自动保存主机** - 首次连接新主机时询问是否保存
4. **可选别名** - 可将 oken 设置为 ssh 别名，工具会自动跳过自身查找真正的 SSH 二进制文件

[
https://github.com/linkwithjoydeep/oken
](
https://github.com/linkwithjoydeep/oken
)
    


### TITLE

## 异步Rust的演进：从Tokio到高级应用

### 核心内容

JetBrains进行了一次直播访谈，由Vitaly Bragilevsky与Tokio创建者Carl Lerche深入探讨了异步Rust的发展历程。Tokio已成为Rust高性能网络编程的事实标准异步运行时，广泛应用于后端服务和数据库等领域。

### 主要讨论点

**关于TokioConf会议**
- TokioConf是首个专注于Tokio生态系统的会议，在俄勒冈州波特兰举行
- 今年是Tokio发布十周年，是社区聚会的自然时机
- Tokio和Rust已成为构建基础设施级网络软件的默认技术选择

**异步Rust的本质**
- 异步Rust不仅关乎性能，还改善了开发者构建事件驱动系统的方式
- 相比传统多线程方式，异步Rust在处理超时、取消操作和管理多个并发任务方面更加容易
- 利用Rust的所有权模型和Drop特性，实现安全且清晰的取消模式

**Tokio的起源**
- Tokio源于早期在Rust中使用非阻塞I/O的实验
- 最初Rust只有阻塞式socket API，构建高效网络系统需要底层抽象
- 发展路径：从Mio（epoll绑定）→ Future trait → async/await
- async/await的设计实现了内存安全和零成本抽象

**关于虚拟线程**
- Rust在1.0版本前曾有绿色线程和协程，但后来被移除
- 绿色线程的开销和栈管理复杂度与Rust的零成本抽象设计目标相冲突

[
https://blog.jetbrains.com/rust/2026/02/17/the-evolution-of-async-rust-from-tokio-to-high-level-applications/
](
https://blog.jetbrains.com/rust/2026/02/17/the-evolution-of-async-rust-from-tokio-to-high-level-applications/
)
    


### TITLE

## Rust中使用get()方法是否会带来显著的性能开销？

### 主要问题

用户提出了两个关于Rust数组索引性能的问题：

1. **使用`get()`方法是否有显著的性能成本**
   - 相比直接索引访问，`array.get(idx)`是否会影响性能

2. **错误处理方式的性能对比**
   - `array.get(idx).ok_or(Error::Whoops)` 与显式使用`if`语句检查边界哪个更快

### 使用场景

- 用户需要进行大量的数组索引操作
- 这些索引操作不太适合用迭代器来处理
- 用户考虑自行进行性能测试，但希望先了解是否已有现成答案

### 背景说明

这是一个关于Rust性能优化的技术问题，涉及到安全索引访问（`get()`）和直接索引访问之间的权衡选择。

[
https://old.reddit.com/r/rust/comments/1rhb97r/is_there_any_significant_performance_cost_to/
](
https://old.reddit.com/r/rust/comments/1rhb97r/is_there_any_significant_performance_cost_to/
)
    


### TITLE

## io_uring 异步运行时：Tokio 之外的探索

### 讨论主题
这是一个关于探讨 **非 epoll 设计的异步解决方案**的讨论帖，主要聚焦于：

### 核心问题
- **io_uring 异步运行时是否已经失败？**

### 讨论要点
- **compio 的使用经验**：开发者们在实际项目中如何使用 compio
- **性能对比**：compio 与 Tokio 相比的性能表现如何
- **应用场景**：具体的使用案例和适用场景

### 背景说明
这个帖子旨在为开发者提供一个空间，分享和讨论基于 io_uring 的异步运行时（如 compio）在 Rust 生态系统中除 Tokio（基于 epoll）之外的替代方案和成功案例。

[
https://old.reddit.com/r/rust/comments/1rh7lfe/life_outside_tokio_success_stories_with_compio_or/
](
https://old.reddit.com/r/rust/comments/1rh7lfe/life_outside_tokio_success_stories_with_compio_or/
)
    


### TITLE

## Concryptor：基于Rust构建的1 GiB/s文件加密CLI工具

### 项目背景
开发者对传统加密工具（如GPG或age）处理大文件时的性能感到不满。这些工具虽然非常安全，但核心加密算法大多是单线程的，速度通常只能达到200-400 MiB/s。开发者希望在加密时能够充分利用Gen4 NVMe硬盘的性能，因此构建了Concryptor。

### 核心技术特性

**无锁三重缓冲（Lock-Free Triple-Buffering）：**
- 采用3阶段轮转状态机，替代传统的异步MPSC通道（后者在处理小数据块时会产生严重的锁竞争）
- 实现并行处理：io_uring将批次N-2写入磁盘，Rayon在所有12个CPU核心上加密批次N-1，同时io_uring读取批次N

**零拷贝O_DIRECT：**
- 使用std::alloc构建自定义的4096字节对齐内存分配器
- 通过填充头部和数据块槽位，使Linux内核能够完全绕过页缓存，直接通过DMA传输到硬盘

**安全架构：**
- 使用ring库实现汇编优化的AES-256-GCM和ChaCha20-Poly1305加密
- 采用类似TLS 1.3的nonce派生机制（base_nonce异或chunk_index）防止数据块重排序攻击

**STREAM风格的AAD（附加认证数据）：**
- 将完整的序列化文件头（包含Argon2id参数、盐值和基础nonce）以及is_final标志绑定到每个数据块的AAD中
- 从数学上防止截断和追加攻击

### 性能表现
- 稳定达到**1+ GiB/s**的加密速度，完全受CPU限制
- 能够随CPU核心数完美扩展

### 项目资源
项目README包含了二进制文件格式、内存对齐算法和威胁模型的详细深入分析。开发者欢迎社区审查架构和代码。

[
https://old.reddit.com/r/rust/comments/1rh9tj5/i_built_a_1_gibs_file_encryption_cli_using_io/
](
https://old.reddit.com/r/rust/comments/1rh9tj5/i_built_a_1_gibs_file_encryption_cli_using_io/
)
    


### TITLE

## Servo v0.0.5 版本发布

Servo 浏览器引擎发布了 v0.0.5 版本，这是一个包含大量新特性和改进的更新。

### 主要新增功能

**HTML/CSS 特性：**
- 支持 `<link rel=preload>` 预加载功能
- 支持 `<style blocking>` 和 `<link blocking>` 阻塞加载
- 支持 `<img align>` 图片对齐属性
- 支持 `<select disabled>` 禁用选择框
- 支持 `<audio>` 标签播放 OGG 音频文件
- 支持 CSS `cursor-color` 光标颜色属性
- 支持 `content: <image>` 在所有元素上使用
- 支持 `<details>` 元素的 `::details-content` 伪元素
- 支持 `:open` 和 `:active` 伪类选择器

**JavaScript API：**
- Origin API
- MouseEvent.detail 属性
- Request.keepalive 属性
- 支持循环导入、导入属性和 JSON 模块
- `navigator.sendBeacon()` 默认启用

**网络与安全：**
- 支持 `https_proxy`、`HTTPS_PROXY` 和 `NO_PROXY` 代理配置
- 在 Crypto 中支持 ML-KEM、ML-DSA 和 AES-OCB 加密算法

### 平台支持

- **MacOS**：需要按照故障排除指南运行（尚未签名）
- **Linux**：如遇共享库错误请参考故障排除指南
- **HarmonyOS**：提供 OpenHarmony 签名版本，HarmonyOS 需自行签名

### 技术改进

版本包含大量布局引擎优化、依赖项更新、代码重构等技术改进，详细内容可查看一月份的博客文章。

[
https://github.com/servo/servo/releases/tag/v0.0.5
](
https://github.com/servo/servo/releases/tag/v0.0.5
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

