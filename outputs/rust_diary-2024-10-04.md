【Rust日报】2024-10-04


### TITLE

这篇文章主要介绍了在Rust中使用原始指针和不安全Rust来实现一个单链队列。它从第二章的单链栈代码出发,因为队列在链表世界中主要是对栈的扩展。该文章承认使用Rc和RefCell来处理引用计数和内部可变性问题会变得复杂,因此需要寻找更好的方式。文中将展示如何避免常见错误,从头开始实现一个安全高效的单链队列数据结构。

[https://rust-unofficial.github.io/too-many-lists/fifth.html
](https://rust-unofficial.github.io/too-many-lists/fifth.html
)
    


### TITLE

这篇文档介绍了Rust crate quick_builder,它提供了一种在编译时生成builder模式的简单而强大的方法。quick_builder的设计理念是尽可能在编译时验证,同时也提供了在运行时强制执行不变量的直观方式。

文档解释了何时应该尝试使用quick_builder,例如当你需要为结构体派生一个builder,并希望忘记设置字段时产生编译错误,以及指定在运行时强制执行的不变量时。它还展示了如何使用#[invariant]属性来定义和强制执行运行时不变量。

接着介绍了quick_builder的一些局限性,例如必须按字段声明顺序执行builder函数、不支持默认/可选参数等。同时也提到了配合使用getset和derive-getters这些流行的crate来保护字段访问和提供getter的方法。

最后,文档讨论了其他一些替代的builder crate,并解释了为什么quick_builder在需要同时实现编译时和运行时验证时具有优势。总的来说,这篇文档清晰地介绍了quick_builder的动机、用法、局限性和替代方案。

[
https://docs.rs/quick-builder/0.1.0/quick_builder/#alternatives
](
https://docs.rs/quick-builder/0.1.0/quick_builder/#alternatives
)
    


### TITLE

这是一个名为Ygen的项目的Github仓库介绍。Ygen是一个工具包,旨在使用内存安全的API构建快速和简洁的编译器。它的主要目标是简单性,API类似于LLVM,大量使用trait来重载函数。该项目目前处于早期开发阶段,可能存在bug和错误编译。仅建议用于玩具编译器。

该介绍提供了一个简单的示例,展示了如何使用Ygen构建一个加法函数。代码生成了一个将两个整数相加的IR函数,并将其打印出来。还提到可以将IR编译为汇编代码。

最后,该仓库由Cr0a3拥有,使用Apache 2.0许可证。目前Ygen支持x64架构的完整IR,但尚不支持完整的ISA。

[
https://github.com/Cr0a3/ygen
](
https://github.com/Cr0a3/ygen
)
    


### TITLE

总结:

这是一位13岁的年轻程序员发布了他自己开发的一个小型代码生成库ygen的0.1.2版本。ygen支持生成一些基本的中间代码,如变量分配、赋值、分支、调用、类型转换、数学运算等。它还具有常量计算、无用代码消除等优化功能,可添加调试元数据。目前只支持x86_64架构,并且经过在Windows和Linux上的测试。

该作者表示由于年龄所限,项目质量可能会有不足,并邀请大家试用和反馈。他还分享了ygen的GitHub仓库链接和简单的介绍网站。虽然作者年纪尚小,但这个项目已经体现出不错的编程能力,并受到社区成员的祝福和赞赏。

[
https://www.reddit.com/r/rust/comments/1fvfgx0/ygen_release_012/
](
https://www.reddit.com/r/rust/comments/1fvfgx0/ygen_release_012/
)
    


### TITLE

这是一个发布说明,介绍了komorebi窗口管理器0.1.29版本的新特性、bug修复、代码重构、文档更新和依赖库升级等内容。主要新增了komorebi-bar组件、跨显示器行为选项、透明度切换命令、焦点窗口切换命令等功能。同时也修复了一些边界案例的bug,优化了日志输出、加载配置等。此外还更新了主题库、隐私政策文档、CLI命令参考等。总的来说是一个相对重量级的版本升级,增强了窗口管理和可定制性。

[
https://github.com/LGUG2Z/komorebi/releases/tag/v0.1.29
](
https://github.com/LGUG2Z/komorebi/releases/tag/v0.1.29
)
    


### TITLE

