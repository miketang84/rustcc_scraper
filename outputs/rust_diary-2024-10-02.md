【Rust日报】2024-10-02


### TITLE

这是一系列Reddit帖子摘要,主要围绕Rust编程语言及相关技术展开讨论。主题包括:

1. 一个Rust到.NET编译器的谷歌编码之夏项目结果。
2. WYSIWYG Markdown编辑器的讨论。
3. Chrome系统广告拦截器的讨论。
4. Blazor文本编辑器NuGet包的新版本发布。
5. Rust开发环境的讨论。
6. 无丑陋代码的类型状态模式。
7. 在Steam Deck上禁用SMT的问题。
8. Android上运行Rust的方案。
9. Rust中FnOnce、Fn和FnMut的区别。
10. Emacs HTTP库plz.el的0.3版发布。
11. 读取大型(100GB+)文件的方法。
12. Cargo Watch面临维护问题。
13. 原子操作的作用。
14. Neovim插件模板更新。
15. Neovim仪表板设计。
16. Rust类型系统的优势。
17. Rust自定义后端优化的可能性。
18. Rust实践库0.1.0版发布。
19. RustConf大会体验分享。
20. Rust crate维护求助。
21. Neovim入门配置。
22. Rust当前主要应用领域。
23. C++的优缺点(来自Rust开发者)。
24. Laravel框架下的Markdown编辑器。
25. Linux二进制分析的TUI工具Binsider。

总的来说,涉及Rust编程语言、生态工具、开发实践、社区活动等多个主题。

[https://www.reddit.com/r/learnrust/s/x4yooOoUy0
](https://www.reddit.com/r/learnrust/s/x4yooOoUy0
)
    


### TITLE

这是一个名为 yew-mdx 的 Rust crate,用于在 Yew Web 框架中渲染 Markdown 内容,并支持嵌入组件。它依赖于 markdown 和 yew 0.20 crate。该 crate 当前处于 alpha 版本,最新版本为 0.1.0-alpha.1,发布于2023年3月30日。它使用 MIT 许可证,大小约为 6KB,有 109 行代码。每月大约有 73 次下载,下载量呈上升趋势。该 crate 在 Lib.rs 上被列为与 Yew Web 框架相关的crate之一,与 termimad、mdbook、glu、unclog、yew-hooks 等类似。Lib.rs 是一个非官方的 Rust/Cargo crate 列表网站,由 kornelski 创建,包含从多个来源收集的数据。

[
https://lib.rs/crates/yew-mdx
](
https://lib.rs/crates/yew-mdx
)
    


### TITLE

总结如下:

这个GitHub仓库展示了用户rambip在过去一年中的代码贡献情况。总的来说,他在这一年中共有493次贡献,贡献分布不均匀。有些时期贡献很多,有些时期则没有贡献。最多的一次是11月17日贡献了28次。另外8月12日、6月3日、11月28日等日期也有较多贡献。大部分时间他的贡献数较少或为0。这份详细的贡献记录反映了他的编码活跃程度在这一年中的变化情况。

[
https://github.com/rambip
](
https://github.com/rambip
)
    


### TITLE

这个项目是一个Rust库,灵感来自于vim-devicons,可以为广泛的常见文件格式提供文件类型图标(图形)。主要特性包括:

1. 可根据文件名/扩展名获取文件或目录图标。
2. 可根据指定的主题为图标检索相关颜色。
3. 支持广泛的文件类型和文件名约定(如dockerfile、makefile等)。
4. 支持浅色和深色两种主题。

使用方式很简单,首先需要cargo add devicons来安装,然后可以通过导入相关模块,并调用提供的函数如icon_for_file()来获取文件图标及相关颜色。值得注意的是,正确显示图标需要使用NerdFont字体。

该项目使用UNLICENSE许可证,在examples目录下提供了更多使用示例,可以通过cargo run --example <example_name>来运行。

[
https://github.com/alexpasmantier/rust-devicons
](
https://github.com/alexpasmantier/rust-devicons
)
    


### TITLE

以下是对Yaxi这个Rust编写的X11库的总结:

Yaxi是一个从头开始编写的X11库,具有以下主要特点:

1. 干净的接口,适合初学者和有经验的开发者使用。
2. 比许多其他X11库更安全的接口。
3. 不是包装器,而是纯Rust实现。

Yaxi的目标包括:

- 授权请求和回复
- 事件处理(大部分)
- 扩展支持(Xinerama, Xft等)
- 全面的文档
- 用Yaxi实现窗口管理器(正在进行中)

提供了一个简单示例,演示如何打开一个窗口并等待键盘输入后退出。

