【Rust日报】2026-04-13


### TITLE

## @karlrobeck/node-rusqlite

这是一个为 Node.js 提供的类型安全的 SQLite 库，基于 Rust 的 rusqlite crate 构建，通过 napi-rs 暴露接口。具有零外部依赖、原生性能和单文件数据库特性，适用于桌面应用、Electron、嵌入式系统或任何需要快速可靠本地数据存储的 Node.js 项目。

### ⚠️ 开发状态

- **当前处于活跃开发阶段**，可能存在 bug 和破坏性变更
- 核心功能已测试可用，但 API 仍在稳定中
- npm 包发布将在功能稳定后推出
- 建议用于实验性开发、学习项目和非关键系统
- 目前需要从 GitHub 克隆并本地构建

### 快速开始

提供了简单的示例代码，展示如何：
- 创建内存数据库
- 创建表
- 插入参数化数据
- 查询数据

### 性能基准测试

提供了与其他 SQLite Node.js 绑定的性能对比，包括 INSERT、SELECT、UPDATE、DELETE 操作的性能指标。

### 安装要求

- Node.js 14.17+
- C/C++ 工具链（用于构建 Rust 绑定）
- 通过 git 克隆仓库并本地构建

### 主要功能特性

**核心数据库操作** ✅
- 文件和内存数据库支持
- CRUD 操作与参数化查询
- 预编译语句
- 批量执行

**事务与并发控制** ✅
- 事务支持（自动回滚）
- 多种事务行为模式
- 保存点（嵌套事务）
- 查询中断功能
- 状态检查

**模式与自省** ✅
- 表/列存在性检查
- 列元数据
- 语句元数据
- PRAGMA 查询

**配置与优化** ✅
- 20+ 数据库配置开关
- 内存优化
- 连接状态跟踪
- 20+ 连接控制标志

**高级功能** ✅
- 全文搜索（FTS3/FTS5）
- 备份和恢复
- 语句性能分析

### API 组件

主要包括：Connection（主入口）、ScopedConnection（事务内连接）、Statement（预编译语句）、Rows/Row（查询结果）、Column 元数据、InterruptHandle（查询取消）等。

[https://github.com/karlrobeck/node-rusqlite
](https://github.com/karlrobeck/node-rusqlite
)
    


### TITLE

## r/rust 社区概览

这是 Reddit 上的 Rust 编程语言官方社区介绍页面。

### 社区定位
一个专注于 Rust 编程语言相关内容的讨论平台。Rust 是一门强调**性能、可靠性和生产力**的开源系统编程语言。

### 社区规则

1. **遵守行为准则**：尊重他人，保持耐心、友善和同理心，遵守 Rust 项目行为准则

2. **内容相关性**：帖子必须与 Rust 相关；问题请使用置顶问答帖；周末允许发布作品展示类帖子；禁止元讨论帖

3. **建设性批评**：鼓励批评但必须建设性、有用且可操作；批评 GitHub 项目时不得直接链接其问题追踪器

4. **保持理性**：不要狂热或极端，保持慈善解读原则

5. **避免重复讨论**：避免翻旧账和过度讨论已解决的话题；避免无谓争论

6. **禁止低质量内容**：禁止表情包、图片宏等；代码和错误信息使用文本格式而非图片；AI 生成内容可能被删除

### 有用资源

- **官方资源**：官网、博客、安装程序、源代码、问题追踪器
- **学习资源**：Rust 电子书、标准库 API 参考、示例教程、在线练习场
- **讨论平台**：官方用户论坛、Discord、Matrix 聊天、Stack Overflow

[
https://old.reddit.com/r/rust
](
https://old.reddit.com/r/rust
)
    


### TITLE

## RustVision - 游戏实时伽马调节工具

RustVision 是一个用 Rust 语言开发的轻量级、高性能工具，专为游戏（如 Rust 游戏）提供实时伽马值调节功能。

### 🛡️ 安全性特点
- **反外挂兼容**：设计为非侵入式，可安全用于 EAC、BattlEye 等反作弊系统
- **无钩子注入**：不与 DirectX、Vulkan 或 OpenGL 交互
- **无 DLL 注入**：作为独立进程运行
- **外部操作**：在系统/显示层面调整伽马值，不修改游戏内存

### ✨ 主要功能
- **实时调节**：无需重启游戏即可即时更改伽马设置
- **可自定义**：支持手动更改目标进程名称以适配任何游戏
- **资源占用低**：CPU 和内存占用极小

### 🚀 使用方法
**安装方式**：
1. 访问 Releases 页面下载最新的 `rustvision.exe`
2. 直接运行，无需安装

**从源码构建**：
```bash
git clone https://github.com/asfrm/rustvision.git
cd rustvision
cargo build --release
```

### 📄 其他信息
- **开源协议**：MIT License
- **开发语言**：100% Rust
- **最新版本**：d6e725f (2026年4月13日)

