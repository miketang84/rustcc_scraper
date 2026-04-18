【Rust日报】2026-04-05


### TITLE

## testx - 通用测试运行工具

testx 是一个通用的测试运行器,能够自动检测项目的编程语言和测试框架,运行测试并显示统一、美观的输出结果,无需任何配置。

### 核心优势

- **一条命令适用所有语言**：无论项目使用何种语言,只需运行 `testx` 即可
- **零配置**：自动识别项目类型和测试框架
- **统一输出**：所有语言的测试结果以一致的格式展示

### 主要特性

- **自动检测** - 从项目文件自动识别语言和测试框架
- **支持11种语言** - Rust、Go、Python、JavaScript/TypeScript、Java、C/C++、Ruby、Elixir、PHP、.NET、Zig
- **多种输出格式** - Pretty(默认)、JSON、JUnit XML、TAP
- **CI分片** - 使用 `--partition` 在CI节点间分割测试
- **压力测试** - 使用 `testx stress` 多次运行测试以发现不稳定的测试
- **影响分析** - 使用 `--affected` 仅运行受git更改影响的测试
- **智能缓存** - 使用 `--cache` 跳过无变化的测试
- **交互式选择器** - 使用 `testx pick` 模糊搜索并选择特定测试
- **监视模式** - 文件更改时自动重新运行测试
- **重试逻辑** - 自动重试失败的测试
- **并行执行** - 并发运行多个测试套件
- **覆盖率集成** - 支持LCOV、Cobertura、JaCoCo、Go coverage
- **插件系统** - 自定义适配器、报告插件、shell钩子
- **历史跟踪** - 跟踪测试健康评分、不稳定测试和趋势
- **Monorepo支持** - 使用 `testx workspace` 扫描和测试工作区中的所有项目

### 安装方式

```bash
# 从 crates.io 安装
cargo install testx-cli

# 从源码安装
cargo install --path .

# 或从发布页面下载预构建二进制文件
```

### 快速开始

```bash
# 在当前目录运行测试
testx

# 在特定项目运行测试
testx -p /path/to/project

# 仅检测框架不运行测试
testx detect

# 传递额外参数给底层运行器
testx -- --filter my_test
```

### CI分片示例

```bash
# 基于切片(确定性、有序)
testx --partition slice:1/4  # 4个任务中的第1个
testx --partition slice:2/4  # 4个任务中的第2个
```

[https://github.com/whoisdinanath/testx
](https://github.com/whoisdinanath/testx
)
    


### TITLE

# 无法访问内容

很抱歉，我无法直接访问您提供的 GitHub Gist 链接内容。作为 AI 助手，我无法：

### 技术限制
- 直接访问外部 URL 链接
- 浏览网页或在线资源
- 读取 GitHub Gist 上的文件内容

### 建议解决方案
如果您希望我帮助总结内容，请：

1. **复制粘贴文本内容** - 将 Gist 中的文本直接粘贴到对话中
2. **截图说明** - 描述主要内容要点
3. **手动输入** - 将关键信息输入给我

### 如何操作
请打开该 Gist 链接，复制其中的文本内容，然后粘贴到我们的对话中，我会很乐意为您：
- 用中文总结要点
- 使用 ## 格式化标题
- 使用 ### 格式化关键点

期待您分享具体内容！

[
https://gist.github.com/cachebag/fcffeb6999da565e3c07ab9900dbddd5
](
https://gist.github.com/cachebag/fcffeb6999da565e3c07ab9900dbddd5
)
    


### TITLE

## OpenVPN 支持开发追踪

这是一个关于在 nmrs 项目中添加 OpenVPN 支持的追踪议题（Issue #288），由 cachebag 于 2026年3月18日创建。

### 核心特性

