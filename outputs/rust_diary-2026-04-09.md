【Rust日报】2026-04-09


### TITLE

## xmltodict-fast 项目总结

### 项目概述
xmltodict-fast 是 xmltodict 库的加速版本，通过添加 Rust 扩展层实现了显著的性能提升。它是一个**完全兼容的替代品**，保持相同的 API 和行为，但运行速度大幅提升。

### 核心特性
- **零API变更**：完全兼容原版 xmltodict，无需修改现有代码
- **Rust加速**：使用 PyO3 + quick-xml 替换性能热点路径
- **自动降级**：如果 Rust 扩展无法加载（不支持的平台或 PyPy），自动回退到纯 Python 实现
- **双向转换**：支持 XML 转 Python 字典，以及字典转 XML

### 性能基准测试

**parse() 解析性能**（在 Apple Silicon M 系列芯片上测试）：
- 小文件（~1 KB）：2.6倍加速
- 中等文件（~600 KB）：2.3倍加速
- 大文件（~7 MB）：2.8倍加速
- 命名空间XML（~300 KB）：2.3倍加速
- **注意**：深度嵌套XML（500+层级）会变慢至0.4倍

**unparse() 生成性能**：
- 中等文件：8.2倍加速
- 大文件：8.4倍加速
- 扁平文件：6.1倍加速

### 适用场景

**Rust后端最佳应用**：
- unparse() 操作（6-8倍加速）
- 典型嵌套深度的 XML 解析（2.3-2.8倍加速）
- 文件对象流式处理（约2倍加速）

**自动回退到Python的情况**：
- 深度嵌套的 XML（500+层级）
- 尚未在 Rust 中实现的功能（如 process_namespaces、process_comments 等）
- PyPy 或没有预编译wheel的平台

### 安装和使用

```bash
pip install xmltodict
```

**基本用法**：
```python
import xmltodict

# XML → 字典
result = xmltodict.parse("<root><item id='1'>hello</item></root>")

# 字典 → XML
xml = xmltodict.unparse(result, pretty=True)
```

**流式处理大文件**：
支持通过 `item_depth` 和 `item_callback` 参数处理大型 XML 文件，无需将整个文档树加载到内存中。

[https://github.com/VoicuTomut/xmltodict-fast
](https://github.com/VoicuTomut/xmltodict-fast
)
    


### TITLE

## 我对 Rust 中缺少 placement-new 的解决方案

### 背景
作者此前发帖询问如何在 Rust 中实现原地初始化（in-place initialization），避免栈上分配。在综合社区建议后，作者改进了自己的方法并创建了一个新的 crate。