这个仓库提供了一个简易的Rust接口,用于在本地使用大型语言模型(LLM)。它支持CPU、CUDA和MacOS,并提供了对常见模型的预设配置。主要特点包括:

1. 自动构建并支持CPU、CUDA和MacOS。
2. 提供预设模型和量化选择。
3. 支持级联提示工作流,用于对因果推理(CoT)和自然语言处理任务。
4. 广泛的配置选项,如采样器参数、重试逻辑、提示缓存、logit偏差和语法。
5. 支持OpenAI、Anthropic、Perplexity和任何兼容OpenAI的API。
6. 提供从概率LLM输出中获取确定性信号的接口。
7. 实现了专门的工作流来约束和控制生成输出,以获得可重复的结果。
8. 提供了一些示例,如基本完成、返回原语、推理和决策。
9. 包含对Nvidia GPU功耗的限制。
10. 使用了llama.cpp进行LLM推理,并支持mistral.rs。

总的来说,这个仓库提供了一个用于本地LLM推理的简单Rust接口,并引入了一种新的级联提示工作流方法来控制和形状化LLM输出,以获得更可靠和可重复的结果。

[
https://github.com/shelbyJenkins/llm_client
](
https://github.com/shelbyJenkins/llm_client
)
    


### TITLE

这是一个GitHub仓库,介绍了一个名为grip-grab (gg)的命令行工具。该工具是一个更轻量级的ripgrep替代品,用于日常搜索用例。文档介绍了安装方法、基准测试结果、用法和示例等内容。

主要内容包括:

1. 安装说明,使用cargo install grip-grab进行安装。

2. 基准测试结果,在小到中等规模的代码库上,gg通常比ripgrep更快。但在大型代码库和多CPU机器上,ripgrep可能更快。

3. 用法说明,gg命令的选项和参数的使用方式。

4. 示例,展示了gg的一些基本用法,如搜索模式、JSON输出、只显示文件名等。

5. 说明gg工具主要基于ripgrep的几个crate,旨在提供一个轻量级的搜索工具,可便于集成到其他程序中。

总的来说,这是一个提供文本搜索功能的命令行工具,相比ripgrep更加轻量,在特定场景下性能更优,可作为日常使用的替代品。

[
https://github.com/alexpasmantier/grip-grab
](
https://github.com/alexpasmantier/grip-grab
)
    


### TITLE

这段内容是关于一个Rust代码片段违反miri(一个Rust内存不安全检测工具)规则的疑问。作者正在编写一个安全但不稳定的队列,学习了堆栈借用但仍不太理解为什么这段代码违反了miri。

代码中的`push`函数用于向队列添加新元素。如果队列为空,就将新元素作为队头;如果不为空,就将新元素追加到队尾。违规的那一行代码试图通过`self.tail`指针修改新分配的`new_tail`节点的`next`字段。

作者知道根据miri规则,不应该通过除`raw_tail`之外的变量修改`new_tail`的值。但他疑惑的是,那行代码是否真的在修改`new_tail`,为什么`self.head = Some(new_tail)`那行没有违反miri。最后作者提供了整个代码的链接以供参考。

[
https://old.reddit.com/r/rust/comments/1fvr8u1/why_this_code_snippet_violates_miri/
](
https://old.reddit.com/r/rust/comments/1fvr8u1/why_this_code_snippet_violates_miri/
)
    


### TITLE

本文介绍了Flawless这个用于Rust的持久执行引擎。持久执行是指即使在发生外部故障的情况下,代码也能一直运行直到完成。Flawless通过将代码中的副作用(如HTTP调用)结果持久化到日志文件中,在发生故障时可以从日志中恢复执行,确保代码的完整执行。

Flawless使用WebAssembly作为编译目标,以确保跨操作系统和CPU架构的确定性。它提供了工具来编程式处理外部故障,如重试中断的请求。

持久执行引擎可以支持长期运行的工作流程,甚至可以无限期运行,在中断时只需恢复执行而不占用CPU和内存资源。它还可以通过Saga模式实现事务行为,跨越多个微服务和数据库进行原子操作。

Flawless旨在简化构建健壮系统的复杂性,开发人员只需关注业务逻辑,错误处理等由引擎自动完成。文章最后提供了Flawless的安装说明以及加入Discord讨论的链接。

