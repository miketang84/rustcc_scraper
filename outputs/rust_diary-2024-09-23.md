【Rust日报】2024-09-23


### TITLE

这个项目名为Hyperion,旨在打破在单个Minecraft世界中同时容纳最多玩家的吉尼斯世界纪录(目前由EVE Online保持,为8825名玩家)。该项目使用Rust编写,采用ECS架构驱动,目标是能够支持1万名玩家同时在一个世界中进行PvP。

目前,该项目的重点是创建一个类似于Overcast Network(但与之无关联)的游戏模式,支持多种游戏机制,包括照明、方块操作、世界生成、渲染、物品栏、战斗、世界持久化、物理引擎、游戏机制(如日夜循环)、音频(临近语音聊天)等。部分功能已经完成,部分正在开发中。该项目还计划提供一个mod/插件API,以便进行扩展。

开发者鼓励社区加入他们的Discord频道,了解最新进展并参与贡献。编译运行该项目需要通过Homebrew安装just工具,并使用相应命令进行调试模式和发布模式的构建。

[
https://github.com/andrewgazelka/hyperion
](
https://github.com/andrewgazelka/hyperion
)
    


### TITLE

该内容介绍了Flecs-Rust项目,这是一个用于Rust语言的Flecs实体组件系统(ECS)框架包装器。Flecs是一个高性能、轻量级的ECS框架,支持数百万个实体,具有完整的实体关系支持、层次结构和预制体、缓存友好的数据结构、支持数百个组件、动态组件注册、反射和JSON序列化等强大功能。该Rust包装器目前处于Alpha阶段,已实现Flecs的核心功能,正在加强安全性、性能、API改进和文档工作。它旨在提供一个安全、高效、符合Rust习惯的Flecs API,并计划支持WebAssembly等特性。该项目欢迎开发者贡献和反馈。

[
https://github.com/Indra-db/Flecs-Rust
](
https://github.com/Indra-db/Flecs-Rust
)
    


### TITLE

以下是对该内容的中文总结:

这是关于在线游戏EVE Online中创下的一项吉尼斯世界纪录。2020年10月6日,在名为"Fury at FWST-8"的一场玩家对战战斗中,共有8,825名玩家参与,创下了史上最大规模的多人在线游戏PvP战斗的记录。这场战役是PAPI阵营试图在敌对阵营Imperium的领地Delve建立一个前沿基地的尝试,但遭到了Imperium的顽强抵抗。最终共有6,746艘舰船和1个Keepstar太空站被摧毁,362艘资本级舰船被击沉,总损失高达14.43万亿ISK(约合1871.2万美元)。这一纪录每天都可能随时被刷新,官方网站会不定期更新最新记录。

[
https://www.guinnessworldrecords.com/world-records/105603-largest-videogame-pvp-battle
](
https://www.guinnessworldrecords.com/world-records/105603-largest-videogame-pvp-battle
)
    


### TITLE

这个仓库是一个名为"Better ADB Sync"的Python工具,可以在计算机和安卓设备之间同步文件,类似于rsync。它是对Google官方的adb-sync工具的重写,主要改进包括:

1. 升级到Python 3的代码风格。
2. 添加 --exclude、--exclude-from、--del、--delete-excluded 等选项,类似于rsync,允许排除某些文件模式以及删除目标端不存在的文件。 
3. 可能的未来工作包括添加备份选项等。

该README还介绍了在Android设备和PC端的设置步骤,以及如何使用该工具在两端同步文件。另外还提到了一个名为adb-channel的工具,可以通过网络连接安卓设备并执行命令。最后列出了一些相关的同步工具供参考。

总的来说,这是一个方便在本地计算机和安卓设备之间高效同步文件的Python工具。

[
https://github.com/jb2170/better-adb-sync
](
https://github.com/jb2170/better-adb-sync
)
    


### TITLE

