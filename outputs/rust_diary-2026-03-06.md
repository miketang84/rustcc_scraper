【Rust日报】2026-03-06


### TITLE

## Rust 文档链接检查工具 - 裸 URL 检测

这是 Rust 编译器中 rustdoc 工具的一个代码模块，用于检测和修复文档中未链接化的 URL。

### 主要功能

- **检测裸 URL**：识别 Markdown 文档中未被链接化的 URL（如 `https://example.com`）
- **自动修复建议**：建议用尖括号包裹 URL（如 `<https://example.com>`）使其自动转换为可点击链接

### 关键技术点

- **正则表达式匹配**：使用正则表达式检测以 `http://` 或 `https://` 开头的完整 URL
  - 匹配子域名、根域名和可选的查询参数或 URL 片段
  
- **Markdown 解析**：通过 pulldown_cmark 解析器遍历文档内容
  - 只检查普通文本内容
  - 跳过代码块和已有链接内的文本

- **智能修复**：
  - 标准情况：在 URL 前后添加 `<` 和 `>`
  - 特殊情况：如果 URL 已被方括号 `[]` 包裹，则替换方括号为尖括号

- **诊断报告**：生成编译器 lint 警告，提示"裸 URL 不会自动转换为可点击链接"，并提供机器可应用的修复建议

### 代码位置

位于 Rust 编译器源码：`src/librustdoc/passes/lint/bare_urls.rs`

[
https://github.com/rust-lang/rust/blob/024757fe/src/librustdoc/passes/lint/bare_urls.rs#L76-L84:
](
https://github.com/rust-lang/rust/blob/024757fe/src/librustdoc/passes/lint/bare_urls.rs#L76-L84:
)
    


### TITLE

## Saikuro - 跨语言IPC库

### 项目简介
Saikuro是一个语言无关、易于使用的IPC（进程间通信）库，用于跨语言集成。它允许开发者用一种语言编写函数，并从任何其他支持的语言中透明调用这些函数。

### 核心特性
- **共享类型模式**：统一的类型定义系统
- **能力强制执行**：权限控制机制
- **可插拔传输**：支持TCP、Unix套接字、WebSocket或内存传输方式
- **语言无关性**：真正的跨语言互操作

### 支持的编程语言
- **Rust** - saikuro (crates.io) ✅
- **TypeScript** - saikuro (npm) ✅
- **Python** - saikuro (PyPI) ✅
- **C#** - Saikuro (NuGet) ✅

*注：目前暂未添加到包管理器，即将推出*

### 使用示例
- **Rust端**：创建Provider，注册"add"函数，通过TCP提供服务
- **TypeScript/Python端**：创建Client，连接服务并调用"math.add"函数