[
https://github.com/asfrm/rustvision
](
https://github.com/asfrm/rustvision
)
    


### TITLE

## FerrisKey - 开源高性能身份与访问管理系统

### 项目简介
FerrisKey 是一个使用 Rust 语言和六边形架构构建的现代化身份与访问管理(IAM)平台,旨在成为重量级 IAM 系统的开源替代方案,具有快速、模块化和云原生的特点。

### 核心特性
- **🦀 高性能优先** - 基于 Rust 开发,异步 I/O,低延迟
- **🧱 六边形架构** - 清晰的领域模型,明确的端口/适配器设计
- **🏢 多租户领域** - 用户/角色/客户端的强隔离
- **🔐 现代化认证** - 支持 OIDC/OAuth2、多因素认证(TOTP)
- **🧩 可扩展性** - 内置 MFA、审计和 Webhook 模块
- **☁️ 云原生** - 官方 Helm chart,可用于 Kubernetes 部署

### 主要功能
| 功能 | 说明 |
|------|------|
| OIDC/OAuth2 | 符合标准的现代应用认证流程 |
| 多租户领域 | 用户、角色、客户端的逻辑隔离 |
| 客户端与服务账户 | 细粒度角色映射,位运算角色系统 |
| MFA(TOTP) | 可插拔的多因素认证策略 |
| 可观测性 | Prometheus 指标和 Grafana 仪表板 |
| Kubernetes 就绪 | 提供 Helm chart,OCI 分发 |

### 快速启动方式
1. **Docker 镜像运行** - 使用 `docker compose --profile registry up -d`
2. **重新构建 Docker** - 使用 `docker compose --profile build up -d`
3. **Helm 部署** - Kubernetes 环境部署
4. **Cargo 源码** - 克隆仓库本地开发

**默认访问地址**: http://localhost:5555 或 5556  
**默认凭据**: admin / admin

### 开源协议
Apache-2.0 许可证,无付费墙,社区优先

[
https://github.com/ferriskey/ferriskey
](
https://github.com/ferriskey/ferriskey
)
    


### TITLE

## FerrisKey

FerrisKey 是一个为云原生平台设计的身份基础设施解决方案，摆脱了传统IAM的复杂性。

### 核心价值

身份与访问管理（IAM）是任何安全平台的基石，它控制着：
- **身份验证**：谁可以访问
- **授权管理**：用户被授权做什么
- **审计追踪**：如何跟踪基础设施中每个服务、团队和环境的访问事件

FerrisKey 采用统一的、以运维为先的方法，专为分布式系统设计，解决了传统方案中认证逻辑分散、缺乏统一审计、安全漏洞随业务增长等问题。

### 主要特性

**Rust原生性能**
- 完全使用 Rust 从零构建
- 约 10MB 的二进制文件
- 低于 10ms 的认证延迟
- 无垃圾回收停顿
- 内存安全且在高负载下占用可预测

**灵活的部署与管理**
提供开箱即用的运维工具，支持多种部署方式：
- Helm chart
- Kubernetes Operator
- ArgoCD
- Docker Compose

**事件驱动的可扩展性**
每个身份事件（登录、令牌颁发、策略变更、域更新）都会发出结构化事件，可用于：
- 触发 Webhooks
- 同步到数据湖
- 驱动自定义工作流
- 支持 Kafka/NATS（即将推出）

**云原生生态系统集成**
原生集成主流云原生工具，无需适配器：
- Prometheus
- OpenTelemetry
- OPA（即将推出）
- AuthZen

[
https://ferriskey.rs
](
https://ferriskey.rs
)
    


### TITLE

## FerrisKey - 云原生平台的身份基础设施

FerrisKey 是一个专为云原生平台设计的身份与访问管理（IAM）解决方案，摒弃了传统IAM的复杂性。

### 核心价值

身份与访问管理是任何安全平台的核心支柱，它控制：
- **身份验证**：谁可以访问
- **授权管理**：用户可以执行哪些操作
- **审计追踪**：如何跟踪每个服务、团队和环境中的访问事件

FerrisKey 通过统一的、运维优先的方法解决了传统方案中认证逻辑分散、缺乏统一审计和安全漏洞等问题。

### 主要特性

**Rust 原生性能**
- 完全使用 Rust 构建（非移植或封装）
- 二进制文件仅约 10MB
- 认证延迟低于 10ms
- 无垃圾回收（GC）停顿
- 内存安全且占用可预测

**灵活的部署和管理**
支持多种运维工具链，无需自定义脚本：
- Helm 图表
- Kubernetes Operator
- ArgoCD
- Docker Compose

**事件驱动的可扩展性**
每个身份事件（登录、令牌颁发、策略变更、域更新）都会发出结构化事件，可用于：
- 触发 Webhooks
- 同步到数据湖
- 驱动自定义工作流
- Kafka / NATS 支持（即将推出）

