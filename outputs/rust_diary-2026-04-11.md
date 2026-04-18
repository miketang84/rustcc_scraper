【Rust日报】2026-04-11


### TITLE

## 404错误页面

### 主要内容
这不是一篇关于微内核操作系统的文章，而是一个**404错误页面**（页面未找到）。

### 关键点
- **页面不存在**：用户试图访问的URL链接无效或已失效
- **幽默提示**：页面以轻松的口吻告知访问者"请求的文档完全不存在"
- **网站信息**：这是Amit Bahree的个人博客网站
- **技术栈**：网站使用Hugo和PaperMod主题构建

### 结论
原链接可能已失效、移动或从未存在过。如需查看关于"构建微内核操作系统"的内容，建议直接访问该博客的主页或搜索功能寻找相关文章。

[https://blog.desigeek.com/post/2026/02/building-microkernel-part0-why-build-an-os/%5D(vscode-file://vscode-app/c:/Users/Amit/AppData/Local/Programs/Microsoft%20VS%20Code/e7fb5e96c0/resources/app/out/vs/code/electron-browser/workbench/workbench.html)
](https://blog.desigeek.com/post/2026/02/building-microkernel-part0-why-build-an-os/%5D(vscode-file://vscode-app/c:/Users/Amit/AppData/Local/Programs/Microsoft%20VS%20Code/e7fb5e96c0/resources/app/out/vs/code/electron-browser/workbench/workbench.html)
)
    


### TITLE

## Flow-Like：本地优先的工作流自动化平台

### 项目概述
Flow-Like 是一个基于 Rust 开发的可视化工作流引擎，可在本地设备（笔记本电脑、服务器或手机）上运行。该项目强调完全类型化、完全可追溯且完全由用户掌控。

### 核心特点
- **本地运行**：无需依赖云服务，可在笔记本电脑、手机、私有服务器或自选云基础设施上运行
- **数据主权**：数据完全保留在用户指定的位置，不会被强制上传到第三方服务器
- **可视化操作**：通过拖放式区块构建工作流
- **完全透明**：提供清晰的数据来源、变更过程和输出结果记录
- **无供应商锁定**：不受限于特定云服务商，避免企业版付费墙

### 技术栈
- 主要使用 **Rust** 开发
- 支持 **WASM** 模板集成
- 包含前端应用、库文件、模板等多个模块
- 配置了 Docker、Kubernetes 等部署方案

### 项目结构
包含应用程序(apps)、库文件(libs)、包(packages)、模板(templates)、测试(tests)等多个核心目录，以及完善的 CI/CD 配置和许可证扫描机制。

### 开发状态
项目持续活跃更新，最新提交涉及文件上传处理、错误处理优化、依赖更新等改进。

[
https://github.com/TM9657/flow-like
](
https://github.com/TM9657/flow-like
)
    


### TITLE

## Flow-Like 框架介绍

Flow-Like 是一个多语言支持的节点式编程框架,允许开发者使用不同编程语言创建自定义节点。

### 主要特点

- **多语言支持**: 支持 Rust、Python、TypeScript 和 Go 四种主流编程语言
- **简洁的代码实现**: 创建自定义节点仅需 12-20 行代码
- **统一的节点结构**: 所有语言都遵循相似的输入(Input)和输出(Output)模式
- **异步执行**: 所有实现都支持异步运行方式

### 核心功能

以情感分析节点 (SentimentAnalyzer) 为例:
- **输入**: 文本字符串 (text)
- **输出**: 情感评分 (score) 和标签 (label)
- **处理流程**: 通过上下文 (Context) 分析文本并设置输出值

### 构建和部署

- 使用 `flow-like build` 命令编译节点
- 自动编译为 WASM 格式(体积小,约 1.2 KB)
- 自动进行类型检查
- 支持沙箱运行并发布到注册表

### 代码行数对比

- Python: 12 行(最简洁)
- TypeScript: 14 行
- Rust: 15 行
- Go: 20 行

[
http://flow-like.com
](
http://flow-like.com
)
    


### TITLE

## 用 Rust 从零构建微内核的学习项目

