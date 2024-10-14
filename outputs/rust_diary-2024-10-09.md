【Rust日报】2024-10-09


### TITLE

该文章介绍了2024年P99 CONF的一些以Rust为主题的技术演讲。P99 CONF是一个为热衷于低延迟工程策略和性能优化的工程师而设的免费虚拟社区活动。以下是对一些重点演讲的概述:

1. Carl Lerche将探讨使用Rust开发数据库应用程序的生产力问题。

2. Amos Wenger将介绍他使用io_uring和ktls在Rust中开发高性能HTTP实现的工作。

3. Luc Lenôtre将分享使用Rust编写操作系统内核的经验教训。

4. Micah Wylde将解释Arroyo流处理引擎的分布式架构和实现。

5. Will Crichton将使用一个案例研究说明性能工程师的工具箱。

6. Byron Wasti将深入分析Rust异步函数指针的性能陷阱。

7. Nikita Lapkov将讨论如何将Rust actor系统elfo转变为分布式服务网格。

8. Brian Sletten和Ramnivas Laddad将探讨在边缘环境中实现WebAssembly的安全隔离和性能平衡。

该文章还提及了一些其他语言(如Zig、C++、Go和Java)的低延迟相关演讲。总的来说,这次会议将广泛关注Rust及其他语言在低延迟编程领域的最新进展。

[https://www.p99conf.io/2024/10/09/rust-tech-talks/
](https://www.p99conf.io/2024/10/09/rust-tech-talks/
)
    


### TITLE

这个仓库包含了两个用Rust语言实现的验证代码生成器:prost-validate和prost-reflect-validate,用于生成Protobuf消息验证代码。

prost-validate是基于派生trait的实现,而prost-reflect-validate则是基于反射的实现。两个实现共享了从protoc-gen-validate移植过来的测试套件。

根据基准测试结果,prost-validate的性能明显优于prost-reflect-validate。

该仓库详细列出了两个实现目前支持的各种验证规则,包括数字、布尔值、字符串、字节、枚举、消息、重复字段、映射、OneOf字段、标量值包装器、Any、Duration和Timestamp等不同类型字段的验证规则。通过表格直观展示了每个规则在两种实现中的支持情况。

总的来说,这个仓库提供了对Protobuf的消息字段进行多种验证的Rust语言解决方案,并对两种不同实现的性能和功能覆盖进行了比较。

[
https://github.com/linka-cloud/prost-validate
](
https://github.com/linka-cloud/prost-validate
)
    


### TITLE

这个问题是关于在Rust中从文件中读取行时,如何在不消费迭代器的情况下获取行数。作者目前的实现方式是:1)将迭代器收集到向量中,2)使用向量的len()方法获取元素数量,3)根据总行数和分页大小验证分页输入,4)使用分页crate获取要切片的索引,5)返回切片数组。

作者希望找到一种更高效的方法,不需要将整个迭代器收集到向量中。他尝试过使用skip和take方法跳过和获取部分元素,然后收集到向量中。但他的问题是,是否有一种方法可以获取迭代器中元素的数量,而不消费迭代器本身。

他还附上了当前代码的示例,展示了如何打开文件、读取行、将行收集到向量中,以及后续的一些计划步骤。总的来说,这个问题旨在寻求一种更高效地获取文件行数的方法,以实现更好的性能。

[
https://old.reddit.com/r/rust/comments/1fzpm8y/is_there_a_way_to_get_the_number_of_elements_for/
](
https://old.reddit.com/r/rust/comments/1fzpm8y/is_there_a_way_to_get_the_number_of_elements_for/
)
    


### TITLE

这位开发者正在开发一个使用Svelte作为前端、Rust作为后端的Web应用程序,并希望添加社交登录功能,让用户可以使用谷歌、微软或苹果账号登录。他评估了几种不同的方案,包括使用Supabase的社交登录支持、Authelia或者自己手动实现使用oauth2-rs或openidconnect-rs crate。

由于他使用的是Rust后端和PostgreSQL数据库,而Supabase和Authelia缺乏稳定的Rust SDK,他不确定它们是否是可行的选择。手动实现OIDC也需要解析Supabase的REST API webhook来获取令牌和用户信息,工作量可能不小于直接使用openidconnect crate。