**云原生生态系统集成**
原生集成主流云原生工具，无需适配器：
- Prometheus（监控）
- OpenTelemetry（可观测性）
- OPA（策略引擎，即将推出）
- AuthZen

[
http://ferriskey.rs
](
http://ferriskey.rs
)
    


### TITLE

## 从C到Rust：系统编程工程师的实用指南

### 核心内容

本书专为C语言和嵌入式系统工程师设计，旨在帮助他们平稳过渡到Rust编程语言。书中不仅介绍新语法，更注重将系统级编程的核心概念在两种语言之间建立清晰的对应关系。

### 主要特点

- **以C开发者视角讲解Rust**：通过C语言的思维方式来理解Rust，使转型过程更加自然
- **系统级思维导向**：关注底层原理和整体设计，而非孤立的语言特性
- **强调概念理解**：深入理解程序行为、内存管理和设计决策，而不仅是语法记忆
- **循序渐进的学习路径**：结构化的内容安排，逐步建立信心
- **实战导向**：与真实系统编程场景紧密结合

### 适合人群

- 希望采用或迁移到Rust的C语言开发者
- 需要在Rust中建立扎实基础的工程师
- 参与C到Rust转型项目的开发人员
- 关注系统软件长期可靠性、可维护性和安全性的工程师
- 希望深入理解现代系统编程概念的开发者

### 学习收获

- 掌握从C到Rust的清晰转换路径
- 建立对Rust安全模型的深刻理解
- 获得安全系统编程的结构化方法
- 理解Rust如何满足现代系统对安全性和可靠性的要求
- 建立在AI辅助开发时代仍然至关重要的编程基础

[
https://www.amazon.com/dp/B0GWXYK8F7
](
https://www.amazon.com/dp/B0GWXYK8F7
)
    


### TITLE

## Rust for C Engineers - 从C到Rust的系统工程师编程指南

### 目标读者
本书主要面向**C语言程序员和系统工程师**，特别是那些习惯用C语言思维方式思考内存、性能、控制和正确性的开发者。

### 核心理念
- **认知映射**：C语言工程师在学习Rust时，往往通过将Rust概念映射回C语言的系统设计方法来理解
- **对比学习**：每个Rust概念都通过以下角度进行评估：
  - 它替代了什么
  - 为什么存在这个规则
  - 相比C语言的手动方法解决了什么问题

### 写作方法
- **明确关联**：本书将C和Rust之间的联系明确化，使学习路径更容易遵循
- **桥梁作用**：作为从C语言的控制性到Rust的安全导向设计之间的桥梁
- **扩展而非替代**：目标不是替换已有知识，而是用更强的保证来扩展它

### 内容特色
- **并列展示**：C语言示例与Rust代码并排呈现
- **熟悉感锚定**：通过熟悉的C语言系统模型来锚定新的Rust概念
- **实用导向**：帮助系统工程师顺利采用Rust编程语言

[
https://fromc2rust.com
](
https://fromc2rust.com
)
    


### TITLE

## Zweriz (v0.3.0) - 现代GPU加速编程语言

### 项目简介
Zweriz是一个快速的动态编程语言，具有原生无缝GPU加速功能，专为高性能计算、深度学习而设计。它让重型计算像编写简单脚本一样容易，可以通过单个代码块直接将繁重的数学运算卸载到NVIDIA GPU上。

### 开发者信息
- 由一位青少年独立开发者创建
- 欢迎志愿者帮助测试和发现bug

### 安装与使用
- **两个版本**：
  - `zweriz`：仅CPU版本
  - `zweriz_cuda`：支持GPU加速和CPU回退
- **运行方式**：
  - 执行脚本：`./zweriz_cuda my_script.zw`
  - 交互式REPL：`./zweriz_cuda`
  - 安全模式：`./zweriz_cuda --safe my_script.zw`（禁用危险命令）

### v0.3.0 新特性
1. **类（Classes）**：创建带默认值的数据结构
2. **错误处理**：原生支持 `try/catch` 块和 `throw` 语句
3. **内存映射（mmap）**：通过 `mmap.load_f64()` 快速加载大型二进制数组
4. **分号支持**：可在单行使用分号分隔多个语句
5. **扩展标准库**：增强数组和字符串操作模块
6. **位运算与数学扩展**：新增位运算符（`|`, `&`, `^`, `<<`, `>>`, `~`）

### 核心语法特性
- **数据类型**：所有数值为64位浮点数
- **支持**：字符串、F-strings、数组、矩阵、字典
- **类定义**：作为数据容器使用
- **控制流**：if/else、循环、错误处理

### 核心功能：GPU加速块
- 使用 `GPU { ... }` 块包裹计算密集型代码
- 自动在NVIDIA GPU上加速执行
- 无GPU时自动回退到CPU执行
- 支持向量化操作和条件函数（如 `blend()`）

### 适用场景
- 高性能计算
- 深度学习
- 大规模数学模拟
- 粒子模拟