一位开发者分享了他们在 AArch64 QEMU virt 平台上使用 Rust 构建小型微内核的学习经历。

### 项目背景
- 作者目前主要从事 AI/ML 工作
- 利用工作间隙重新学习系统基础知识
- 在 `no_std`、裸机环境中体验 Rust 开发

### 实现的功能
- **启动引导**：EL2 → EL1 特权级切换
- **串口通信**：通过 MMIO 实现 PL011 UART 日志输出
- **进程间通信**：基于端点的消息传递 IPC
- **任务调度**：从协作式调度演进到抢占式调度
- **中断处理**：定时器中断 + 上下文切换
- **内存管理**：4 级页表 + MMU 启用
- **地址转换验证**：VA→PA 转换验证（`0xDEADBEEF` 写入/读取测试）

### Rust 语言的优势体现
- **明确的安全边界**：在内核代码中清晰标识 `unsafe` 边界
- **局部化的不安全代码**：虽然仍需使用 `unsafe`，但范围受限且更易推理
- **编译期检查**：类型系统和所有权检查能在编译时捕获运行时难以调试的问题

### 资源分享
作者提供了完整的系列教程（Part 0-4），从第 0 部分开始介绍"为什么要构建操作系统"，并在文章顶部和底部提供了导航链接。

作者谦虚地表示自己并非 Rust 或操作系统开发专家，只是希望分享经验帮助其他学习者。😊

[
https://old.reddit.com/r/rust/comments/1sh6jyu/i_built_a_microkernel_in_rust_from_scratch/
](
https://old.reddit.com/r/rust/comments/1sh6jyu/i_built_a_microkernel_in_rust_from_scratch/
)
    


### TITLE

## Aegraph：Cranelift的中端优化器

### 背景与动机

**什么是Aegraph？**
- Aegraph（acyclic egraph，无环等价图）是Cranelift中端优化器的核心数据结构
- 作者于2022年引入这一方法，经历了完全重写、多次迭代改进和社区讨论
- 在PLDI 2023的EGRAPHS研讨会和最近的Dagstuhl研讨会上进行了展示

**核心问题：传递排序问题（Pass-Ordering Problem）**
- 2022年5月引入别名分析和相关优化后，出现了优化传递集成的问题
- 当时的优化传递包括：GVN（全局值编号）、LICM（循环不变代码移动）、常量传播和代数重写
- 不同优化传递之间需要紧密配合，可能需要任意长的传递调用序列才能完全简化代码

### 设计理念转变

**新的理解角度**
- "节点之海"（sea-of-nodes）方面比"多重表示"或"等价类"部分更为基础
- 采用"节点之海优先"的介绍方式
- 首先展示"单个电子节点的简单等价类"版本（无联合节点），然后再引入联合节点

**历史发展**
- 最初是希望集成e-graphs，aegraphs是为了使其实用而创建
- 教学方法和设计分类是随时间逐渐清晰的

### 三个构建模块

优化器包含三个核心组件：
1. **重写（Rewrites）**
2. **代码移动（Code Motion）**
3. **规范化（Canonicalization）** - 如GVN全局值编号

### 解决方案方向

需要一个统一框架来处理所有传递的可能重写，以细粒度方式交织执行：
- 对局部表达式可以连续应用RLE和GVN多次
- 无需对整个函数体运行每个传递
- 实现"单一不动点循环"，以细粒度迭代直到优化完成

[
https://cfallin.org/blog/2026/04/09/aegraph/
](
https://cfallin.org/blog/2026/04/09/aegraph/
)
    


### TITLE

## 使用 Rust 和 Tauri 成功在 iOS App Store 上架应用的经验分享

一位开发者分享了使用 Rust 和 Tauri 开发跨平台应用 flow-like.com 并成功在 App Store 上架的完整经验。这是一个功能完整的离线自动化应用，支持本地 AI 模型运行。

### 技术架构

应用约 80% 的核心逻辑使用 Rust 编写，前端主要负责 API 集成，技术栈包括：

