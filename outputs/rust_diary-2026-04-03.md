【Rust日报】2026-04-03


### TITLE

## bigoish - Rust 算法复杂度断言库

### 主要功能
这是一个用于断言和验证函数计算复杂度的 Rust 库，可以自动测试函数是否符合预期的时间复杂度模型。

### 核心特性

**复杂度验证**
- 使用 `assert_best_fit` 宏来断言函数的最佳拟合复杂度模型
- 支持常见复杂度模型，如 O(n*log(n))、O(n) 等
- 通过多组不同规模的输入来拟合复杂度曲线

**测试输入生成**
- 手动方式：提供多个 `(输入大小, 实际输入)` 配对的数组
- 自动方式：使用 `growing_inputs()` 辅助函数自动生成递增规模的测试输入

### 使用示例

**测试排序函数的复杂度：**
1. 定义被测函数（如 `sort` 函数）
2. 创建输入生成器（如 `make_vec` 生成随机向量）
3. 使用 `assert_best_fit` 断言复杂度为 `N * Log(N)`
4. 提供测试数据：从 10 到 100,000 个元素的不同规模输入

**简化输入生成：**
- `growing_inputs(10, make_vec, 25)` 可自动生成 25 组从规模 10 开始递增的测试输入
- 无需手动编写多个不同规模的测试用例

[https://docs.rs/bigoish/latest/bigoish/
](https://docs.rs/bigoish/latest/bigoish/
)
    


### TITLE

## bigoish - Rust 算法复杂度断言库

这是一个用于验证函数计算复杂度的 Rust 测试库的文档示例。

### 主要功能

该库提供了 `assert_best_fit` 函数，用于断言某个函数的实际运行时间复杂度是否符合预期的计算复杂度模型。

### 核心组件

- **N, Log** - 用于表达复杂度模型的符号（如 O(n*log(n))）
- **assert_best_fit** - 核心断言函数，验证最佳拟合的复杂度模型
- **growing_inputs** - 辅助函数，自动生成递增规模的测试输入

### 使用示例

代码演示了如何测试排序函数的复杂度：

1. **定义被测函数**：实现一个排序函数 `sort()`
2. **创建输入生成器**：`make_vec()` 函数生成指定大小的随机数向量
3. **手动指定测试输入**：提供多组不同规模的输入（10、100、1000、10000、100000）
4. **使用辅助函数**：通过 `growing_inputs()` 自动生成 25 组递增的测试输入
5. **断言复杂度**：验证排序函数确实符合 O(n*log(n)) 的时间复杂度

### 适用场景

适合在单元测试中验证算法实现是否达到了预期的时间复杂度性能指标。

[
https://docs.rs/bigoish/latest/bigoish/
](
https://docs.rs/bigoish/latest/bigoish/
)
    


### TITLE

## 内容总结

### 主要内容
这是一个Imgur图片分享页面的界面文本，包含以下元素：

### 关键功能
- **分享选项**：可以通过Facebook、Twitter、Bluesky、Reddit、Email等平台分享
- **操作功能**：包括收藏、嵌入、下载、举报、删除等选项
- **页面信息**：显示8次浏览量，发布时间为2小时前

### 内容片段
页面底部显示了一些热门内容的标题：
- "睡前最柔软的嗷呜声"
- "桌布戏法，波兰版"
- "那么然后呢，同志？"
- "当你从中国电商网站购物时"
- "实际上那些还挺不错的"

**注：** 这些似乎是相关或推荐的其他帖子标题

[
https://imgur.com/a/Udfkbx6
](
https://imgur.com/a/Udfkbx6
)
    


### TITLE

## 我构建了"我构建"构建器（I Built I Built Builder）

### 项目背景
作者注意到 Reddit 的 Rust 板块最近出现了大量"我构建了..."（I built...）类型的帖子，意识到真正缺少的不是另一个 CLI 工具，而是一个专门用于构建这类工具的工具。

### 项目功能
**I Built I Built Builder** 是一个 CLI 工具，具有以下功能：
- 自动搭建 Rust CLI 工具的脚手架
- 生成 README 文档
- 构建发布工作流
- 自动生成"我构建了"帖子的内容
- 帮助开发者更有条理、更高效地创建工具并分享