这是一个用于为 Actix Web 应用程序添加路由级别的速率限制中间件的 Rust 库。它允许你为特定的路由设置一个持续时间和在该持续时间内允许的最大请求数量。如果请求数量超过了限制，中间件会返回 HTTP 429 Too Many Requests 响应，并在响应头中包含速率限制的详细信息。

该库提供了一个 LimiterBuilder 来构建速率限制器实例。使用示例展示了如何为一个路由设置 20 秒的持续时间和最多 2 个请求的限制。通过在 App 实例上调用 wrap 方法并传入 RateLimiter 实例，可以将该中间件应用到应用程序的所有路由上。该库提供了一种简单的方式来实现基于路由的速率限制功能。

[
https://github.com/harr1424/Actix-Route-Rate-Limiter
](
https://github.com/harr1424/Actix-Route-Rate-Limiter
)
    


### TITLE

这位Rust程序员遇到了一个关于特征(trait)层次结构设计的问题。他定义了三个特征A、B和C,其中B依赖于A,而C又依赖于B。在创建类型C的实例时,需要同时指定B和A的具体类型,这显得很冗余。他希望能够在创建C的实例时,只指定B的类型,而不必重复指定A,因为A已经由B隐含了。

他尝试过使用关联类型(associated types)和类型别名(type aliases),但还是不满意结果。他想知道是否有更好的方式来设计这种特征层次结构,避免重复指定类型,同时也不希望使用特征对象(trait objects)或dyn关键字。他希望能获得一些见解或替代方法来解决这个问题。

[
https://old.reddit.com/r/rust/comments/1fo4482/generics_how_to_design_nested_traits_without/
](
https://old.reddit.com/r/rust/comments/1fo4482/generics_how_to_design_nested_traits_without/
)
    


### TITLE

这是对一个名为rustc_codegen_clr的Rust到.NET编译器项目的介绍。这个项目的目标是允许Rust代码可以无缝调用.NET代码,反之亦然。文章着重解释了panic(发生运行时错误)和unwinding(堆栈回溯)在这个编译器中的实现方式。

主要内容包括:

1. 项目概述 - 这个编译器后端将Rust内部表示(MIR)转化为.NET的中间语言(CIL),使得编译后的Rust代码可以在.NET运行时执行。

2. panic和unwinding的区别 - panic是Rust语言层面的错误处理机制,unwinding则是panic的一种实现方式,通过回溯堆栈来释放资源。

3. MIR基本块(basic block)的结构 - 每个函数由一系列基本块组成,基本块内只有语句,控制流转移在终结符(terminator)处发生。

4. 清理块(cleanup block) - 编译器为需要在unwinding时释放资源的地方生成这种特殊块,其作用是正确地释放栈上分配的对象。

5. 通过一个小示例展示了清理块的工作原理。

总的来说,这篇文章解释了Rust到.NET编译器中panic和unwinding的实现细节,重点在于清理块这个编译器层面的概念。

[
https://fractalfir.github.io/generated_html/rustc_codegen_clr_v0_2_1.html
](
https://fractalfir.github.io/generated_html/rustc_codegen_clr_v0_2_1.html
)
    


### TITLE

这是一个名为Hyperion的高性能Minecraft游戏引擎项目的介绍。作者最初受到EVE Online 8825人同时在线的世界记录的启发,认为这个记录在Minecraft这样流行的游戏中是可以被打破的。但遗憾的是,原版Minecraft在几百人在线时就会卡顿,而且是单线程的。

因此,作者耗费数月时间基于flecs构建了高性能的Hyperion引擎。与其他Rust编写的Minecraft服务器项目不同,Hyperion的目标不是与原版Minecraft实现功能对等,而是采用模块化设计,只实现举办大规模定制活动所需的功能。

目前该引擎的性能估计可以支持约5万名并发玩家。作者已与一些创作者接洽,希望他们在YouTube或直播中使用这个项目。最后提供了GitHub和Discord链接,供有兴趣的人参与贡献。

