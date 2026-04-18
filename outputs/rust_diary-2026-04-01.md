【Rust日报】2026-04-01


### TITLE

## winit_single

这是一个小型实用工具库，旨在简化单窗口应用程序中 winit 的使用。

### 支持声明

- 作者目前忙于开发游戏（Penta Terra），因此对该库提供的支持有限
- 欢迎自由使用项目中的任何内容，无需注明出处

### 桌面端设置

**安装依赖：**

在 `Cargo.toml` 的 dependencies 部分添加：

```toml
[dependencies]
winit_single = { git = "https://github.com/tilde35/winit_single", branch = "winit_0_30" }
```

**main.rs 快速入门示例：**

```rust
use winit_single::{SingleWindow, winit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = SingleWindow {
        title: "Simple App".to_string(),
        ..Default::default()
    };
    
    cfg.init(|_event_loop, win, init| {
        // 执行图形初始化等操作
        win.request_redraw();
        
        init.run(move |event_loop, win, event| {
            match &event {
                winit::event::Event::WindowEvent { event: w, .. } => match w {
                    winit::event::WindowEvent::CloseRequested => {
                        event_loop.exit();
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        win.request_redraw();
                        // 在此处执行绘制操作
                    }
                    _ => {}
                },
                _ => {}
            }
            Ok(())
        })
    })
}
```

### Web 端设置（WASM）

详细信息请参阅 WebSetup.md 文档。

[
https://github.com/tilde35/winit_single
](
https://github.com/tilde35/winit_single
)
    


### TITLE

## AeroFTP - 跨平台文件传输客户端

### 项目概述
AeroFTP是一款开源的跨平台桌面文件传输客户端,支持FTP、SFTP、WebDAV、S3等多种协议。采用GPL-3.0许可证,使用Rust(Tauri 2)和React 18构建。

### 核心功能

**协议支持**
- 支持25种传输协议:FTP、FTPS、SFTP、WebDAV、S3、Google Drive、Dropbox、OneDrive、MEGA、Box、pCloud等云存储服务
- 单一界面统一管理所有连接

**安全特性**
- AeroVault v2加密容器(AES-256-GCM-SIV、Argon2id加密)
- 支持Cryptomator保险库(格式8)
- 集成操作系统密钥环
- 零遥测,所有凭证本地存储

**智能功能**
- 内置AeroAgent AI助手,支持19个AI提供商(OpenAI、Anthropic、Gemini、Ollama等)
- 47种AI工具用于文件操作、代码编辑和工作流自动化

**编辑与管理**
- Monaco代码编辑器(VS Code引擎)支持远程文件编辑
- 集成终端,支持主题同步
- 双面板文件浏览器,支持拖放操作
- AeroPlayer媒体播放器,内置14种可视化效果
- 支持压缩文件浏览(ZIP、7z、TAR、RAR)及选择性解压

**其他特性**
- AeroCloud个人同步服务,支持后台双向同步
- 完整CLI工具(aeroftp-cli),包含30个子命令和批处理脚本
- 批量重命名功能
- 支持47种语言界面
- 4种主题(浅色、深色、Tokyo Night、Cyber)
- 跨平台支持(Linux、Windows、macOS)

### 目标用户
高级终端用户、开发人员、系统管理员、IT专业人员

### 最新版本
v3.3.1(最近更新:3分钟前)

[
https://sourceforge.net/projects/aeroftp/
](
https://sourceforge.net/projects/aeroftp/
)
    


### TITLE

## AeroFTP - 多协议文件传输与云客户端

### 核心功能
AeroFTP 是一款现代化的文件传输客户端，支持 **23 种协议**，提供统一优雅的界面连接各类服务器和云存储服务。

### 支持的协议（23种）
- **传统协议**：FTP/FTPS (TLS 1.3)、SFTP、WebDAV
- **S3 兼容**：AWS、Backblaze B2、Wasabi、MinIO
- **主流云盘**：Google Drive、Dropbox、OneDrive、MEGA.nz、Box、pCloud
- **加密云存储**：Filen、Internxt Drive（端到端加密）
- **其他服务**：4shared、Azure Blob、Zoho WorkDrive、kDrive、Koofr、Jottacloud、FileLu、Yandex、OpenDrive、GitHub