Yaxi遵循MIT许可证。

[
https://github.com/proxin187/Yaxi
](
https://github.com/proxin187/Yaxi
)
    


### TITLE

这是一篇来自即将担任一个工程/生物学团队软件主管的人的求助帖。他们的团队需要开发一个具有吸引力和交互性/动画效果的前端网站,包含大量详细的技术文档页面。这些页面可能包含交互式组件,但主要是基于文本的。

他们之前使用Astro框架,因为它允许使用JSX编写组件,使用MDX编写页面,非技术人员也可以轻松与之交互。他想知道是否有任何Rust Web框架提供类似的功能。他粗略查看过Dioxus/Leptos/Yew等框架,发现虽然有一些Markdown交互库,但似乎需要将Markdown内容分别存储在多个文件中,并在Rust文件中导入作为字符串传递给库。

他还发现了yew-mdx crate,但文档和示例非常少。他可能需要导入某个他找到的宏来实现所需功能。他正在寻求对这些Rust Web框架的功能和易用性的建议和意见。

[
https://old.reddit.com/r/rust/comments/1futdgf/accessible_frontend_web_development_using_rust/
](
https://old.reddit.com/r/rust/comments/1futdgf/accessible_frontend_web_development_using_rust/
)
    


### TITLE

这位用户正在考虑从wgpu切换到Metal图形API，但希望保留Rust语言来编写CPU端代码。他想咨询一下是否有人在使用metal-rs这个Rust binding库,看看将现有代码移植过去的难度如何,以及是否需要使用XCode等。总的来说,他对使用Rust语言配合Metal API有一些疑虑,希望能与有经验的人交流讨论。

[
https://old.reddit.com/r/rust/comments/1fuu9sw/anyone_using_metal/
](
https://old.reddit.com/r/rust/comments/1fuu9sw/anyone_using_metal/
)
    


### TITLE

根据所述内容,该人在生产环境中使用多个SQL服务器,并且需要使用连接池(如pgbouncer)来处理高流量。他正在考虑使用事务模式。

尽管pgbouncer从1.21版本开始支持预编译语句,但当前使用1.22版本时,仍然收到"prepared statement 'sqlx_s_1' does not exist"的错误。如果使用会话模式,则可以正常工作,但CPU使用率会升高,延迟也会从47毫秒增加到240毫秒。整个基础设施都在本地服务器上。

在sqlx文档中发现了raw_sql,但尚未尝试,因为它不支持bind,需要自己构建查询字符串,这可能需要进行重大代码更改。

[
https://old.reddit.com/r/rust/comments/1fuwhtb/production_sqlx_and_pgbouncer/
](
https://old.reddit.com/r/rust/comments/1fuwhtb/production_sqlx_and_pgbouncer/
)
    


### TITLE

这是一个实用且简单的Rust库crate,提供了各种常见文件格式的文件类型字形图标。它的主要用途是在终端用户界面(tui)应用程序中显示与不同文件类型对应的图标。作者欢迎大家根据需要自由使用、分享或复制该库的部分代码。这个库支持多种文件格式,在GitHub上有开源代码。总的来说,这是一个方便在Rust终端程序中显示不同文件类型图标的实用工具库。

[
https://old.reddit.com/r/rust/comments/1fuh7w6/rustdevicons_a_rust_library_that_provides/
](
https://old.reddit.com/r/rust/comments/1fuh7w6/rustdevicons_a_rust_library_that_provides/
)
    


### TITLE

以下是对该GitHub仓库的中文总结:

这个仓库包含了一个从Rust编程语言的子集编译到极小化的Brainfuck编程语言的编译器。该编译器支持基本类型(usize、char、bool、&str)、整数运算(+、-、*、/、%)、布尔逻辑(&&、||、!)、复合类型(数组、元组、结构体)、引用(&、&mut)、控制流语句(if、match、loop、while、break、continue)、函数(包括递归)、动态内存分配、输入/输出等Rust语言特性。

该项目的目标是将理论转化为实践,通过实际编译一种功能完备的高级语言(包括间接寻址、动态内存分配和递归等特性)到Brainfuck语言。虽然Brainfuck指令集非常简单,但它是图灵完备的,理论上可以计算任何可计算函数。

该编译器分为三个主要部分:前端进行词法分析和语法分析,将Rust代码解析为抽象语法树(AST);中间端将AST转换为自定义的中间表示(IR);后端则将IR翻译为Brainfuck代码。该项目还包括IR和Brainfuck解释器,可以直接运行编译后的程序。