- **实现方式**：OpenVPN 使用 NetworkManager VPN 插件模型（`connection.type = "vpn"`，`vpn.service-type = "org.freedesktop.NetworkManager.openvpn"`），这与 WireGuard 的原生连接类型有本质区别
- **配置存储**：所有配置保存在扁平的 `Dict<String, String>` 映射中（`vpn.data` 和 `vpn.secrets`）
- **兼容性**：所有改动都是非破坏性的（仅小版本升级），因为 `VpnType`、`VpnCredentials` 等相关类型都标记为 `#[non_exhaustive]`

### 开发任务清单

**1. 核心 VPN 配置**
- VpnConfig trait 和 WireGuardConfig 重构
- OpenVpnAuthType 枚举和 OpenVpnConfig 结构体
- 带验证功能的 OpenVpnBuilder
- build_openvpn_connection() D-Bus 设置字典
- 重构 core/vpn.rs 以支持 VPN 类型调度
- OpenVPN 输入验证

**2. .ovpn 文件导入**
- .ovpn 文件解析器
- 证书存储（`~/.local/share/nmrs/certs/`）
- NetworkManager::import_ovpn() 公共 API

**3. 高级选项**
- TLS 加固（tls-auth、tls-crypt、tls-crypt-v2、证书验证）
- 压缩和代理支持
- 路由/分离隧道/连接弹性
- VpnConnectionInfo 上的 VpnDetails 枚举

**4. 文档**
- mdbook 中的 OpenVPN 指南页面

[
https://github.com/cachebag/nmrs/issues/288
](
https://github.com/cachebag/nmrs/issues/288
)
    


### TITLE

## nmrs 项目概述

nmrs 是一个用 Rust 编写的 NetworkManager 异步 API 库，通过 D-Bus 提供 Linux 系统上的 Wi-Fi 连接管理功能。

### 核心特点

- **异步优先设计**：基于 zbus 构建，提供可靠的 D-Bus 通信
- **双重组件**：
  - `nmrs`：核心库，提供 NetworkManager 绑定和 Wi-Fi 管理 API
  - `nmrs-gui`：基于 GTK4 的 Wayland 兼容图形界面

### 主要功能

- **网络扫描**：列出可用的 Wi-Fi 网络及信号强度、安全类型
- **网络连接**：支持 WPA-PSK 等安全协议的网络连接
- **错误处理**：提供详细的错误类型（认证失败、网络未找到、超时等）

### 安装方式

- Arch Linux (AUR)：`yay -S nmrs` 或 `paru -S nmrs`
- Nix：`nix-shell -p nmrs`

### 集成支持

- **Waybar**：可配置点击启动
- **平铺窗口管理器**：支持 Hyprland、Sway、i3
- **自定义样式**：通过 `~/.config/nmrs/style.css` 自定义界面

### 技术要求

- Rust 1.94.0+
- NetworkManager（通过 D-Bus 可访问）
- Linux 系统

### 开源许可

双重许可：MIT License 和 Apache License 2.0

[
https://github.com/cachebag/nmrs/tree/dev-openvpn
](
https://github.com/cachebag/nmrs/tree/dev-openvpn
)
    


### TITLE

## nmrs - Rust 异步 NetworkManager API

### 项目概述
nmrs 是一个基于 Rust 的异步优先（async-first）NetworkManager API，通过 D-Bus 进行通信。项目旨在为 Linux 系统上的 Wi-Fi 连接管理提供安全、简单的高级 API。

### 项目结构
- **nmrs**：核心库，提供 NetworkManager 绑定和 Wi-Fi 管理 API
- **nmrs-gui**：兼容 Wayland 的 GTK4 图形界面

### 主要特性
- 基于 zbus 构建，提供可靠的 D-Bus 通信
- 异步操作设计
- 支持网络扫描和连接管理
- 完善的错误处理机制

### 使用示例

#### 扫描网络
可以扫描并显示可用的 Wi-Fi 网络，包括信号强度和安全类型

#### 连接网络
支持连接到 WPA-PSK 保护的网络，可以检查当前连接状态

### 文档资源
- 用户指南：包含全面的教程和示例
- API 文档：docs.rs 上的完整 API 参考
- Discord 社区：为开发者和用户提供支持