[
https://old.reddit.com/r/rust/comments/1fmznnh/hyperion_10k_player_minecraft_game_engine/
](
https://old.reddit.com/r/rust/comments/1fmznnh/hyperion_10k_player_minecraft_game_engine/
)
    


### TITLE

这篇Reddit帖子探讨了Rust标准库中一些错误类型为什么没有实现`std::error::Error`特征。

作者使用了一个依赖函数返回值实现`std::error::Error`特征的错误处理库。但他发现标准库中有些错误类型如`std::sync::poison::PoisonError<T>`没有实现这个特征。他想知道是否有一个好的理由解释这种设计。

虽然可以通过其他方式规避这个问题,但作者希望理解标准库设计者的初衷,为什么有些错误类型会有这种例外情况。他希望得到一个合理的解释。

[
https://old.reddit.com/r/rust/comments/1fnj4st/why_does_some_errortypes_in_the_std_library_not/
](
https://old.reddit.com/r/rust/comments/1fnj4st/why_does_some_errortypes_in_the_std_library_not/
)
    


### TITLE

这是一个关于 adb-sink 工具的介绍。adb-sink 是一个简单的命令行工具,用于在安卓设备和计算机之间递归同步目录,实现备份和恢复。它通过比较文件路径、大小和最后修改日期来差异化文件树,从而更快速、更准确地同步文件。与 better-adb-sync 不同,它能正确同步文件的最后修改时间,且不会错过已修改的文件。目前它使用系统wide的 adb 二进制文件,但正在开发自己的 adb usb 协议实现(不确定是否会完成)。总的来说,adb-sink 是一个比 better-adb-sync 更好的安卓文件同步工具。

[
https://old.reddit.com/r/rust/comments/1fnwj60/adbsink_even_better_adb_sync_for_syncing/
](
https://old.reddit.com/r/rust/comments/1fnwj60/adbsink_even_better_adb_sync_for_syncing/
)
    


### TITLE

这是一篇来自 Rust 社区的求助帖。作者在重构代码时遇到了一个令人困惑的编译器错误。具体情况如下:

作者将一些类型从 crate A 移动到了 crate B 中,并在 crate A 中使用 `pub use` 语句引用这些类型。这种重构方式一直工作正常,直到遇到一个特定的结构体 `UnivariateLinearRegressionSolution`。在引用该结构体后,编译器开始报出大量莫名其妙的错误,比如找不到 `len`、`as_ptr` 等方法,甚至连 `alloc` 模块都找不到了。

作者尝试了多种方式来解决,包括重新开始重构、清理缓存、更新工具链版本、更换操作系统等,但问题依旧存在。后来在 Rust 论坛的帮助下,作者发现测试代码中有一个禁止的 impl 块导致了这个问题。不过作者依然不理解为什么这个 impl 块会引起如此严重的错误。

最终,这个问题被认定为编译器的一个 bug。

[
https://old.reddit.com/r/rust/comments/1fnrn5w/pub_use_breaks_my_compiler_in_refactoring_please/
](
https://old.reddit.com/r/rust/comments/1fnrn5w/pub_use_breaks_my_compiler_in_refactoring_please/
)
    


### TITLE

这是一个针对2025年在波兰华沙举办的首届 Rustikon 会议的征集论文通知。会议组委会欢迎与 Rust 编程语言相关的各种主题,包括 Rust 的使用案例、性能和内存效率、类型安全、函数式编程、异步、并发、分布式系统、Web/GUI/嵌入式以及日常使用的库、数据工程、大数据、流媒体和数据库、机器学习和人工智能、工具、开发者体验、教学 Rust 以及建立社区等。征集的演讲时长为30分钟,另有5分钟的问答环节。投稿截止日期为2024年10月13日。如有任何疑问,可发送电子邮件至rustikon@rustikon.dev。会议的更多详情可在https://www.rustikon.dev上查阅。