因此,他希望了解Rust社区如何通常为Rust应用程序添加社交登录支持,是否有更简单、集成良好的解决方案(类似Supabase Auth SDK)适用于他的技术栈。

[
https://old.reddit.com/r/rust/comments/1g022ti/social_login_for_the_existing_rust_backend_and/
](
https://old.reddit.com/r/rust/comments/1g022ti/social_login_for_the_existing_rust_backend_and/
)
    


### TITLE

本文讨论了Rust语言的设计目标应该围绕编写高质量代码展开。作者认为,相比从语言本身的角度出发,更好的方式是从编写出优秀代码的角度来框架化设计目标。他提出了四个原则:

1. 自构建正确性(Correctness by construction)——优秀代码通过静态检查和不可表示无效状态来防止bug,减轻审阅负担。

2. 目的明确(Clarity of purpose)——优秀代码只关注应用程序的重要特性,避免不必要的复杂性,可在不同抽象层次上迭代理解和编写。

3. 通过组合实现强大(Power through composition)——优秀代码由易于理解的小构建块组成,适当地利用现有库进行高级组合表达。

4. 灵活变更(Flexibility to change)——优秀代码能够在不影响正确性的情况下,接受来自不同贡献者的变更。

文章的核心思想是将用户体验放在首位,尤其是非科技狂热者和非自愿使用Rust的程序员。优秀的代码能让所有相关人员在工作中感到愉悦,而出色的语言则应支持编写出优秀代码。语言的优雅固然重要,但不应过度强调,以免忽视了更大的机会——基于优秀代码构建产品,从而使团队和最终用户受益。

[
https://tmandry.gitlab.io/blog/posts/the-main-thing/
](
https://tmandry.gitlab.io/blog/posts/the-main-thing/
)
    


### TITLE

这是一份关于Rust编程语言的会议通知。P99大会(免费在线)将有一系列的Rust技术讲座,包括由Tokio创建者Carl Lerche主持的主题演讲。会议鼓励Rust社区成员参与讨论,演讲者将现场解答问题。这是一个学习和了解Rust最新进展的绝佳机会。

[
https://old.reddit.com/r/rust/comments/1fzsxh1/rust_talks_at_p99_conf/
](
https://old.reddit.com/r/rust/comments/1fzsxh1/rust_talks_at_p99_conf/
)
    


### TITLE

Fusio是一个为多种异步运行时(包括基于轮询的tokio和基于完成的tokio-uring、monoio)提供读写多种存储后端(如本地磁盘、亚马逊S3等)Trait的Rust库。主要特点包括:

1. 体积小巧,二进制大小比其他类似库小14倍以上。
2. 与裸存储后端相比,Trait定义可无额外开销地分派文件操作。
3. 可扩展,允许第三方crate实现存储后端。

Fusio目前处于预览版,欢迎加入社区讨论其发展和语义/行为。它旨在为Tonbo项目提供一种灵活高效的跨多存储后端、多异步运行时进行文件和文件系统操作的方式。

Fusio提供了运行时无关的Trait接口、对象安全和非对象安全的Trait版本、支持常见文件系统操作、可选的S3支持等。与object_store相比,fusio不依赖bytes,性能开销更小;与opendal相比,功能更精简,默认体积更小。未来还将支持更多存储后端、网络支持等。

[
https://github.com/tonbo-io/fusio
](
https://github.com/tonbo-io/fusio
)
    


### TITLE

以下是对该内容的中文总结:

这份报告由filtra公司发布,主要概述了2024年9月份Rust语言相关工作岗位的市场状况。报告显示,9月份Rust工作机会数量达到了867个新高,共有109家公司在招聘Rust相关岗位,其中亚马逊、IBM和微软是三大主要雇主。除了传统科技公司,Disney等其他行业公司也开始使用Rust语言。

就行业分布来看,云计算/基础设施、加密货币、系统/硬件、生产力和咨询行业是Rust语言使用最多的领域。此外,报告还分析了不同级别(初级、中级和高级)Rust岗位的供给情况,9月份初级岗位数量有所增加。

该报告的数据来源包括filtra平台的数据以及公开的招聘信息。filtra公司表示,欢迎其他公司加入其Rust工作机会指数。

[
https://filtra.io/rust-sep-24
](
https://filtra.io/rust-sep-24
)
    