### 使用示例
```bash
i-built-i-built-builder --name tomlshave --solves "trimming whitespace in embedded TOML snippets"
```
该命令可以自动搭建工具、生成文档，并构建围绕该工具的"我构建了"叙事内容。

### 核心理念
不再是先构建工具、再单独撰写介绍文章，而是通过这个工具同步完成工具开发和"我构建了"内容的创建，提高一致性和效率。

### 项目地址
https://github.com/jams246/i-built-i-built-builder

[
https://www.reddit.com/r/rust/
](
https://www.reddit.com/r/rust/
)
    


### TITLE

## i-built-i-built-builder

一个用于快速搭建 Rust 命令行工具项目的脚手架工具，通过单个命令即可生成包含完整文档、CI 工作流和发布文案的项目结构。

### 核心功能

- **一键生成完整项目**：通过指定工具名称和问题描述，自动生成包含以下内容的 Rust CLI 项目：
  - 基于 clap 的参数解析器
  - 完整的 README.md（包含安装、使用、动机和路线图）
  - CHANGELOG.md
  - GitHub Actions CI/CD 工作流
  - 预写好的 Reddit 发布文案
  - 许可证文件（MIT/Apache 2.0/双许可）
  - Rust 项目专用的 .gitignore

### 使用方式

**基本用法：**
```bash
i-built-i-built-builder --name my-tool --solves "描述你要解决的问题"
```

**完整选项：**
- `--name`：工具名称（必需）
- `--solves`：要解决的问题（必需）
- `--why`：创建工具的理由（可选）
- `--humility`：文档语气级别（low/medium/high）
- `--reddit-tone`：发布文案风格（humble/curious/feedback-welcome）
- `--audience`：目标用户群体（nobody/solo-dev/power-users/teams）
- `--ci`：CI 模式（github/no-ci）
- `--license`：许可证类型（mit/apache2/dual）

### 设计理念

该工具认为任何足够小众的问题都值得拥有一个精心打磨的命令行界面和专业的项目展示。它不仅生成代码，更创建一个"发布就绪"的完整生态系统，包括现代化的仓库呈现、开箱即用的 CI 和面向社区的发布材料。

### 安装方法

```bash
cargo install i-built-i-built-builder
```

或从源码安装：
```bash
git clone https://github.com/jams246/i-built-i-built-builder.git
cd i-built-i-built-builder
cargo install --path .
```

[
https://github.com/jams246/i-built-i-built-builder
](
https://github.com/jams246/i-built-i-built-builder
)
    


### TITLE

## Rust闭包学习求助

### 背景
一位开发者正在学习Leptos框架（用于Tauri前端GUI项目），发现代码中大量使用闭包，但对闭包概念理解不够深入。

### 当前理解程度
该开发者已知闭包是：
- **匿名函数**
- 能够**捕获定义环境**中的变量
- 捕获方式包括三种：
  - 不可变引用
  - 可变引用
  - 获取所有权

### 需求
希望获得关于Rust闭包更详细、清晰的解释，以便更好地理解和使用Leptos框架中的闭包特性。

[
https://old.reddit.com/r/rust/comments/1sagkit/can_someone_give_a_nice_explanation_of_closures/
](
https://old.reddit.com/r/rust/comments/1sagkit/can_someone_give_a_nice_explanation_of_closures/
)
    


### TITLE

## Rust 社区对新 3D 物理引擎的兴趣讨论

### 背景
一位开发者在制作游戏时，因为对 rapier3d（现有的物理引擎）感到不满，自己实现了一个 3D 物理库，包含碰撞检测等功能。

### 项目特点
- 功能相对基础但运行速度快
- 几乎没有外部依赖
- 已经作为独立项目开发完成，可以直接使用

### 开发者的疑虑
1. **发布经验**：从未在 crates.io 上发布过项目，不确定最佳实践
2. **必要性质疑**：考虑到 rapier 已经存在，不确定发布自己的库是否有意义
3. **有趣的转折**：最初遇到的 rapier 问题实际上是自己的错误，但直到用自己的库替换后才发现这一点

### 询问社区
作者想了解社区是否对这样一个新的物理引擎感兴趣。

