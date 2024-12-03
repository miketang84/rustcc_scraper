【Rust日报】2024-11-29


### TITLE

这是一个用于为LLDB调试器添加Rust特定格式化输出的Python脚本。由于最近CodeLLDB移除了Rust专用的格式化输出,导致调试Rust(尤其是枚举类型)变得很痛苦。这个脚本旨在临时解决这个问题,直到生态系统状况改善。

该脚本提供了在LLDB独立实例或VSCode调试适配器(CodeLLDB和lldb-dap)中使用的说明。它支持最新的LLDB(19.0.0)和Rust(1.62.0)版本,但可能无法与旧版本一起使用。由于Rust标准库内部和LLDB表示的变化,这只是一个临时的解决方案,可能会过时。

作者希望Rust自身的漂亮打印机功能最终能够正常运行,从而取代这个临时解决方案。该脚本计划在头部保持更新,并尽快退役。作者欢迎改进脚本或添加第三方crate常用集合类型支持的拉取请求,前提是提供测试用例以确保可维护性。

如果这个脚本对你有所帮助,请给一颗星表示感谢,这也许能向Rust项目维护人员展示,大家渴望一个可靠的Rust调试解决方案。

[https://github.com/cmrschwarz/rust-prettifier-for-lldb
](https://github.com/cmrschwarz/rust-prettifier-for-lldb
)
    


### TITLE

以下是对该更新日志的中文总结:

1.11.1版本:
- 升级嵌入的Python版本到3.12
- 支持步入目标(Step Into Targets),在调试foo(bar(), baz())这样的语句时,可以直接步入foo,绕过bar和baz
- 支持重启请求,可在保留当前会话的情况下重新启动被调试程序,加快重启速度
- 增加了preTerminateCommands序列,preRunCommands和postRunCommands序列将被执行
- 为Cargo配置增加了"cwd"属性,运行Cargo时添加--color=always选项
- 修复了几个问题,包括反汇编视图、高亮断点等

1.11.0版本:  
- 升级了LLDB到19.1.0版
- 实现CodeLLDB Python API的模块名改为codelldb
- 可以通过codelldb.get_config()读取工作区配置
- 移除了对Rust语言服务和自定义数据格式化程序的支持,将只基于原生LLDB

1.10.0版本:
- 升级了LLDB到17.0.0版
- 修复了几个问题,如调用栈、全局变量分类等

1.9.2版本:
- 实现了排除调用者(Excluded Callers)功能
- 增加了create_webview()Python API,允许脚本创建和操作VSCode网页视图
- 支持在异常断点上设置条件

1.9.1版本:
- 支持envFile配置
- 增加breakpointMode设置,可控制断点解析方式
- 允许在launch和attach请求中指定targetCreateCommands和processCreateCommands
- 修复了几个问题

1.9.0版本:
- 升级了LLDB到16.0.0版
- 支持在变量查看器中组合数字格式和数组格式说明符
- 支持原生VSCode反汇编视图
- 修复了几个语法错误和显示问题

还有一些其他版本的更新,包括bug修复和小改进。

[
https://github.com/vadimcn/codelldb/blob/c06be2ebc5a5fae802edf872b2a73db903e55de3/CHANGELOG.md#changed
](
https://github.com/vadimcn/codelldb/blob/c06be2ebc5a5fae802edf872b2a73db903e55de3/CHANGELOG.md#changed
)
    


### TITLE

这篇文章描述了在 Rust 中解析一种类似树形结构的二进制文件的挑战。每个节点由以下字节组成:0-3字节表示节点类型ID,4-7字节表示头部大小,8-11字节表示总大小,12字节到头部大小之间是有效负载数据,0字节到总大小之间是整个节点包括其所有子节点。有多种节点类型,其中一种是叶子节点没有子节点。作者目前的解析方式是跳到每个节点头部大小的结尾,直到遇到叶子节点,将最后一个节点设置为当前节点的父节点。然后解析所有叶子节点作为最后一个节点的子节点,再向后跳到上一个节点,重复这个过程。作者寻求在 Rust 的所有权规则下,更好的解决方案或方向。

[
https://old.reddit.com/r/rust/comments/1h2umpu/parsing_treelike_binary_files/
](
https://old.reddit.com/r/rust/comments/1h2umpu/parsing_treelike_binary_files/
)
    


### TITLE