该仓库还提供了一些示例程序,如计算斐波那契数列、确定给定日期是星期几、打印质数、求解柯里兹序列等。虽然支持了很多Rust语言特性,但也有一些限制,如不支持枚举、模式匹配、方法、泛型、模块、特征和生命周期等。同时附带了一个小型标准库,提供了I/O、堆内存分配和提前终止程序等功能。

该项目的主要目的是展示可以将Rust编译到Brainfuck语言,而不一定追求生成高效代码。未来的工作可以着眼于优化生成的Brainfuck代码的性能。

[
https://github.com/AlexBuz/rust_to_bf
](
https://github.com/AlexBuz/rust_to_bf
)
    


### TITLE

该内容是来自Rust生态系统的一位贡献者,对Diesel ORM和查询生成器添加了SQLite的JSON/JSONB支持的公告。作者成功将此功能合并到了Diesel的主干分支,使得之前仅在PostgreSQL后端可用的JSON/JSONB数据类型现在也可以在SQLite后端使用。文中给出了一个具体的使用示例,以及相关文档链接。最后,Diesel的维护者表示下一步将着手添加内置的SQL JSON函数的支持,以进一步增强对JSON数据的操作能力。这是作者对Rust生态系统的一个重要贡献。

[
https://old.reddit.com/r/rust/comments/1fuc2gy/support_for_sqlites_jsonjsonb_has_landed_on_diesel/
](
https://old.reddit.com/r/rust/comments/1fuc2gy/support_for_sqlites_jsonjsonb_has_landed_on_diesel/
)
    


### TITLE

这段代码是一个性能优化的例子,它使用了一个switch语句来根据输入的数值执行不同的代码分支。switch语句包含了11个case分支,每个分支对应一个不同的字符串标签,这些标签可能代表了一些函数或代码块。该代码可能是在编译器的优化过程中生成的,旨在提高特定情况下的执行效率。不过仅从这一小段代码很难推断出其具体的功能和用途。

[
https://oxc.rs/docs/learn/performance.html
](
https://oxc.rs/docs/learn/performance.html
)
    


### TITLE

以下是对该内容的中文总结:

今天,作者高兴地宣布了Yaxi,一个Rust编写的X11库。他认为现有的Rust X11库要么极度不安全,要么文档很差。因此,Yaxi的目标是提供一个功能强大、向后兼容,同时拥有全面文档(甚至包括之前在Xlib中存在的函数)的X11 Rust库。作者欢迎任何反馈、贡献和星标,并表示感谢。该库的GitHub链接为https://github.com/proxin187/Yaxi。

[
https://old.reddit.com/r/rust/comments/1funrkh/yaxi_a_postmodern_x11_rust_library_focusing_on_a/
](
https://old.reddit.com/r/rust/comments/1funrkh/yaxi_a_postmodern_x11_rust_library_focusing_on_a/
)
    


### TITLE

这篇文章讨论了当使用Rust编程语言时不应将其视为Java,而应该接受Rust语言本身的特性和理念。作者最初尝试将Rust代码写成类似Java的面向对象风格,将一切都设计为接口(trait)并注入依赖关系,但这种做法并不有效且复杂。相反,Rust更适合使用函数式编程,将服务分解为单个函数,并将依赖项作为参数传递。作者认为,不要试图将Rust塞进对象编程的框架中,而是应该拥抱Rust语言本身的独特之处和设计理念,这样编写Rust代码会更加愉悦和高效。总的来说,这篇文章提醒Rust开发者不要将其等同于Java,而是要适应和接受Rust语言的固有风格。

[
https://jgayfer.com/dont-write-rust-like-java
](
https://jgayfer.com/dont-write-rust-like-java
)
    


### TITLE

这是官方宣布Tauri 2.0稳定版本的发布博客。Tauri是一个框架,用于为所有主要桌面(macOS、Linux、Windows)和移动(iOS、Android)平台构建小巧快速的二进制文件。开发人员可以集成任何编译为HTML、JavaScript和CSS的前端框架来构建用户体验,同时在需要时利用Rust、Swift和Kotlin等语言进行后端逻辑处理。

文章介绍了Tauri的定义、何时使用、受欢迎程度,以及2.0版本的开发过程和主要贡献者。它强调了Tauri对开发人员体验的重视,包括新的create-tauri-app工具可以快速开始项目。2.0版本带来了许多改进,如热模块替换、插件支持、移动端支持、安全加固、IPC重写、分发指南等。最后呼吁大家试用Tauri 2.0,并概述了未来路线图。

[
https://v2.tauri.app/blog/tauri-20/
](
https://v2.tauri.app/blog/tauri-20/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