### 安全特性
- **AeroVault v2**：军用级加密容器（AES-256-GCM-SIV + Argon2id）
- **主密码保护**：支持自动锁定超时
- **Cryptomator 保险库**支持（格式 8）
- **系统密钥环集成**
- **零遥测**：所有凭证本地存储

### 主要功能模块
- **AeroCloud**：将任何 FTP 服务器转变为私有云，支持双向同步和后台服务
- **AeroTools 开发套件**：
  - Monaco 编辑器（VS Code 引擎）用于远程文件编辑
  - 集成 SSH 终端
  - **AeroAgent AI 助手**：支持 19 个 AI 提供商（OpenAI、Anthropic、Gemini 等），提供 45 种 AI 工具
- **AeroSync 可靠同步**：SHA-256 校验、断点续传、错误重试机制

### 其他特色
- 4 种应用主题 + 3 种图标样式
- 压缩文件浏览器（ZIP、7z、TAR、RAR）
- AeroPlayer 媒体播放器（14 种可视化效果 + 10 段均衡器）
- 双窗格浏览器，支持拖放和批量重命名
- **支持 47 种语言**

### 技术信息
- **开发技术**：Rust (Tauri 2) + React 18
- **许可证**：GPL-3.0
- **官网**：www.aeroftp.app
- **文档**：docs.aeroftp.app

**注意**：Snap 版本仅能访问 ~/home 和可移动媒体，如需系统级访问请使用 GitHub 发布的 .deb 或 .AppImage 版本。

[
https://snapcraft.io/aeroftp
](
https://snapcraft.io/aeroftp
)
    


### TITLE

## AeroFTP - 现代化多协议文件管理平台

### 项目概述
AeroFTP 是一个以 FTP 为核心的多协议文件管理平台，集成了6个产品模块，支持47种语言。它是一个 FTP 优先设计的客户端，支持完整加密（可配置 TLS 模式）、证书验证控制、机器可读列表（RFC 3659）和断点续传功能。

### Aero 产品家族的六大模块

1. **AeroCloud** - 个人云存储
   - 支持26种协议的同步和分享功能
   
2. **AeroFile** - 专业文件管理器

3. **AeroSync** - 双向同步引擎

4. **AeroVault** - 军用级加密

5. **AeroTools** - 集成代码编辑器、终端和 AI 聊天
   - 内含 AeroAgent：AI 助手（47种工具，19个提供商）

6. **AeroFTP CLI** - 生产级命令行客户端
   - 支持配置文件、JSON 输出、批处理脚本

### AeroCloud 核心功能

**支持的主要协议：**
- **FTP/FTPS** - 支持显式/隐式 TLS 加密
- **SFTP** - SSH 密钥认证
- **WebDAV** - 支持 Nextcloud、Koofr 等服务
- **S3** - 兼容 AWS、MinIO、Backblaze B2 等
- **主流云存储** - Google Drive、Dropbox、OneDrive、MEGA、Box、pCloud
- **其他服务** - Azure Blob、4shared、Filen、Internxt、kDrive、Zoho WorkDrive、Jottacloud 等
- **代码托管** - GitHub（使用 PAT/Device Flow）、SourceForge（SFTP）

**云同步特性：**
- 双向同步，后台托盘同步
- 选择性同步（文件夹排除）
- .aeroignore 模式支持
- 文件版本控制（.aeroversions/）
- 分享链接功能
- 每个项目独立的本地文件夹
- 存储配额显示
- 冲突检测和命名机制

### 安全与加密
多种加密方案包括客户端 AES 加密（MEGA、Filen、Internxt）、OAuth2 认证、零知识加密等，适用于不同隐私需求场景。

[
https://github.com/axpnet/aeroftp
](
https://github.com/axpnet/aeroftp
)
    


### TITLE

## Gitea CI 自动扩缩容器介绍

本文介绍了一个用 Rust 构建的小型服务 `gitea-ci-autoscaler`，用于按需为 Gitea Actions 自动配置和销毁 CI 运行节点。

### 构建原因

- 团队在项目中使用 Gitea Actions 进行 CI
- 固定基础设施的自托管运行器存在问题：要么为空闲服务器付费，要么在高峰期排队等待
- Gitea 不提供内置的自动扩缩容功能
- 因此构建了这个服务，在 CI 任务等待时启动 Hetzner Cloud 节点，空闲时将其销毁

