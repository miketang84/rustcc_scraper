【Rust日报】2024-10-16


### TITLE

这是一个名为"You Have Mail"的小应用程序,用于在您的电子邮件账户收到新邮件时通知您。当默认通知机制不工作时(例如未安装Google Play服务的Android设备),或者您不希望一直打开网页界面/电子邮件客户端时,这个应用可能会很有用。

目前,该应用程序支持Proton Mail后端,仅报告收件箱中的新邮件。该存储库分为多个项目,包括移动端共享代码和Android应用程序。

最新稳定版本可以从Github发布页面和F-Droid下载。作者建议如果您希望捐赠,可以考虑捐赠给GrapheneOS项目。

[https://github.com/LeanderBB/you-have-mail
](https://github.com/LeanderBB/you-have-mail
)
    


### TITLE

这篇内容主要比较了C++和Rust两种编程语言中的消息传递通信库。

C++ Communicator Message-Passing Library (MPL)提供了TCPConnector和TCPResponder之间的消息传递通信。它使用TCPClientSocket和TCPServerSocket类在TCPConnector实例和ClientHandler实例之间建立同步双向连接。图2展示了16个TCPConnector客户端与它们的服务器端ClientHandler之间典型的会话集合。客户端并发地向其他客户端发送1000个4K的消息。每个客户端都有一个发送线程和一个接收线程,因此客户端无需等待回复就可以发送下一条消息。

Rust Communicator RustComm是一个在Connector和Listener之间发送消息的工具。它使用std::net::TcpStream和std::net::TcpListener类型在Connector和Listener::Process组件之间建立通信。Listener::Process的作用与MPL的ClientHandler相同,处理Listener与其关联的Connector之间的消息事务。对于每个传入连接,Listener都会分派一个线程池线程来为关联的Connector执行Listener::Process代码。图2说明了16个客户端及其关联的Listener::Process组件之间的通信会话。

与MPL类似,客户端在其他客户端也在发送时发送1000个4K的消息。每个客户端都有一个发送线程和一个接收线程,因此客户端无需等待回复就可以发送下一条消息。

[
https://jimfawcett.github.io/Post_CommCompare.html
](
https://jimfawcett.github.io/Post_CommCompare.html
)
    


### TITLE

这篇文章是关于可观测性(Observability)的重要性。可观测性是指能够深入了解复杂系统的内部运作状态,及时发现并解决潜在问题。文章指出,随着系统复杂度不断增加,单靠传统监控已难以全面洞悉系统行为,很可能会错失一些重要的故障征兆。可观测性通过收集全面的遥测数据(指标、事件、日志等),结合机器学习等技术进行智能分析,帮助工程师主动发现系统异常,从而提高系统的可靠性和可维护性。文章建议,无论系统大小,都应重视可观测性,并采用专业的可观测性解决方案,以确保系统的健康运行。

[
https://stackoverflow.blog/2024/10/08/think-you-don-t-need-observability-think-again/?cb=1
](
https://stackoverflow.blog/2024/10/08/think-you-don-t-need-observability-think-again/?cb=1
)
    


### TITLE

这篇博文介绍了Polar Signals云和Parca(其核心组件)的一个新功能:自定义标签。通过使用自定义标签,用户可以为堆栈跟踪添加关键值对的注释,从而更好地按照所需维度(如数据库用户名、OpenTelemetry追踪ID等)过滤和聚合分析数据。

文章详细介绍了如何在Rust、Go和C/C++等语言中使用自定义标签,并附有示例代码。对于Rust,需要引入custom-labels crate;对于Go,可以直接使用runtime/pprof包;对于C/C++,则可以链接提供的低级C库libcustomlabels.so。

最后,作者希望这个新功能对用户有所裨益,并鼓励反馈bug或新功能请求。如果需要在其他语言中使用类似功能,也可以在相关issue中投票。

[
https://www.polarsignals.com/blog/posts/2024/10/16/custom-labels-for-rust-go-and-cpp
](
https://www.polarsignals.com/blog/posts/2024/10/16/custom-labels-for-rust-go-and-cpp
)
    


### TITLE

这是一篇关于 Rust 编程语言的帖子,作者分享了一个名为 sqlite-watcher 的开源项目。该项目提供了一些构建模块,可以观察 SQLite 数据库中表的变化,类似于 iOS 上的 Core Data 和 Android 上的 Room。这些构建模块是独立的,不依赖于任何特定的数据库驱动程序或实现。

该仓库提供了两个集成示例,分别使用了 rusqlite 和 sqlx 这两个流行的 SQLite 驱动程序。其他驱动程序或实现也可以通过实现所需的 trait 来添加支持。

作者最初是为了自己的项目而编写了这个库,但认为它也可能对其他人有用,因此决定分享出来。

[
https://old.reddit.com/r/rust/comments/1g57apb/sqlitewatcher_building_blocks_to_observe_changes/
](
https://old.reddit.com/r/rust/comments/1g57apb/sqlitewatcher_building_blocks_to_observe_changes/
)
    


### TITLE

这是微软开源的一个名为OpenVMM的新型虚拟机监视器(VMM)项目,用Rust语言编写,支持Windows和Linux系统。该项目提供了开发者指南和入门指引,欢迎贡献和建议。大多数贡献需要同意一份贡献者许可协议(CLA),声明你有权利授予微软使用你的贡献。该项目采纳了微软开源行为准则。关于商标和徽标的使用需要遵循微软的商标和品牌指南,避免引起混淆或暗示微软赞助。使用任何第三方商标或徽标也需遵循相关政策。该项目的文件内容以Markdown格式存放在代码库中,文档需要与代码保持同步。