[
https://old.reddit.com/r/rust/comments/1safm14/would_people_be_interested_in_another_3d_physics/
](
https://old.reddit.com/r/rust/comments/1safm14/would_people_be_interested_in_another_3d_physics/
)
    


### TITLE

# Jon Gjengset 谈 Rust 内部机制、氛围编程和流媒体教学

## 视频信息

这是一个时长约1小时35分钟的访谈视频，主讲人是 Jon Gjengset，主要讨论了以下话题：

### 主要议题

- **Rust 内部机制（Rust Internals）**: 深入探讨 Rust 编程语言的内部工作原理
- **氛围编程（Vibe Coding）**: 讨论编程时的状态和感觉
- **通过直播进行教学（Teaching by Streaming）**: 分享通过流媒体平台进行技术教学的经验

### 视频详情

- 平台：YouTube
- 时长：1:34:43
- 形式：直播/录播访谈

*注：由于提供的内容主要是视频播放界面信息，未包含具体的访谈内容细节，因此无法提供更详细的讨论要点总结。*

[
https://youtu.be/Cg8gASzqqOs
](
https://youtu.be/Cg8gASzqqOs
)
    


### TITLE

## Bigoish：一个测试算法实证计算复杂度的 Rust 工具箱

### 核心功能
- **bigoish** 是一个新的 Rust crate，用于测试函数或数据结构方法的实际运行复杂度是否符合预期
- 可以编写测试来断言某个函数具有特定的**实证计算复杂度**（empirical computational complexity）
- 通过将预期的复杂度模型与一组常见复杂度模型进行拟合对比，验证哪个模型最符合实际测量的运行时间

### 与理论大O记号的区别
- 真正的大O记号是数学证明的上界
- bigoish 提供的是基于实际测量的经验性复杂度评估

### 使用示例
测试 Rust 内置 `sort()` 函数是否符合 `n*log(n)` 复杂度：
- 定义待测试的函数
- 创建不同规模的测试输入
- 使用 `assert_best_fit()` 断言预期的复杂度模型
- 可以手动指定输入对，或使用 `growing_inputs()` 自动生成递增规模的输入

### 错误提示
当断言的复杂度模型不匹配时，会：
- 触发 panic 错误
- 显示可视化图表，对比实际测量值（红点）与预期模型
- 建议最佳匹配的复杂度模型

### 应用场景
适用于验证自定义算法实现（如排序算法）是否存在性能缺陷或复杂度退化问题

[
https://old.reddit.com/r/rust/comments/1samn44/bigoish_test_the_empirical_computational/
](
https://old.reddit.com/r/rust/comments/1samn44/bigoish_test_the_empirical_computational/
)
    


### TITLE

## Flow-Like 项目介绍

Flow-Like 是一个基于 Rust 的工作流自动化平台，强调本地运行和数据隐私。

### 核心特点

- **完全本地化运行**：可在笔记本电脑、手机或服务器上运行，无需依赖云服务
- **数据自主权**：数据完全由用户控制，不会被强制上传到第三方服务器
- **完全可追溯**：提供清晰的数据流向记录，包括数据来源、变更过程和输出结果
- **可视化操作**：支持拖放式工作流构建界面
- **强类型系统**：基于 Rust 开发，提供类型安全保障

### 项目优势

- **灵活部署**：支持在笔记本、手机、私有服务器或自控云基础设施上部署
- **离线可用**：无需网络连接即可正常工作
- **无供应商锁定**：不受特定云服务商限制
- **透明可信**：所有流程可见可控，无黑盒操作

### 技术栈

- 后端：Rust (Cargo)
- 前端：包含多个应用和库
- 支持：Docker、Kubernetes、WASM 模板
- 包管理：使用 Bun

### 最新更新

最近更新包括测试版应用发布（2026年4月3日），添加了跟踪授权、错误处理改进、日志增强等功能。

[
https://github.com/TM9657/flow-like
](
https://github.com/TM9657/flow-like
)
    


### TITLE

## Rust 开发者使用的 Cargo 工具包

这是一个 Reddit 上的讨论帖，作者询问其他 Rust 开发者使用哪些工具来改善开发体验（DX）。

