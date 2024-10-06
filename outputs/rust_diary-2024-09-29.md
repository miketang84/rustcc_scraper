【Rust日报】2024-09-29


### TITLE

这个文档介绍了Rust crate ureq中的transport模块,它提供了一种可插拔的传输层来实现HTTP/1.1协议在两点间的数据传输。主要内容包括:

1. ureq使用Connector trait来提供不同的连接方式,例如TCP套接字、SOCKS代理、HTTPS/TLS等。

2. DefaultConnector是默认提供的连接器,可满足大部分HTTP/1.1需求。

3. 连接器可以链式组合,ChainedConnector辅助设置连接器链。

4. 模块还定义了一些用于连接的数据结构,如Connection Details、Buffers等。

5. Transport trait抽象了HTTP/1.1的传输,可由Connector创建。

6. 提供了一些具体的连接器实现,如TcpConnector、SocksConnector等。

总的来说,这个模块为ureq提供了网络连接和HTTP数据传输的底层支持,并且可扩展以满足不同的需求。

另一个链接是一个Rust序列化基准测试的repo,包含了多种序列化库的基准测试用例和结果,用于评估它们在序列化/反序列化速度、大小、压缩等方面的表现。

[https://docs.rs/ureq/3.0.0-rc1/ureq/transport/index.html
](https://docs.rs/ureq/3.0.0-rc1/ureq/transport/index.html
)
    


### TITLE

总结:

Pot是一种自描述、安全、紧凑的数据序列化格式。它最大的节省空间特点是在序列化时不会重复出现多次相同的符号/标识符。当序列化结构体数组时,这种特性可以带来显著的空间节省。上述示例展示了在序列化1000个包含100个条目的LogArchive对象时,Pot占用的空间(2,627,586字节)明显小于其他自描述格式如CBOR和消息包(msgpack)。尽管Pot不如一些非自描述格式(例如Bincode)紧凑,但它仍然具有很好的空间效率,同时保留了自描述的优势,这使得它在生产环境中运行时安全可靠。

[
https://docs.rs/pot/
](
https://docs.rs/pot/
)
    


### TITLE

Postcard是一个针对Serde设计的无标准库序列化和反序列化库。它的主要设计目标是为受限环境(如嵌入式系统)提供支持,同时支持大部分Serde特性,允许用户自定义序列化和反序列化行为。

Postcard从1.0.0版本开始,采用了文档化和稳定的线格式。它对大于8位的有符号无符号整数使用了变长整数编码。Postcard支持Flavors系统,允许通过插件的方式修改序列化或反序列化的处理方式。

该库提供了多种序列化和反序列化函数,支持将数据序列化或反序列化为各种目标,如Vec、slice、I/O流等,并可选择是否启用COBS编码或CRC32校验。Postcard还支持无堆分配。

Postcard采用了Apache 2.0或MIT两种开源许可,欢迎贡献代码。总的来说,Postcard是一个为受限环境设计、灵活可扩展的Serde序列化/反序列化方案。

[
https://docs.rs/postcard/
](
https://docs.rs/postcard/
)
    


### TITLE

这是对serde-brief二进制格式规范的总结。serde-brief是一种自描述的二进制格式,类似于JSON但更高效。它目前处于不稳定状态。该格式包含了数据结构的信息,从而无需额外的模式就可解析数据,支持向后兼容性,但也会增加数据大小和解析开销。

文档定义了该格式支持的数据类型及其编码方式,如null、布尔值、有无符号整数(使用VarInt编码)、浮点数、字节数组、字符串等。整数使用ZigZag编码来支持有符号整数。

该格式使用不同的类型字节来区分序列(SeqStart/SeqEnd)和映射(MapStart/MapEnd),可嵌套使用。对于Rust的数据类型,该文档解释了如何将它们映射到serde-brief格式,包括结构体、枚举、集合等,有两种模式:一种使用字段名字符串作为键,另一种使用无符号整数索引作为键。

总的来说,这份文档描述了serde-brief二进制格式的细节,支持的数据类型、编码方式,以及如何将Rust数据类型序列化和反序列化到该格式。

[
https://github.com/FlixCoder/serde-brief/blob/main/docs/format-specification.md
](
https://github.com/FlixCoder/serde-brief/blob/main/docs/format-specification.md
)
    


### TITLE

ureq是一个轻量级、同步的HTTP/1.1库。3.x版本从头开始重写,采用了Sans-IO模式,大量使用了类型状态变量。该库现在分为协议crate(hoot)和客户端(ureq)两个部分。开发者已经将3.x的第一个发行候选版本推送到了crates.io。

一些亮点包括:

- 基于HTTP crate的API
- 可以自带传输层(和定制的解析器)
- 禁止使用unsafe代码
- 减少内存分配