### 主要内容
- **创建的 crate**：[placenew](https://crates.io/crates/placenew)
  - 这是一个过程宏（proc macro），简化了手动原地初始化的过程
  - 仍存在一些局限性，且并非完全安全

### 关键改进（2.0.0 版本）
1. **安全性提升**：修复了主要的不安全问题
   - 原先不检查结构体初始化是否正确
   - 现在通过添加 lambda 表达式来返回结构体初始化，强制 Rust 进行检查
   - 该建议来自用户 u/lenscas

2. **功能扩展**：
   - 现在支持原地构造非结构体类型（如切片或整数）
   - 可以完全替代所有 `Box::new` 调用

### 技术讨论要点
- **实现方式**：每个独立值先在栈上创建，然后复制过去
- **嵌套支持**：支持嵌套结构（例如大型数组作为大型结构体的字段）
- **编译时检查**：通过未调用的闭包确保所有字段都被设置，在编译时捕获错误
- **优化考虑**：使用 `let _ =` 而非 `let _ensure_struct_correct =` 以便更好地优化

[
https://www.reddit.com/r/rust/comments/1rnkkpq/my_solution_to_the_lack_of_placementnew_in_rust/
](
https://www.reddit.com/r/rust/comments/1rnkkpq/my_solution_to_the_lack_of_placementnew_in_rust/
)
    


### TITLE

## Redox OS 2026年3月月报

Redox OS是一个完全用Rust编写的类Unix通用微内核操作系统。3月份取得了令人振奋的进展。

### 图形系统改进
- Jeremy Soller成功在COSMIC合成器中运行libcosmic演示，这是合成器中首次绘制高级窗口内容
- bjorn3改进了DRM API和GPU内存映射
- 新贡献者Alexander Usenko帮助消除临时的Redox特定ioctl，并开始支持planes

### 赤字加权轮转调度器
- Akshit Gaur实现了新的CPU调度器，减少空闲进程占用活动进程的CPU时间，改善系统性能
- 启用了uutils的nice命令，实现了renice以调整进程优先级

### 命名空间和进程CWD作为能力
- Ibuki Omatsu将命名空间和进程CWD迁移为能力机制，提高了安全性
- 计划未来基于此实现沙箱工具

### 内核死锁检测
- Wildan Mubarok实现了调优自旋互斥锁/读写锁计数器的方法，更容易触发和检测死锁
- 这将消除用户空间挂起，简化测试和调试

### Unicode全面支持
- Wildan Mubarok更新CPython、PHP、GNU Nano、Vim、ncdu和GNU Readline以使用支持Unicode的ncurses库变体(ncursesw)
- Vim获得了Unicode字符支持

### pkgar压缩
- Wildan Mubarok在pkgar包中实现了LZMA2压缩
- 包大小减少约3-5倍，解压时间相对较短
- 有助于减少下载和安装时间

### 动态链接器存储缓存
- Wildan Mubarok实现了存储缓存以减少程序初始化延迟
- 在受控环境中将GCC初始化时间减少50%
- 计划移至RedoxFS文件系统服务以获得更好的性能和安全性

### 基于校验和的包更新
- Wildan Mubarok实现了使用`make push`命令的包更新支持
- 当包校验和变化时安装更新，显著提高开发者工作流程速度

### AI政策和贡献条款
- 明确禁止接受由大语言模型(LLM)生成的贡献
- 制定了贡献者需同意的贡献条款

### 其他改进
- 引导系统、内核和代码清理方面的多项改进

[
https://www.redox-os.org/news/this-month-260331/
](
https://www.redox-os.org/news/this-month-260331/
)
    


### TITLE

## Appwrite Rust SDK 发布公告

### 主要功能模块

**数据库服务 (TablesDB)**
- 创建数据库、表、列和行
- 支持过滤、排序和分页查询

**账户管理 (Account)**
- 管理会话、OAuth认证、多因素认证(MFA)和用户偏好设置

**用户管理 (Users)**
- 管理用户、目标对象和标签

**团队管理 (Teams)**
- 管理团队、成员资格和邀请

**存储服务 (Storage)**
- 创建存储桶、上传和下载文件

**令牌管理 (Tokens)**
- 创建和管理文件访问令牌，确保文件安全访问

**函数服务 (Functions)**
- 创建、部署和管理无服务器函数

**消息服务 (Messaging)**
- 通过Twilio、Sendgrid等提供商发送电子邮件、短信和推送通知

**网站服务 (Sites)**
- 创建、部署和管理Web应用程序

**本地化服务 (Locale)**
- 获取用户地区信息、语言列表、货币、国家和大陆信息

**头像服务 (Avatars)**
- 生成用户首字母头像、二维码、国旗、浏览器图标、网站图标和网页截图

[
https://appwrite.io/blog/post/announcing-appwrite-rust-sdk
](
https://appwrite.io/blog/post/announcing-appwrite-rust-sdk
)
    


### TITLE

# GitHub 平台概览

## 主要功能板块

### AI 代码创建工具
- **GitHub Copilot**: 使用 AI 编写更好的代码
- **GitHub Spark**: 构建和部署智能应用
- **GitHub Models**: 管理和比较提示
- **MCP Registry**: 集成外部工具（新功能）

### 开发者工作流
- **Actions**: 自动化任何工作流程
- **Codespaces**: 即时开发环境
- **Issues**: 计划和跟踪工作
- **Code Review**: 管理代码变更

### 应用安全
- **GitHub Advanced Security**: 查找和修复漏洞
- **Code Security**: 在构建时保护代码安全
- **Secret Protection**: 在泄露前阻止密钥泄露

### 解决方案分类

**按公司规模**:
- 企业
- 中小型团队
- 初创公司
- 非营利组织

**按使用场景**:
- 应用现代化
- DevSecOps
- DevOps
- CI/CD

**按行业**:
- 医疗保健
- 金融服务
- 制造业
- 政府

### 开源社区
- **GitHub Sponsors**: 资助开源开发者
- **Security Lab**: 安全实验室
- **Maintainer Community**: 维护者社区
- **GitHub Stars**: 明星开发者计划
- **Archive Program**: 归档计划

### 企业附加服务
- 企业级安全功能
- 企业级 AI 功能（Copilot for Business）
- 企业级 24/7 技术支持

[
https://github.com/appwrite/appwrite):
](
https://github.com/appwrite/appwrite):
)
    


### TITLE

## xmltodict-fast：使用 Rust 加速的 XML 解析库

一位开发者使用 Rust 对 Python 的 xmltodict 库进行了性能优化，创建了 **xmltodict-fast** 项目。

项目地址：https://github.com/VoicuTomut/xmltodict-fast

### 性能基准测试

**测试环境**：Apple Silicon、Python 3.13、20次运行取最佳结果

#### parse() 解析性能提升：

- **小文件** (~1 KB)：22 → 57 MB/s（**2.6倍**提升）
- **中等文件** (~600 KB)：42 → 96 MB/s（**2.3倍**提升）
- **大文件** (~7 MB)：27 → 78 MB/s（**2.8倍**提升）
- **宽结构文件** (~800 KB)：26 → 67 MB/s（**2.5倍**提升）
- **命名空间文件** (~300 KB)：39 → 89 MB/s（**2.3倍**提升）
- **深层嵌套文件** (~500 KB, 500层)：495 → 194 MB/s（**0.4倍，性能下降**）⚠️

#### unparse() 反解析性能提升：

- **中等文件** (~600 KB)：34 → 277 MB/s（**8.2倍**提升）
- **大文件** (~7 MB)：22 → 185 MB/s（**8.4倍**提升）
- **宽结构文件** (~800 KB)：18 → 110 MB/s（**6.1倍**提升）

### 总结

该项目在大多数场景下都带来了显著的性能提升，特别是在 unparse() 操作中提升超过 6 倍。唯一的不足是在处理深层嵌套 XML 时性能反而下降。

[
https://old.reddit.com/r/rust/comments/1sgjvzl/xmltodictfast/
](
https://old.reddit.com/r/rust/comments/1sgjvzl/xmltodictfast/
)
    


### TITLE

## Rust Placement New 3.0.0 版本更新

这是一个关于 Rust crates `placenew` 库更新的帖子。

### 主要更新内容
- 库更新至 3.0.0 版本，新增了第二个宏 `place_into!`
- 新宏的语法更接近 C++ 的 placement new 语法
- 由于涉及原始指针操作，使用时需要包裹在 `unsafe` 块中

### 作者诉求
- 寻求改进库的反馈意见
- 希望了解社区对此功能的使用场景（作者本人有自己的使用需求，这也是更新的原因）

### 相关链接
- Crates.io 地址：https://crates.io/crates/placenew
- 首次发布帖：提到这是针对 Rust 缺少 placement new 功能的解决方案

[
https://old.reddit.com/r/rust/comments/1sgdc6q/placenew_300/
](
https://old.reddit.com/r/rust/comments/1sgdc6q/placenew_300/
)
    


### TITLE

## 在禁用 CGO 的情况下从 Go 调用 Rust 库

### 主要内容

这段代码展示了一个**汇编语言编写的调用跳板函数**（`abiCallTrampoline`），用于在 Go 和 Rust 之间进行函数调用桥接。

### 关键技术点

- **函数声明**：使用 `NOSPLIT|NOFRAME` 标志，表示不需要栈分裂和标准函数帧
- **栈管理**：分配 40 字节栈空间用于保存寄存器状态
- **寄存器保存**：保存调用者保存寄存器（BX 和 R12）以符合调用约定
- **参数传递**：
  - 从帧指针（R12）中加载函数指针和参数
  - 按照 x86-64 调用约定将参数放入相应寄存器（RDI, RSI, RDX, RCX, R8）
- **函数调用**：通过 `CALL R10` 执行实际的 Rust 函数
- **返回值处理**：将返回值（RAX）保存回帧结构
- **清理工作**：恢复保存的寄存器并释放栈空间

### 应用场景

这种技术允许在**禁用 CGO 的情况下**实现 Go 与 Rust 的互操作，绕过了 CGO 的限制，适用于需要直接调用 Rust 库的 Go 项目。

[
https://stoolap.io/blog/2026/04/08/calling-a-rust-library-from-go-with-cgo-disabled/
](
https://stoolap.io/blog/2026/04/08/calling-a-rust-library-from-go-with-cgo-disabled/
)
    


### TITLE

## 为 TypeScript 构建 Rust 运行时的经验总结

### 项目背景
- **开发规模**：67,000 行 Rust 代码，历时两年
- **核心挑战**：在同一进程中使 Node.js 和 Rust 协同工作
- Encore 最初是纯 Go 框架，在支持 TypeScript 时选择用 Rust 从头编写新运行时

### 选择 Rust 的关键原因

1. **多语言扩展性**：借鉴 Prisma 和 Pydantic 的经验，使用 Rust 核心绑定不同语言运行时，避免为每种语言重复实现基础设施
2. **性能优势**：Node.js 本质上是单线程的，将非业务逻辑移至 Rust 后，HTTP 请求生命周期、数据库连接管理、消息队列、追踪等都运行在 tokio 的多线程环境中
3. **功能覆盖**：运行时处理完整的 HTTP 请求生命周期、数据库连接池、跨三个云服务商的消息队列、分布式追踪、指标收集、对象存储、缓存和基于 Pingora 的 API 网关

### 为何不扩展 Go 运行时

- **Sidecar 方案的问题**：
  - Go 运行时作为独立进程与 Node.js 通过 IPC 通信会产生延迟
  - 每个 API 请求需要跨越进程边界 6-7 次
  - 基准测试显示每个请求增加 2-4ms 开销
  - 运维复杂：需要监控两个进程、处理独立崩溃、关联两套日志

- **同进程方案**：运行时需要与 Node.js 事件循环在同一进程中，使用 Rust + napi-rs 提供内存安全、线程安全保障，并利用 tokio 异步生态处理大量并发连接

### 架构设计

**核心运行时结构**：
- 由多个管理器（Manager）组成，每个负责一个基础设施关注点
- 主要管理器包括：API（HTTP 生命周期）、数据库、消息队列、对象存储、指标、密钥等
- 采用延迟初始化，基于两个 protobuf 配置：应用元数据（编译时生成）和运行时配置

[
https://encore.dev/blog/rust-runtime
](
https://encore.dev/blog/rust-runtime
)
    


### TITLE

## Redox OS 2026年3月进展总结

### 主要更新内容

- **COSMIC集成**：libcosmic演示程序现已在COSMIC合成器上运行
- **内核改进**：
  - 新的CPU调度器
  - 内核死锁运行时检测功能
  - 将命名空间和进程当前工作目录(CWD)作为能力(capabilities)实现
- **系统优化**：
  - 软件包更新和压缩功能改进
  - 增强Unicode支持
- **显示支持**：Orbital VirtIO-GPU显示器调整大小支持
- **政策制定**：新增AI相关政策
- **其他**：众多改进和错误修复

详细信息请访问：https://www.redox-os.org/news/this-month-260331/

[
https://old.reddit.com/r/rust/comments/1sg5hsy/this_month_in_redox_os_march_2026/
](
https://old.reddit.com/r/rust/comments/1sg5hsy/this_month_in_redox_os_march_2026/
)
    


### TITLE

## Surelock - 防止死锁的锁排序机制

### 核心概念
通过对锁进行排序来避免死锁问题。无论线程以何种顺序请求锁，系统都会按照统一的顺序获取锁。

### 工作原理示例
两个线程操作同一组账户（acct_1 和 acct_2）：

**线程 A：** 请求锁的顺序为 (acct_1, acct_2)
**线程 B：** 请求锁的顺序为 (acct_2, acct_1)

### 执行过程

1. **初始化阶段**
   - 两个线程都创建 LockSet
   - 系统自动将锁排序为：[acct_1, acct_2]

2. **获取锁阶段**
   - 线程 A 获取 acct_1 锁
   - 线程 B 等待 acct_1 锁
   - 线程 A 获取 acct_2 锁

3. **释放锁阶段**
   - 线程 A 释放 acct_2 锁
   - 线程 A 释放 acct_1 锁
   - 线程 B 获取 acct_1 锁
   - 线程 B 获取 acct_2 锁

### 关键优势
✅ **无循环依赖** - 通过统一的锁顺序避免了死锁的可能性
✅ **执行成功** - 两个线程都能顺利完成操作

[
https://notes.brooklynzelenka.com/Blog/Surelock
](
https://notes.brooklynzelenka.com/Blog/Surelock
)
    


### TITLE

## Rust Cargo 工作区缺少直接添加依赖的命令行工具

### 核心问题
作者在使用 Cargo 工作区（workspace）时发现，无法通过命令行直接在工作区级别添加依赖项，只能手动编辑 TOML 文件。

### 期望的功能
希望有类似以下的命令：
```
cargo add tokio --workspace
```
这个命令应该能将依赖项直接添加到根目录 `Cargo.toml` 的 `[workspace.dependencies]` 部分，而不是添加到单个 crate 中。

### 现有工具的局限性
- **cargo add** - 只能添加到单个 crate，不支持工作区级别
- **cargo-edit** - 同样不支持将 `[workspace.dependencies]` 作为目标
- **cargo-inherited** - 工作流程相反：需要先将依赖添加到各个 crate，然后再运行命令将它们提升到工作区级别。适合重构，但不适合新建项目

### 作者的疑问
- 这是否是一个已知的功能缺口？
- 是否有自己不知道的 crate 或工作流程可以解决？
- 为什么 `cargo` 官方还没有实现这个看似自然的 `cargo add` 扩展功能？

[
https://old.reddit.com/r/rust/comments/1sg4dbc/why_is_there_no_way_to_add_a_dependency_directly/
](
https://old.reddit.com/r/rust/comments/1sg4dbc/why_is_there_no_way_to_add_a_dependency_directly/
)
    


### TITLE

## Crates.io 钓鱼事件警告

### 事件概述
有人今天发送钓鱼邮件，试图诱导用户以"在 crates.ws 上确认电子邮件地址"为由登录 GitHub（注意：这是一个假冒域名，真实域名是 crates.io）。

### 安全提醒
- **不要点击**这些链接中的任何内容
- 点击后可能会导致 GitHub 账户被盗

### 应对措施
如果已经点击了这些链接：
- 请联系官方邮箱：help@crates.io
- 无需感到羞愧，团队只是想确保你的账户安全

### 事件进展
**更新（UTC时间21:25）**：钓鱼网站使用的 CDN 提供商已采取行动，此次特定事件已解除警报。但请继续保持警惕！

[
https://old.reddit.com/r/rust/comments/1sfyv4k/cratesio_phishing_be_alert_but_not_alarmed/
](
https://old.reddit.com/r/rust/comments/1sfyv4k/cratesio_phishing_be_alert_but_not_alarmed/
)
    


### TITLE

## 寻找一篇关于Rust生命周期的优秀博客文章

### 背景
一位Reddit用户正在寻找几年前读过的一篇关于Rust生命周期（lifetimes）的博客文章，希望分享给刚开始学习Rust的朋友。

### 博客特点
- **内容质量**：对生命周期概念讲解非常出色，即使是已经了解Rust的读者也会被吸引从头到尾完整阅读
- **目标受众**：面向相对初学者，内容详尽全面
- **阅读时间**：作者在3-5年前阅读过该文章

### 视觉特征
- **页面设计**：采用深色主题，背景为深灰色或黑色
- **互动元素**：文章中有一个卡通动物形象（可能是猫头鹰、奶牛或其他动物），通过对话气泡提出问题，作者在文中进行回答

### 当前状态
作者表示搜索未果，希望Reddit社区的用户能够帮助识别这篇博客文章。

[
https://old.reddit.com/r/rust/comments/1sgjq50/trying_to_find_an_excellent_blog_i_read_in_the/
](
https://old.reddit.com/r/rust/comments/1sgjq50/trying_to_find_an_excellent_blog_i_read_in_the/
)
    


### TITLE

## 本周 Rust 646 期（2026年4月8日）

### 社区更新

**官方公告**
- docs.rs 默认构建更少的目标平台
- WebAssembly 目标和未定义符号处理的变更
- 领导委员会 2026年3月更新

**基金会动态**
- Rust 创新实验室的未来规划
- Rust 基金会互操作性倡议更新：从研究到实施

### 项目和工具更新

**重要项目**
- **Surelock**：CPython 的 Rust 实现进度更新
- **RustRover 2026.1**：原生集成 cargo-nextest 的专业测试工具
- **Toasty**：适用于 Rust 的异步 ORM，已发布到 crates.io
- **Proxelar 0.4.0**：拦截和修改流量的工具
- **Ply 1.1**：在 Rust 中构建精美 UI
- **Dumap v1.1**：跨平台磁盘使用树状图可视化工具

**性能案例**
- 使用 Rust 在 15 秒内处理 100 万个国际象棋游戏

### 技术文章

**深度分析**
- 调用图分析
- 修复 Rust 编译器中的问题
- 供应链安全：Rust 可能面临的攻击及缓解措施
- 在 Rust 中构建基于 SSA 的声明式渲染图

**教程**
- 通过构建 Brainfuck 解释器学习 Rust 基础
- uv 的底层工作原理
- 在 Rust 中构建 Postgres 兼容性

### 本周推荐 Crate

**aimdb-core**：类型安全且平台无关的数据管道，使用 Rust 类型系统作为模式，通过 trait 实现定义行为。

### Rust 项目更新

- 上周合并了 **479 个拉取请求**
- 编译器改进：优化投影类型计算、布局循环处理
- 标准库更新：新增整数截断和扩展方法、稳定新的 Range 类型和迭代器
- Cargo 更新：发出未使用依赖项的 lint 警告

### 征集活动

**会议征稿**
- NDC Techtown（挪威，截止日期：2026-04-14）
- EuroRust（西班牙巴塞罗那，截止日期：2026-04-27）

[
https://this-week-in-rust.org/blog/2026/04/08/this-week-in-rust-646/
](
https://this-week-in-rust.org/blog/2026/04/08/this-week-in-rust-646/
)
    


### TITLE

## Appwrite 发布官方 Rust SDK

Appwrite 团队宣布推出官方 Rust SDK，为 Rust 开发者提供了集成 Appwrite 平台的原生支持方案。

### 关于 Appwrite

- **定位**：开源后端平台，用于构建 Web、移动端和服务器应用程序
- **替代方案**：可视为 Firebase/Supabase 与 Vercel/Netlify 的单一产品替代品
- **核心功能**：提供身份验证、数据库、存储、消息传递、云函数和部署支持等基础服务
- **部署方式**：完全开源、支持自托管，也提供托管云服务

### Rust SDK 特性

- **用途**：专为服务器端使用设计
- **官方支持**：为 Rust 开发者提供官方维护的客户端，无需依赖第三方库
- **技术特点**：
  - 支持异步工作流
  - 注重 API 设计、类型安全和操作清晰度
  - 适合在生产环境的 Rust 服务中使用

### 发布意义

团队认识到 Rust 开发者对 API 设计、类型安全和操作透明度有较高期望，因此推出符合这些标准的官方 SDK，让 Rust 团队能够放心地在实际系统中使用。

[
https://old.reddit.com/r/rust/comments/1sggk2p/announcing_a_new_official_appwrite_sdk_for_rust/
](
https://old.reddit.com/r/rust/comments/1sggk2p/announcing_a_new_official_appwrite_sdk_for_rust/
)
    


### TITLE

## Chrome 147 发布说明

**稳定版发布日期：2026年4月7日**

适用于 Android、ChromeOS、Linux、macOS 和 Windows 平台的 Chrome 147 稳定版本。

### 主要更新内容

#### CSS 和 UI 功能

**1. 元素作用域视图过渡（Element-scoped view transitions）**
- 在任意 HTML 元素上提供 `element.startViewTransition()` 方法
- 过渡伪元素受祖先元素的裁剪和变换影响
- 支持在不同元素上同时运行多个过渡效果

**2. CSS contrast-color() 函数**
- 帮助满足无障碍对比度要求
- 可在 CSS 任何需要颜色值的地方使用
- 根据背景色自动返回"黑色"或"白色"以提供最高对比度

**3. Timeline 命名范围滚动**
- 为视图时间线的命名范围集添加了 `scroll` 范围
- 扩展了现有的 `entry`、`exit`、`cover` 和 `contain` 范围

**4. CSS border-shape 属性**
- 允许创建非矩形边框，支持任意形状（多边形、圆形等）
- 与 `clip-path` 不同，它定义边框形状并装饰边框，仅裁剪内部
- 提供两种变体：描边形状和填充两个形状之间

**5. CSSPseudoElement 接口**
- 在 JavaScript 中表示伪元素
- 通过 `Element.pseudo(type)` 返回，支持 `::after`、`::before`、`::marker`
- 提供 `type`、`element`、`parent` 属性和 `pseudo(type)` 方法

**6. 事件的伪目标（Pseudo target on events）**
- 特定事件现在包含 `.pseudoTarget` 属性
- 提供更精确的事件来源信息（区分伪元素点击和原始元素点击）
- 适用于 `UIEvent`、`AnimationEvent` 和 `TransitionEvent`

**7. 解耦 *-width 和 *-style 属性**
- `border-width`、`outline-width` 和 `column-rule-width` 的行为更新
- 计算值始终反映作者指定的值，不再受对应 `*-style` 属性影响
- 与 Firefox 和 WebKit 行为保持一致

**8. SVG <textPath> 元素支持 path 属性**
- 支持使用 `path` 属性内联定义文本路径几何
- 减少对单独定义 `<path>` 元素的需求
- 当 `path` 和 `href` 同时存在时，优先使用 `path` 属性

[
https://developer.chrome.com/release-notes/147
](
https://developer.chrome.com/release-notes/147
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

