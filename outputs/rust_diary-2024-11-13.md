【Rust日报】2024-11-13


### TITLE

这是一个名为 ipass 的命令行工具,用于与 Apple macOS 密码(iCloud密钥链)进行交互。它是使用 Rust 语言重写的 apw 项目,因为原先的 apw 项目在作者的 M3 macOS 15.0.1 系统上无法运行。

ipass 利用 macOS 14 及更高版本中的内置辅助工具来实现此功能。你可以通过发行页面下载二进制文件,或使用 cargo 命令从源码安装。使用前需要先通过 `ipass start` 启动后台守护进程,并使用 `ipass auth` 进行身份验证。

它允许你列出特定域名的可用密码、获取密码等操作。作者欢迎其他人对这个项目的贡献,如提出改进建议、提交代码Pull请求等。这个项目使用 GPL V3.0 开源许可证。

[https://github.com/kezhenxu94/ipass
](https://github.com/kezhenxu94/ipass
)
    


### TITLE

以下是内容的中文总结:

这个项目名为Wild,是一个新的链接器,目标是在迭代开发过程中实现非常快速的链接。它目前还是一个正在进行的工作,不应该被用于生产环境的二进制文件链接。

Wild链接器主要的动机是现有的mold链接器虽然很快,但不支持增量链接,而作者也没有计划添加这个功能。Wild虽然目前还没有增量链接,但这是最终目标。通过用Rust编写,希望能够解决增量链接的复杂性。

目前Wild支持在Linux上对x86-64架构进行静态链接、静态PIE链接、动态链接和共享对象链接,并支持Rust proc-macro。但还缺少很多功能,如增量链接、其他架构支持、更多链接器参数支持、链接脚本支持、Mac和Windows支持以及LTO支持等。

该项目进行了一些基准测试,表明在非增量链接场景下,Wild的速度相当高效并能较好利用多线程。但测试还非常初步,对其他场景如大型二进制文件链接等性能还未可知。

作者欢迎贡献,并提供了使用Wild链接Rust代码的cargo命令示例。该项目取名"Wild"是因为链接器的名称通常以"ld"结尾,加上期望实现增量(Incremental)链接的"I",再加上开源项目习惯使用递归首字母缩略词,所以就叫Wild。

[
https://github.com/davidlattimore/wild
](
https://github.com/davidlattimore/wild
)
    


### TITLE

这是一个个人技术博客网站,主要分享作者关于Rust编程语言的相关内容。博客中包括以下几个主题:

1. 介绍了作者在GOSIM China 2024大会上的一场关于链接器(linker)的演讲视频。

2. 探讨了Rust动态链接库(dylib)中遇到的一些技术困难和rabbit holes。

3. 分享了作者开发的一个新型链接器的测试过程。

4. 介绍了一些加速rustc编译速度的技巧,比如利用lazy compilation。

5. 上传了作者在悉尼Rust meetup上关于"野生链接器"的视频分享。

6. 更新了"野生链接器"项目的最新进展。

7. 分享了一些加快Rust编辑-构建-运行循环的技巧。

8. 介绍了一种名为Cackle的新工具,旨在增强Rust供应链的安全性,防止供应链攻击。

总的来说,这个博客侧重于Rust语言相关的系统级软件、编译器、链接器等底层技术探索与实践。

[
https://davidlattimore.github.io/
](
https://davidlattimore.github.io/
)
    


### TITLE

这个项目是一个名为Scooter的交互式终端查找和替换UI应用程序。它可以使用固定字符串或正则表达式进行搜索,输入替换内容,并交互式地选择要替换的实例。您还可以为要搜索的文件路径指定正则表达式模式。如果您尝试替换的实例自上次搜索以来已发生更改(例如,如果您已切换分支并且该行不再存在),则不会发生该特定替换。该应用程序遵守.gitignore和.ignore文件,支持在替换字符串中使用搜索正则表达式的捕获组。您可以通过cargo或从源代码构建来安装Scooter。运行scooter即可在终端中启动它,然后输入搜索文本、替换文本、选择固定字符串等,并为文件名输入正则表达式模式。屏幕底部会显示更多键映射说明。

[
https://github.com/thomasschafer/scooter
](
https://github.com/thomasschafer/scooter
)
    


### TITLE

该项目是一个命令行工具,用于从西班牙皇家学院词典中搜索并显示西班牙语单词的定义。它支持在macOS上使用Homebrew进行安装,对于Linux和Windows系统,需要社区的支持与测试来构建发行版包。