[
https://github.com/toiabzahoor/zweriz
](
https://github.com/toiabzahoor/zweriz
)
    


### TITLE

## Distill-CBL：COBOL到WebAssembly的极简编译器

### 项目核心理念

Distill-CBL 是一个将复杂性降到最低的项目。它采用固定格式的COBOL子集，直接用单个Rust文件将其转换为原始WebAssembly字节码。该项目的核心论点是：**一旦剥离机构神秘感，这个问题本质上很小，因此编译器也应该很小。**

### 主要特点

- **无外部依赖**：不使用LLVM、解析器生成器或运行时编译器栈
- **透明性**：所有代码都可追溯，从源语句到字节序列都在单一文件中可见
- **内置演示**：浏览器界面默认展示REDEFINES示例，可通过GitHub Pages直接部署

### 法证二进制功能

编译器生成的模块包含两个自定义段：

1. **source段**：原始COBOL UTF-8源代码
2. **integrity段**：完整性见证记录（版本字节、算法ID、FNV-1a摘要）

**目的**：解决遗留金融系统的噩梦——二进制文件存在但源代码丢失。每个编译模块都携带法证恢复所需的文本和验证载荷，无需第三方服务。

### 存储语义

- 使用**字节可寻址的WASM线性内存**（而非局部变量）
- 工作存储从偏移量1024开始
- 新字段按4字节对齐
- **REDEFINES**重用相同基址，实现真正的内存别名
- 数值字段使用显式 i32.load/store

### 技术架构

- **编译器**位于 `core/src/compiler.rs`
- **词法分析器**仅扫描有效COBOL列，跟踪行/列位置用于诊断
- **解析器**直接从词法分析器流式处理

### 设计哲学

**"不使用LLVM"不是缺少优化，而是关于软件透明度的架构声明。** 项目认为如果目标是WASM，编译器就应该直接了解WASM二进制格式，所有编码都应自行实现，无需通过不透明的中间层。

[
https://github.com/StealthEyeLLC/distill-cbl
](
https://github.com/StealthEyeLLC/distill-cbl
)
    


### TITLE

## 使用 Napi-rs 和 Rust 构建 Node.js SQLite 驱动

一位开发者分享了他们正在开发的项目 @karlrobeck/node-rusqlite——一个为 Node.js 设计的类型安全 SQLite 驱动。

### 项目背景
- 传统的 Node.js 生态系统主要依赖 C/C++ 绑定（如 better-sqlite3 或 node-sqlite3）来实现数据库驱动的原生性能
- 该项目目标是使用 Rust 构建一个可行的替代方案，利用 rusqlite crate 和 napi-rs 桥接 V8 引擎，完全不依赖 C/C++
- 目前核心功能已实现（文件/内存数据库、CRUD 操作、预处理语句和严格的事务行为）

### 当前状态与挑战
- 项目处于 alpha 阶段
- 早期基准测试显示性能接近成熟的 C++ 绑定库
- 尚未实现针对性的 Rust 端优化
- 在 FFI 边界上管理预处理语句和事务句柄的生命周期以防止数据库锁是一大挑战

### 寻求帮助的方向
1. **FFI 开销与内存分配**：在 V8 和 rusqlite 之间转换值时是否存在不必要的克隆或分配？
2. **内存管理**：是否有更符合 Rust 惯例或更安全的方式来管理跨 napi-rs 边界的状态/生命周期？
3. **并发性**：关于优化多线程或中断处理实现的建议
4. **性能分析**：推荐什么工具或工作流程来分析 Node.js + Rust 混合应用以找出瓶颈？

### 项目链接
GitHub 仓库：https://github.com/karlrobeck/node-rusqlite

[
https://old.reddit.com/r/rust/comments/1sjos65/building_a_nodejs_sqlite_driver_using_napirs_and/
](
https://old.reddit.com/r/rust/comments/1sjos65/building_a_nodejs_sqlite_driver_using_napirs_and/
)
    


### TITLE

## 监控伽马值调节工具

一位Reddit用户分享了他们用Rust开发的显示器伽马值调节实用工具。

### 开发背景
- 用户的显示器在处理游戏中的暗场景时效果很差，夜间场景几乎看不清任何内容
- 之前需要每隔15分钟在NVIDIA控制面板手动调整伽马值，非常麻烦

### 工具功能
- 可以在游戏激活时自动调整伽马值
- 支持通过快捷键绑定手动调节
- 用Rust语言编写

### 分享目的
- 这个工具显著改善了用户的游戏体验
- 希望分享给有类似需求的人使用
- 欢迎反馈和建议

### 项目地址
GitHub: https://github.com/asfrm/rustvision

[
https://old.reddit.com/r/rust/comments/1sjyltu/monitor_gamma_utility/
](
https://old.reddit.com/r/rust/comments/1sjyltu/monitor_gamma_utility/
)
    