### 技术要求
- 需要在 Linux 系统上运行 NetworkManager
- 需要 D-Bus 访问权限
- 使用 Tokio 异步运行时

### 最新更新
- 2026年4月4日：添加 AP 模式的热点检查功能
- 2026年4月3日：Docker 镜像加固和依赖更新
- 采用双重许可：Apache-2.0 和 MIT

[
https://github.com/cachebag/nmrs
](
https://github.com/cachebag/nmrs
)
    


### TITLE

## 完成Rust教程后应该做什么项目？

一位开发者分享了他完成Rust官方教程书后的学习经历和项目选择困惑。

### 学习背景
- 刚完成整本Rust书籍的学习，因母语缺乏在线资源，不仅通读全书还将所有内容和代码示例数字化并添加了注释
- 患有ADHD且有完美主义倾向，学习期间保持项目简单，专注于一件事
- 在数字营销公司工作，编程和计算机科学纯粹是业余爱好

### 编程经验
- 有一定编程基础但较为有限
- 主要使用Python做基础后端开发和桌面/CLI工具
- 擅长自动化脚本开发
- 曾帮助公司实现技术基础设施现代化和自动化

### 当前困惑
- 想找适合初学者的Rust项目来巩固知识和加深实践理解
- 考虑过用clap构建CLI工具，但担心这些项目只是自动化脚本或小工具，无法真正提升Rust技能
- 之前做过一个简单的代码分析CLI工具，但觉得水平不够

### 寻求建议
希望社区推荐能帮助其成长为Rust开发者（而非仅仅是普通程序员）的项目

[
https://old.reddit.com/r/rust/comments/1sc3lr3/just_finished_the_rust_book_what_projects_should/
](
https://old.reddit.com/r/rust/comments/1sc3lr3/just_finished_the_rust_book_what_projects_should/
)
    


### TITLE

## Toasty：Rust 异步 ORM 正式发布

2026年4月3日，Toasty 正式登陆 crates.io。这是一个为 Rust 编程语言设计的异步 ORM（对象关系映射）框架，注重易用性，同时支持 SQL 和 NoSQL 数据库。

### 主要特性

- **数据库支持**：目前 NoSQL 方面仅支持 DynamoDB，计划未来几个月增加更多支持
- **简洁的 API**：通过 `#[derive(toasty::Model)]` 宏定义模型，支持自动 ID、唯一索引、关系映射等特性
- **关系映射**：支持 `has_many`、`belongs_to` 等常见的数据库关系定义
- **异步操作**：原生支持异步数据库操作

### 开发初衷

- **提高 Rust 生产力**：作者认为 Rust 可以成为高级应用（包括 Web 应用）的高效开发语言
- **统一技术栈**：使用一种语言应对从基础设施到浏览器（WASM）的各类场景，提升整体生产力
- **AI 助力 Rust**：作者相信 AI 工具将成为 Rust 普及的推动力，强类型系统能为 AI 辅助编程提供更好的保障

### 开发历程

- **开发周期长**：首次提交距今已超过 2 年，正式发布前已公开开发 1 年多
- **大幅调整**：根据社区反馈放弃了最初的 schema 文件方案，改用 `#[derive(Model)]` 方式
- **多次重写**：在探索过程中对内部实现进行了多次重构

### 核心架构

Toasty 不仅是传统意义上的 ORM，更是一个"应用级查询引擎"：

- **模式解耦**：应用模式与数据库模式完全分离
- **图状结构**：应用模式更接近图结构而非关系表
- **智能转换**：查询引擎根据目标数据库能力自动将 Toasty 语句转换为相应的数据库操作
- **统一 API**：在 SQL 和 NoSQL 之间提供一致的 API 接口，但仍需开发者理解底层数据库特性

[
https://tokio.rs/blog/2026-04-03-toasty-released
](
https://tokio.rs/blog/2026-04-03-toasty-released
)
    


### TITLE

## 关于 Rust UPnP 库使用情况的讨论