这位Rust开发者分享了他为FS3000系列气流传感器实现的一个嵌入式设备驱动程序,该驱动程序同时支持阻塞(Blocking)和异步(Async)两种模式。他使用了一个标记特征trait ClientType来区分阻塞和异步实现。该方法在单个crate中工作良好,但当涉及多个驱动程序时可能会变得复杂。他在Reddit上征求大家的意见,寻求在设备驱动程序中同时支持同步和异步的更好方法,并期望获得关于改进这个驱动程序的其他建议。

[
https://old.reddit.com/r/rust/comments/1h2rzlr/handling_syncasync_for_embeddedhal_device_drivers/
](
https://old.reddit.com/r/rust/comments/1h2rzlr/handling_syncasync_for_embeddedhal_device_drivers/
)
    


### TITLE

以下是内容总结:

这是Rust编程语言1.83.0版本的发布公告。主要变化和新特性包括:

1. 提升了const上下文中的功能,现在可以在const fn中使用可变引用、原生指针和内部可变性等。还可以在const中引用静态变量。

2. 将多个API标准化,使其可以在const上下文中使用,如Option、Result、NonNull等类型的方法。

3. 将ErrorKind的多个变体标准化。

4. 其他一些标准库API的新增和标准化,如BufRead::skip_until、ControlFlow相关API等。

总的来说,该版本大幅提升了const上下文中可用的功能和API,为const代码开辟了新的应用场景。读者可以通过rustup update stable升级到最新版本。

[
https://blog.rust-lang.org/2024/11/28/Rust-1.83.0.html
](
https://blog.rust-lang.org/2024/11/28/Rust-1.83.0.html
)
    


### TITLE

这篇论文指出,即使在完全安全的Rust代码中,Rust的内存安全保证也不能扩展到任意第三方代码。作者通过一系列反例来说明这一点。为了补充这些例子,作者进行了初步的实验研究,探讨现有的程序分析和验证工具是否能够检测或缓解这些风险?这些攻击模式是否可以通过对真实世界的Rust库公开暴露的函数进行输入来实现?现有的Rust供应链攻击在多大程度上利用了类似的攻击?所有示例和相关数据都作为GitHub上的开源仓库提供。作者希望这篇论文能够启发未来重新思考Rust中的安全性,特别是超越安全/不安全的区分,并针对可能在野外使用的更强大的威胁模型来加固Rust。

[
https://dl.acm.org/doi/abs/10.1145/3691621.3694943
](
https://dl.acm.org/doi/abs/10.1145/3691621.3694943
)
    


### TITLE

这个GitHub仓库中包含了一个Rust库structre,它通过静态检查将正则表达式解析为结构体或枚举。它可以避免常见的正则表达式问题,如捕获索引偏移、尝试获取不存在的捕获组以及正则表达式捕获组名称与字段名称不同步等。

structre不像serde那样可以处理任意结构体/枚举,而是要求结构体/枚举的定义与正则表达式相匹配。它提供了一个derive宏,可以为结构体/枚举实现from_str方法。

该库支持将正则表达式中的选择(|)捕获组解析为枚举变体,并支持几种字段类型,如简单类型、Option和元组。它还提供了一些示例和限制说明,如暂不支持非Unicode解析、无法确保正则表达式中的数字有效性等。

总的来说,structre是一个方便地将正则表达式与Rust数据结构相结合的工具库。

[
https://github.com/andrewbaxter/structre
](
https://github.com/andrewbaxter/structre
)
    


### TITLE

这是一个名为 cppdoc 的 C++ 文档生成器项目,灵感来自于 rustdoc。它具有以下主要特性:

1. 采用类似 rustdoc 的文档注释风格,使用 Markdown 渲染。
2. 支持使用代码块进行文档测试。
3. 可以编写额外的 Markdown 页面。
4. 支持 Mermaid 图表。
5. 允许用户提供自定义样式表进行灵活的样式设置。
6. 基于 libclang 解析器,支持记录、枚举、函数和命名空间。
7. 生成文档的性能不错,通常比基于 libclang 的 Doxygen 更快。

该项目还提供了一个在线演示,展示了使用三种不同样式表渲染的文档效果。总的来说,cppdoc 旨在提供一种现代、灵活且高性能的 C++ 文档生成解决方案。

[
https://github.com/rdmsr/cppdoc/
](
https://github.com/rdmsr/cppdoc/
)
    


### TITLE

这是关于Rust编程语言的随机数生成库getrandom的0.3版本的更新说明。getrandom库从操作系统获取随机数据,大多数用户通过高级库如rand隐式使用它。新版本包含一些重要变更,可能会影响下游用户。

最主要的变更是针对Web WASM的可选后端处理方式。以前使用js特性来启用基于wasm-bindgen的后端,现在改为使用getrandom_backend配置标志。这个新标志不仅允许用户选择后端,还允许覆盖默认后端(之前出于安全原因被限制)。