开发者欢迎回答任何问题。

[
https://old.reddit.com/r/rust/comments/1frgkum/ureq_3x_release_candidate_1/
](
https://old.reddit.com/r/rust/comments/1frgkum/ureq_3x_release_candidate_1/
)
    


### TITLE

Serde-Brief是一个用于在Rust环境下编码和解码自描述二进制格式数据的crate。它具有以下设计目标和特性:

1. 与Rust生态系统良好集成,支持serde派生实现的所有功能。
2. 具有向后兼容性,可以轻松添加或重新排序字段/变体。
3. 支持#![no_std]和std环境。
4. 资源高效,高性能、低内存占用。
5. 跨架构无缝通信。
6. 经过良好测试,确保安全性。

Serde-Brief的二进制格式规范已在其文档中定义。它与Postcard、Pot、JSON等其他格式具有一些差异,在空间效率、速度、自描述能力等方面存在权衡。

Serde-Brief提供了简单的序列化/反序列化示例,以及基准测试结果。作者希望这个crate能为Rust社区提供有用的工具。

[
https://old.reddit.com/r/rust/comments/1frpa6a/releasing_serdebrief_a_selfdescribing_binary/
](
https://old.reddit.com/r/rust/comments/1frpa6a/releasing_serdebrief_a_selfdescribing_binary/
)
    


### TITLE

这个帖子是一个Rust初学者分享了自己学习迭代器(iterator)、map、filter、collect等函数式编程概念的经历。他举了一个使用这些概念的代码示例,表示如果用命令式编程来实现同样的功能,代码会变得非常冗长和难以理解。他花了几天时间才理解如何正确使用trim函数。

他不太确定这种编程风格的名称,猜测可能是函数式编程。由于他之前只在大学Java课程中接触过流(stream)的相关概念,所以他还不太熟悉函数式编程。他希望能获得一些学习函数式编程的良好资源建议,这些资源不一定是Rust语言相关的,因为他的困惑不是特定于Rust语言。

根据其他人的回复,他确认自己遇到的主要困难是函数式编程的概念。因此,他希望获得一些学习函数式编程的资源推荐。

[
https://old.reddit.com/r/rust/comments/1fs1y8h/learning_about_iterators_map_filter_collect_etc/
](
https://old.reddit.com/r/rust/comments/1fs1y8h/learning_about_iterators_map_filter_collect_etc/
)
    


### TITLE

这个项目是一个名为submap的Rust crate,提供了一个高性能的pub/sub(发布/订阅)系统映射数据结构。主要特性包括:

1. 支持主题订阅,包括单个主题和通配符匹配模式。可自定义通配符和主题分隔符。

2. 支持公式订阅,允许根据某些条件(如重要性级别)订阅主题。 

3. 提供BroadcastMap用于根据掩码模式获取客户端列表。

4. 提供基于SubMap的ACL(访问控制列表)映射,用于快速访问控制检查。

5. 支持两种底层存储引擎:标准库BTreeMap或更快的indexmap(需启用crate特性)。

6. 自带单元测试示例说明使用方法。

7. 发布在crates.io上,可通过cargo直接集成到Rust项目中。

总的来说,这是一个为pub/sub系统量身定制的高效的主题映射数据结构和工具集。

[
https://github.com/alttch/submap
](
https://github.com/alttch/submap
)
    


### TITLE

以下是对该内容的中文总结:

这是"嵌入式Rust工程师"第29期的双月刊。本期重点内容包括:

- 新闻和文章综述,涉及嵌入式Rust、WebAssembly、Linux内核等相关领域的最新进展。
- 教育资源,包括教程、案例研究、嵌入式系统开发实践等。
- 值得关注的课程、书籮和时事通讯。
- 社区更新,如框架和库的新版本发布、即将到来的活动等。
- 企业采用嵌入式Rust的最新情况跟踪。
- 相关的工作机会信息。

该期刊旨在为嵌入式Rust开发者提供全面的资源汇总和社区动态,推动嵌入式Rust的广泛应用和持续发展。

[
https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-29
](
https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-29
)
    


### TITLE

以下是对给定内容的中文总结:

该内容描述了一个命令行选项解析器的几个关键函数:

1. is_opt_end 用于检查是否遇到选项结束标志 "--"。

2. is_long 用于检查当前值是否为长选项参数。

3. is_short 用于检查当前值是否为短选项参数。

4. as_long 是一个解析器函数,用于返回参数名,如果字符串包含"=",还会返回相应的值。

5. as_short 是一个解析器函数,用于返回一个 ShortArgument 结构体。

总的来说,这些函数是用于解析和识别命令行选项参数的关键组成部分。

[
https://traxys.me/sheshat_pantheon_3.html
](
https://traxys.me/sheshat_pantheon_3.html
)
    