### 主要内容

这是一个来自 Reddit r/rust 社区的讨论帖，有人询问是否有人使用过 rustupnp 库。

### 关键点

- **rust-upnp 库**：GitHub 上有 22 个星标和 7 个分支，使用者可能不多
- **rupnp 库**：GitHub 上有 41 个星标和 12 个分支，使用者同样较少
- **结论**：这两个 UPnP 相关的 Rust 库都不太流行，可能都没有被广泛使用
- 回复者以半开玩笑的方式（标注了 /s 表示讽刺）指出这些库的使用率很低，并反问提问者为什么要询问这个问题

### 背景

这反映了 Rust 生态系统中某些细分领域（如 UPnP 协议支持）的库可能还不够成熟或流行。

[
https://old.reddit.com/r/rust/comments/1scsqw9/anyone_used_the_rustupnp_library/
](
https://old.reddit.com/r/rust/comments/1scsqw9/anyone_used_the_rustupnp_library/
)
    


### TITLE

## Waterfox 将集成 Brave 广告拦截引擎，默认启用搜索广告

### 主要内容

**核心变化：**
- Waterfox 将直接集成 Brave 的开源广告拦截引擎到浏览器中，取代依赖扩展程序的拦截方式
- 该消息在 Waterfox 项目 15 周年纪念博客文章中宣布（2026年4月3日）

### 关键要点

**技术实现：**
- 新的内置拦截器将使用 Brave 的广告拦截库，在浏览器主进程中运行而非作为扩展
- 这将使广告拦截更快速、集成更紧密，减少对扩展 API 的依赖
- 选择 Brave 库部分原因是其 MPL 2.0 许可证更适合 Waterfox，而 uBlock Origin 的 GPLv3 许可证集成更复杂

**商业模式：**
- 默认允许在其搜索合作伙伴页面（目前是 Startpage）显示文字广告，作为浏览器的收入来源
- 这是 Waterfox 自己的营收决定，不是从 Brave 技术继承而来
- 用户可通过简单设置禁用所有广告，也可继续使用第三方拦截器

**背景信息：**
- Waterfox 由创始人 Alex Kontos 于 2011 年（时年16岁）创建，最初是 64 位 Firefox 版本
- 目前拥有约 100 万月活跃用户
- 面临财务压力，因 Bing 终止了与第三方的搜索协议

[
https://alternativeto.net/news/2026/4/waterfox-to-integrate-brave-adblock-engine-with-search-ads-enabled-by-default/
](
https://alternativeto.net/news/2026/4/waterfox-to-integrate-brave-adblock-engine-with-search-ads-enabled-by-default/
)
    


### TITLE

## Rust嵌入式开发中的IO trait使用困惑

### 背景
作者在过去一年多使用嵌入式Rust开发，对Embedded-HAL非常满意，特别赞赏其驱动的可移植性。

### 遇到的问题
- **I2C驱动开发**：使用embedded-HAL traits可以轻松实现跨平台复用（微控制器、单板计算机、Mac等）
- **串口(UART)驱动开发**：
  - embedded-HAL中没有相应的trait
  - embedded-io中虽然有相关trait，但流行的crate（如serialport）不支持该trait
  - 导致驱动无法绑定到相应接口

### 求助
作者询问是否遗漏了什么信息，希望获得帮助解决串口设备驱动的跨平台复用问题。

[
https://old.reddit.com/r/rust/comments/1sca0xh/are_people_using_embeddedio_and_embeddedioasync/
](
https://old.reddit.com/r/rust/comments/1sca0xh/are_people_using_embeddedio_and_embeddedioasync/
)
    


### TITLE

## Rust跨平台进程级沙箱实现探讨

### 背景需求
作者正在为一个Tauri应用开发插件系统，计划使用Node作为插件运行时，并希望实现进程级别的沙箱隔离，以便为不同插件设置不同的权限（如某个插件可以访问Todoist，另一个可以访问下载文件夹等）。

