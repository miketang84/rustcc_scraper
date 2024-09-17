【Rust日报】2024-09-14


### TITLE

这是一个Rust编程语言的构建配置文件,用于定义目标架构和编译器选项。主要内容如下:

1. 目标架构设置为 thumbv7em-none-eabihf,这是一种用于Cortex-M4F处理器(如nRF52832芯片)的交叉编译目标。

2. 对于该目标架构,设置了三个rustflags编译选项:
   - -Clinker=rust-lld 指定使用 rust-lld 作为链接器
   - -Clink-arg=-Tlink.x 将链接器脚本link.x传递给链接器
   - 这些选项通常用于嵌入式系统开发,以正确配置内存布局和链接过程。

总的来说,这个配置文件定义了为Cortex-M4F处理器(如nRF52832)交叉编译Rust程序时所需的编译器设置,适用于嵌入式系统开发场景。

[https://gist.github.com/BlinkingApe/9b4f5202c0294ce47a883633fc94e71b
](https://gist.github.com/BlinkingApe/9b4f5202c0294ce47a883633fc94e71b
)
    


### TITLE

这是一篇关于使用 Rust 构建跨平台 Hacker News 阅读器应用程序的教程文章。作者首先介绍了命令行界面(CLI)、文本用户界面(TUI)和图形用户界面(GUI)等不同类型的界面。然后作者解释了为什么选择使用 Rust 语言,列举了使用 Rust 的优势,如性能、安全性和并发能力。

接下来,作者详细介绍了使用 Rust 构建 TUI 应用程序的过程。他使用了 ratatui 和 reqwest 这两个 Rust 库,分别用于构建 TUI 和进行网络抓取。文章还介绍了使用 GitHub Actions 进行持续集成和持续部署(CI/CD)的过程。

总的来说,这是一篇非常实用的教程,向读者展示了如何使用 Rust 及相关库和工具构建跨平台的 TUI 应用程序,同时也涵盖了 CI/CD 的内容。无论是对于想学习 Rust 还是想了解 TUI 开发的读者,这篇文章都是很好的参考资源。

[
https://medium.com/codex/hacker-pulse-building-a-cross-platform-hacker-news-reader-in-rust-72965e5700b9?sk=e4109da4baedf105e0a93d5e55734851
](
https://medium.com/codex/hacker-pulse-building-a-cross-platform-hacker-news-reader-in-rust-72965e5700b9?sk=e4109da4baedf105e0a93d5e55734851
)
    


### TITLE

根据该Reddit帖子的内容,提问者想要使用Rust游戏引擎Bevy开发一款多人实时赛车游戏。他之前有使用Bevy开发小游戏的经验,但对于开发这种实时多人游戏可能会面临的挑战和需要学习的知识领域有些担忧。

因此,他在寻求建议,比如在开发这种实时多人游戏时可能会遇到的shortcomings(缺陷或不足),以及值得参考的资源、指南或需要提前学习的主题。总的来说,他希望在真正投入开发这款游戏之前,能够获得一些专业人士的建议和指导,以更好地准备和规避可能遇到的问题。

[
https://old.reddit.com/r/rust_gamedev/comments/1fgsx3b/what_approach_should_i_take_on_making_a_realtime/
](
https://old.reddit.com/r/rust_gamedev/comments/1fgsx3b/what_approach_should_i_take_on_making_a_realtime/
)
    


### TITLE

以下是对给定内容的中文总结:

1. 错误和无结果情况在编程中是无法避免的。Rust语言通过Option和Result两种特殊类型优雅地处理这些情况。

2. Option表示一个值可能存在或不存在。None表示无值,Some(x)表示有值x。我们可以用match表达式或if let语法来处理Option。

3. Result表示一个操作可能成功(Ok(x))或失败(Err(e))。我们同样可以用match或?符号来处理不同情况。

4. Option和Result是Rust独有的类型,编译器可以静态检查是否正确处理了可选值和错误情况,帮助我们减少bug。

5. Rust的这些设计使得错误处理变得优雅简洁,提高了程序员的幸福感。

6. 关于嵌入式Rust,你提出了在ESP32上如何控制CPU执行、设置存储器映射等问题,这些都是嵌入式系统开发中需要解决的重要问题。

总之,本文介绍了Rust处理错误和可选值的机制,展示了其优雅性,同时你的问题涉及嵌入式系统开发中的一些关键技术要点。

[
https://bitfieldconsulting.com/posts/rust-errors-option-result
](
https://bitfieldconsulting.com/posts/rust-errors-option-result
)
    


### TITLE

该段内容描述了作者在学习Rust时遇到的一个问题。作者试图实现一个可重用的TCP通信处理器组件 TcpComHandler,它使用Tokio库来异步处理来自服务器的数据。组件的构造函数需要传入一个实现ByteReceiver trait的对象,以定义当接收到字节数据时的行为。

但是作者在构造函数中启动一个新的任务(task)来接收数据时遇到了Rust的生命周期和借用相关的错误。根本原因是构造函数中创建的临时变量reader在将其移动到新创建的TcpComHandler实例后就不再可用,因此无法将其传递给新任务。作者尝试只保存任务句柄而不持有reader实例,但这也导致了作用域问题。

总的来说,作者在Rust中构造涉及相互依赖的字段的结构时遇到了麻烦,并寻求解决这个生命周期和借用相关的问题的方法。