### 项目结构
- **crates/**：核心Rust库（协议、模式、传输、路由、运行时、代码生成）
- **adapters/**：各语言适配器（Rust、TypeScript、Python、C#）
- **docs/**：文档站点

### 许可证
Apache-2.0开源协议

[
https://github.com/Nisoku/Saikuro
](
https://github.com/Nisoku/Saikuro
)
    


### TITLE

## Saikuro - 跨语言调用框架

### 核心功能
Saikuro 是一个跨语言调用框架，支持 TypeScript、Python、C#、Rust 等多种语言之间进行类型安全的函数调用、流传输和通道通信。所有通信通过单一的传输协议完成，无需手写绑定代码。

### 解决的问题
在多语言协作开发时，传统方案需要编写大量 HTTP 样板代码、维护 JSON 格式同步，并处理类型转换和缓冲区丢失等问题。Saikuro 通过统一协议和轻量级适配器替代这些繁琐工作，只需定义一次函数，任何支持的语言都可以直接调用。

### 主要特性
- **传输无关协议**：同一种数据格式支持进程内、Unix socket 或 WebSocket 等多种传输方式
- **严格的模式验证**：运行时强制执行函数、类型、能力和命名空间的声明，避免类型不匹配
- **六种调用原语**：提供 call、cast、stream、channel、batch 和 resource 等调用方式
- **语言适配器**：为 TypeScript、Python、C# 和 Rust 提供轻量级客户端
- **开发模式自动发现**：开发阶段自动发布模式，生产环境可冻结代码生成

### 使用示例
**TypeScript 提供方：**
```typescript
const provider = new Provider({ namespace: "math" });
provider.register("add", (a: number, b: number) => a + b);
await provider.serve();
```

**Python 调用方：**
```python
client = Client()
result = await client.call('math.add', [1, 2])
print(result)  # 输出 3
```

### 安装方式
- TypeScript: `npm install @nisoku/saikuro`
- Python: `pip install @nisoku/saikuro`
- C#: `dotnet add package Saikuro`
- Rust: 添加依赖 `saikuro = "0.1"`

[
https://nisoku.github.io/Saikuro/docs/
](
https://nisoku.github.io/Saikuro/docs/
)
    


### TITLE

## authx-rs 项目总结

authx-rs 是一个为 Rust 开发的生产级身份认证和授权框架。

### 核心设计理念
- **零成本抽象**：基于 trait 的异步原生设计
- **插件化架构**：所有功能均为插件形式，无硬编码
- **框架无关**：核心层不依赖特定 Web 框架

### 架构层次
该项目采用四层架构设计：

1. **HTTP 层 (authx-axum)**
   - 提供 Session 层、限流层、CSRF 保护和处理器

2. **插件层 (authx-plugins)**
   - 邮箱密码登录、TOTP、魔法链接、密码重置
   - 管理后台、OAuth（即将推出）

3. **核心引擎 (authx-core)**
   - 零依赖核心：加密、JWT/EdDSA、RBAC/ABAC
   - 防暴力破解锁定、密钥轮换、事件系统

4. **存储层 (authx-storage)**
   - 提供存储接口适配器
   - 内置内存存储（测试用）和 PostgreSQL 存储
   - 支持自定义适配器

### 安全特性
- **密码哈希**：仅使用 Argon2id 算法
- **会话令牌**：SHA-256 哈希后存储
- **JWT 签名**：采用 Ed25519/EdDSA
- **CSRF 防护**：基于可信源的 Origin/Referer 检查

### 项目结构
- **crates/**：核心 Rust 库
- **examples/**：Axum 和 Actix-web 集成示例
- **packages/**：TypeScript SDK 及 React/Vue 示例应用

[
https://github.com/hamzzy/authx-rs
](
https://github.com/hamzzy/authx-rs
)
    


### TITLE

## Zyn 性能基准测试结果

这份文档比较了 Rust 宏库 `zyn` 与常见替代方案的性能表现，包括 `darling`（属性解析）、原生 `syn + quote`（宏展开）和 `heck`（命名风格转换）。

### 属性解析对比：zyn vs darling

测试内容：解析带有 `#[my_attr(name = "hello", count = 5)]` 的结构体并提取两个字段。

| 基准测试 | 耗时 | 相对基线 | 说明 |
|---------|------|---------|------|
| parse（基线） | 3.15 µs | - | 仅 syn::parse2，无属性提取 |
| darling | 3.38 µs | +7% | darling 提取字段 |
| zyn | 2.92 µs | -7% | zyn 提取相同字段 |

**结论**：zyn 比 darling 更快，甚至比原始 syn 解析基线还快 7%。

### 宏展开流程对比：zyn vs 原生 syn + quote

测试内容：对 5 个字段的结构体执行典型的派生宏流程（解析输入、提取字段、生成 getter 实现）。

| 阶段 | 原生方式 | zyn | 开销 | 说明 |
|------|---------|-----|------|------|
| 解析 | 5.48 µs | 5.55 µs | +1.2% | TokenStream → 解析的结构体 |
| 提取 | 486 ns | 494 ns | +1.6% | 解析的结构体 → 字段 |
| 代码生成 | 4.87 µs | 4.97 µs | +2.0% | 字段 → getter 实现 |
| 完整流程 | 10.60 µs | 11.26 µs | +6.2% | 端到端全流程 |

**结论**：使用 zyn 相比原生方案，总开销约为 660 纳秒（5 字段结构体）。这发生在编译期，对于耗时数秒的编译过程，每次宏调用增加不到 1 微秒的开销可以忽略不计。

### 命名风格转换对比：zyn vs heck

测试内容：将 snake_case 字符串（`"first_name_field"`）转换为三种常见命名风格。

| 转换类型 | heck | zyn | 差异 | 备注 |
|---------|------|-----|------|------|
| snake_case | 253 ns | 183 ns | -27% | zyn 单遍字符扫描更快 |
| PascalCase | 243 ns | 252 ns | +4% | 基本相当，在误差范围内 |
| camelCase | 252 ns | 302 ns | +20% | zyn 使用两遍处理（先 Pascal 再小写首字母） |

**结论**：
- zyn 的 snake_case 转换显著更快（快 27%）
- PascalCase 性能相当
- camelCase 较慢，因为 zyn 采用两次字符串分配而非一次直接处理（这是已知的权衡取舍）

### 重要说明

所有这些操作都发生在**编译期**，而非运行时。用户的最终二进制文件不会承担任何这些开销成本。

[
https://github.com/aacebo/zyn/blob/main/benches/RESULTS.md
](
https://github.com/aacebo/zyn/blob/main/benches/RESULTS.md
)
    


### TITLE

## Zyn - Rust 过程宏模板引擎和框架

Zyn 是一个为 Rust 过程宏设计的模板引擎和框架，它用单一的 crate 替代了传统开发中需要拼凑的多个库（如 syn、quote、大小写转换库、诊断工具和属性解析样板代码）。

### 核心优势

**1. 内置控制流**
- 传统方式需要在模板外使用 Rust 代码处理条件和循环
- Zyn 支持 `@if`、`@for`、`@match` 等内联指令，无需跳出模板

**2. 集成大小写转换**
- 传统方式需要导入 heck 库并手动调用转换函数
- Zyn 提供内置管道操作符：`{{ name | snake }}`、`{{ name | pascal }}` 等

**3. 统一的诊断系统**
- 传统方式中错误、警告、提示信息使用不同机制
- Zyn 提供统一的宏：`error!`、`warn!`、`note!`、`help!`、`bail!`

**4. 简化属性解析**
- 传统方式需要为每个项目手写属性解析器
- Zyn 支持 `#[derive(Attribute)]` 自动生成类型化的属性结构体

**5. 可复用的代码生成**
- 传统方式只能通过返回 TokenStream 的函数实现
- Zyn 提供 `#[zyn::element]` 装饰器创建可组合的模板组件

**6. 依赖简化**
- 传统方式需要 5 个独立的依赖包（syn、quote、proc-macro2、heck、proc-macro-error）
- Zyn 只需单一依赖：`zyn = "0.3"`

### 技术特性

- **模板语法**：使用 `{{ }}` 进行插值，代码可读性强
- **命名格式化**：`{{ name | ident:"get_{}" }}` 一行完成转换
- **自定义转换**：支持 `#[zyn::pipe]` 创建自定义管道
- **入口宏**：`#[zyn::derive]` 和 `#[zyn::attribute]` 替代标准过程宏装饰器
- **调试工具**：`zyn::debug!` 用于打印展开结果

[
https://aacebo.github.io/zyn
](
https://aacebo.github.io/zyn
)
    


### TITLE

## zyn - Rust 过程宏模板引擎

这是一个名为 **zyn** 的 Rust 过程宏框架项目，旨在简化过程宏的开发工作。

### 核心特点

**1. 统一的解决方案**
- 替代了传统开发中需要组合使用的多个crate（syn、quote、heck、proc-macro-error等）
- 将所有功能整合到单一框架中

**2. 内联控制流的模板系统**
- 支持 `@if`、`@for`、`@match` 等控制流语法
- 无需使用 `.iter().map().collect()` 等繁琐代码
- 模板语法更直观简洁

**3. 内置格式转换和管道操作**
- 提供13种内置管道函数：snake、camel、pascal、screaming、kebab、upper、lower、str、trim、plural、singular、ident、fmt
- 支持管道链式调用
- 自动处理命名转换

**4. 可复用组件系统**
- 通过 `#[zyn::element]` 将模板转换为可调用组件
- 支持类型化参数
- 组件可以相互组合

**5. 简化的过程宏入口**
- `#[zyn::derive]` 和 `#[zyn::attribute]` 替代原生宏注解
- 自动解析输入
- 支持提取器模式

**6. 友好的诊断系统**
- 提供 `error!`、`warn!`、`note!`、`help!`、`bail!` 等宏
- 无需手动处理 `syn::Error`
- 编译器输出更清晰

**7. 类型化属性解析**
- 通过 `#[derive(Attribute)]` 生成类型化结构
- 简化辅助属性的处理

### 安装方式
```bash
cargo add zyn
```

[
https://github.com/aacebo/zyn
](
https://github.com/aacebo/zyn
)
    


### TITLE

## Ply：一个用Rust构建应用的新框架

作者想用Rust开发一款多人棋盘游戏（包含服务器、客户端和共享游戏逻辑），但发现现有框架都存在问题，最终决定自己开发Ply框架。

### 现有Rust框架的问题

- **Bevy**：强制使用ECS架构，UI系统基于宏和节点，代码冗长，充斥着`impl Bundle`和`..default()`
- **Iced**：代码中到处是`..default()`和`.into()`，嵌套结构不清晰，代码阅读顺序颠倒
- **egui**：需要手动调用`.add_space()`和分配矩形，简单UI可以，复杂应用很累
- **Slint**：虽然嵌套清晰，但使用独立的标记语言，无法与Rust代码干净集成
- **macroquad**：只是渲染库而非应用引擎，缺少布局系统、文本输入等UI结构

### Clay的尝试与失败

作者尝试使用Clay（C语言布局库）配合macroquad，创建了Clayquad：
- 初期开发速度不错
- 后来出现大量bug：内存泄漏、竞态条件、无纹理管理
- 功能限制：不支持着色器、旋转、滚动需手动实现、无文本输入

### Ply的诞生

**开发过程：**
- 2025年底持续在Clayquad上添加功能
- 2月开始重写：将布局引擎完全移植到Rust
- 实现新API设计，添加着色器、可访问性、CLI、网络等功能

**核心设计：**
- **语法**：构建器模式 + 闭包
- **类型转换**：大量使用`Into<T>`，如`.background_color()`接受多种颜色格式，`.image()`接受多种图片来源
- **便利性**：`use ply_engine::prelude::*`导入所有内容

### 设计哲学

**核心原则：** 在提供完全控制的同时降低使用难度

**即时模式（Immediate-mode）优势：**
- 每帧重建UI实际上比跟踪变化更快
- 布局计算只占帧时间的极小部分
- 无论如何都需要每帧重绘
- 提供对渲染内容和时机的完全控制

[
https://plyx.iz.rs/blog/introducing-ply/
](
https://plyx.iz.rs/blog/introducing-ply/
)
    


### TITLE

## Ply Engine - 用 Rust 构建美观 UI 的跨平台引擎

### 项目概述
Ply 是一个使用 Rust 构建应用程序的引擎,支持 Linux、macOS、Windows、Android、iOS 和 Web 平台。一套代码库,全平台运行。提供 GPU 加速渲染、文本编辑、样式、无障碍、着色器、网络、音频等功能。

### 核心特性
- **统一元素系统**：所有内容都是元素，使用构建器模式和闭包式子元素
- **布局引擎**：类 Flexbox 布局，支持内边距、间距、对齐、滚动、浮动元素
- **文本输入**：光标、选择、撤销/重做、多行、密码模式、键盘快捷键
- **富文本样式**：内联颜色、波浪、脉冲、渐变、打字机、淡入淡出效果
- **着色器**：GLSL 片段着色器、内置效果、SPIR-V 构建管道
- **无障碍支持**：桌面端使用 AccessKit，Web 端使用 JS 桥接
- **调试视图**：Chrome DevTools 风格的检查器
- **网络功能**：HTTP + WebSocket，基于轮询，不阻塞 UI
- **图像和矢量**：PNG、TinyVG 矢量、纹理渲染、程序化矢量
- **音频**：WAV/OGG 播放、音量控制、循环播放

### 平台支持
- **桌面端**：Linux、macOS、Windows（cargo build）
- **Web**：WASM（plyx web）
- **移动端**：Android（plyx apk）、iOS（plyx ios）

### 示例项目
- 着色器游乐场（207 行）
- 贪吃蛇游戏（295 行）
- 待办事项列表（242 行）

### 许可证
零条款 BSD 许可证 - 可用于任何用途，无需署名

[
https://github.com/TheRedDeveloper/ply-engine
](
https://github.com/TheRedDeveloper/ply-engine
)
    


### TITLE

## Ply 交互示例集

这是一个展示 Ply 框架实时 WebAssembly 演示的交互式示例集合。

### 核心示例项目

1. **着色器编辑器（Shader Playground）**
   - 功能：实时编写和预览 GLSL 着色器
   - 代码量：207 行

2. **贪吃蛇游戏（Snake）**
   - 功能：完全使用 Ply 元素构建的经典贪吃蛇游戏
   - 代码量：295 行

3. **待办事项列表（Todo List）**
   - 功能：简单的任务管理应用，支持添加、切换状态、删除和滚动功能
   - 代码量：242 行

### 特点

所有示例均配备实时 WASM（WebAssembly）演示，用户可以直接在浏览器中体验交互效果。

[
https://plyx.iz.rs/examples/
](
https://plyx.iz.rs/examples/
)
    


### TITLE

## Ply 引擎快速入门指南

这是一个关于 Ply 游戏引擎的快速入门教程，帮助开发者在一分钟内搭建第一个项目。

### 安装步骤

- **安装 CLI 工具**：使用 `cargo install plyx` 安装 Ply 的命令行工具 plyx
- **创建项目**：运行 `plyx init .` 通过交互式界面创建项目
- **选择功能**：初始化时可选择内置着色器、文本样式、矢量图形支持、自定义着色器管线等功能模块

### 项目结构

- 项目包含 `Cargo.toml`、`assets/fonts` 字体目录和 `src/main.rs` 主文件
- 使用 `cargo run` 即可运行，显示居中的 "Hello, Ply!" 文本

### 核心概念

- **预导入模块**：通过 `use ply_engine::prelude::*` 一次性导入所有必需组件
- **窗口配置**：设置窗口标题、尺寸、DPI、抗锯齿和 WebGL 版本
- **主循环**：每帧清屏、调用 `ply.begin()` 构建 UI、用 `ui.show()` 布局渲染
- **元素构建**：使用构建器模式创建 UI 元素，支持嵌套结构

### 尺寸控制宏

- `grow!()` - 填充所有可用空间
- `fit!()` - 收缩以适应内容
- `fixed!(200.0)` - 固定像素值
- `percent!(0.5)` - 父元素的百分比

### 许可协议

采用零条款 BSD 许可证，可免费用于任何商业或个人项目，无需署名。

[
https://plyx.iz.rs/docs/getting-started/
](
https://plyx.iz.rs/docs/getting-started/
)
    


### TITLE

## Plyx - 用Rust构建精美应用

### 核心特点

**极简API**
- 采用构建器模式和基于闭包的子元素
- 零样板代码
- 会写Rust就能构建UI

**跨平台支持**
- 单一代码库支持Web、桌面、Android和iOS
- 无需编写平台特定代码

**视觉效果**
- 支持GLSL着色器、旋转效果
- GPU加速的内置特效

**无障碍访问**
- 集成AccessKit和JS桥接
- 内置键盘导航和屏幕阅读器支持

**文本编辑功能**
- 支持文本选择、撤销/重做
- 多行文本、样式文本和虚拟键盘

**调试工具**
- 内置类似浏览器DevTools的检查器

**布局引擎**
- 类Flexbox布局系统
- 支持内边距、间距、对齐、增长、固定、适应、滚动、浮动等

**交互式文档**
- 通过实时可编辑示例进行文档说明
- 可在浏览器中即时调整并查看结果

### 许可证
完全免费开源，采用0-Clause BSD许可证，可用于任何用途，无需署名。

[
https://plyx.iz.rs
](
https://plyx.iz.rs
)
    


### TITLE

## r/rust 子版块介绍

这是 Reddit 上专门讨论 Rust 编程语言的社区，Rust 是一个强调性能、可靠性和生产力的开源系统编程语言。

### 社区规则

1. **遵守行为准则**：尊重他人，保持耐心、友善和同理心，遵守 Rust 项目行为准则

2. **内容相关性**：帖子必须与 Rust 相关，标题需包含有用上下文；Rust 问题应使用置顶问答帖；周末允许发布手工艺类帖子；不允许元帖子

3. **建设性批评**：鼓励批评但必须具有建设性、实用性和可操作性；批评 GitHub 项目时不得直接链接问题追踪器

4. **保持理性**：不要过度狂热，要以慈善的心态对待他人，给予他人怀疑的余地

5. **避免重复讨论**：避免重复已解决或讨论过度的话题，避免纠结细枝末节

6. **禁止低质量内容**：不允许表情包、图片宏等；代码和错误信息需使用正确格式的文本；AI 生成内容可能被删除

### 有用资源

- **官方资源**：官方网站、博客、安装器、源代码、问题追踪器
- **学习资源**：Rust 电子书、标准库 API 参考、示例教程、在线练习场
- **交流平台**：官方用户论坛、Discord、Matrix 聊天、Stack Overflow

[
https://old.reddit.com/r/rust
](
https://old.reddit.com/r/rust
)
    


### TITLE

## 官方 r/rust "招聘专区" - 求职者与招聘方信息汇总

这是 Reddit 上 r/rust 社区的官方招聘专贴，为 Rust 开发者提供求职和招聘信息交流平台。

### 帖子基本信息
- 定期置顶在 r/rust 版块顶部以提高可见度
- 每六周（Rust 新版本发布时）更新一次
- 可通过"最新大型主题"下拉菜单或侧边栏"有用链接"找到

### 求职者规则
- 不要创建顶级评论（顶级评论留给招聘方）
- 可以回复顶级评论提出相关问题
- 求职者应回复置顶的顶级评论
- 元讨论仅限于底部的特定评论区

### 招聘方规则
- 必须是直接招聘，不接受第三方猎头
- 每个雇主只能发一条顶级评论
- 多个职位空缺需合并描述或在回复中补充
- 发布后需校对并编辑错误
- 内容限制：50 行或 500 字以内
- 远程职位需明确说明地域或时区限制

### 招聘帖必需信息模板
1. **公司名称**（可附网站链接）
2. **职位类型**（全职/兼职/实习/合同等）
3. **办公地点**（包括工作语言）
4. **远程选项**（需明确地域/时区限制）
5. **签证担保**
6. **职位描述**（公司业务、Rust 应用场景、经验要求、级别）
7. **薪酬范围**（**必需项**，部分司法管辖区法律要求）
   - 必须提供至少粗略的薪资预期
   - 如使用非法币补偿（加密货币/期权/股权）必须明确说明
   - 不可仅填写"不确定"
8. **联系方式**

### 特别提醒
- 许多地区（包括美国多个州）法律要求职位发布必须包含薪资范围
- 使用非传统货币补偿必须明确标注，否则帖子将被删除

[
https://www.reddit.com/r/rust/comments/1qkkqi9/official_rrust_whos_hiring_thread_for_jobseekers/
](
https://www.reddit.com/r/rust/comments/1qkkqi9/official_rrust_whos_hiring_thread_for_jobseekers/
)
    


### TITLE

## Rust编程语言社区规则与资源总结

这是Reddit上Rust编程语言社区的规则和资源介绍。

### 社区规则

1. **遵守行为准则**
   - 尊重、耐心、友善和同理心对待他人
   - 遵守Rust项目行为准则

2. **内容必须相关**
   - 帖子必须与Rust相关
   - 标题需包含有用的上下文
   - Rust问题请使用置顶的问答帖
   - 艺术创作类帖子仅限周末发布
   - 禁止元讨论帖

3. **建设性批评**
   - 鼓励批评，但必须具有建设性和可操作性
   - 批评GitHub项目时不得直接链接问题追踪器

4. **保持理性**
   - 避免狂热和极端态度
   - 以慈善的意图对待他人

5. **避免重复讨论**
   - 避免重复已解决或讨论过度的话题
   - 这不是官方论坛，不接受功能请求

6. **禁止低质量内容**
   - 禁止表情包、图片宏等
   - 使用正确格式的文本分享代码
   - AI生成内容可能被删除

### 主要资源

- **官方资源**：官方网站、博客、安装程序、源代码、问题追踪器
- **学习资源**：Rust电子书、标准库API参考、示例代码、在线练习场
- **讨论平台**：官方用户论坛、Discord、Matrix聊天、Stack Overflow

[
https://old.reddit.com/r/rust
](
https://old.reddit.com/r/rust
)
    


### TITLE

## Rust动态库中的资源泄漏问题

### 核心问题
作者发现Rust中静态变量的设计使得动态库卸载时很容易发生资源泄漏。由于第三方crate广泛使用全局变量，这导致无法在不产生内存泄漏的情况下卸载使用第三方crate的Rust代码。

### 问题背景：为什么这很重要？

作者来自Windows底层开发背景，在以下场景中动态模块的正确清理至关重要：

- **内核模块**需要支持无内存泄漏的卸载
- **用户态动态库**在卸载时不应留下泄漏
- **高级应用场景**：需要在不终止主进程的情况下远程升级小型动态库

### C++中的全局变量清理

- C++通过编译器/操作系统特定机制来析构全局变量
- 但在进程退出时等待优雅终结耗时很长
- 操作系统会自动释放内存，无需手动释放数千个堆分配
- 堆损坏问题可能在释放时才暴露
- 微软甚至实现了"容错堆"(Fault Tolerance Heap)来故意忽略主函数结束后的释放调用

### Rust的设计选择

Rust**刻意避免释放全局变量**：
- 所有全局变量具有'static生命周期，永不调用drop方法
- 理由是程序终止时操作系统会释放所有资源
- **但这个理由不适用于动态库场景**——进程继续运行，操作系统不会释放资源

### 问题示例

文中给出rustdocs源码中的例子：使用`LazyLock`和`Regex`的静态变量，其分配的内存（可能至少1KB）永远不会被释放。

### 作者的观点

对于一门系统编程语言，这种设计是有问题的。开发者在使用数百个第三方crate的用户态环境中，实际上没有合理的方法在可卸载的上下文中使用Rust。

在Linux内核或Windows内核驱动等特定环境中，可以通过no-std和限制内核中的Rust代码来缓解这个问题。

[
https://old.reddit.com/r/rust/comments/1rmhnuz/is_it_possible_to_create_a_nonleaking_dynamic/
](
https://old.reddit.com/r/rust/comments/1rmhnuz/is_it_possible_to_create_a_nonleaking_dynamic/
)
    


### TITLE

## Rust从零构建机器学习聊天机器人项目

一位开发者分享了他们正在进行的个人项目：**完全使用Rust语言从零开始构建AI聊天机器人，不使用任何外部库**。

### 项目动机
- 追求深入理解和掌握技术能力
- 享受从失败到成功的学习过程
- 拒绝使用现成的TensorFlow等框架或微调方案

### 已完成的技术实现

**1. 基础感知器**
- 从零开始构建了基本的感知器模型

**2. 文本处理机制**
- 创建唯一词汇字典，为每个词分配唯一ID（token）
- 对token进行归一化处理以用于权重计算

**3. 位置编码系统**
- 防止词序混淆（例如区分"hi, how are you"和"how hi are you"）

**4. 注意力层**
- 实现了基础的注意力机制
- 让词语之间相互关联，确保不同组合产生不同的上下文理解

### 未来计划

**文本生成方案**
- **神经元专家系统**：每个神经元作为专家分类句子，通过标签识别意图（如情感分析：-1到1之间的值）
- **马尔可夫模型**：计划使用二元组(bigram)或三元组(trigram)马尔可夫模型生成文本
- 可能将马尔可夫模型与神经元系统结合以克服局限性

[
https://old.reddit.com/r/rust/comments/1rmmbom/i_am_building_a_machine_learning_model_from/
](
https://old.reddit.com/r/rust/comments/1rmmbom/i_am_building_a_machine_learning_model_from/
)
    


### TITLE

## Saikuro：让多语言项目变得简单

一位开发者发布了名为 **Saikuro** 的开源项目，这是一个跨语言调用框架，旨在简化不同编程语言之间的互相调用，无需繁琐的 RPC 样板代码。

### 核心目标
- 让跨语言函数调用像本地调用一样自然
- 保持类型安全、显式化和可预测性

### 技术特点
**支持的语言适配器：**
- TypeScript
- Python
- Rust
- C#

**运行时采用 Rust 构建的原因：**
- 跨平台可移植性强
- 强类型支持
- 并发场景下表现可预测
- 安全嵌入
- 性能优异，适合作为多语言中间层

### 使用示例
**TypeScript 服务提供方：**
```typescript
provider.register("math.add", (a, b) => a + b)
await provider.serve("tcp://127.0.0.1:7700")
```

**Python 调用方：**
```python
client = await Client.connect("tcp://127.0.0.1:7700")
print(await client.call("math.add", [10, 32]))  # 输出 42
```

### 工作原理
- 运行时是独立的 Rust 进程
- 各语言适配器负责序列化/反序列化和注册服务提供者
- 运行时强制执行类型模式并路由调用
- 使用 MessagePack 进行消息封装
- 支持可插拔传输层（TCP、WebSocket、Unix socket、内存）
- 提供六种调用原语：call、cast、stream、channel、batch 和 resource

### 项目资源
- **文档**：https://nisoku.github.io/Saikuro/docs/
- **GitHub**：https://github.com/Nisoku/Saikuro

项目目前处于早期阶段，核心功能已可端到端运行，欢迎反馈和讨论。

[
https://old.reddit.com/r/rust/comments/1rmx6ko/saikuro_making_multilanguage_projects_easy/
](
https://old.reddit.com/r/rust/comments/1rmx6ko/saikuro_making_multilanguage_projects_easy/
)
    


### TITLE

## AuthX：Rust 身份验证工具包

### 项目背景
- 作者在过去几个月的业余时间为 Rust 生态系统开发了一个新工具
- 灵感来源于 TypeScript 中使用 BetterAuth 等工具的良好开发体验
- 发现在构建 Rust 服务时缺少类似的原生身份验证基础组件

### 解决方案
- 创建了 AuthX-RS 项目
- 为 Rust 提供一套完整的身份验证基础功能
- 项目地址：https://github.com/hamzzy/authx-rs

### 核心价值
- 填补 Rust 生态系统中身份验证工具的空白
- 提供类似 TypeScript 生态中优秀工具的开发体验
- 为 Rust 开发者提供开箱即用的身份验证解决方案

[
https://old.reddit.com/r/rust/comments/1rn1jmr/authx_an_authentication_toolkit_for_rust/
](
https://old.reddit.com/r/rust/comments/1rn1jmr/authx_an_authentication_toolkit_for_rust/
)
    


### TITLE

## zyn - Rust 过程宏的模板引擎

作者厌倦了在多个项目中重复构建相同的过程宏脚手架代码（包括 syn 解析、quote 代码生成、heck 大小写转换等），因此创建了 zyn 来解决这个问题。

### 主要特性

**1. 内联控制流**
- 支持 `@if`、`@for`、`@match` 等控制流语句直接在模板中使用
- 无需像 `quote!` 那样使用 `.iter().map().collect()` 的繁琐写法
- 代码更简洁直观

**2. 大小写转换和格式化**
- 内置 13 种管道操作符：`snake`、`camel`、`pascal`、`screaming`、`kebab`、`upper`、`lower`、`str`、`trim`、`plural`、`singular`、`ident`、`fmt`
- 支持链式调用，例如：`{{ name | snake | ident:"get_{}" }}`
- 简化了之前需要手动导入 heck 和使用 `format_ident!` 的流程

**3. 可复用组件**
- 使用 `#[zyn::element]` 将模板转换为可调用组件
- 组件支持类型化参数、接收子块，并可相互组合
- 提高代码复用性

**4. 过程宏入口点**
- `#[zyn::derive]` 和 `#[zyn::attribute]` 替代原生的过程宏注解
- 自动解析输入，使用提取器获取所需内容
- 函数名自动转换为 PascalCase

**5. 诊断功能**
- 内置 `error!`、`warn!`、`note!`、`help!`、`bail!` 等宏
- 无需手动处理 `syn::Error`
- 直接输出编译器友好的错误信息

**6. 类型化属性解析**
- 通过 `#[derive(Attribute)]` 从辅助属性生成类型化结构体
- 支持默认值设置
- 简化属性配置的解析过程

### 总结
zyn 通过提供模板语法、内置工具函数和自动化解析，大幅简化了 Rust 过程宏的开发流程，减少了样板代码。

[
https://old.reddit.com/r/rust/comments/1rmr5xi/zyn_a_template_engine_for_rust_proc_macros/
](
https://old.reddit.com/r/rust/comments/1rmr5xi/zyn_a_template_engine_for_rust_proc_macros/
)
    


### TITLE

## CEL与Rust实现接近原生速度的解释执行

这篇文章介绍了在构建Agentgateway时，如何优化嵌入式表达式语言CEL(Common Expression Language)的性能，使其在Rust中达到接近原生代码的执行速度。

### 背景与需求

- **应用场景**：需要一种嵌入式表达式语言让用户在运行时编写自定义逻辑
- **典型用例**：
  - 提取日志字段（如`request.headers["user-agent"]`）
  - 评估授权条件（如`jwt.sub == "admin" || request.path == "/public"`）
  - 操作请求/响应字段
- **性能要求**：每个请求需要评估数百个表达式，必须足够快速

### 初始性能问题

使用Rust的CEL实现后，虽然功能完善，但性能存在瓶颈：
- CEL占用10-20%的CPU
- 性能测试结果：
  - 基线：60K QPS
  - 构建CEL上下文100次：15K QPS
  - 评估快速表达式100次：37.2K QPS
  - 评估慢速表达式100次：9K QPS

### 核心性能瓶颈

1. **变量物化问题**：在每次评估前必须将原生类型（如`http::Request`）转换为`Value`类型，成本极高
2. **堆分配开销**：所有`Value`对象都是堆分配的，克隆成本高
3. **哈希查找开销**：所有结构都使用哈希映射，每次字段访问约需20ns，嵌套表达式会累积延迟

### 优化方案：原生类型支持

作者提出的解决方案是允许CEL直接在原生Rust类型上解析字段，无需先转换为`Value`类型，只在最终结果时才进行转换，从而大幅减少不必要的类型转换和内存分配开销。

[
https://blog.howardjohn.info/posts/cel-fast/
](
https://blog.howardjohn.info/posts/cel-fast/
)
    


### TITLE

## FORTRAN 到 Rust 转换：第一部分

### 背景
作者对旅行者号太空探测器的图像处理产生兴趣，想要重新实现其处理流程。旅行者号于1977年发射，代码主要用FORTRAN编写。作者希望使用Rust来重新实现这个系统。

### 关键问题
- **SPICE工具包**：NASA的SPICE是一套用于空间几何问题的数据格式、工具和API，主要用FORTRAN 77编写
- **现有解决方案的局限**：
  - 从Rust调用C API会失去Rust的安全保障
  - 可能导致内存损坏和错误结果
  - 应用程序分发更复杂，移植性差
  - 严重依赖全局状态，完全不支持线程安全
  - 某些计算开销很大，无法并发运行

### 解决方案
作者决定将整个SPICE工具包直接从FORTRAN翻译成纯Rust代码：
- 开发了**f2rust**：一个FORTRAN编译器，可以生成Rust代码
- 创建了**rsspice**：50万行纯Rust代码的SPICE工具包移植版本
- 成功通过了完整的SPICE测试套件

### FORTRAN 77 介绍
文章介绍了FORTRAN 77的基本语法特点：
- **子程序声明**：使用SUBROUTINE定义函数
- **隐式类型**：默认情况下，I-N开头的变量是整数，其他是实数（可以用IMPLICIT NONE禁用）
- **数组操作**：支持多维数组
- **循环语句**：DO循环和语句标签系统
- **代码示例**：展示了一个向量加法的简单子程序

[
https://zaynar.co.uk/posts/f2rust-1/
](
https://zaynar.co.uk/posts/f2rust-1/
)
    


### TITLE

## Ply：一个全新的 Rust UI 应用引擎

### 开发背景
开发者在尝试用 Rust 构建多人棋盘游戏时，发现现有的 UI 框架都存在不足：
- **Bevy**：简单功能需要大量 ECS 查询代码
- **Iced**：代码充斥着 `.default()` 和 `.into()`，嵌套难以阅读
- **egui**：简单场景表现不错，但需要手动调用 `.add_space()` 和分配矩形

最终基于 macroquad 渲染库，开发者创建了 Ply 引擎。

### Ply 核心特性
- **简洁的 API**：采用构建器模式，使用闭包处理子元素，只需一行导入
- **跨平台支持**：通过 `plyx` CLI 实现单一代码库运行于 Linux、macOS、Windows、Android、iOS 和 Web
- **灵活的类型转换**：支持多种输入格式（如十六进制整数、浮点元组、颜色对象等）

### 1.0 版本功能
- **布局引擎**：类 Flexbox 的尺寸调整、内边距、间距、对齐、滚动、浮动元素
- **文本输入**：支持选择、撤销/重做、多行、密码模式及标准键盘快捷键
- **富文本样式**：内联颜色、波浪、脉冲、渐变、打字机效果、阴影、逐字符动画
- **GLSL 着色器**：内置效果和 SPIR-V 构建管线
- **无障碍支持**：桌面端使用 AccessKit，Web 端使用 JS 桥接
- **调试工具**：内置 Chrome DevTools 风格的检查器
- **网络功能**：非阻塞的 HTTP 和 WebSocket
- **矢量图形**：TinyVG 格式的按需光栅化
- **其他功能**：旋转、音频播放（WAV/OGG）

### 文档特色
提供**交互式文档**，包含实时 WASM 演示环境，可在浏览器中直接修改代码并即时查看效果。

### 开源信息
- **许可证**：0BSD（无需署名，可用于任何用途）
- **官网**：https://plyx.iz.rs
- **GitHub**：https://github.com/TheRedDeveloper/ply-engine

[
https://old.reddit.com/r/rust/comments/1rmh4kv/after_trying_bevy_iced_and_egui_i_built_my_own/
](
https://old.reddit.com/r/rust/comments/1rmh4kv/after_trying_bevy_iced_and_egui_i_built_my_own/
)
    


### TITLE

## r/rust 官方招聘帖 - 求职者指南

### 帖子概述
这是 r/rust 社区的官方招聘主题帖，定期置顶以提高可见度，每六周随 Rust 新版本发布更新一次。求职者可以查看本帖及往期招聘帖。

### 求职者规则
- **不要**创建顶层评论（顶层评论仅供雇主发布）
- 可以回复顶层评论提出相关问题
- 求职者应回复置顶的专用顶层评论
- 元讨论仅限于底部的特定评论区

### 雇主发帖规则

**基本要求：**
- 必须是直接招聘，不接受第三方猎头
- 每个雇主只能发一条顶层评论
- 多个职位需整合描述或在回复中补充
- 发帖后需校对并修正错误
- 内容限制在 50 行或 500 字以内

**必填信息模板：**
1. **公司名称**（可附官网链接）
2. **职位类型**（全职/兼职/实习/合同等）
3. **办公地点**（包括办公室位置及工作语言）
4. **远程政策**（**需明确说明**地域/时区限制或工作时间要求）
5. **签证支持**
6. **职位描述**（公司业务、Rust 应用场景、经验要求、职级等）
7. **薪资范围**（**强烈建议提供**）
8. **联系方式**

### 薪资信息特别要求
- **必须**尽可能提供薪资预估或范围
- 多地法律要求（包括美国多州）职位发布必须包含薪资信息
- 如薪酬包含**非法定货币**（加密货币、股票期权等），**必须明确说明**
- 仅写"不确定"且未说明非法定货币的帖子**将被删除**

[
https://old.reddit.com/r/rust/comments/1rmra27/official_rrust_whos_hiring_thread_for_jobseekers/
](
https://old.reddit.com/r/rust/comments/1rmra27/official_rrust_whos_hiring_thread_for_jobseekers/
)
    


### TITLE

## Rust中使用下划线类型转换（as _）的讨论

### 背景问题
作者在项目中遇到API类型冲突问题：
- 不同类型定义混杂（i16、u32、usize等）
- 进行数学运算时需要频繁类型转换
- 单行代码中常出现3-4个`as`类型转换，代码冗长

### 提出的解决方案
作者使用 `as _` 进行类型转换：
- 认为某些类型转换纯粹是为了语法需要
- 中间步骤使用usize还是u32对读者来说并不重要
- `as _` 既能完成转换，又不暗示转换具有语义价值
- 可以减少代码字符数

### 核心疑问
作者想知道这种做法是：
- 有实际意义的改进？
- 还是仅仅为了节省几个字符的"贪婪"行为？

[
https://old.reddit.com/r/rust/comments/1rmu9mj/when_if_ever_is_using_underscore_casts_egx_as/
](
https://old.reddit.com/r/rust/comments/1rmu9mj/when_if_ever_is_using_underscore_casts_egx_as/
)
    


### TITLE

## Rust 中的下划线变量使用陷阱

### 核心问题

在 Rust 中使用 `let _ = ...` 来忽略同步锁(synchronization lock)时会导致错误。代码 `let _ = cvar.wait(guard).map_err(|_| ())?;` 会触发编译警告,因为这个锁没有被绑定到变量上,而是立即被丢弃了。

### 关键点

- **错误类型**: `let_underscore_lock` 警告(属于 `let_underscore` 的一部分),在默认情况下被设置为 `deny` 级别
- **问题原因**: 使用 `let _ =` 会导致锁立即被释放,而不是在作用域结束时释放,这可能不是开发者的本意

### 解决方案

编译器提供了两种修复建议:

1. **绑定到未使用的变量**: 
   ```rust
   let _unused = cvar.wait(guard).map_err(|_| ())?;
   ```
   使用命名变量(如 `_unused`)可以让锁在作用域结束时才被释放

2. **显式丢弃值**:
   ```rust
   drop(cvar.wait(guard).map_err(|_| ())?);
   ```
   使用 `drop()` 函数明确表达立即丢弃的意图

### 重要提示

在处理锁和其他需要生命周期管理的资源时,要特别注意 `let _ =` 的使用,避免意外的早期释放导致并发问题。

[
https://gaultier.github.io/blog/rust_underscore_vars.html
](
https://gaultier.github.io/blog/rust_underscore_vars.html
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