- **Tauri 2**：跨平台框架，支持 iOS、Android、macOS、Windows、Linux
- **Wasmtime**：用于沙箱化工作流节点，iOS 上使用 Cranelift AOT 编译（因苹果不允许 JIT）
- **ONNX Runtime**：运行完全离线的本地 AI 模型
- **LanceDB**：向量存储
- **DataFusion**：分析查询
- **SQLite**：本地数据持久化

### 优点

- **统一代码库**：一套代码在所有平台上行为基本一致
- **前端灵活性**：支持 TypeScript/JavaScript 及所有能静态输出的框架
- **生态系统完整**：可使用完整的 Rust 和 TS 生态
- **原生集成**：通过 Tauri 插件可集成平台原生功能（如推送通知）
- **交叉编译简单**：Rust crates 到 iOS/Android 的编译基本开箱即用
- **行为一致性**：核心逻辑在 Rust 中实现，移动端和桌面端表现完全一致
- **离线 AI 推理**：ORT 在所有平台上都能运行，无需云依赖

### 缺点

- **原生功能集成复杂**：如推送通知需要自定义插件，显得有些 hack
- **iOS 适配困难**：处理安全区域等前端问题较为痛苦
- **模拟器问题**：iOS 模拟器无法正常工作，需使用真机测试
- **依赖兼容性**：偶尔遇到不支持 `aarch64-apple-ios` 的依赖，需要打补丁或寻找替代方案
- **编译时间长**：全栈构建时间较长，需依赖增量构建
- **调试困难**：移动端 Rust panic 调试比桌面端困难，堆栈跟踪不够清晰

### 结论

开发者表示会 100% 再次选择使用 Tauri，并提供了开源代码供参考。

[
https://old.reddit.com/r/rust/comments/1shgahq/shipped_a_rustheavy_app_on_the_ios_app_store_with/
](
https://old.reddit.com/r/rust/comments/1shgahq/shipped_a_rustheavy_app_on_the_ios_app_store_with/
)
    


### TITLE

## Rust中使用假ZST类型在堆上存储切片的UB问题讨论

### 核心思路
- **问题背景**：某些泛型算法需要使用像`AtomicPtr`这样的瘦指针（thin pointer），但对于`Box<[T]>`或`String`等胖指针（fat pointer）无法直接使用，可能需要二次装箱
- **解决方案**：提出使用带自定义析构函数的假ZST（零大小类型）标记类型配合`Box`使用，在drop时释放堆数据
- **实现方式**：最初想法是`Box<Zst<T>>`，后改为`Pin<Box<Zst<T>>>`

### 技术实现细节
- **内存布局**：在堆上以类似C结构体的方式存储切片数据，包含长度、填充和切片内容
- **大小计算**：`size_of::<T>().max(size_of::<usize>()) + size_of::<T>() * length`
- **对齐处理**：当`T`大于`usize`时，长度字段和切片之间会有未使用的填充字节
- **去除所有权**：使用ZST作为标记类型移除`Box`指针的所有权信息（provenance），防止其在drop时调用dealloc

### 安全性约束
代码定义了严格的使用限制以避免未定义行为（UB）：
- 禁止以任何方式获取该类型的实例
- 禁止从空指针创建引用或`Box`
- 禁止创建构造函数或解引用该类型指针
- 禁止在公共API中暴露`&mut ZstSlice`和`Box<ZstSlice>`（必须使用`Pin`包装）
- 禁止实现`Clone`和`Unpin` trait
- 警告：即使`&ZstSlice`在公共API中暴露，也不应在unsafe代码中使用（编译器可能优化为悬垂指针）

### 验证状态
- 已通过Miri验证基本实现
- 但Miri警告：暴露所有权信息（exposed provenance）的安全性无法完全证明
- 承认这种实现与编译器假设相悖，需要每一步都对抗UB，实现较为脆弱

[
https://old.reddit.com/r/rust/comments/1shydnh/is_it_ub_to_store_owned_slices_on_the_heap_as/
](
https://old.reddit.com/r/rust/comments/1shydnh/is_it_ub_to_store_owned_slices_on_the_heap_as/
)
    


### TITLE

## Rust在Windows上的开发环境配置

一位开发者分享了他们在Windows上使用Rust的经验，并寻求社区建议。

### 关键要点