### TITLE

以下是对该文章的中文总结:

该文章介绍了在同一台机器上使用Rust进行进程间通信(IPC)的几种方法的性能测试。作者在前人的工作基础上,对以下几种IPC机制进行了基准测试:Unix域(Stream和Datagram)套接字、内存映射文件以及使用iceoryx2框架的共享内存。

文章首先解释了实验设置,包括生成随机数据负载、CPU亲和性设置、CPU预热等优化和测量技术。然后详细介绍了每种IPC方法的实现细节和遇到的一些挑战。

1. Unix域Stream套接字是最基本的一种IPC形式。

2. Unix域Datagram套接字需要处理"地址已在使用"、"消息过大"和"缓冲区不足"等错误,实现起来相对复杂些。

3. 内存映射文件通过将文件内容映射到调用进程的虚拟地址空间来实现IPC,需要使用MAP_SHARED标志使一个进程的更改对另一个进程可见,并且需要进程间同步。

4. 共享内存使用iceoryx2框架,是一种无锁队列的高性能实现。

该文章提供了不同IPC方法在不同数据负载大小下的性能测试结果,有助于根据具体需求选择合适的IPC解决方案。

[
https://pranitha.rs/posts/rust-ipc-ping-pong/
](
https://pranitha.rs/posts/rust-ipc-ping-pong/
)
    


### TITLE

bacun是一个背景Rust代码检查器,旨在最小化交互,让您能够在编辑器旁边运行它,并获得Rust代码中的警告、错误或测试失败通知。它以简洁的方式在小终端中显示相关信息。您无需记住命令,关键命令都会列在底部,其他命令通过按"h"键显示。

您可以通过cargo install --locked bacon来安装bacon。启动bacon后,它会基于cargo check监视源目录并显示错误和警告。您可以通过按"t"键或运行bacon test/bacon nextest(如果您使用nextest)来启动和观察测试。遇到失败时,按"f"键只查看失败的测试,按"esc"键返回所有测试。在bacon中,您可以通过按"c"键查看Clippy警告,按"esc"返回上一个作业。您还可以通过按"d"键在浏览器中打开cargo doc。

您可以在bacon.toml文件中定义自己需要的作业,如测试、特定目标编译、示例等,并在编码时查看结果。运行bacon --help可以查看所有启动参数。

全局首选项可在prefs.toml文件中定义,如键绑定、总是启用摘要模式或换行模式等。项目设置在bacon.toml文件中定义,包括作业、示例检查、特殊参数运行、Clippy设置以及作业快捷键等。

总之,bacon让您在编码时能方便地检查和监控Rust代码,提供自动化的错误、警告和测试结果反馈,并支持自定义配置。

[
https://dystroy.org/bacon/
](
https://dystroy.org/bacon/
)
    


### TITLE

这个GitHub仓库包含了一个Rust库nonany,它提供了具有可定制空值(niche values)的整数类型。主要好处是可以让编译器进行内存布局优化,使得一个整数的Option类型和整数本身的大小相同。

这个库为所有整数类型定义了通用类型和一些常用别名,如NonAny*、NonMin*、NonMax*和NonZero*。内部使用了标准库中的NonZero*类型,并通过与空值进行异或运算来存储实际值。这样可以在稳定版Rust中实现,但需要进行额外的异或操作。

该库的最低支持Rust版本为1.56.0,计划在1.0版本发布前保持不变。该库采用Apache 2.0或MIT两种许可证。

[
https://github.com/rick-de-water/nonany
](
https://github.com/rick-de-water/nonany
)
    


### TITLE

这段内容讨论了在不支持 expose_provenance 功能的目标平台上,相关函数应该如何处理的问题。作者表示,个人倾向于在不支持的平台上直接不提供这些函数,而不是给出一些默认或无效的实现。这样,当用户尝试在不支持的目标平台上运行依赖这些函数的库时,就会得到明确的错误提示,而不是出现一些意外行为。不过,作者也指出,目前几乎所有目标平台都支持这一功能,因此在大多数情况下,提供这些函数不会有什么问题。这只是一个值得考虑的边缘case。

[
https://github.com/rust-lang/rust/pull/130350
](
https://github.com/rust-lang/rust/pull/130350
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