### 作者目前使用的工具

- **cargo-zigbuild**: 用于交叉编译
- **cargo-edit**: 主要用于 set-version 命令和版本升级
- **cargo-audit**: 检查依赖项的安全漏洞

### 讨论目的

作者想了解是否还有其他优秀的 Cargo 工具值得使用，希望社区能够分享他可能遗漏的实用工具。

[
https://old.reddit.com/r/rust/comments/1sb3prh/what_are_the_cargo_toolkits_you_use_to_make_your/
](
https://old.reddit.com/r/rust/comments/1sb3prh/what_are_the_cargo_toolkits_you_use_to_make_your/
)
    


### TITLE

## Rust学习资源讨论：《Rust实战》是否适合初学者

### 背景情况
一位有一定编程经验的学习者提出了关于学习Rust的问题：
- **编程经验**：具有Next.js网页开发经验，不是完全的编程初学者
- **Rust实践**：已完成几个小型Rust项目（CLI网页爬虫、AI工具、文件读取器）
- **学习时长**：过去一周在探索Rust和系统编程
- **教育背景**：目前正在攻读计算机科学学位

### 核心疑问
- 是否需要预先具备系统级编程知识才能继续深入学习Rust
- 《Rust实战》(Rust in Action)这本书是否适合像他这样的学习者

[
https://old.reddit.com/r/rust/comments/1sb3x20/is_the_book_rust_in_action_great_for_beginners/
](
https://old.reddit.com/r/rust/comments/1sb3x20/is_the_book_rust_in_action_great_for_beginners/
)
    


### TITLE

## RustQC - 高性能RNA-seq质量控制工具

### 主要特点

**结果一致性**
- 产生与R语言的dupRadar和featureCounts相同或几乎相同的输出结果
- 忠实重新实现了8个RSeQC工具(包括TIN)、preseq和samtools的功能

**显著提速**
- 在AWS上处理约1.86亿条reads仅需14分54秒
- 而传统顺序执行工具流程需要约15小时34分钟
- 其中单独TIN分析在传统工作流中就需要9小时45分钟

**一体化解决方案**
- 用单个CLI命令替代14个以上独立的QC工具调用
- 只需准备一次数据
- 只需遍历BAM文件一次
- 最小化数据传输和I/O开销

**广泛兼容性**
- 接受SAM、BAM和CRAM格式的比对文件
- 支持跨染色体并行化处理
- 支持多个BAM文件并行处理,充分利用可用资源

[
https://seqeralabs.github.io/RustQC/
](
https://seqeralabs.github.io/RustQC/
)
    


### TITLE

# 无法访问内容

很抱歉，我无法访问您提供的链接内容。根据返回的信息，该网站正在进行安全验证，页面显示了机器人检测机制，导致无法获取文章的实际内容。

## 建议

如果您想让我总结关于 **Rust 生命周期（Lifetimes）** 的内容，您可以：

### 选项1：复制文章内容
直接将文章的文本内容复制粘贴给我，我可以为您进行总结。

### 选项2：提供其他来源
提供其他可访问的关于 Rust 生命周期的文章链接。

### 选项3：询问相关知识
如果您想了解 Rust 生命周期的概念，我可以直接为您讲解这个主题的核心要点。

请告诉我您希望采用哪种方式继续！

[
https://vinayakdsci.hashnode.dev/exploring-rust-lifetimes?trk=feed-detail_main-feed-card_feed-article-content
](
https://vinayakdsci.hashnode.dev/exploring-rust-lifetimes?trk=feed-detail_main-feed-card_feed-article-content
)
    


### TITLE

## 三年间分享了800个Rust终端项目

### 项目背景
作者在过去3年中发现并分享了约800个开源Rust CLI项目，并在社交媒体（X/Mastodon/Bluesky/LinkedIn）上定期发布这些项目的信息。

### 分享动机
1. **推广Rust语言** - 希望更多人看到Rust的潜力并使用它
2. **发展Ratatui生态** - 作者维护Ratatui项目，分享相关项目有助于生态系统成长
3. **发掘优质项目** - 帮助展示那些可能被忽视的优秀项目
4. **纯粹的乐趣** - 分享过程本身很有趣