[
https://flawless.dev/docs/
](
https://flawless.dev/docs/
)
    


### TITLE

这是一位来自使用高级语言（如TypeScript、Python等）背景的年轻程序员,想要学习Rust语言,但遇到了一些障碍。主要障碍包括:

1. Rust语言涉及许多抽象概念,如内存管理、指针等,对初学者来说难以理解。

2. 虽然他还年轻、有足够时间学习,但这些陌生概念令他有些失去动力。

3. 他对学习Rust仍然保持兴趣,但需要一些建议来克服当前的困难。

总的来说,这位年轻程序员热衷于学习Rust,但面临着来自语言本身抽象概念的挑战,需要获得合适的指导来保持动力并逐步突破障碍。

[
https://old.reddit.com/r/rust/comments/1fwc3n0/im_having_trouble_getting_into_it/
](
https://old.reddit.com/r/rust/comments/1fwc3n0/im_having_trouble_getting_into_it/
)
    


### TITLE

这是关于一个Rust crate quick-builder的介绍和发布公告。quick-builder是一个在编译时进行验证的builder生成工具,同时也允许强制执行运行时不变量。它通过derive宏的方式自动生成builder代码,开发者可以指定需要在编译时和运行时验证的不变量条件。作者希望收集反馈,以完善不变量违反时的错误处理方式,目前是返回Option<T>。该工具的目标是在编译时就捕获尽可能多的错误,同时还能够在运行时进行额外的检查,因此介于纯编译时和纯运行时验证之间。作者认为它具有一定的用武之地,并鼓励大家试用并提供反馈意见。

[
https://old.reddit.com/r/rust/comments/1fw5ij0/quickbuilder_010_a_compiletime_verified_builder/
](
https://old.reddit.com/r/rust/comments/1fw5ij0/quickbuilder_010_a_compiletime_verified_builder/
)
    


### TITLE

这是一篇关于Rust编程语言中ygen项目的更新日志。ygen是一个基于LLVM的IR(中间表示)代码生成器。最新版本(0.1.2)增加了支持phi节点,这是一个重要的里程碑。

phi节点用于优化条件语句等分支情况下的代码,可以避免使用栈变量,从而提高性能。作者给出了一个Rust代码示例,并分别展示了使用phi节点优化前后的LLVM IR代码,说明了phi节点的作用和优势。

接下来,作者解释了在ygen中添加phi节点支持意味着可以实现类似LLVM的mem2reg优化pass,将store、load指令优化为phi节点,从而生成更高效的代码。

最后,作者提供了ygen项目的GitHub链接,并表达了对读者反馈的欢迎态度。

[
https://old.reddit.com/r/rust/comments/1fw463d/ygen_now_supports_phi/
](
https://old.reddit.com/r/rust/comments/1fw463d/ygen_now_supports_phi/
)
    


### TITLE

这位用户正在尝试使用Rust的clap包来解析一系列命令,但是遇到了一些问题。他想解析的命令示例包括像"toggle <FEATURE NAME> [on|off]"、"login name <USER NAME> key <digits>"、"setup [init|position <POSITION STRING>]"和"apply <STRING1> [STRING2 ... STRINGN]"这样的格式。

他表示对于简单的命令,使用clap的一些功能如ValueEnum可以解析得很好。对于复杂一些的命令,他通过嵌套子命令的方式也设法搭建了一个模拟解决方案。

但是他遇到的主要问题是,clap似乎无法解析单个词作为参数的命令。比如他可以解析"toggle Debug on",但无法解析"toggle Debug Info on"。

他希望能找到解决这个问题的方法,让clap也能妥善处理单个词作为参数的命令解析场景。

[
https://old.reddit.com/r/rust/comments/1fwgihn/using_clap_for_entirely_the_wrong_purpose/
](
https://old.reddit.com/r/rust/comments/1fwgihn/using_clap_for_entirely_the_wrong_purpose/
)
    


### TITLE

该内容是一篇公告,介绍了作者之前在 Rust 论坛发布过的关于使用 egui 库构建状态栏的开发视频系列的最新进展。作者宣布,状态栏已经作为 komorebi v0.1.29 版本的一部分发布。

如果有兴趣看看用 egui 编写的状态栏的样子(自定义字体、热重载、自定义主题等),可以观看深入探讨状态栏和配置系统的视频。作者还分享了一个快速入门视频,5分钟内就能运行起来,方便有兴趣的人试用该状态栏。