### TITLE

## 从 Futures 到运行时：Async Rust 的实际工作原理

### 异步编程简介

**同步编程的局限性：**
- 任务在单线程上按顺序线性执行
- 当某个任务缓慢时，整个程序会被"阻塞"等待其完成

**异步编程的两种主要策略：**
1. **并发（Concurrency）**：像洗衣服时去做饭一样，在单线程上交错执行多个任务
2. **并行（Parallelism）**：像一个人做饭、另一个人洗衣服，在多个线程上同时执行任务

本文重点关注 Rust 中利用并发处理阻塞 I/O 操作。

### Async Rust 的特点

**Rust 的独特之处：**
- 大多数现代编程语言（如 Go、JavaScript）内置了异步运行时
- Rust 是**运行时无关**的，因为它应用于从裸机到 Web 开发的各个领域
- 标准库提供了名为 **Futures** 的零成本抽象来管理并发状态
- 任务的执行和调度交给第三方运行时（如 tokio）处理

**示例代码结构：**
- 使用 `async fn` 定义异步函数
- 使用 `.await` 等待异步操作完成
- 使用 `#[tokio::main]` 宏启动 tokio 运行时
- 使用 `JoinSet` 并发执行多个任务

### Future 的本质

**核心概念：**
- `async` 关键字实际上是语法糖，将输出包装在 Future 中
- Future 是"可能尚未完成计算的值"
- Future 是**状态机**，跟踪内部操作的进度
- 提供 `poll` 方法与状态交互

**零成本抽象：**
- 状态机编译为类似枚举的结构体
- 在栈上分配，性能开销极小

[
https://dev.to/rosewrightdev/from-futures-to-runtimes-how-async-rust-actually-works-4gec
](
https://dev.to/rosewrightdev/from-futures-to-runtimes-how-async-rust-actually-works-4gec
)
    


### TITLE

## FerrisKey v0.5.0 发布 🦀

FerrisKey v0.5.0 正式发布，这是一个用 Rust 开发的开源身份认证与访问管理(IAM)系统。本次更新为现代身份验证和多租户管理奠定了重要基础。

### 主要新功能

- **🔑 Passkeys（通行密钥）**: 原生 WebAuthn 支持，实现无密码身份验证，无需复杂的基础设施

- **✨ Magic Link（魔法链接）**: 为不想使用密码的用户提供无缝体验，易于集成和使用

- **🏢 Organizations（组织管理）**: 原生多租户设计，支持结构化空间管理、成员管理和上下文隔离

- **🎨 Email Builder Theme（邮件主题构建器）**: 直接从界面自定义事务性邮件模板，提升登录之外的用户体验

### 项目更新

- **📋 路线图公开**: 发布说明和未来规划现已在 ferriskey.rs 上公开，用户可以了解项目方向并参与贡献

- **开源项目**: FerrisKey 是 100% 开源项目，公开开发，依靠社区支持前进

### 如何支持

- ⭐ GitHub 点星是动力来源
- 🤝 赞助支持是长期发展的保障

**项目链接**: 
- 官网：https://ferriskey.rs
- GitHub：https://github.com/ferriskey/ferriskey

[
https://old.reddit.com/r/rust/comments/1sk3o7a/media_ferriskey_v050_an_opensource_iam_in_rust/
](
https://old.reddit.com/r/rust/comments/1sk3o7a/media_ferriskey_v050_an_opensource_iam_in_rust/
)
    


### TITLE

## Nailpit - 恶意爬虫陷阱工具

### 项目简介
Nailpit 是一个攻击性安全工具，专门针对无视 robots.txt 文件和 Disallow 指令的恶意网络爬虫。通过向这些爬虫发送大量垃圾和有毒内容，消耗其资源，阻止其抓取私有/非公开网站内容。

### 核心功能
- **高性能生成**：4个工作线程可在 AMD 7950X CPU 上轻松输出 450 MB/s，每秒处理约 6K 个 64kB 页面请求
- **响应速度**：最多 1-2ms 即可返回完整响应，索引/警告页面可达 270K req/s
- **资源优化**：使用马尔可夫链生成内容，比 LLM 便宜数个数量级，减少 CPU 和内存压力
- **攻击模式**：支持限流和慢速攻击（slow-loris）模式，可选"辛辣模式"进一步干扰爬虫

### 免责声明
- Nailpit 不应公开暴露给普通用户，仅针对机器人/爬虫
- 进入陷阱的链接应对用户隐藏
- 项目不对配置错误或相关后果负责
- 用户需确保仅针对无视 robots.txt 等网络标准的代理使用

### 系统要求
- **x86_64/amd64**：至少支持 x86-64-v3 级别（支持 AVX2）
- **aarch64**：A53 及以上处理器（支持 NEON，如树莓派 3 B+）
- **armv7 和 RISCV64**：无指令优化编译