### 工作原理

- 服务运行在 K3s 集群内，每 5 秒轮询一次
- 每次迭代查询：
  - Gitea 的等待任务和已注册运行器
  - Hetzner 的已配置服务器
  - Kubernetes 的节点和运行器 Pod
- **扩容**：当有任务等待时，创建新的 Hetzner 服务器，使用 cloud-init 脚本自动加入 K3s 集群
- **缩容**：当节点空闲时，分多步骤销毁：
  1. 注销 Gitea 运行器
  2. 排空 Kubernetes 节点
  3. 删除 Kubernetes 节点
  4. 删除 Hetzner 服务器
- 仅在付费计费小时接近结束时销毁服务器（Hetzner 按小时计费）

### Rust 实现亮点

**1. 使用枚举建模节点生命周期**

```rust
pub enum NodeState {
    Provisioning,           // 配置中
    Busy { ... },          // 忙碌
    Idle { ... },          // 空闲
    Deregistering { ... }, // 注销中
    Draining { ... },      // 排空中
    Removing,              // 移除中
}
```

- 每个状态变体只携带该阶段相关的数据
- 编译器防止意外访问已清理资源的数据

**2. 使用 Trait 进行测试**

- 为三个外部 API（Gitea、Hetzner、Kubernetes）定义 trait
- 生产环境使用基于 HTTP 的实现
- 测试环境使用模拟实现
- 可以在不启动真实服务器的情况下验证完整的销毁生命周期
- 测试可验证状态转换顺序和每步的副作用

[
https://rustunit.com/blog/2026/03-30-gitea-ci-autoscaler/
](
https://rustunit.com/blog/2026/03-30-gitea-ci-autoscaler/
)
    


### TITLE

## 用Rust打破AI基础设施垄断 - Tracel AI访谈

这是对Tracel AI公司Nathaniel Simard的采访，重点讨论了GPU编程市场的结构性问题以及Tracel如何通过基于Rust的技术栈来解决这些问题。

### GPU编程生态系统的碎片化问题

- **专用语言需求**：GPU编程无法使用Java或TypeScript等常规语言，必须使用专门的内核语言，如CUDA、Metal、Vulkan、HIP、WebGPU和WGSL等
- **开发模式复杂**：开发者需要用Python或C++等标准语言编写主应用程序（CPU端），然后将GPU内核直接嵌入应用中

### 碎片化带来的核心挑战

- **跨平台困难**：CPU开发可以"一次编写，到处部署"，但GPU编程则完全不同
- **性能优化矛盾**：使用WebGPU等可移植语言无法充分利用硬件特定指令；要获得最佳性能，必须使用硬件专用语言（NVIDIA用CUDA，Apple用Metal，Android用Vulkan）
- **重复开发成本**：在不同GPU上运行同一模型，需要为每个平台重写内核代码
- **硬件特定优化**：即使在同一供应商的不同GPU型号间，也需要针对具体硬件属性进行算法优化

### AI专用芯片的特殊性

- **NPU编程难度**：许多专用处理器（如NPU）不具备传统可编程性，缺少GPU的着色器语言
- **TPU的独特方式**：Google TPU使用Python DSL（Pallas）编程，但本质是配置专有编译器而非直接编写代码
- **统一平台缺失**：无法用单一平台为所有不同硬件类型编写代码

### Tracel的解决方案：CubeCL

- **核心定位**：CubeCL是Rust的扩展，允许开发者使用Rust为GPU或AI加速器编写代码
- **发展目标**：计划支持TPU和各类GPU，专注于数值计算性能优化
- **技术价值**：通过统一的Rust接口解决GPU编程的碎片化问题

[
https://filtra.io/rust/interviews/tracel-mar-26
](
https://filtra.io/rust/interviews/tracel-mar-26
)
    


### TITLE

## Rust：内核空间的内存安全

### 核心观点

这篇文章解释了为什么Rust适合用于操作系统开发，重点关注其内存安全特性。

### Rust OS的独特之处

- **问题的核心不在于速度**，而在于编译器能阻止你犯什么错误
- C语言虽然快速，但缺乏关键的安全保障机制