使用方式是在命令行中输入rae-cli <palabra>来搜索单词定义。如果单词存在,工具将显示它的定义;如果有建议,将显示一个菜单供选择所需单词。

对于定义较长的单词(尤其是动词),作者建议将rae-cli与less分页工具组合使用,以方便浏览。README中还提供了在.bashrc或.zshrc中添加别名的技巧,以默认使用less分页。

该项目由Dimitar Nanov用MIT许可证授权。

[
https://github.com/nanov/rae-cli
](
https://github.com/nanov/rae-cli
)
    


### TITLE

以下是对该GitHub仓库的中文总结:

Venator是一个用于记录、查看和过滤Rust程序日志和跨度的库和GUI应用程序。它专门用于本地快速开发。目前处于"测试版"状态,可能存在一些bugs和怪异行为,但功能应该是完整的。欢迎提交bug报告和未来功能请求。

使用方法:

1. 在instrumentd的Rust程序中添加依赖项venator。
2. 安装Venator应用程序(需要Rust 1.76或更新版本):cargo install venator-app

该仓库提供了一些Venator的屏幕截图,展示了其GUI界面。Venator旨在为使用tracing crate进行instrumentation的Rust程序提供日志记录、查看和过滤功能,方便本地开发调试。

[
https://github.com/kmdreko/venator
](
https://github.com/kmdreko/venator
)
    


### TITLE

这是作者分享他的第一个Rust项目 ipass 的帖子。ipass是一个用于与苹果macOS密码(iCloud密钥链)进行交互的命令行界面工具,非macOS用户无法使用。作者表示这是他从未编写过Rust"Hello World"程序之前就直接编写的第一个Rust项目,可能会有一些代码对有经验的Rust开发者来说看起来很糟糕。他诚挚地邀请大家试用、评审代码并提出任何意见,无论是关于项目功能、bug、代码风格、实现、文件结构等。如果你觉得这个项目做得不错,请为它点赞。作者对此表示感谢。

[
https://old.reddit.com/r/rust/comments/1gqdr41/invite_reviews_and_comments_on_my_very_first_rust/
](
https://old.reddit.com/r/rust/comments/1gqdr41/invite_reviews_and_comments_on_my_very_first_rust/
)
    


### TITLE

该帖子询问了有关Rust语言中处理多维数据的最佳选择。主要涉及以下几点:

1. 提到了ndarray这个Rust crate,询问它是否为当前唯一且最佳的多维数据处理库。

2. 询问ndarray的性能如何,与Python的NumPy相比如何。

3. 暗示该作者没有发现任何Rust深度学习框架使用ndarray作为底层数据容器。

总的来说,该帖子在探讨Rust语言中高效处理多维数据的方案,并希望能找到一个性能出色且得到深度学习框架支持的库。这反映了处理多维数据在诸如深度学习等领域的重要性。

[
https://old.reddit.com/r/rust/comments/1gq9ebm/whats_the_best_multidimensional_data_processing/
](
https://old.reddit.com/r/rust/comments/1gq9ebm/whats_the_best_multidimensional_data_processing/
)
    


### TITLE

以下是对内容的中文总结:

这个开源项目名为NativeLink,它发起了一个Bug赏金计划,邀请工程师为项目贡献代码、修复Bug和实现新功能,并有机会赢取现金奖励。该计划分为Easy Mode和Hard Mode两个级别,Easy Mode首次获胜可获1000美元,第二次2000美元;Hard Mode首次获胜可获5000美元,第二次10000美元。

要获得奖金,提交的代码需满足三个标准:1)高质量代码和测试,包括功能完整、测试覆盖率至少80%及遵循Rust编码规范;2)提供工程设计文档,解释解决方案的方法;3)及时回应审查意见。

参与者需先fork项目仓库,提交拉取请求并通过身份验证。获胜后将根据首选方式支付奖金。贡献的代码将在开源许可下发布。参与者可合作,但奖金需分享。项目会持续更新新的Bounty机会。

[
https://gist.github.com/MarcusSorealheis/17307f41b6c632cb751c397280c70ed4
](
https://gist.github.com/MarcusSorealheis/17307f41b6c632cb751c397280c70ed4
)
    


### TITLE

这是一篇关于Rust编程语言的帖子。作者几周前在GOSIM开源会议上就他用Rust编写的链接器Wild做了一个演讲,现在这个演讲的视频已经上传到YouTube上了。他感谢GOSIM组织者给予他演讲的机会,也很高兴能见到其他Rust程序员。