[
https://old.reddit.com/r/rust/comments/1fnkyam/call_for_papers_rustikon_2025_warsaw_poland/
](
https://old.reddit.com/r/rust/comments/1fnkyam/call_for_papers_rustikon_2025_warsaw_poland/
)
    


### TITLE

这个错误信息说明在Rust代码中,let绑定不能与同名的常量(const)产生影射(shadow)。具体来说,代码中定义了一个名为`x`的常量,但是在另一个地方又试图使用`let`关键字绑定一个同名的变量`x`。Rust编译器不允许这种行为,因为它会导致命名冲突和混淆。

要解决这个问题,需要对导致冲突的变量或常量重新命名,使它们的名称唯一。或者,如果需要在不同作用域中重用同一个名称,可以考虑使用更小的代码块或函数来隔离变量的作用域。总的来说,这个错误提醒程序员在代码中保持良好的变量命名习惯,避免不必要的命名冲突。

[
https://sabrinajewson.org/blog/truly-hygienic-let
](
https://sabrinajewson.org/blog/truly-hygienic-let
)
    


### TITLE

总结:

这是一篇Reddit帖子,作者分享了他的第一个重要的Rust项目 - 一个用于Actix Web框架的路由速率限制器中间件。

该项目的GitHub仓库地址是https://github.com/harr1424/Actix-Route-Rate-Limiter,并已发布到crates.io(https://crates.io/crates/actix_route_rate_limiter)。

作者希望通过分享该项目获得反馈和建议,同时也向Rust社区展示了他的作品。该中间件可以应用于基于Actix Web框架的项目中,用于限制特定路由的请求频率,以防止滥用和过载。

[
https://old.reddit.com/r/rust/comments/1fo08hy/my_first_library_crate_rate_limiting_middleware/
](
https://old.reddit.com/r/rust/comments/1fo08hy/my_first_library_crate_rate_limiting_middleware/
)
    


### TITLE

本文介绍了Slint 1.8版本的新特性和改进。主要内容包括:

1. 新增属性变化回调功能,可以更好地响应界面元素属性变化。

2. 新增Timer和SwipeGestureHandler元素,分别用于定时触发回调和识别滑动手势。

3. 增强了实时预览和VSCode扩展的用户体验。

4. 新增了用于STM32嵌入式开发板的C++项目模板,简化了开发流程。

5. 数学函数支持后置语法,提高了可读性。并新增了atan2函数。

6. 修复了一些bug,提高了整体性能和稳定性。

7. 感谢了一些贡献者的工作。

总的来说,这个版本增加了一些实用的新功能,提升了开发体验,并进一步优化了跨平台支持和性能表现。

[
https://slint.dev/blog/slint-1.8-released
](
https://slint.dev/blog/slint-1.8-released
)
    


### TITLE

iocraft 是一个用于在终端或日志中创建漂亮的文本输出和界面的 Rust 库。它提供了一个声明式的 API,可以轻松构建复杂的布局和交互元素。主要特性包括:

1. 使用简洁可读的语法定义 UI。
2. 使用 flexbox 布局组织 UI 元素。
3. 将有色且样式化的 UI 输出到终端或任何支持 ASCII 的地方。
4. 创建动画或交互元素,并添加事件处理和钩子。 
5. 轻松构建全屏终端应用程序。
6. 通过引用传递属性和上下文,避免不必要的克隆。

iocraft 借鉴了 React 的概念,但针对文本输出,并使用 Rust 语言实现。使用 `element!` 宏声明 UI 元素,库提供了一些内置组件如 Box、Text 和 TextInput,也可以使用 `#[component]` 宏定义自定义组件。该库受到 Dioxus 和 Ink 的启发,并采用 Apache 2.0 或 MIT 许可证授权。

总的来说,iocraft 是一个强大的库,可以帮助开发人员在 Rust 中方便地创建优雅的文本界面。

[
https://github.com/ccbrown/iocraft
](
https://github.com/ccbrown/iocraft
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

