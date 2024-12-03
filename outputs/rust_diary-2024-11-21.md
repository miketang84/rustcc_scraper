【Rust日报】2024-11-21


### TITLE

该存储库包含了谷歌的 Diff Match Patch 库,它提供了三个算法用于对纯文本进行同步操作:

1. Diff: 比较两段纯文本并高效地返回差异列表。
2. Match: 给定一个搜索字符串,在一段纯文本中找到最佳模糊匹配,同时考虑精确度和位置。 
3. Patch: 将一系列补丁应用于纯文本,即使底层文本不匹配也会尽最大努力应用补丁。

该库最初于2006年为 Google Docs 构建,现已移植到 C++、C#、Dart、Java、JavaScript、Lua、Objective C 和 Python 等多种语言。它实现了迈尔斯差异算法,并通过一些预处理和后处理步骤来提高性能和输出质量。该库还实现了一种 Bitap 匹配算法,作为灵活的匹配和修补策略的核心。该存储库包含了库的使用演示、API 参考、不同层级的差异比较、处理结构化内容的指南、补丁序列化格式以及开发者支持渠道等资源。

[https://github.com/google/diff-match-patch
](https://github.com/google/diff-match-patch
)
    


### TITLE

这门课程由Nathan Stocks主讲,是一门快节奏、有趣且内容丰富的Rust编程语言实践速成课程。课程旨在为学习者提供Rust语言的基础知识,帮助他们攻克Rust的陡峭学习曲线。

该课程采用动手实践的方式,针对每个主题都设有专门的练习,并包含多个综合项目让学习者运用所学知识。课程内容涵盖了使用cargo创建项目、编写代码、编译和运行程序等方方面面的知识。

Rust是一种系统编程语言,能够消除整个类别的错误和安全漏洞,具有像C和C++一样的零开销抽象,编程乐趣无穷,让系统程序员也能拥有良好的编程体验。因此,Rust在游戏引擎、高性能计算、嵌入式设备和Web编程等多个领域都受到广泛关注。

通过本课程,学习者将学习如何编写高性能、安全且无崩溃隐患的代码。同时也将加入到一个充满活力的开发者社区,在这里,多元化、包容性和互相尊重都是首要目标。

本课程是Rust系列课程的第一门,如果你喜欢这门课程,接下来就可以继续学习"终极Rust 2:中级概念"了。无论你是有志于成为系统程序员、软件开发者、工程师,还是巫师、勇士或者霍比特人,只要你需要快速、高效、安全、在严格约束下运行代码,或者希望代码拥有最小的错误率,那么本课程将是开始学习Rust的绝佳选择。

[
https://www.udemy.com/course/ultimate-rust-crash-course/?couponCode=GIVETHANKS2024
](
https://www.udemy.com/course/ultimate-rust-crash-course/?couponCode=GIVETHANKS2024
)
    


### TITLE

这是一个GitHub仓库的自述文件,介绍了一个名为par的Rust库。该库实现了会话类型(session types)的概念,用于描述并行、消息传递过程的整个外部行为,包括消息顺序、分支、递归和高阶模式等。

会话类型可以为并发程序提供以下保证:

1. 协议遵从性 - 满足期望,履行义务。
2. 无死锁 - 静态地排除循环通信。

通过使用会话类型,可以在类型级别对并发交互进行建模,确保实现了指定的协议,避免了意外消息和未履行义务的情况,并防止死锁发生。这对于实现复杂的并发系统至关重要。

该库提供了清晰的文档作为学习工具,包括队列(Queue)、服务器(Server)等标准并发模式。它建立在async/.await之上,与运行时无关,设计人性化,不使用unsafe代码。该库还包含一些示例,如聊天服务器示例。总的来说,这个库提供了在Rust中实现会话类型并进行并发编程的能力。

[
https://github.com/faiface/par
](
https://github.com/faiface/par
)
    


### TITLE

这篇文章采用有趣的虚构故事情节,比喻性地讲述了Rust编程语言的兴起和应对大规模并发请求挑战的过程。

故事背景设置在数字世界中,一个名为Scalator的反派通过发起大量并发请求攻击,导致系统崩溃和性能下降。传统编程语言如C等无法应对这种攻击。

2009年,Mozilla工程师Graydon Hoare创造了Rust编程语言,将系统级编程与卓越的安全性和速度融合,为对抗Scalator做好准备。

随后,Rust凭借其所有nership模型、Option类型、借用检查器等特性,成功防御了Scalator的各种攻击手段。Rust以其异步超级能力处理了大规模并发请求。

最终,Rust战胜了Scalator,成为数字世界的保护者,让系统能够轻松处理大规模请求。现在,Rust正在传播其影响力,激励新一代程序员。