作者提到之前已经在自己的博客上发布过关于Wild链接器的文章,该链接器的GitHub仓库也给出了链接。

在演讲中,作者提到Linux会锁定正在运行的可执行文件,这对他计划实现的热代码重载功能有一些影响。但是在会后与内核开发者Gary Guo交谈后,他获知Linux最近已经移除了这种锁定,所以可能会简化热代码重载的实现。

[
https://old.reddit.com/r/rust/comments/1gq0x3t/video_of_wild_linker_talk_at_gosim_2024/
](
https://old.reddit.com/r/rust/comments/1gq0x3t/video_of_wild_linker_talk_at_gosim_2024/
)
    


### TITLE

以下是对该内容的中文总结:

RAGIT是一个类似Git的软件,可将本地文件转换为知识库。该项目的主要目标是使知识库易于创建和共享。与其他RAG框架不同,RAGIT为每个数据块添加了标题和摘要,使AI可以很容易地对数据块进行重新排序。它使用tfidf分数而不是向量搜索,首先要求AI从查询中生成关键词,然后使用这些关键词运行tfidf搜索。它支持带有图像的markdown文件,并支持多轮查询(实验性)。您可以像Git一样克隆/推送知识库,但推送命令仍在开发中。该仓库还包含了构建数据块、配置、贡献、评估和提示工程的指南,以及快速入门指南。

[
https://github.com/baehyunsol/ragit
](
https://github.com/baehyunsol/ragit
)
    


### TITLE

该播客集中讨论了在编译期进行一些棘手操作的技巧,主要涉及以下内容:

1. 去重列表的问题 - 通过宏和const fn来去除列表中重复的元素。

2. 在编译期无法使用内存分配,需要一些技巧来解决这个限制。

3. 使用声明式宏来输入标识符,对列表进行操作。

4. 探讨如何在编译期进行更复杂的操作,如生成列表的列表、霍夫曼编码等。

5. 讨论这些技巧在嵌入式系统、物联网等领域中的应用前景。

6. 提及了一些rust中的概念和crate,如HashSet、syn、rubicon、Huffman编码、const fn等。

总的来说,这一集着重介绍了在编译期完成一些复杂计算的方法和挑战,为了在受限的环境中完成所需的工作。

[
https://sdr-podcast.com/episodes/compile-time-crimes/
](
https://sdr-podcast.com/episodes/compile-time-crimes/
)
    


### TITLE

这篇新闻公告宣布了Rust基金会发布了一份关于C++与Rust语言互操作性的问题陈述。该文件概述了三个关键的战略方针:

1. 改进现有工具并解决Rust项目中的一些短期问题,以降低互操作性的摩擦和风险。

2. 就需要对Rust语言本身进行更改以实现长期目标达成共识,并制定可行的战术方法来逐步实现。

3. 与C++社区和委员会合作,提高两种语言互操作的质量,实现安全性和性能的共同目标。

这项"互操作性倡议"于2024年2月启动,由Google提供100万美元的资助。该倡议认识到C++和Rust在系统编程的未来都扮演着重要角色,高效地将两者结合使用对于追求安全性、性能和可维护性的组织至关重要。该文件旨在为社区提供反馈和参与的基础,共同塑造C++/Rust互操作性的战略方向和技术实施。

Rust基金会欢迎社区对此问题陈述的反馈意见,并参与到互操作性倡议中来。相关进展将通过多种渠道分享和呈现给Rust基金会董事会。

[
https://foundation.rust-lang.org/news/rust-foundation-releases-problem-statement-on-c-rust-interoperability/
](
https://foundation.rust-lang.org/news/rust-foundation-releases-problem-statement-on-c-rust-interoperability/
)
    


### TITLE

这是一个使用Rust编程语言开发的终端用户界面(TUI)应用程序。它的名称是Scooter,主要功能是进行查找和替换操作。该应用程序允许用户精确选择需要替换的实例,如果在选择和替换阶段文件发生变化,受影响的实例将不会被更改,并在最后显示错误信息。作者刚刚发布了这个首个Rust项目,并欢迎任何反馈。文中包含了一个展示应用程序运行效果的GIF动画和项目的GitHub链接。

[
https://old.reddit.com/r/rust/comments/1gqm5rw/interactive_find_and_replace_tui/
](
https://old.reddit.com/r/rust/comments/1gqm5rw/interactive_find_and_replace_tui/
)
    