### C语言在内核中的问题

- **用户空间vs内核空间**：
  - 用户空间的错误通常可以被OS捕获和处理
  - 内核空间没有上层保护，错误会导致整个系统崩溃
- **典型漏洞**：use-after-free（释放后使用）、double-free（重复释放）等内存错误
- **C编译器的缺陷**：默认不会警告这些危险操作

### Rust的强制保障机制

- **所有权模型**：
  - 每个值只有一个所有者
  - 可以借用引用，但规则严格
  - 要么一个可变引用，要么多个不可变引用，不能同时存在
- **编译时检查**：use-after-free等错误在编译阶段就会被拒绝

### unsafe关键字

- **必要性**：内核开发无法100%使用安全Rust（需要操作硬件、原始指针等）
- **作用**：明确标记不安全代码块，将风险区域局部化
- **对比**：C语言中每一行都隐式不安全，Rust中只有标记部分不安全

### 并发安全

- **数据竞争问题**：多核环境下的常见bug
- **类型系统保障**：通过`Sync`和`Send` trait编码线程安全性
- **编译时检查**：未正确同步的共享数据会导致编译错误

### 对业余内核开发者的建议

- **C语言仍有价值**：工具成熟、文档完善，有助于理解底层问题
- **Rust的优势**：
  - unsafe代码区域很小
  - 大部分内核逻辑可用安全Rust编写
  - 提供额外的安全保障层
- **适用场景**：新项目或遇到大量内存安全问题时值得考虑

[
https://oshub.org/users/OSHub/posts/rust-memory-safety-in-kernel-space-9178dd
](
https://oshub.org/users/OSHub/posts/rust-memory-safety-in-kernel-space-9178dd
)
    


### TITLE

## Rust 编写 CUDA 内核的现状

### 当前状况
- **Burn-rs** 是目前较流行的选择，但它更像是一个高层框架
- **Rust CUDA** 似乎符合需求，但最新版本停留在2022年，且缺乏与 PyTorch 的互操作性

### 用户需求
- 在项目中完全切换到 Rust 比较困难
- 更可行的方案是采用 Rust 实现底层函数（如 CUDA 内核），然后在 PyTorch 中调用这些函数
- 需要能够与 PyTorch 良好集成的解决方案

### 主要问题
缺乏成熟且持续维护的 Rust CUDA 库，特别是能够与主流深度学习框架（如 PyTorch）无缝集成的工具

[
https://old.reddit.com/r/rust/comments/1s9drd4/current_state_of_rust_writing_cuda_kernel/
](
https://old.reddit.com/r/rust/comments/1s9drd4/current_state_of_rust_writing_cuda_kernel/
)
    


### TITLE

## 视频播放错误提示

### 主要问题
- 视频无法正常播放
- 出现播放错误信息

### 建议解决方案
- 重启设备
- 取消当前操作并在电脑上登录YouTube
- 稍后重试

### 其他提示
- 用户当前处于未登录状态
- 观看的视频可能会被添加到电视的观看记录中
- 获取分享信息时发生错误

**注：** 提供的内容似乎是YouTube视频播放页面的错误提示信息,而非实际视频内容的文字稿或描述。

[
https://youtu.be/VdoH1eS8c_Q?si=iVD0HeFtsg40oQk0
](
https://youtu.be/VdoH1eS8c_Q?si=iVD0HeFtsg40oQk0
)
    


### TITLE

## beamterm - GPU加速的终端渲染器

beamterm 是一个高性能终端渲染库，专注于实现亚毫秒级的渲染时间，支持 WebGL2 和 OpenGL 3.3，基于单一 Rust 代码库开发。

**重要说明**：beamterm 是终端渲染器而非完整的终端模拟器 - 它负责显示层，用户需要自行提供终端逻辑。

### 主要特性

- **跨平台 GL 支持** - 通过 glow 库，单一渲染核心同时支持 WebGL2（WASM）和 OpenGL 3.3（原生）
- **单次绘制调用** - 一次实例化绘制即可渲染整个终端（例如 200×80 单元格）
- **灵活的字体图集** - 支持静态预生成图集或带 LRU 缓存的动态按需光栅化
- **Unicode 和 Emoji 支持** - 完整的 Unicode 支持，包含字素聚类
- **选择支持（WASM）** - 鼠标驱动的文本选择和剪贴板集成（块/线性模式）
- **可选的 JS/TS 绑定（WASM）** - 提供 JavaScript/TypeScript API 以便集成