**尝试过的方案：**
- **RustRover**：感觉不适合，不符合个人习惯
- **Git Bash**：功能受限
- **UCRT**：功能强大但缺少互操作性
- **交叉编译**：特别是在测试GUI应用时体验不佳，工作流程混乱

### 当前采用的解决方案

- **主要环境**：WSL + Neovim（使用rustaceanvim插件进行极简配置）
- **双工具链**：
  - Windows上安装Rust用于构建
  - WSL中安装Rust用于LSP支持
- **构建策略**：混合使用Cargo和Makefile来：
  - 扩展环境变量
  - 映射到正确的工具链
  - 处理路径差异
  - 从WSL在Windows上进行本地构建，同时将目标目录保留在Windows文件系统中

### 提出的问题

作者想了解其他开发者在Windows上使用Rust的最佳开发环境配置，希望获得建议和经验分享。

[
https://old.reddit.com/r/rust/comments/1shsta1/whats_your_rust_setup_on_windows/
](
https://old.reddit.com/r/rust/comments/1shsta1/whats_your_rust_setup_on_windows/
)
    


### TITLE

## Rust程序员在阅读复杂开源项目代码时感到不知所措

### 主要困惑
- 作者在研究大型开源Rust项目时产生了自我怀疑
- 这些代码库结构优秀、处理复杂问题的能力让作者开始质疑自己是否适合编程
- 面对如此优雅和复杂的代码，作者担心自己可能永远无法达到那个水平，或者是否缺少某些基础知识

### 寻求建议
- 作者明知与他人比较不是好习惯，但面对高质量代码时很难避免
- 询问其他开发者是否经历过类似阶段
- 寻求克服这种感觉并持续进步的方法和建议

[
https://old.reddit.com/r/rust/comments/1shin0g/getting_overwhelmed_by_complex_rust_codebases_in/
](
https://old.reddit.com/r/rust/comments/1shin0g/getting_overwhelmed_by_complex_rust_codebases_in/
)
    


### TITLE

## Rust 供应链安全噩梦：攻击方式与防范措施

### 背景
- 近期多个流行软件包遭到攻击，如 axios 等 JavaScript 生态系统中的关键包（周下载量近1亿次）
- 软件行业对供应链攻击的重视程度远不如航空业对安全事故的重视
- 软件行业缺乏从他人错误中学习的机制，改进措施往往未触及问题根源

### Rust 生态系统的攻击面

**核心漏洞：**
- Rust 采用与 JavaScript 类似的不安全依赖管理方式
- 标准库较小，依赖中心化包仓库 crates.io
- 中心化仓库成为单点故障，为攻击者提供可乘之机

**惊人发现：**
- 在 crates.io 上最流行的 999 个包中，约 **17%** 包含与其代码仓库不匹配的代码
- 这意味着大量流行 Rust 包含有未知功能的代码

### 主要攻击方式

1. **目标侦察与初始访问**
   - 利用 crates.io 的 API 列出最受欢迎的包
   - 在 GitHub 上跟踪维护者
   - 发送钓鱼邮件或攻击维护者使用的项目

2. **购买被盗凭证**
   - 从黑客论坛购买被窃取的 API 令牌、Cookie 等凭证
   - 通过后门程序窃取开发者电脑上的认证信息
   - 建议：定期轮换密钥、API 令牌等敏感信息

3. **域名抢注攻击（Typosquatting）**
   - 创建与热门包名称相似的恶意包（改变1-2个字母）
   - 示例：`num_cpu`（恶意）vs `num_cpus`（正版，4亿+下载量）

4. **误导性包名**
   - crates.io 使用全局命名空间，无组织级作用域
   - 攻击者可上传带有合法前缀的恶意包
   - 示例：`tokio-backdoor`、`tokio-workerpool` 等伪装成 tokio 生态的包
   - 通过伪造 README、仓库链接和标签欺骗用户

### 关键启示
软件行业需要像航空业学习，建立系统化的安全事故学习与改进机制，重视供应链安全问题的根本原因。

[
https://kerkour.com/rust-supply-chain-nightmare
](
https://kerkour.com/rust-supply-chain-nightmare
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