### 部署方法
**准备工作**：
1. 创建 `input` 目录
2. 添加至少一个 `.txt` 纯文本文件（如 lorem ipsum 文本）
3. 多个文本文件将创建不同的马尔可夫链，产生更多样化的内容

**Docker 部署**（推荐）：
- 支持平台：linux/amd64、linux/arm64、linux/arm/v7、linux/riscv64
- 构建镜像：`docker build . -t nailpit`
- 运行容器：
```bash
docker run -v ./configuration/:/app/configuration -v ./input/:/app/input -p 3001:3001/tcp nailpit:latest
```
- 可通过 `-e NAILPIT_SOCKET=0.0.0.0:3001` 覆盖监听端口

### 配置卷
- `/app/configuration`：配置文件目录
- `/app/input`：用户输入文件目录

[
https://tangled.org/sachy.dev/nailpit
](
https://tangled.org/sachy.dev/nailpit
)
    


### TITLE

## Rust for C Engineers: Safe Systems Programming

一本名为《Rust for C Engineers: Safe Systems Programming》的新书已经发布，由 FromC2Rust Publications 出版。

### 核心要点

- **目标读者**：专为C语言工程师编写，帮助他们迁移到Rust语言

- **教学方法**：通过直接对比的方式，依次展示C和Rust代码示例，通过比较来讲解Rust基础知识

- **内容重点**：建立Rust语言基础，侧重于安全的系统编程

- **获取方式**：
  - 官方网站：https://fromc2rust.com
  - 亚马逊购买：https://www.amazon.com/dp/B0GWXYK8F7

作者希望获得读者的真实反馈。

[
https://old.reddit.com/r/rust/comments/1sjt9vi/rust_for_c_engineers_safe_systems_programming/
](
https://old.reddit.com/r/rust/comments/1sjt9vi/rust_for_c_engineers_safe_systems_programming/
)
    


### TITLE

## sqlc-gen-sqlx

sqlc-gen-sqlx 是一个 sqlc 插件，用于从 SQL 查询生成类型安全的 sqlx Rust 代码。

### 主要功能

该插件为每个带有 sqlc 注解的 SQL 查询生成：
- **SQL 常量**：包含查询文本的 `const SQL: &str`
- **强类型行结构体**：用于 `:one` / `:many` 的 `QueryNameRow`
- **可选参数结构体**：当查询有 2 个或更多参数时生成 `QueryNameParams`
- **执行方法**：在 `pub struct Queries<E>` 上的 `&mut self` 方法来执行查询

`Queries<E>` 在构造时包装一个执行器，支持连接池（`&pool`）或事务（`&mut tx`）。

### 安装配置

在 `sqlc.yaml` 中添加插件配置，支持以下选项：
- `output`：输出文件名（默认 `queries.rs`）
- `overrides`：类型覆盖配置
- `row_derives`、`enum_derives`、`composite_derives`：额外的 derive 宏
- `copy_cheap_types`：标记为可廉价复制的类型

### 支持的 PostgreSQL 类型

涵盖常见类型如：
- 基本类型：`bool`、`int2/4/8`、`float4/8`、`text`、`bytea`
- 日期时间：`timestamptz`、`timestamp`、`date`、`time`
- 特殊类型：`uuid`、`json/jsonb`、`inet`、`macaddr`、`hstore`
- 范围类型：`int4range`、`tsrange` 等
- 数组、枚举和复合类型

### 查询注解

支持多种查询模式：
- **`:exec`**：执行查询，丢弃结果
- **`:execrows`**：返回受影响行数
- **`:one`**：获取单行
- **`:many`**：获取所有行
- **`:batchexec/batchone/batchmany`**：批量执行（返回 Stream）
- **`:copyfrom`**：批量插入

### sqlc 扩展

- **`sqlc.slice()`**：支持动态 IN 子句的运行时占位符扩展
- **`sqlc.embed(table)`**：将嵌入表的列生成为嵌套结构体

### 许可证

MIT 或 Apache-2.0 双许可

[
https://github.com/mathematic-inc/sqlc-gen-sqlx
](
https://github.com/mathematic-inc/sqlc-gen-sqlx
)
    


### TITLE

## Rust Analyzer 更新日志 #323

**发布日期**: 2026年4月13日 (v0.3.2862)

### 新功能

- **命令增强**
  - 增强可运行命令的占位符功能
  - 允许向 cargo metadata 传递额外参数
  - 改进 rustfmt 对相对自定义命令的支持

- **代码补全改进**
  - 在函数返回位置支持 `→` 符号补全
  - 为后缀 `print!` 类补全自动添加分号
  - 改进限定路径中类型锚点的导入补全
  - 在 `ImportAssets` 中考虑路径上下文

- **配置处理**
  - 防止 Cargo [env] 变量覆盖进程变量
  - 将 Markdown 文件加载到 VFS 并监视更改
  - 在虚拟工作空间中加载 rust-analyzer.toml
  - 添加 Cargo 配置时刷新工作空间