### TITLE

这是一篇宣布 iceoryx2 v0.4.0 版本发布的博客文章。iceoryx2 是一个旨在实现进程间快速通信的服务库,比 Unix 域套接字或消息队列快数量级,使用也更简单。该版本实现了许多里程碑,接近了与前身 iceoryx 同等的功能水平。

新版本的主要特性包括:

1. 无需中央守护进程,架构更高效,速度提高10倍。
2. 更动态灵活,不再需要编译时配置内存池。
3. 支持高级 QoS 设置。
4. 非常模块化,可定制化支持 GPU、FPGA 等。
5. 完全去中心化,更加健壮。
6. 重构 API 和资源管理,支持真正的零拷贝通信。
7. 开箱即用的 C/C++ 语言绑定,后续将支持 Python 等语言。
8. 预计将添加网关支持 zenoh、DDS、MQTT 等协议。

该版本在某些平台上的延迟甚至低于 100ns。新增了 C/C++ 示例、新网站、Bazel/CMake 构建支持、命令行调试工具等。后续计划完善 C/C++ 绑定、事件多路复用、动态负载服务、健康监控和文档等。

[
https://ekxide.io/blog/iceoryx2-0-4-release/
](
https://ekxide.io/blog/iceoryx2-0-4-release/
)
    


### TITLE

以下是总结:

作者提到了Dafny这种编程语言,并询问Rust开发者对它的看法。Dafny是一种通过形式化证明来确保正确性的语言,这一特性比Rust强调的内存安全性和无未定义行为更进一步。

Dafny的目标是通过静态程序验证来消除程序中的bug,从而提高软件质量。它允许开发人员编写形式化规范和推理,证明代码的正确性。

作者想了解Rust开发者对这种基于证明的编程范式的看法,因为Rust也非常重视内存安全和正确行为。这可能会引发关于两种范式、权衡和融合的有趣讨论。

[
https://old.reddit.com/r/rust/comments/1fs12l9/what_do_you_rustaceans_think_of_dafny_language/
](
https://old.reddit.com/r/rust/comments/1fs12l9/what_do_you_rustaceans_think_of_dafny_language/
)
    


### TITLE

这是一款放松自由建造游戏。玩家可以在游戏中自由创建漂亮的建筑和景观,不需要管理或战斗,只专注于创作。游戏使用了无网格化的建造系统,能够精细地组装每一块砖石和木板,根据玩家的构思自动调整。玩家绘制通路,游戏就会自动生成门户;提高建筑高度,支撑梁柱也会自动生成。游戏没有错误情况,玩家可以随时改变心意。无论创作什么,效果都会显得温馨舒适。轻松的氛围音乐,建筑被绿植环绕,小羊穿行其间,萤火虫点缀夜色,都让这个世界充满生机。游戏旨在让玩家放松心情,沉浸在生动有趣的创作过程中。

[
https://store.steampowered.com/app/2198150/Tiny_Glade/
](
https://store.steampowered.com/app/2198150/Tiny_Glade/
)
    


### TITLE

这个仓库提供了一个名为 `serdebug` 的 Rust crate,它是 `#[derive(Debug)]` 的一个替代品。它底层使用了 `serde::Serialize` 来提供更高级的输出序列化控制。

与原生的 `#[derive(Debug)]` 不同,`serdebug` 允许您使用 `serde` 的属性来自定义调试输出的格式。您可以重命名枚举项和字段名称、跳过某些字段的序列化、为第三方类型提供自定义序列化等。这使得调试输出更加清晰和可控。

该仓库提供了一个示例,展示了如何使用 `serdebug` 派生 trait,以及如何使用 `serde` 属性来自定义调试表示形式。总的来说,`serdebug` 为 Rust 程序员提供了更灵活的调试能力。

[
https://github.com/RReverser/serdebug
](
https://github.com/RReverser/serdebug
)
    


### TITLE

这是SEGGER公司发布的一条新闻,主要内容是SEGGER的Ozone调试器现已支持Rust编程语言。Ozone为Rust提供了源代码和汇编级调试、系统状态检查、数据分析等重要功能。SEGGER的创始人表示,Rust作为一种高效、安全的语言日益受到欢迎,有望超过C/C++,而SEGGER的Ozone调试器对Rust语言的支持将助力Rust在嵌入式领域的发展。Ozone专为嵌入式应用设计,以其速度和易用性而著称,结合SEGGER的J-Link/J-Trace调试探针,能提供系统分析、代码优化等强大功能。Ozone支持Windows、Mac和Linux多平台。

[
https://www.segger.com/news/pr-240927-ozone-support-rust/
](
https://www.segger.com/news/pr-240927-ozone-support-rust/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