### 性能表现

在 Chrome 浏览器（WebGL2/WASM）中的测试数据：

- **渲染时间**：低端硬件目标 <1ms @ 16k 单元格；2019年硬件实现 <1ms @ 45k 单元格
- **绘制调用**：每帧仅 1 次
- **内存使用**：约 8.9MB
- **更新带宽**：60 FPS 时约 22 MB/s

### 快速开始

使用 `beamterm-core` 开发原生桌面应用，基本流程：

1. 加载字体图集
2. 创建终端网格
3. 更新单元格并上传到 GPU
4. 渲染

### 单元格数据结构

每个终端单元格包含：
- **symbol**：显示的字符或字素
- **style**：字体样式（普通、粗体、斜体、粗斜体）
- **effect**：字形效果（无、下划线、删除线）
- **fg/bg**：前景色/背景色（24位 RGB 值）

[
https://github.com/junkdog/beamterm
](
https://github.com/junkdog/beamterm
)
    


### TITLE

## 简化的 Winit 单窗口应用程序

作者看到一些关于 winit 设置过于冗长的讨论后，分享了一个针对单窗口应用程序的简化代码库：**winit_single**

### 主要特点：
- **跨平台支持**：适用于桌面设备和 Web 平台
- **WASM 兼容**：支持 WebAssembly，可在浏览器中运行
- **在线演示**：提供了 WGPU Boids 示例供测试

### 未来计划：
作者的下一步工作是开发 3D 图形 API

[
https://old.reddit.com/r/rust/comments/1s938fg/simplified_winit_singlewindow_applications/
](
https://old.reddit.com/r/rust/comments/1s938fg/simplified_winit_singlewindow_applications/
)
    


### TITLE

## AeroFTP：一个完全由 Rust 驱动的桌面文件传输应用

一位开发者分享了他花费数月开发的开源项目 AeroFTP，这是一个后端完全使用 Rust 编写的跨平台文件传输应用。

### 核心功能

- **多协议支持**：原生支持 FTP、FTPS、SFTP、S3、WebDAV 以及 20+ 云服务提供商 API（Google Drive、OneDrive、Dropbox、MEGA、pCloud、Azure Blob 等）
- **纯 Rust 实现**：无需依赖外部工具或 C 库封装，例如 MEGA 集成是零依赖的加密客户端（AES-128-CTR、PBKDF2-SHA512、RSA 会话认证）
- **跨平台**：基于 Tauri + React 构建，支持 Linux、macOS、Windows

### Rust 的优势

- **性能与安全**：使用 Tokio 实现异步处理，强类型系统，编译时捕获错误，零未定义行为
- **并发同步引擎**：支持文件版本控制、选择性同步、.aeroignore 模式匹配、冲突解决
- **安全特性**：Cryptomator vault 加密（AES-256-GCM-SIV）、Sigstore 供应链验证、OS keyring 凭证隔离、后端日志脱敏
- **安全审计**：通过 Aikido Security 独立审计，排名前 5%，0 个未解决问题，完全符合 OWASP/ISO 27001/NIS2 标准

### 开发者工具

- **AeroTools**：集成 Monaco 编辑器，支持语法高亮、diff 视图、远程文件预览
- **AeroAgent**：可配置的 AI 助手，支持自定义 API 密钥（Anthropic、OpenAI 等），可浏览远程文件、生成传输计划、执行操作
- **CLI**：功能强大的命令行界面，包含 MCP 服务器用于工具集成
- **GitHub 集成**：直接浏览仓库、发布版本和文件
- **SourceForge SFTP**：预配置支持

### 技术栈

- **后端**：Rust（Tauri 2、Tokio、russh、aws-sdk、reqwest、ring、aes-gcm-siv）
- **前端**：React、TypeScript、TailwindCSS
- **打包**：支持 .deb、.rpm、.AppImage、Snap、.dmg、.msi、便携式 .zip
- **国际化**：支持 47 种语言

### 最新版本 v3.3.0