其他变更包括:
- 将MSRV提升到1.63
- 重命名一些函数
- 移除对wasm32-wasi目标的支持
- 用配置标志替代了一些特性标志
- 增加了配置后端的宏和错误码处理
- 针对不同操作系统优化了后端实现
- 为WASM增加了两种新的目标支持
- 增加了其他可选后端
- 增加了内存检测支持
- 新增生成u32和u64随机数的函数

总的来说,该发布旨在提高getrandom的可配置性、兼容性和安全性。

[
https://users.rust-lang.org/t/121819
](
https://users.rust-lang.org/t/121819
)
    


### TITLE

这个Reddit帖子探讨了为什么Rust在web开发领域比C/C++更受欢迎和推崇。帖子作者提出,虽然C/C++也有web库,但并不广泛采用,而Rust的现代web库却广受好评并被活跃使用。如果想用C/C++开发后端,多数人会反对,但用Rust则会得到正面的建议。作者好奇造成这种差异的原因,并澄清自己没有对C/C++持有负面态度,只是出于好奇和学习的目的提出这个问题。

[
https://old.reddit.com/r/rust/comments/1h2k12k/why_arent_other_system_programming_languages_cc/
](
https://old.reddit.com/r/rust/comments/1h2k12k/why_arent_other_system_programming_languages_cc/
)
    


### TITLE

总结如下:

1. codelldb 是一个流行的 Rust 调试器插件,但它默认不带有 Rust 的漂亮打印功能。

2. 有一个已知问题(issue #1166)指出缺少这个功能。

3. 有一个临时的解决方案,可以使用 rust-prettifier-for-lldb 这个第三方工具来添加漂亮打印功能。

4. 原帖提到也尝试过使用 GDB 调试 Rust 程序,但感觉有些过时了。

5. 原帖征求社区对于 Rust 调试器的其他想法和建议。

总的来说,是在讨论 Rust 调试工具的使用体验,特别是漂亮打印这一常用功能的缺失及临时解决方案,并希望获得社区的反馈。

[
https://old.reddit.com/r/rust/comments/1h2s10f/psa_codelldb_doesnt_ship_with_pretty_printing_for/
](
https://old.reddit.com/r/rust/comments/1h2s10f/psa_codelldb_doesnt_ship_with_pretty_printing_for/
)
    


### TITLE

这是一个名为"Advent of CodSpeed"的算法速度挑战赛。每天12月份都会在午夜东部标准时间(UTC-5)释放一个新的有两部分的问题,来自Advent of Code。参与者有36小时的时间来创建最快的算法解决这个问题,并将解决方案提交到CodSpeedHQ/Advent仓库。36小时后的中午(UTC-5),所有提交的当日解决方案将运行并根据解决两个部分所需的总时间进行排名。每天将根据参与者的排名给予积分。每5天一行的日历将为前三名参与者提供奖品。整个活动的前5名参与者也将获得奖品。

规则包括:每个人只能使用一个账户和仓库参加、代码必须公开、只能提交自己的代码、解决方案必须正确、主办方保留取消任何作弊者资格的权利。每个人每行只能赢得一次奖品。

[
https://codspeed.io/advent
](
https://codspeed.io/advent
)
    


### TITLE

这个版本更新包含了许多新功能、改进和错误修复。主要亮点包括:

1. 改进实体生成体验的必需组件功能。
2. 实体选取/选择系统,支持跨上下文选择实体。
3. 动画系统改进,包括泛型实体动画、动画遮罩、加法混合和动画事件。
4. 新增曲线trait、循环样条曲线和常用缓动函数。
5. 反射系统改进,包括函数反射、唯一反射和远程类型反射。
6. 新的Bevy远程协议(BRP),允许外部客户端与正在运行的Bevy游戏交互。
7. 可见性位掩码和改进的环境光遮蔽(VBAO)算法。
8. 新的色差渲染效果。
9. 体积雾效改进,包括"雾体积"定义和对点光源/聚光灯的支持。
10. 新的独立于顺序的透明度算法。
11. 改用Cosmic Text进行文本渲染,提高非拉丁语系字体的支持。
12. 将游戏手柄表示为实体,方便集成。

此外,还包括大量其他改进、错误修复和质量提升。这是由195名贡献者、1203个拉取请求、社区审阅人员和捐助者共同努力取得的重大版本升级。

[
https://bevyengine.org/news/bevy-0-15
](
https://bevyengine.org/news/bevy-0-15
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