### 技术方案调研
- **为何不选择其他方案：**
  - WASM：缺乏包生态系统，开发体验差（无SDK、库和工具）
  - Deno：仅有用户空间权限系统，不兼容原生插件，导致许多有用的包（如sharp）无法使用

- **操作系统级沙箱方案：**
  - Linux：Bubblewrap 或 Landlock
  - macOS：Seatbelt
  - Windows：受限令牌（Restricted Tokens）
  - 这些方案都有较好的Rust绑定

### 面临的问题
1. 这是一个复杂的安全问题，而作者并非安全工程师
2. 现有的Rust crate要么已停止维护（如gaol），要么非常新（如sandbox-rs、ai-sandbox、zerobox）
3. **核心疑问：** 是否应该自己尝试实现这样的解决方案？

### 求助方向
寻求是否有成熟的crate可以解决跨平台进程沙箱问题，或者询问自行开发是否可行。

[
https://old.reddit.com/r/rust/comments/1scm3zh/crossplatform_processlevel_sandboxing/
](
https://old.reddit.com/r/rust/comments/1scm3zh/crossplatform_processlevel_sandboxing/
)
    


### TITLE

## TestX - 通用测试运行器 v0.1.1 发布

一位开发者发布了 TestX 项目的 v0.1.1 版本，这是一个支持 11 种编程语言的通用测试运行器工具。

### 核心特点

- **一键运行**：使用单一命令 `testx`，自动识别并运行相应的测试框架，无需配置
- **多语言支持**：开箱即用支持 11 种语言（Rust、Go、Python、JS/TS、Java、C++、Ruby、Elixir、PHP、Zig、.NET）
- **智能检测**：以 Rust 为例，能自动检测并运行 `cargo test`

### 主要优势

- **多语言项目友好**：在包含多种语言的代码仓库中特别有用，无需记忆每个目录应该使用 pytest、go test 还是 cargo test
- **压力测试模式**：通过 `testx stress -n 50` 命令可以重复运行测试 N 次，用于在合并到主分支前发现不稳定的测试用例

### v0.1.1 新功能

- **Monorepo 支持**：`testx workspace` 命令可以遍历整个代码仓库，自动发现所有项目并使用隔离的适配器并行运行测试

### 项目信息

- GitHub 地址：https://github.com/whoisdinanath/testx
- 开发者正在征求改进意见和功能建议

[
https://old.reddit.com/r/rust/comments/1scy42e/a_universal_test_runner_for_11_languages_testx/
](
https://old.reddit.com/r/rust/comments/1scy42e/a_universal_test_runner_for_11_languages_testx/
)
    


### TITLE

## Rust NetworkManager DBus 绑定库 (nmrs) 介绍

### 项目概述
作者开发了 **nmrs**，一个用于在 Linux 上通过 DBus 与 NetworkManager 交互的 Rust 库。许多人误解了这个项目，以为它是调用 `nmcli` 等外部程序，但实际上它是一个**异步优先的纯 Rust API**。

### 核心特点
- **异步优先**：原生支持 `tokio` 和其他异步框架
- **功能全面**：覆盖 NetworkManager 的大部分主要操作
- **VPN 支持**：WireGuard 已完整支持，OpenVPN 支持即将完成
- **易用性强**：提供简洁的 API，避免了原始 DBus 调用的复杂性

### 优势对比
与原始 DBus 调用相比（161 行代码 vs 简洁代码），nmrs 的优势包括：

1. **代码简洁**：无需手动构造复杂的 `HashMap<String, PropMap>` 嵌套结构
2. **类型安全**：无需记忆 DBus 接口字符串、方法名和属性类型
3. **开发友好**：自动处理设备查找和过滤逻辑
4. **错误处理**：提供有上下文的错误信息，而非模糊的 DBus 错误
5. **现代化**：异步设计，不像现有方案（如 `networkmanager-rs`）是同步阻塞的

### 使用场景
适用于 IoT 设备、GUI 应用程序等需要在 Rust 中管理网络的场景。