[
https://old.reddit.com/r/rust/comments/1fgzcby/need_help_with_background_tcp_reader/
](
https://old.reddit.com/r/rust/comments/1fgzcby/need_help_with_background_tcp_reader/
)
    


### TITLE

这是一位开发者从C语言转向使用Rust语言进行嵌入式开发的过程。他在准备好Rust工具链、编写了一个简单的LED闪烁程序并编译后,发现在使用readelf命令检查生成的可执行文件时出现了一些问题。

他提供了以下项目细节:

- 操作系统: Linux Mint 21
- 目标硬件: NRF52-DK (一款Nordic的开发板)
- 项目结构包含了Cargo.toml文件、memory.x文件、主源码main.rs以及Cargo配置文件

他在寻求社区的帮助,希望能解决遇到的这个问题,顺利完成在嵌入式系统上使用Rust的尝试。

[
https://old.reddit.com/r/rust/comments/1fgus4o/embedded_rust_just_starting_out_successful_build/
](
https://old.reddit.com/r/rust/comments/1fgus4o/embedded_rust_just_starting_out_successful_build/
)
    


### TITLE

这个代码库提供了Rust中类型证人(type witness)的几种使用示例。类型证人的概念是通过构造一种类型来验证某些属性,主要优点是在编译期就完成了检查,不需要运行时开销。

代码库中列出了三种主要的类型证人用法:

1. 在不进行类型擦除的情况下进行trait检查([bears])
2. 将一个值提升为类型([auth])
3. 将一个类型转换为值([i18n])
4. 附加:类型同构([equals])

第一个例子展示了如何在不擦除具体类型的情况下,通过构造一个Certified包装器类型来确保某个值实现了Bear trait。这样既能调用Bear trait方法,又能访问具体类型的方法。

总的来说,这个代码库探索了如何在Rust中使用类型系统进行编译期检查和验证,避免运行时的类型错误,同时保留了类型信息。

[
https://github.com/tinybeachthor/bear_witness
](
https://github.com/tinybeachthor/bear_witness
)
    


### TITLE

这是一个关于使用Rust编程语言替代Java来节省服务器成本的讨论。作者表示,他已经在Google上搜索了一些相关统计数据,但发现这些数据范围相差很大,而且大多已经过时。鉴于Rust语言一直在不断改进,作者希望能够获得更新更准确的信息,来评估将Java服务器端代码迁移到Rust所能带来的成本节省。

[
https://old.reddit.com/r/rust/comments/1fgu1vc/server_cost_savings_in_rust_versus_java/
](
https://old.reddit.com/r/rust/comments/1fgu1vc/server_cost_savings_in_rust_versus_java/
)
    


### TITLE

这是 lexical-core 数字解析和写入 crate 的 1.0.0 版本发布公告。开发者发现了一些安全性和正确性问题,因此进行了大规模重构:

1. 尽可能删除不安全代码,将不安全函数数量从 160 减少到 8。
2. 修复了一些整数解析的正确性问题(浮点数没受影响)。
3. 增加了安全更新的私人联系方式,希望社区成员能够在发现重大安全漏洞时协助处理。
4. 一周后将取消所有可能存在漏洞的旧版本 crate。
5. 进行了全面测试,包括模糊测试、Miri 未定义行为检测和 Valgrind 运行。
6. 一些重大变更:移除了不安全 API、安全和 nightly 特性,最低支持 Rust 1.63.0。

除此之外保持了向后兼容性。开发者呼吁使用新版本,如有任何问题请反馈。这次重大发布着重加强了安全性和稳定性。

[
https://old.reddit.com/r/rust/comments/1fgyopo/lexicalcore_version_100_released_major_security/
](
https://old.reddit.com/r/rust/comments/1fgyopo/lexicalcore_version_100_released_major_security/
)
    


### TITLE

这是一篇关于Rust编程语言中使用u8和const u8的讨论。作者发现,对于大多数用例,使用一对指针比使用slice unsafe API更快。只有在需要获取最后几个百分点的性能,或者在处理零大小类型(ZST)时,slice才会更有优势。这是一个罕见但重要的用例,因为Rust还没有特化(specialization)功能。

作者对这一发现感到困惑,不确定如何处理这一信息,并询问是否有人为此开发了一个crate(Rust的库/包)。

总的来说,这是一个关于权衡Rust编程中性能和安全性的讨论,作者希望就此获得更多建议。

[
https://old.reddit.com/r/rust/comments/1fgsmtc/u8_vs_const_u8_const_u8/
](
https://old.reddit.com/r/rust/comments/1fgsmtc/u8_vs_const_u8_const_u8/
)
    


### TITLE

本文介绍了一种名为Dune的新型Shell,旨在提供强大的脚本编程能力。Dune结合了Bash和Lisp的特性,一方面可以执行常规的Shell操作,如管道、文件重定向和运行程序;另一方面还提供了标准库和函数式编程抽象,用于各种编程和系统管理任务。

文章给出了用Cargo安装Dune的方法,并解释了.dune-prelude文件的作用,该文件可用于设置环境变量、定义函数等。作者解释了创建Dune的初衷,希望它能提供比Bash更加亲切、个性化和可定制化的Shell体验。文中还列出了Dune支持的算术运算符,并强调它们的行为类似于Python。总的来说,Dune旨在成为一种功能强大、个性化的新型Shell。

[
https://adam-mcdaniel.github.io/dune-website/
](
https://adam-mcdaniel.github.io/dune-website/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