### 项目发现渠道
- Discord服务器（如Ratatui、Grindhouse、Terminal Collective）
- Terminal Trove新闻通讯
- GitHub搜索功能
- 社交媒体和口碑传播

### 数据统计（仅Mastodon平台）
- **分析帖子数**：786条
- **发布时间范围**：2022年12月15日至2026年3月31日
- **总点赞数**：10,655
- **总转发数**：4,529
- **总回复数**：658
- **平均每帖点赞**：13.56
- **平均每帖转发**：5.76
- **活跃天数**：653天

### Top 99 Rust终端工具排行榜
作者基于Mastodon上的互动数据（点赞×转发×回复）整理出了99个最受欢迎的Rust终端项目，前三名为：

🥇 **cyme** - 现代跨平台的lsusb工具
🥈 **gurk** - Signal Messenger终端客户端
🥉 **ratthew** - 终端3D地牢爬行游戏

**注**：排名中排除了Ratatui项目和作者个人项目以保持公平性

[
https://blog.orhun.dev/800-rust-projects/
](
https://blog.orhun.dev/800-rust-projects/
)
    


### TITLE

## Rust GUI框架选择：为传统PLC开发现代化IDE

### 项目背景
- 作者是一名控制系统/自动化工程师，经常使用1980-1990年代的西门子STEP5/S5 PLC设备
- 官方编程软件最初为CP/M程序，后过渡到MS-DOS，最后版本仅支持Windows XP单核模式
- 现有第三方替代软件多为90年代MFC应用，体验不佳

### 项目进展
- 使用Rust重新开发S5-DOS软件的现代化版本
- 目标支持Linux和Windows双平台
- 已完成：
  - S5-DOS文件格式解析
  - 约80%的AS511串行通信协议实现
  - 使用GTK4+Relm4框架的部分GUI实现（导航和元数据编辑）

### 当前问题
- GTK4应用在Linux运行良好，但在Windows上会随机崩溃或冻结
- 不确定是否应在深入开发前切换GUI框架

### 功能需求
- **必需功能**：
  - 树形导航结构（支持多层嵌套文件夹/代码块）
  - STL代码编辑器（语法高亮、自动格式化、行号显示）
  - 选项卡式窗口
  
- **未来可能需求**：
  - 可视化编程语言支持（LAD梯形图、FBD功能块图）
  - 图形绘制和文本标注功能

### 核心问题
**寻求建议**：2026年有哪些Rust GUI框架适合开发具有语法高亮、选项卡窗口、树形导航等功能的现代化IDE？

[
https://old.reddit.com/r/rust/comments/1sb6oks/suggestions_for_a_gui_framework_for_a_modern_ide/
](
https://old.reddit.com/r/rust/comments/1sb6oks/suggestions_for_a_gui_framework_for_a_modern_ide/
)
    


### TITLE

## Rust学习者寻求高质量代码项目推荐

### 背景
一位Rust学习者已经：
- 学习Rust一段时间
- 开发了几个个人项目
- 完成了代码降临节日历(AOC)和Code Crafters的一些挑战
- 正在开发一个新项目

### 遇到的问题
- 感觉自己写的代码不够地道(idiomatic)
- 代码性能和生产就绪程度不足
- 虽然可以使用AI辅助，但无法判断AI输出的质量
- 缺乏评估最佳实践的能力

### 寻求帮助
正在寻找**高质量的开源Rust项目**推荐，这些项目应该：
- 代码质量达到高级Rust工程师水平
- 展示Rust特定的设计模式
- 体现良好的Rust编码实践
- 适合生产环境的应用代码

### 学习目标
- 通过阅读优质代码学习
- 培养对生产级Rust编程的"感觉"
- 掌握Rust的最佳实践和设计模式

[
https://old.reddit.com/r/rust/comments/1sb1v2j/looking_for_projects_with_high_quality_rust_code/
](
https://old.reddit.com/r/rust/comments/1sb1v2j/looking_for_projects_with_high_quality_rust_code/
)
    


### TITLE

## docs.rs:默认构建更少的目标平台