[
https://old.reddit.com/r/rust/comments/1fwdge9/komorebi_v0129_status_bar_written_in_egui/
](
https://old.reddit.com/r/rust/comments/1fwdge9/komorebi_v0129_status_bar_written_in_egui/
)
    


### TITLE

这篇文章总结了gitoxide项目最近的一些重大进展和成就。主要包括:

1. 实现了计算两个或多个提交的合并基础的功能,提高了性能。

2. 实现了高性能的树编辑功能,为即将到来的树合并功能做准备。

3. 引入了内存中对象写入功能,避免频繁的磁盘IO。 

4. 正在进行blob合并的工作,为多根树合并做准备。

5. 发现并修复了一个安全漏洞。

6. 作者参加了GitMerge 2024大会,与Git社区交流,分享了gitoxide的发展。

7. 获得了libgit2维护者的捐助计划,为将来申请资金做准备。  

8. 持续集成得到优化,运行更快。

9. "blame"功能逐步完善。

10. 虽然Cargo集成还没有进展,但重置操作的改进为将来做好了基础。

总的来说,gitoxide项目在功能、性能、社区互动等方面都取得了长足的进展,为Git生态发展做出了贡献。

[
https://github.com/Byron/gitoxide/discussions/1614
](
https://github.com/Byron/gitoxide/discussions/1614
)
    


### TITLE

该内容介绍了一个名为 llm_client 的 Rust 库,旨在通过大型语言模型(LLM)来代替传统的控制流(如 if 语句)。这个库可通过 crates.io 安装,并自动为 Windows、Linux 和 Mac 进行构建,支持使用或不使用 CUDA。它类似于 Rust 版的 Ollama,但重点在于使用 LLM 来替代传统控制流。

该库提供了一个 `reason()` 方法,可以设置指令并返回一个原始值(整数、布尔值或自定义字符串)。作者举了一个示例,通过给定的问题得到一个整数作为输出。使用较小的模型和 GPU,该过程大约需要一秒钟。因此,该库旨在用于智能体行为和自然语言处理任务。

该库还可以根据可用的 VRAM 估计所选模型的最大 quant,或者指定本地模型、设备配置,甚至同时运行多个模型。作者的目标是使该库尽可能易于使用,因此从 crates 安装是一个重要目标。为了在 Mac 上测试,作者不得不购买一台 MacBook。最后提供了该库的 GitHub 链接。

[
https://old.reddit.com/r/rust/comments/1fwbdxs/llm_client005_the_easiest_way_to_integrate/
](
https://old.reddit.com/r/rust/comments/1fwbdxs/llm_client005_the_easiest_way_to_integrate/
)
    


### TITLE

这个帖子介绍了一个名为 Grip-Grab 的小项目,它类似于著名的代码搜索工具 ripgrep。作者邀请大家去试用并体验这个新工具。贴文中包含了 Grip-Grab 项目的 GitHub 链接,可以直接访问并获取更多详细信息。该项目声称比 ripgrep 更快、更轻量级,是一个值得关注的代码搜索替代品。

[
https://old.reddit.com/r/rust/comments/1fvzfnb/gg_a_fast_more_lightweight_ripgrep_alternative/
](
https://old.reddit.com/r/rust/comments/1fvzfnb/gg_a_fast_more_lightweight_ripgrep_alternative/
)
    


### TITLE

根据这篇博客文章,主要内容可以总结如下:

2025年第三季度是ZLUDA重建的目标期限,旨在将"新"ZLUDA的状态恢复到和回滚前相似的水平。这里的"相似状态"是一个比较主观的概念,没有精确的标准,但是复杂程度相当的应用程序应该可以同样良好地运行。由于新的优先级,并非所有回滚前的应用程序都会再次得到支持。

所以2025年被称为ZLUDA的"重建之年",是将ZLUDA重新建设到一个可以满足复杂应用需求的阶段。这需要花费一年的时间,重点是开发出与之前相当的功能和复杂性,但具体支持哪些应用会根据新的优先级而定。

[
https://vosen.github.io/ZLUDA/blog/zludas-third-life/
](
https://vosen.github.io/ZLUDA/blog/zludas-third-life/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