[
https://github.com/microsoft/openvmm
](
https://github.com/microsoft/openvmm
)
    


### TITLE

总结:

这是Rust Search Extension浏览器扩展程序的一个重大更新,已经发布了2.0版本。Rust Search Extension是一个方便的浏览器扩展,可以让你从地址栏快速搜索Rust文档和crate。如果你不想安装扩展,也可以使用网页版https://query.rs。

维护Rust Search Extension一直是一项艰巨的任务,因为librustdoc经常更改搜索索引的格式。扩展需要保持与旧版文档的兼容性,因为docs.rs从不为旧文档重新生成最新格式的搜索索引。为了确保这种兼容性,作者不得不维护大量复杂的代码。

作者希望这个扩展对你的Rust开发之旅有所帮助。祝你编程愉快!最新的更改日志可以在https://github.com/huhu/rust-search-extension/releases/tag/v2.0.0查看。

[
https://old.reddit.com/r/rust/comments/1g4u9qh/rust_search_extension_v20_has_been_released/
](
https://old.reddit.com/r/rust/comments/1g4u9qh/rust_search_extension_v20_has_been_released/
)
    


### TITLE

这个帖子比较了用Rust和C++编写的基于TCP的通信程序的性能。这些程序使用了线程池和消息队列。测试在Windows和Linux系统上进行,包括在两台不同机器上运行的情况,以及在虚拟机上运行的情况。通过对比,我们可以看到Rust和C++在此类任务上的执行效率。这种跨语言的基准测试有助于评估Rust的性能特点,并将其与经典的C++进行对比。

[
https://old.reddit.com/r/rust/comments/1g5axtz/compare_rust_and_c_via_message_passing/
](
https://old.reddit.com/r/rust/comments/1g5axtz/compare_rust_and_c_via_message_passing/
)
    


### TITLE

作为 memmap2-rs 的当前维护者,该作者正在寻找一个人来转移项目的所有权,因为他将无法继续为这个项目工作。该项目本身已经基本完成,偶尔会有一些拉取请求,但大体上已经不需要进一步改进或添加新特性。作者希望保持项目现状,不引入新的依赖项或增加复杂性。

这个项目很重要,因为它已经有7200万次下载量。鉴于今年发生过类似的安全事件(xz lib 后门),作者不想将项目转移给任何随机的个人,而是希望找到一个与 Rust 项目关系密切的人。作者询问 rust-lang-nursery 或 rust-bus 是否仍然存在,以便寻求建议。

[
https://old.reddit.com/r/rust/comments/1g4tc9h/looking_for_a_maintainer_for_memmap2rs/
](
https://old.reddit.com/r/rust/comments/1g4tc9h/looking_for_a_maintainer_for_memmap2rs/
)
    


### TITLE

这段内容展示了一个简单的SMTP(简单邮件传输协议)会话流程。发件方首先通过EHLO命令向接收方发起连接请求,接收方回复250 OK表示连接成功。然后发件方使用MAIL FROM命令指定发件人地址,RCPT TO命令指定收件人地址,DATA命令开始输入邮件内容。接收方会返回适当的响应码。发件方输入邮件主题、正文内容,最后使用单独的句号(.)结束输入。接收方确认接收邮件后,发件方使用QUIT命令结束会话。这个过程展示了SMTP协议在传输电子邮件时的基本交互流程。

[
https://www.windmill.dev/blog/smtp-server
](
https://www.windmill.dev/blog/smtp-server
)
    


### TITLE

这篇博文提出了一种新的设计思路,称为"UnpinCell",旨在以一种更符合现有Rust特性的方式实现"固定位置"(pinned places)的功能。与之前提出的"固定字段"(pinned fields)概念不同,UnpinCell利用一种新的"cell"类型来支持在固定引用中访问不可移动(不实现Unpin trait)的值。

UnpinCell是一个非常简单的API,它通过DerefMut特性允许在固定引用中可变访问其内部值,即使该值不实现Unpin trait。这种设计避免了引入新的语言特性,而是利用了与"内部可变性"(interior mutability)类似的思路。

通过UnpinCell,固定引用可以像普通引用一样支持投影,只要目标类型满足以下条件之一:1)实现Unpin trait;2)不实现Drop trait;3)使用fn drop(&pin mut self)作为析构函数签名。

这种设计使得"固定位置"成为一个相对较小的语言改动,而不需引入新的语言特性类别。总的来说,UnpinCell提供了一种支持在固定引用中访问不可移动值的一致性解决方案。

[
https://without.boats/blog/unpin-cell/
](
https://without.boats/blog/unpin-cell/
)
    


### TITLE

这段错误信息解释了在Rust中使用String和&str时需要注意的一些事项。

具体来说,当将一个String类型的值作为参数传递给一个函数时,所有权被转移到了函数内部。因此,在函数外部再次使用这个值会导致错误。错误信息建议,如果不需要获取所有权,可以将函数参数类型改为借用(&str)。

另一种解决方案是在传递参数时克隆(clone)该值,但这可能会带来性能开销。

总的来说,这段错误信息旨在帮助开发者正确理解和使用Rust中的所有权概念,避免在处理字符串时出现由所有权转移导致的错误。

[
https://steveklabnik.com/writing/when-should-i-use-string-vs-str/
](
https://steveklabnik.com/writing/when-should-i-use-string-vs-str/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