但故事的结尾揭示,一个名为Magnitor的新威胁将带来千万亿级的并发请求,对Rust提出了更大挑战。故事将有新的续篇。

[
https://brutally-honest.medium.com/the-rise-of-rust-as-high-performance-superhero-8f898e769331
](
https://brutally-honest.medium.com/the-rise-of-rust-as-high-performance-superhero-8f898e769331
)
    


### TITLE

以下是对该README文件的中文总结:

skim是一个通用的模糊查找器工具,能帮你节省大量时间。它提供了一个单一的可执行文件sk,可以在任何你想使用grep的地方替代grep使用。

该README详细介绍了skim的安装方法、使用方式、按键绑定、搜索语法、退出代码含义、自定义配置(如按键重新绑定、排序规则、颜色主题等)、高级主题(交互模式、执行外部程序、预览窗口、字段支持、作为库使用)以及FAQ。

它支持多种包管理器进行安装,也提供了作为Vim插件的安装方式。使用时可以作为过滤器,也可以作为交互界面调用其他命令。支持各种搜索语法组合,按键绑定丰富。同时,skim高度可定制化,支持自定义按键绑定、排序规则、颜色主题等。

总的来说,skim作为一个通用模糊查找器,功能强大、扩展性好,是一个提高效率的实用工具。

[
https://github.com/skim-rs/skim
](
https://github.com/skim-rs/skim
)
    


### TITLE

这个仓库包含一个脚本,旨在为LLDB调试器添加Rust相关的漂亮打印功能。由于最近从CodeLLDB中移除了Rust特定的漂亮打印功能,调试Rust(特别是枚举类型)变得非常困难。该脚本旨在作为一种临时修复,直到生态系统状况得到改善。

该脚本提供了在独立LLDB、VSCode中使用CodeLLDB扩展和lldb-dap扩展时加载脚本的说明。它还包括一些配置示例,以及与第三方crate集合类型的兼容性说明。

该脚本目前支持LLDB 19.0.0、19.1.0-codelldb版本和最新稳定版Rust 1.62.0。但由于Rust标准库内部和LLDB表示的变化,这只是一种临时的解决方案,可能会过时。作者希望Rust自身的Pretty Printers最终能发布功能齐全的版本,从而取代这种临时修复。

最后,作者感谢了CodeLLDB作者的贡献,欢迎拉取请求改进该脚本,并希望该脚本能帮助展示调试Rust的重要性。

[
https://github.com/cmrschwarz/rust-prettifier-for-lldb
](
https://github.com/cmrschwarz/rust-prettifier-for-lldb
)
    


### TITLE

总结如下:

作者正在使用一种被认为不太高效的解释型语言来处理WebSocket连接。WebSocket接收来自多个用户的文本修改(patches),需要对这些修改应用于原始文本并返回修改后的文本。

作者考虑将应用文本修改这一特定任务委托给一个Rust应用程序来处理,以提高性能。但同时也担心服务和Rust应用之间的通信可能太慢,抵消了性能提升。作者计划使用Redis缓存进行服务和Rust应用之间的通信。

他在询问这种架构设计(将特定任务委托给Rust)是否合理,以及服务和Rust应用通信是否会成为性能瓶颈。

[
https://old.reddit.com/r/rust/comments/1gwsdxf/should_i_delegate_this_task_from_my_service_to_a/
](
https://old.reddit.com/r/rust/comments/1gwsdxf/should_i_delegate_this_task_from_my_service_to_a/
)
    


### TITLE

这个问题涉及到Rust语言中字符串切片的使用。作者声明了一个全局数组 MY_ARR，它是一个包含字符串切片的数组引用(&[&str])。作者想知道为什么需要使用引用(&)，而不能直接使用字符串切片数组([str])。

原因是Rust中的字符串切片([str])没有固定大小，它的大小取决于运行时数据。而在定义静态变量时,Rust需要知道确切的大小,因此无法使用[str]。使用&[&str]可以让编译器推断出大小,因为&str的大小是已知的。

所以为了在程序的全局作用域中使用字符串切片数组,必须使用&[&str]这种引用数组的形式,而不能直接使用[str]。这是Rust为了内存安全而做出的限制。

[
https://old.reddit.com/r/rust/comments/1gwqeph/jog_my_memory_on_string_slices/
](
https://old.reddit.com/r/rust/comments/1gwqeph/jog_my_memory_on_string_slices/
)
    


### TITLE

以下是对给定内容的中文总结:

作者提到,由于即将到来的美国感恩节假期,他感谢自己有机会学习新知识,也感谢Rust编程语言的存在。为了庆祝这个节日,他分享了两门Rust编程课程的优惠券链接,可以在接下来的5天内或优惠券被领用1000次之前免费获取这两门Udemy在线课程,分别是"终极Rust速成课程"和"终极Rust 2:中级概念"。作者祝大家感恩节快乐。

[
https://old.reddit.com/r/rust/comments/1gwtyq1/learn_rust_free_video_courses/
](
https://old.reddit.com/r/rust/comments/1gwtyq1/learn_rust_free_video_courses/
)
    


### TITLE

这是一篇关于Rust编程语言并发会话类型(session types)的讨论帖子。作者最近发布了一个实现会话类型的Rust crate库,名为par。他想了解程序员为什么没有广泛采用会话类型,以及Rust社区对会话类型的看法和印象。

会话类型允许指定整个并发通信协议,使得可以编写在行为方面经过类型检查的安全并发应用程序,还有助于预防死锁。作者希望通过这个帖子收集反馈,了解大家对于采用会话类型的障碍和疑虑,以推动会话类型在Rust中的更广泛应用。

[
https://old.reddit.com/r/rust/comments/1gwjeid/poll_why_are_you_not_using_session_types_for_your/
](
https://old.reddit.com/r/rust/comments/1gwjeid/poll_why_are_you_not_using_session_types_for_your/
)
    


### TITLE

这是一篇关于Rust编程语言在高性能领域崛起的文章。文章提到,Cloudflare的Pingora代理曾经使用Nginx处理网络流量,但由于requests数量的增长,Nginx已经达到了极限。于是Cloudflare决定使用Rust语言重写Pingora,新版本每天能够处理超过1万亿个请求。

作者将Rust比作一位超级英雄,专门对抗能发送大量请求并使系统瘫痪的"Scalator"恶棍。文中通过一个有趣的小故事,诗意地展现了Rust语言在高性能计算领域的优势和未来发展前景。最后,作者邀请读者阅读他撰写的关于这个题材的小品文,并分享评论。

[
https://old.reddit.com/r/rust/comments/1gwbzuk/the_rise_of_rust_as_highperformance_superhero/
](
https://old.reddit.com/r/rust/comments/1gwbzuk/the_rise_of_rust_as_highperformance_superhero/
)
    


### TITLE

这是对Rust编程语言中一个名为"skim"的模糊查找器工具的最新版本发布的通知。在被搁置了一年多之后,skim终于发布了新版本,不仅修复了一些bug,还更新了依赖项。未来几周内,开发者计划添加与tmux更紧密集成的参数、一些改善用户体验的新功能和标志、运行时生成shell补全的参数、支持更多shell的补全功能,以及改进文档等。长期来看,他们正在使用ratatui重写skim,这可能使其在Windows上也能运行,但由于要确保不破坏任何现有功能,因此重写工作将是一个漫长的过程。

[
https://old.reddit.com/r/rust/comments/1gwsbsx/skim_is_back/
](
https://old.reddit.com/r/rust/comments/1gwsbsx/skim_is_back/
)
    


### TITLE

该文章主要介绍了Rope数据结构中的一些优化技术,以提高文本编辑器在转换offset和point之间的效率。

1. 背景介绍:Rope数据结构是一种用于存储字符串的数据结构,由一个B树组成,每个节点最多存储128个字节的字符串。

2. 问题所在:在将offset转换为point(行号和列号)时,需要遍历每个字符,统计换行符,这个过程效率很低。

3. 优化方法:利用了一些位运算优化技术,如并行位计数、无分支选择等,大大加快了统计换行符的速度。

4. 通过带有视频的配对编程,作者学习并分享了这些位运算优化技术,进一步优化了Rope在处理制表符时的效率。

该文章深入探讨了在文本编辑器等需要高效字符串处理的应用中,如何通过数据结构和算法层面的优化来提升性能。

[
https://zed.dev/blog/zed-decoded-rope-optimizations-part-1
](
https://zed.dev/blog/zed-decoded-rope-optimizations-part-1
)
    


### TITLE

这是一个名为Sail的开源项目,旨在统一流处理、批处理和计算密集型(AI)工作负载。Sail目前提供了一种可替代Apache Spark SQL和Spark DataFrame API的解决方案,可在单机和分布式环境中使用。

该项目提供了在Kubernetes上部署Sail的指南,以及通过pip安装Sail的说明。文档中还包括了Sail的入门指南。另外,还分享了一篇博客文章,对Sail与Spark的基准测试结果进行了详细对比。

该项目欢迎各种贡献,包括提交问题、功能请求和代码修改等。同时也提供了开发指南来帮助新的贡献者入门。最后,该文档提供了支持选项的相关信息。

[
https://github.com/lakehq/sail
](
https://github.com/lakehq/sail
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