### 主要变更
从 **2026年5月1日** 起,docs.rs 将对其构建行为进行重大调整:
- **当前行为**:如果 crate 未在 docs.rs 元数据中定义目标列表,默认为 5 个目标平台构建文档
- **新行为**:除非明确请求,否则仅为默认目标平台构建文档

### 变更原因
- 大多数 crate 不会针对不同目标平台编译不同代码
- 减少构建时间
- 节省 docs.rs 的资源

### 影响范围
此变更仅影响:
- 新发布的版本
- 旧版本的重新构建

### 默认目标平台的选择
- 如果未设置 `default-target`,docs.rs 使用其构建服务器的目标:**x86_64-unknown-linux-gnu**
- 可通过在 `Cargo.toml` 中设置来覆盖默认值:
```toml
[package.metadata.docs.rs]
default-target = "x86_64-apple-darwin"
```

### 如何为额外目标平台构建文档
如需为多个目标平台构建文档,需在 `Cargo.toml` 中明确列出:
```toml
[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu", "x86_64-apple-darwin", "x86_64-pc-windows-msvc", ...]
```

### 补充说明
- docs.rs 仍支持 Rust 工具链中的所有目标平台
- 仅默认行为发生变化

[
https://blog.rust-lang.org/2026/04/04/docsrs-only-default-targets/
](
https://blog.rust-lang.org/2026/04/04/docsrs-only-default-targets/
)
    


### TITLE

## 从零开始构建基于GPU的浏览器渲染器

一位开发者正在开发名为 **Aurora** 的浏览器引擎，该项目专注于使用 wgpu 构建 GPU 优先的渲染管线。

### 项目特点

- **纯GPU渲染**：使用 wgpu 和 vello，不依赖 Skia 或 Chromium
- **自定义管线**：包含自定义布局和渲染管线
- **独立开发**：不依赖现有的浏览器引擎

### 当前进展

开发者最近达成了一个重要里程碑：成功渲染了**静态Google主页**的HTML fixture。这是首次通过自己的管线端到端渲染真实世界的页面结构。

### 现有局限

项目尚未成为完整的浏览器：
- 文本整形（text shaping）仍然很粗糙
- 布局功能被简化

### 下一步计划

- 改进文本整形功能（正在尝试 HarfBuzz 之外的方案）

### 测试页面

开发者使用了一个包含以下元素的静态Google主页进行测试：
- 顶部导航栏（Gmail、Images、Apps、Sign-in链接）
- Google Logo（彩色字母）
- 搜索框
- 搜索按钮
- 页脚信息

开发者希望获得反馈，特别是来自有渲染引擎或GPU管线开发经验的开发者。

[
https://old.reddit.com/r/rust/comments/1sb5vcq/building_a_gpubased_browser_renderer_from_scratch/
](
https://old.reddit.com/r/rust/comments/1sb5vcq/building_a_gpubased_browser_renderer_from_scratch/
)
    


### TITLE

## I built I built builder - 一个用于构建"I built"工具的元工具

### 背景
作者注意到 r/rust 社区最近出现大量"I built..."（我构建了...）的帖子，于是产生了一个想法：与其继续开发各种CLI工具，不如开发一个专门用于构建这些"I built"工具的元工具。

### 工具介绍
**I built I built builder** 是一个CLI脚手架工具，专门用于：
- 搭建Rust CLI工具的基础框架
- 自动生成README文档
- 配置发布工作流
- 生成"I built"帖子内容

### 核心理念
该工具旨在帮助开发者更加一致、快速、系统化地完成"我构建了某工具"的全流程，将工具开发和宣传文案写作整合在一起。

### 使用示例
```bash
i-built-i-built-builder --name tomlshave --solves "trimming whitespace in embedded TOML snippets"
```

这条命令可以：
- 搭建工具框架
- 生成文档
- 创建"I built"叙事内容

### 项目地址
https://github.com/jams246/i-built-i-built-builder

### 总结
作者通过这个元工具，让其他开发者能够更方便地构建工具并发布"I built"帖子，形成了一个有趣的递归概念。

[
https://old.reddit.com/r/rust/comments/1sb4no2/i_built_i_built_builder_a_tool_for_helping_i/
](
https://old.reddit.com/r/rust/comments/1sb4no2/i_built_i_built_builder_a_tool_for_helping_i/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