### 项目状态
项目已开发 6-7 个月，目前已足够稳定可靠，可用于生产环境。作者欢迎反馈和贡献。

[
https://old.reddit.com/r/rust/comments/1sckbck/why_my_rust_bindings_for_networkmanager_over_dbus/
](
https://old.reddit.com/r/rust/comments/1sckbck/why_my_rust_bindings_for_networkmanager_over_dbus/
)
    


### TITLE

## Rust协程(原生成器)需要更好的语法支持

### 背景问题
作者在编写流式XML解析器时大量使用嵌套协程。当输入缓冲区字符耗尽时，需要一路yield回到入口点，目前只能通过宏来实现这个效果。

### 当前解决方案
- 创建了 `run_to_completion!` 宏来处理协程的恢复和yield传播
- 宏通过循环不断resume协程，将Yielded状态向上传递，Complete状态则返回结果
- 在实际使用中需要手动调用这个宏来包装嵌套的协程调用

### 改进建议

1. **语法糖选项**：
   - 类似async函数的 `.await`，为协程提供 `.yield` 语法
   - 引入类似 `?` 运算符的新操作符，专门处理CoroutineState（yield而非return）

2. **辅助函数需求**：
   - 希望CoroutineState能像Option和Result一样提供辅助方法
   - 特别需要 `map_yield` 和 `map_complete`/`map` 等函数
   - 目前只能使用match/if let语句处理，缺乏便利性

### 现状
这是在nightly版本中测试新特性的体验，反映了Rust协程功能仍在完善中。

[
https://old.reddit.com/r/rust/comments/1scrcr4/would_be_nice_if_coroutinesformerly_gererators/
](
https://old.reddit.com/r/rust/comments/1scrcr4/would_be_nice_if_coroutinesformerly_gererators/
)
    


### TITLE

## Learn Tokio - Rust异步编程学习项目

这是一个渐进式的Rust异步编程学习资源，通过Tokio框架提供系统化的实战练习。

### 项目概述
- 包含一系列循序渐进的编程作业
- 每个作业都基于前一个作业的概念，逐步引入新的异步原语和模式
- 每个作业文件都包含完整的需求说明和提示
- 建议先自行实现，再参考 `src/` 目录中的解决方案

### 前置要求
- 熟悉Rust基础知识（所有权、借用、trait、泛型、错误处理）
- 了解Rust中async/await的基本语法

### 作业列表

**作业1：并发网页抓取器**
- 核心概念：`tokio::spawn`、`JoinHandle`、基础异步错误处理

**作业2：限流任务队列**
- 核心概念：`Semaphore`、有界并发、背压控制

**作业3：基于Channel的聊天服务器**
- 核心概念：`TcpListener`、广播通道、`select!`宏

**作业4：优雅关闭编排器**
- 核心概念：`CancellationToken`、`signal::ctrl_c`、超时控制

**作业5：生产者-消费者流水线**
- 核心概念：有界mpsc通道、多阶段流水线

**作业6：带退避策略的异步重试**
- 核心概念：异步泛型、`tokio::time`、`#[tokio::test]`

**作业7：连接池**
- 核心概念：`Mutex`、`Semaphore`、`Deref`/`Drop`、守卫模式

**作业8：自定义迷你运行时（附加挑战）**
- 核心概念：`Future`、`Waker`、`RawWaker`、`Poll`、`Pin`

### 快速开始
运行作业：
```bash
cargo run --bin hw1
```

运行测试：
```bash
cargo test --bin hw6
```

**注意**：将 `hw1` 替换为对应的作业编号（如 `hw2`、`hw3` 等）

### 学习资源
- **Tokio官方教程** — 从设置到构建mini-Redis的完整指南
- **Tokio API文档** — 所有模块和类型的参考文档
- **mini-redis** — 使用这些模式的真实项目示例

[
https://github.com/freddiehaddad/tokio-lessons
](
https://github.com/freddiehaddad/tokio-lessons
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