### TITLE

这位作者最近几个月开始学习西班牙语,发现皇家西班牙语言学院(RAE)的词典和网站对于学习西语很有帮助,尤其是在初学阶段之后。但由于工作繁忙,作者很少有时间动手编程了。为了重拾编程,同时也为了能在终端中快速查询RAE词典,作者决定使用Rust语言编写一个小型命令行界面(CLI)程序。

作者在GitHub上分享了这个项目https://github.com/nanov/rae-cli,称这个程序虽然简单,但让他颇为赞赏Rust的一些原则和优秀的编译错误报告。作者希望能获得一些反馈意见,也希望这个小程序能为他人带来一些用处。

[
https://old.reddit.com/r/rust/comments/1gqi2nw/cli_for_real_academia_española_spanish_language/
](
https://old.reddit.com/r/rust/comments/1gqi2nw/cli_for_real_academia_española_spanish_language/
)
    


### TITLE

该内容介绍了一个名为Venator的日志和跨度查看器工具,作者花费了近6个月的时间将其打造为最小可行产品,并实现了大部分功能和修复了大部分bug。

Venator旨在成为本地开发的良好伴侣,因为基本日志很难跟踪,而其他解决方案又过于繁琐。它的设置很简单:只需在使用tracing库检测的应用程序中设置venator订阅器,它就会将事件和跨度数据导出到Venator应用程序。

作者渴望获得各种反馈,如bug报告、UI改进、功能请求、拉取请求等。在进行一轮性能改进、重构和实战测试后,作者打算发布1.0版本。

作者希望其他人能发现Venator的实用性。

[
https://old.reddit.com/r/rust/comments/1gqgd6a/venator_my_log_and_span_viewer_for_tracing_is_now/
](
https://old.reddit.com/r/rust/comments/1gqgd6a/venator_my_log_and_span_viewer_for_tracing_is_now/
)
    


### TITLE

以下是对该篇内容的中文总结:

这是filtra公司发布的2024年10月Rust工作报告。报告显示,10月Rust工作岗位数量大幅增加到959个,同比增长近100个。共有114家公司在招聘Rust工程师,其中亚马逊是最大的Rust雇主,提供239个职位。IBM、Apple、微软和Datadog等科技巨头也在大量招聘。

云基础设施、生产力工具、加密货币等是使用Rust最多的行业。此外,Rust还广泛应用于系统/硬件、咨询、数据科学、物联网/机器人等领域。尽管如此,报告指出Rust的通用性越来越强,可适用于几乎所有领域。

在招聘级别方面,中级岗位最多(531个),高级岗位次之(406个),初级岗位较少(22个)。报告认为,雇主愿意招聘初级工程师将获得大量有潜力的人才。

总的来说,Rust就业市场保持着强劲增长,越来越多公司开始采用这种系统级编程语言。

[
https://filtra.io/rust/jobs-report/oct-24
](
https://filtra.io/rust/jobs-report/oct-24
)
    


### TITLE

这篇文章讨论了在Linux系统上产生子进程的不同方法,主要包括以下几点:

1. 介绍了使用编程语言提供的API(如Rust的std::process::Command)来生成子进程的简单方式。

2. 解释了有时可能需要使用更底层的API,如fork、vfork、posix_spawn和clone等系统调用,来完成更复杂的任务,如使用Linux namespaces。

3. 详细介绍了经典的fork和exec模型,fork创建当前进程的副本,exec在当前进程中加载并执行新程序。在fork之后,可以在子进程中进行设置,如配置文件描述符、会话、用户、组或namespaces。

4. 指出在多线程程序中使用fork时需要注意"fork安全"问题,因为只有调用fork的线程被复制到子进程中,可能导致锁等机制失效。

5. 介绍了"zygote"模式作为解决多线程fork安全问题的一种方法,即在成为多线程前先fork一个子进程(zygote),后续通过IPC让zygote进程产生其他子进程。

6. 最后提供了一个简单的流程图,帮助决定在不同情况下应采用哪种产生子进程的方式。

总的来说,这篇文章深入探讨了Linux系统上产生子进程的各种底层机制及其适用场景,对于需要精确控制进程生命周期的程序员很有参考价值。

[
https://maelstrom-software.com/blog/spawning-processes-on-linux/
](
https://maelstrom-software.com/blog/spawning-processes-on-linux/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