- 全新设计的 IntroHub 前端界面
- 选项卡式界面、命令面板（Ctrl+K）
- 收藏夹、网格/列表视图、动态表单选项卡
- 默认凭证脱敏，防止截图或日志泄露

该项目完全开源免费，开发者秉持互联网应开放、无障碍访问的理念，欢迎社区反馈和建议。

[
https://old.reddit.com/r/rust/comments/1s9enqc/aeroftp_a_fully_rustdriven_desktop_file_transfer/
](
https://old.reddit.com/r/rust/comments/1s9enqc/aeroftp_a_fully_rustdriven_desktop_file_transfer/
)
    


### TITLE

# Slint 默认原生样式变更

## 关键要点

### 什么是 Slint
- **基于 Rust 的 UI 工具包**：用于创建响应式和流畅的用户界面
- **跨平台支持**：适用于从资源受限的嵌入式设备到强大的移动设备和桌面电脑等多种目标平台
- **广泛的系统兼容性**：支持 Android、Windows、Mac、Linux 以及裸机系统

### 核心特性
- **易学的 DSL**：采用领域特定语言（DSL），易于学习和使用
- **原生代码编译**：编译为原生代码，针对目标设备的能力进行优化
- **协作友好**：促进设计师和开发者在共享项目上的协作
- **多语言支持**：业务逻辑开发支持 Rust、C++、JavaScript 或 Python

### 性能优化
- 针对不同设备能力进行优化，确保从低配置嵌入式设备到高性能桌面系统都能流畅运行

[
https://slint.dev/blog/default-native-style-change
](
https://slint.dev/blog/default-native-style-change
)
    


### TITLE

## Claw-Code 项目总结

### 项目背景
- **创纪录的GitHub项目**：这是GitHub历史上增长最快的仓库，发布后仅2小时就突破5万星标
- **创建动机**：2026年3月31日凌晨4点，Claude Code源代码泄露后，作者在压力下连夜将核心功能从原代码移植到Python
- **开发工具**：整个项目使用oh-my-codex (OmX)工具协调开发，这是一个基于OpenAI Codex构建的工作流层

### 项目定位
- **目标**：更好地利用工具，而不仅仅是存储泄露的Claude Code档案
- **性质**：这是一个清洁室(clean-room)重写的Python版本，捕获了Claude Code代理harness的架构模式，但没有复制任何专有源代码

### 技术进展
- **Python版本**：基础Python实现已完成并可正常运行
- **Rust版本**：正在dev/rust分支开发中，预计很快合并到主分支
- Rust实现目标是提供更快、内存安全的harness运行时

### 主要更新（截至2026年3月31日）
- 修复：自动压缩阈值默认为20万tokens
- 修复：工具输入{}前缀bug
- 功能：claude.com的默认OAuth配置
- 修复：关键性问题 - 启用工具、默认权限等

### 作者背景
- 被《华尔街日报》专题报道的Claude Code重度用户
- 2025年单独使用了250亿个Claude Code tokens
- 对harness工程有深入研究兴趣，专注于代理系统如何连接工具、编排任务和管理运行时上下文

[
https://github.com/instructkr/claw-code
](
https://github.com/instructkr/claw-code
)
    


### TITLE

## Rust贡献者考虑撰写详细教程

一位Rust开发者在Reddit上询问社区是否对深入的Rust语言贡献指南感兴趣。

### 主要内容

- **教程计划**：作者打算从Rust代码库中选取一个随机issue，记录从头到尾的完整贡献过程
- **内容特色**：
  - 展示真实的问题解决方法
  - 包含遇到困难和迷茫的时刻
  - 分享查找资源和解决问题的过程
  - 保留试错过程，而非只展示最终完美方案

### 动机

- 作者认为Rust贡献过程对新手来说较为intimidating（令人生畏）
- 希望通过详细的工作流程演示，帮助想要参与但不知从何开始的开发者
- 作者近期在研究语言特性，发现通过尝试"打破"Rust能学到很多东西

### 征求意见

作者询问社区这样的教程是否有用，还是自己想多了

[
https://old.reddit.com/r/rust/comments/1s9btfb/thinking_about_writing_a_detailed_rust/
](
https://old.reddit.com/r/rust/comments/1s9btfb/thinking_about_writing_a_detailed_rust/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