- **诊断与修复**
  - 为被 `#![cfg]` 排除的 crate 发出诊断信息
  - 改进 `#[cfg_attr]` 和 `#[cfg]` 的处理
  - 当字段为私有时禁用缺失字段修复
  - 修复自定义检查命令的过时诊断问题

- **类型处理优化**
  - 在"填充结构体字段"中检查强制转换而非统一性
  - 改进块状表达式上的类型不匹配 "Add `Some`" 修复
  - 改进宏调用内的类型不匹配修复
  - 改进 `add_missing_match_arms` 标签

- **编辑器改进**
  - 改进 `onEnter` 功能
  - 修复 SyntaxEditor 中节点的映射问题

### 内部改进

- 添加工作流以更新生成的 lints
- 将多个辅助功能迁移到 SyntaxEditor
- 用 SyntaxFactory 替换部分 make 构造函数
- 改进 SyntaxEditor 中的令牌可变性处理
- 代码清理:移除冗余的 `clone_subtree`
- 为 `vfs::ChangeKind` 派生 Clone、Copy、Hash 特征
- 升级 ESLint 和 TypeScript
- 修复 Code 扩展中的字段名称并使用官方 npm 注册表

[
https://rust-analyzer.github.io/thisweek/2026/04/13/changelog-323.html
](
https://rust-analyzer.github.io/thisweek/2026/04/13/changelog-323.html
)
    


### TITLE

## 扁平错误码是不够的

### 核心观点

文章提出了一个良好的错误处理系统应包含两个关键组成部分：

### 1. 面向用户的错误消息
- **目的**：为用户提供清晰的错误信息
- **要求**：沿着调用链累积高层业务上下文
- **避免**：报告难以理解的底层错误（如"文件或目录不存在"）
- **现状**：这已经是 Go 和 Rust 中的常见实践

### 2. 扁平的错误码/错误类型枚举
- **目的**：用于可靠的程序化错误处理和恢复
- **设计原则**：
  - 每个库只应有一个这样的枚举
  - 应保持最小化
  - 不应暴露库可能遇到的每个具体底层错误
  - 只暴露与调用库的错误处理逻辑相关的细节层级
- **理由**：详细的用户报告已由错误消息覆盖

### 总结
错误处理需要双重设计：人类可读的上下文消息 + 精简的程序化错误码，两者分工明确，互为补充。

[
https://home.expurple.me/posts/flat-error-codes-are-not-enough/
](
https://home.expurple.me/posts/flat-error-codes-are-not-enough/
)
    


### TITLE

## Snoop - 基于 eBPF 的 Linux 系统调用追踪工具

### 项目概述
Snoop 是一个用于 Linux 的系统调用追踪器，基于 eBPF 技术构建。它类似于 strace，但提供了实时交互式界面(TUI)、智能过滤器和更易读的参数解码功能。

### 核心优势
- **性能优越**：使用 eBPF tracepoints 而非 ptrace，不会在每次系统调用时停止进程，让被追踪进程保持全速运行
- **可读性强**：将系统调用参数解码为易于理解的格式
- **实时监控**：提供实时 TUI 界面和保存/重放/对比追踪的功能

### 主要功能
- **交互式界面**：实时可滚动的系统调用流、热门系统调用面板、搜索、分类过滤、暂停/恢复
- **多种输出格式**：
  - `--raw`（strace 兼容格式）
  - `--json`（JSON Lines 格式）
  - `--explain`（高级活动摘要）
- **灵活过滤**：支持文件、网络、慢调用、特定系统调用等过滤
- **参数解码**：支持 60+ 种系统调用的参数解码（路径、标志、套接字地址等）
- **进程追踪**：可附加到运行中的进程或启动新进程，支持追踪派生子进程
- **容器支持**：`--docker` 和 `--pod` 可追踪容器内的所有活动
- **TLS 捕获**：通过 uprobes 钩取 SSL_write/SSL_read 显示明文
- **堆追踪**：追踪 malloc/free/calloc/realloc
- **记录与重放**：可保存追踪数据并离线重放
- **差异对比**：比较两次运行之间的变化
- **火焰图**：生成性能分析火焰图

### 技术特点
- 使用纯 Rust 编写的 eBPF 程序（基于 aya 框架）
- 无需内核模块或 C 工具链
- eBPF 程序嵌入在二进制文件中

[
https://github.com/pandaadir05/snoop
](
https://github.com/pandaadir05/snoop
)
    


### TITLE

## Zweriz：一个支持GPU的脚本语言项目

一位Rust学习者分享了他花6天时间开发的脚本语言**Zweriz**，目标是让GPU编程像写简单脚本一样容易，避免每次都要编写CUDA内核。

### 核心特性

- **GPU支持**：通过`GPU { }`代码块将张量运算直接分发到显存，自动生成PTX字符串并调用自定义CUDA内核
- **神经网络模块**：完整的`nn`模块，包含自动微分（`nn.track`、`nn.backward`、`nn.step`）、标准激活函数（ReLU、GELU、Swish、Sigmoid等）和dropout
- **标准库**：内置数学、网络（HTTP GET/POST、原始TCP）、I/O（包括内存映射二进制文件实现零拷贝数据集加载）、字符串处理和随机数模块
- **国际象棋引擎集成**：为测试GPU集成和查找bug而构建，大部分代码用Zweriz编写，移动生成和启发式算法运行在C++ CUDA内核上

### 语法示例

```
X = random.uniform(2048, 512, -1.0, 1.0)
W1 = random.uniform(512, 1024, -0.1, 0.1)

GPU {
    pred = nn.sigmoid(X @ W1)
    avg_pred = mean(pred)
}
```

### 现状与需求

- 目前仅提供**Linux版本**（CPU版和CUDA版）
- 项目处于早期阶段
- 寻求社区反馈：语言可用性、标准库功能建议、bug报告

**GitHub地址**：https://github.com/toiabzahoor/zweriz

[
https://old.reddit.com/r/rust/comments/1sk4hvn/i_spent_6_days_building_zweriz_a_scripting/
](
https://old.reddit.com/r/rust/comments/1sk4hvn/i_spent_6_days_building_zweriz_a_scripting/
)
    


### TITLE

## Distill-CBL：单文件 COBOL 到 WASM 编译器

Distill-CBL 是一个用 Rust 编写的单文件 COBOL 到 WASM 编译器。它直接生成原始 WASM 字节码，无需依赖 LLVM。该编译器会将原始 COBOL 源代码嵌入到二进制文件中，并提供了一个实时十六进制视图演示。

### 主要特点

- **单文件编译器核心**：整个编译器实现在一个 Rust 文件中
- **直接生成 WASM 二进制**：不依赖 LLVM，直接输出 WASM 字节码
- **线性内存存储模型**：支持 REDEFINES 别名功能
- **嵌入式源代码段**：用于取证恢复
- **完整性见证段**：覆盖嵌入的源代码载荷
- **实时浏览器演示**：提供十六进制视图和源代码恢复功能

### 项目链接

- **代码仓库**：https://github.com/StealthEyeLLC/distill-cbl
- **在线演示**：https://stealtheyellc.github.io/distill-cbl/

### 注意事项

- 这是一个有意简化的、可检查的 COBOL 子集实现，而非完整的 COBOL 实现
- 完整性见证功能用于可审计的恢复，而非签名或来源验证系统

[
https://old.reddit.com/r/rust/comments/1sjzy4u/a_nollvm_coboltowasm_compiler_in_one_rust_file/
](
https://old.reddit.com/r/rust/comments/1sjzy4u/a_nollvm_coboltowasm_compiler_in_one_rust_file/
)
    


### TITLE

## 没有人欠你供应链安全

这篇文章讨论了关于 crates.io（Rust 包管理平台）供应链攻击的批评，以及为什么一些常见的解决方案并不可行。

### 主要观点

**关于供应链攻击的常见误解：**

1. **拼写劫持问题**
   - 恶意库使用与正规库相似的名称（如 num_cpu vs num_cpus）
   - 常被提议的解决方案（直接 URL、命名空间）实际上并不有效
   - 使用 Git URL 同样存在风险，因为组织名称也可能被仿冒
   - 让包标识符变长只会增加用户记忆难度，反而更难识别拼写劫持

2. **沙箱化的局限性**
   - Rust 的构建脚本和过程宏拥有完全的系统访问权限
   - rust-analyzer 在打开项目时会运行 cargo check，可能导致零点击代码执行
   - 虽然可以让 cargo build 变安全，但 cargo test 和 cargo run 无法沙箱化
   - 真正的安全需要系统级隔离，超出了 cargo 的职责范围

3. **代码版本控制问题**
   - crates.io 上的代码和 Git 仓库中的代码并不总是匹配
   - crates.io 设计为永久存档，不允许删除版本（吸取了 npm left-pad 事件的教训）
   - 如果让 crates.io 仅作为 DNS 映射到仓库，维护者可能通过删除内容破坏下游用户
   - 即使从仓库拉取代码，维护者也可能事后强制推送修改历史
   - 某些情况下代码不匹配是合理的（如自动生成的代码）

4. **审核责任的核心问题**
   - 所有这些问题都基于一个未被承认的假设：阻止恶意代码进入 crates.io 是"Rust"的责任
   - 文章质疑这种假设是否合理

### 作者立场

作者认为对 crates.io 的批评偏离了重点，供应链安全不应该完全由平台方负责，许多提议的技术解决方案要么不可行，要么会带来其他问题。

[
https://purplesyringa.moe/blog/no-one-owes-you-supply-chain-security/
](
https://purplesyringa.moe/blog/no-one-owes-you-supply-chain-security/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

