【Rust日报】2024-11-09


### TITLE

本文分享了印度尼西亚CIMB Niaga银行在提供优质银行服务体验的过程中,如何将其核心微服务从Java迁移到Rust编程语言。文章强调了以客户为中心的理念对银行决策的指导作用。由于用户群不断增长,Java微服务在高峰时段遇到了性能瓶颈问题,影响了用户体验。经评估,团队决定将核心微服务迁移到Rust,以提高系统性能、安全性和可扩展性。

选择Rust的主要原因包括:1)内存安全性高,能在编译时消除常见bug;2)性能优秀,能很好地处理并发操作;3)安全性强,能有效防范安全漏洞。文章还分享了团队採取分阶段无缝迁移策略的过程,确保了对客户服务零中断。最后,作者总结了Rust在提高性能、确保安全性和提升开发效率方面的积极影响,有助于银行持续为客户提供卓越的服务体验。

[https://medium.com/cimb-niaga-engineering/delivering-superior-banking-experiences-bc7ca491eae5
](https://medium.com/cimb-niaga-engineering/delivering-superior-banking-experiences-bc7ca491eae5
)
    


### TITLE

这是一个关于Rust编程语言的Reddit帖子。原帖者是一名机器学习库开发者,之前一直在使用Gurobi优化库进行线性/二次约束优化,但现在由于不再是学生身份无法免费使用Gurobi。他在寻求Rust生态系统中替代Gurobi的开源优化库。

其他用户给出了一些建议,如clarabel用于凸优化、OSQP处理二次规划、HiGHS用于线性/整数规划等。有人询问了原帖者需要解决的具体优化问题类型,以更好地推荐合适的库。其他被推荐的库还有good_lp、optirustic等。总的来说,这是一个关于在Rust中寻找优化求解库替代Gurobi的讨论。

[
https://www.reddit.com/r/rust/comments/1g7wu5b/good_optimization_libraries/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
](
https://www.reddit.com/r/rust/comments/1g7wu5b/good_optimization_libraries/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
)
    


### TITLE

这是一个用于提升算法研究的 Rust 库 MiniBoosts 的介绍。该库实现了多种提升算法,如 AdaBoost、LPBoost、ERLPBoost 等,以及一些弱学习器,如决策树、回归树等。研究人员可以使用该库来比较自己的新算法与现有算法的性能。该库提供了两个主要特性:Booster 和 WeakLearner,用户只需实现相应的 trait 即可引入新的提升算法或弱学习器。该库还支持 Gurobi 求解器的使用。文档中给出了如何使用该库的示例代码,包括读取数据、初始化提升器、构建弱学习器、运行算法、获取预测结果等步骤。总的来说,这个库为提升算法研究人员提供了一个统一的实验平台。

[
https://github.com/rmitsuboshi/miniboosts
](
https://github.com/rmitsuboshi/miniboosts
)
    


### TITLE

总结如下:

这是一个使用Rust编写的小型web服务器项目,名为Cblt。它的主要功能包括:

1. 使用KDL配置文件进行配置
2. 提供文件服务器功能,可从指定目录服务静态文件
3. 反向代理功能,可将请求代理到其他服务器
4. 支持HTTPS

该项目提供了Cargo和Docker两种运行方式。作者还进行了一些基准测试,使用ab工具模拟并发请求情况下,与Nginx和Caddy的性能对比。测试结果显示,在一些并发场景下,Cblt的性能表现相对更好。

该项目的配置文件示例展示了文件服务器、反向代理和HTTPS的配置方法。总的来说,这是一个简约但功能实用的小型Web服务器实现。

[
https://github.com/evgenyigumnov/cblt
](
https://github.com/evgenyigumnov/cblt
)
    


### TITLE

以下是内容的中文总结:

这是一篇介绍以色列2024年政治变化的文章。随着哈马斯恐怖分子袭击以及随后的钢铁之剑战争爆发,以色列执政联盟受到了广泛批评。文章分析了中右翼政治力量的变化,包括贝尼·甘茨、加尼·艾森科特和加德尔·萨阿尔等人的动向。同时也谈到了左翼阵营的新面孔,如亚伊尔·戈兰和阿维·谢克德有望加入工党。文章最后还简要介绍了如何选择合适的犹太纯净旅游团。

[
http://www.zeustech.net/
](
http://www.zeustech.net/
)
    


### TITLE

该仓库包含了一个名为 WIDE (websocket ide) 的轻量级代码服务器,它基于 Rust 构建,允许您构建自定义的基于 WebSocket 的 IDE。它具有以下主要特性:

1. 文件操作(读取/写入/监视)
2. 语言服务器协议支持(目前仅支持 Rust,提供自动补全、悬停提示、跳转定义等功能)
3. 实时 WebSocket 通信
4. 事件批处理以提高性能

该项目支持客户端与服务器之间的双向通信,客户端可以发送如打开文件、保存文件、获取代码补全等消息,服务器则响应这些请求并返回相应的结果。

作者正在积极开发该项目,欢迎社区贡献新特性、优化性能、添加测试、改进文档等。该项目旨在为基于 Web 的编码环境、自托管解决方案或自定义 IDE 实现提供一种可靠高效的解决方案。

[
https://github.com/JaLnYn/websocket-ide
](
https://github.com/JaLnYn/websocket-ide
)
    


### TITLE

这是一个开源的Rust框架项目,名为violin_rs,旨在为Minecraft Bedrock版本的附加内容开发提供支持。该项目托管在GitHub上,由bedrock-crustaceans组织维护。根据README文件,该框架使用Rust语言构建,为Minecraft基岩版游戏提供了附加内容的开发能力。项目包含了源代码、构建脚本、配置文件等多个目录和文件。通过这个框架,开发者可以使用Rust语言开发Minecraft Bedrock版本的mod、数据包等附加功能。

[
https://github.com/bedrock-crustaceans/violin_rs
](
https://github.com/bedrock-crustaceans/violin_rs
)
    


### TITLE

这段回答解释了Rust编程语言中"static"的含义。它指出"static"不是以代码行数来定义的,而是一个更低级的概念。当一个引用被标记为"static"时,它所引用的目标将存在于整个程序运行的生命周期中,内存也会一直保持有效直到程序终止。

回答还解释了main函数的一些特性。虽然main函数大多被当作普通函数对待,但在main返回后程序并不一定立即终止。在Unix系统上,可以使用atexit注册退出处理程序,这些程序会在main返回后执行。Rust中,main函数可以返回任何实现了Termination trait的类型,该trait有一个方法会在main返回后被调用。其他线程可能在main返回后仍在运行,操作系统会在程序退出时杀死它们。

总之,这段回答阐述了"static"的含义,并解释了main函数与程序生命周期的一些关系。

[
https://old.reddit.com/u/plugwash
](
https://old.reddit.com/u/plugwash
)
    


### TITLE

根据这篇文章,印度尼西亚主要银行CIMB Niaga从Java/Kotlin转向使用Rust编程语言来构建他们的核心银行系统。他们采用Rust的主要原因是Rust能够提供更好的内存安全性、并发性和性能。文章中还展示了CIMB Niaga工程团队使用Rust开发的一些关键组件,如核心银行服务、实时交易处理等。通过使用Rust,该银行能够提高系统的稳定性、安全性和效率,从而为客户提供更出色的银行体验。这表明Rust不仅在基础设施和系统编程领域受到青睐,在传统行业如银行业也开始获得认可和应用。

[
https://old.reddit.com/r/rust/comments/1gmgyuf/major_indonesian_bank_cimb_niaga_transition_from/
](
https://old.reddit.com/r/rust/comments/1gmgyuf/major_indonesian_bank_cimb_niaga_transition_from/
)
    


### TITLE

以下是对该帖子内容的中文总结:

您好,Rust爱好者们!

感谢整个Rust社区的帮助,我终于发布了一个小型机器学习库MiniBoosts(crates.io链接在这里)。

(我之前发过一个求助帖,得到了大家的热心帮助。我爱你们,Rust爱好者们!)

这只是一个业余爱好项目,但我欢迎任何评论和建议。

[
https://old.reddit.com/r/rust/comments/1gn7i6m/miniboosts_v035/
](
https://old.reddit.com/r/rust/comments/1gn7i6m/miniboosts_v035/
)
    


### TITLE

这个帖子讨论了在Rust语言中使用多单词生命周期名称的优缺点。作者提供了一个例子,在一个结构体中使用了描述性的生命周期名称'descriptive_name'。他询问在大型项目中是否有人也使用过这种描述性的生命周期名称。作者自己尽量保持单词命名,但有时觉得使用两个单词或缩写单词作为生命周期名称也不错。他想知道其他人对于非单字符生命周期名称的看法。

总的来说,这是一个关于命名风格的讨论,探讨在Rust中使用可读性更强的多单词生命周期名称是否是一个好的做法,以及在实践中是否被广泛采用。

[
https://old.reddit.com/r/rust/comments/1gn5x4q/is_it_bad_to_use_multi_word_lifetime_names_in/
](
https://old.reddit.com/r/rust/comments/1gn5x4q/is_it_bad_to_use_multi_word_lifetime_names_in/
)
    


### TITLE

这是一个关于命令行工具UI设计的讨论。原帖子作者正在用Rust开发一个命令行工具,并想为它设计一个漂亮的文本用户界面(TUI),以更友好的方式显示日志、错误和消息。但作者注意到,大多数著名的命令行工具都没有色彩丰富、美观的UI,只有简单的文本提示。作者想知道,这是出于某种设计决策,还是开发人员懒于编写更多代码的原因。总的来说,这是一个关于命令行工具UI设计哲学的探讨。

[
https://old.reddit.com/r/rust/comments/1gndvkb/why_great_cli_tools_do_not_have_beautiful_ui/
](
https://old.reddit.com/r/rust/comments/1gndvkb/why_great_cli_tools_do_not_have_beautiful_ui/
)
    


### TITLE

Jaws 是一个用 Rust 编写的 JavaScript 到 WebAssembly 编译器。它类似于 porffor,都能生成独立的 WASM 二进制文件,无需解释器即可执行。但 Jaws 采用了不同的实现方式。这是一个实验性工具,目前还不适合用于生产环境。许多语言特性和内置类型仍然缺失或不完整。不过,最终目标是支持 JavaScript 100% 的特性。

项目的动机是为了支持在 WebAssembly 场景中运行脚本语言,避免使用解释器导致二进制文件过大和内存使用增加。作者认为,利用现代 WASM 提案,可以在无需编译解释器的情况下实现 JavaScript 100% 的特性,因为 WASM 运行时本身就是解释器。

目前 Jaws 已经实现了闭包、作用域、try/catch、Promise API 和 async(但还没有 await)等功能。它还支持变量声明、赋值、while 循环、字符串、数字、布尔值、数组和对象字面量等。

由于使用了一些相对较新的 WASM 提案,如 WASM GC、异常处理和 WASIp2,生成的二进制文件暂时无法在所有运行时中可移植运行。作者目前使用 V8(通过 Chromium 或 Node)及一个 JavaScript WASIp2 polyfill 进行开发,等待运行时支持标准化提案后,这些二进制文件就可以在任何支持 WASM GC、异常处理和 WASIp2 API 的运行时上运行。

接下来作者计划实现生成器和 await 关键字支持,然后逐步完善语法、内置类型和 API,最终实现对 JavaScript 100% 特性的支持。

该项目主要通过将 JavaScript 语法翻译为 WASM 指令,利用 WASM GC、异常处理和尾调用优化提案新增的指令来工作。

[
https://github.com/drogus/jaws
](
https://github.com/drogus/jaws
)
    


### TITLE

总结如下:

作者分享了他学习新编程语言Rust的经验过程。首先通过阅读教程来了解语法、惯用法、设计理念等基础知识。然后通过编写一些小项目来实践使用标准库和流行框架。为了更深入地学习,作者没有像往常一样编写宠物项目,而是尝试自己实现一些库,比如ORM、JSON处理、Actor模型、MVC Web框架、日志等。尽管这些库可能无人问津,但有助于更好地理解Rust语言。令人惊讶的是,作者竟然成功编写了一个Web服务器,他认为这得益于Rust作为系统级编程语言,对性能优化是被默许的。

作者发现Rust语言缺少像Nginx、Lighttpd、Caddy、HAProxy、Apache、Tomcat、Jetty等成熟的Web服务器,只有一些Web框架如Actix、Axum、Rocket、Hyper等。因此作者决定自己实现一个Rust版的Web服务器cblt。

cblt支持使用KDL(KDL Document Language)格式的配置文件,可配置静态文件服务器、反向代理、TLS等。它可通过Cargo或Docker两种方式运行。

作者对cblt进行了基准测试,使用Apache Benchmark(ab)工具测试了300次请求,100并发连接,加载一个5MB的静态图片文件。结果显示,在50%、75%和100%的请求响应时间,cblt性能优于Caddy,但略逊于Nginx。最后附上了基准测试的输出结果。

[
https://old.reddit.com/r/rust/comments/1gncdq4/cblt_a_safe_fast_and_minimalistic_web_server_in/
](
https://old.reddit.com/r/rust/comments/1gncdq4/cblt_a_safe_fast_and_minimalistic_web_server_in/
)
    


### TITLE

该项目是一个命令行工具和库,用于将Markdown内容转换为预设样式的PDF文档。它支持从文件或字符串直接读取Markdown,为用户和开发人员提供了灵活性。该项目包括二进制文件和库:

二进制文件(cli):命令行界面,使用核心库轻松将Markdown转换为PDF。
库(lib):可以集成到Rust项目中,用于解析Markdown或以编程方式生成PDF文档。

该项目目前正在积极开发中,将持续改进和添加新功能。

目前还没有跨平台简化的官方安装方法,您需要克隆存储库并使用Cargo自行构建项目。

该工具支持通过TOML配置文件自定义生成PDF的样式选项。您可以在主目录中创建mdprc.toml文件,并根据自己的偏好修改值。

该工具支持指定Markdown文件路径、直接提供Markdown内容或设置输出PDF路径等选项。您可以使用示例命令将Markdown文件或字符串转换为PDF。

总的来说,该项目提供了一种灵活的方式来将Markdown内容转换为PDF文档,并支持自定义样式。

[
https://github.com/theiskaa/mdp
](
https://github.com/theiskaa/mdp
)
    


### TITLE

这个Github存储库介绍了Actuate,一个高性能的响应式用户界面框架,用于Rust语言。该框架提供了一个通用库,允许你使用声明式、借用检查器友好的语法来定义UI。它受到了Xilem的启发,使用类似的类型安全反应性方法。与Xilem不同的是,Actuate引入了作用域(scope)的概念,组件将其状态存储在自己的作用域中,当该作用域发生更新时,就会重新渲染该组件。Actuate的状态管理思路借鉴了React和Dioxus。该框架旨在提供高性能、安全和简单的响应式UI开发体验。

[
https://github.com/actuate-rs/actuate
](
https://github.com/actuate-rs/actuate
)
    


### TITLE

该文档探讨了在 C++ 和 Rust 编程语言之间建立互操作性的挑战和重要性。主要内容包括:

1. 动机 - 内存安全语言如 Rust 可以避免70%的安全漏洞,因此有压力迁移到这些语言。政府要求大型供应商提供转向内存安全语言的路线图。

2. C 互操作性 - 由于 C 语言相对简单,实现 C 互操作相对容易。

3. C++ 互操作性 - C++ 语言复杂特性如函数重载、模板、隐式转换等,使得实现 C++ 互操作性极其困难。

4. 互操作分类 - 探讨了从 C++ 调用 Rust、从 Rust 调用 C++ 以及跨语言使用 std::string 等情况。 

5. Rust 互操作覆盖范围 - 讨论了 Rust 支持的互操作范围及 Swift 的互操作权衡。

6. 异常处理 - C++、Rust 在异常展开和 RTTI 方面的差异。

7. 安全交付 - 通过扩展 C++ 和 Rust 来减少互操作摩擦,从而促进向 Rust 等安全语言的转移,提高软件质量。

总的来说,文档强调了增强 C++ 和 Rust 互操作性的重要性,以加速向内存安全语言的转移。

[
https://www.circle-lang.org/interop.html
](
https://www.circle-lang.org/interop.html
)
    


### TITLE

以下是对给定内容的中文总结:

Mergiraf是一个用于解决Git合并冲突的工具。它可以理解文件中的树状结构,从而更好地协调合并双方的需求。Mergiraf支持声明式地教授新语言,而不需要使用命令式的方式。

它可以替代Git默认的合并启发式算法,增强git merge、revert、rebase、cherry-pick等命令。在遇到冲突时,Mergiraf会尽力保留冲突标记,避免草率解决冲突。如果自行解决了所有冲突,它会提示你使用mergiraf review命令审查解决方案。

Mergiraf的目标是:不回避冲突、提供语法感知的合并算法、保持足够快以便交互使用,并在适当时使用基于行的合并方式。它致力于在保持快速的同时,尽可能协调解决合并冲突,带来和平与融洽。

[
https://mergiraf.org/introduction.html
](
https://mergiraf.org/introduction.html
)
    


### TITLE

这是一个基于Rust编写的IDE服务器代码,它允许你构建任何你想要的前端界面,如Web、终端、VR或AI等。该服务器通过一个简单的WebSocket API处理所有繁重的工作,如文件操作、LSP等。

目前,它支持基本的文件操作、LSP支持(如自动补全、悬停提示、定义跳转,仅测试了"rust-analyzer")和实时文件监视。作者承认自己在IDE架构方面是初学者,希望能获得关于代码结构、性能瓶颈、缺失功能、安全考虑等方面的反馈意见。

作者邀请大家一同构建酷炫的前端界面,提交bug或缺失功能的issue,分享想法,并Star该repo以跟踪更新。作者也期待看到大家创造性的界面设计,并欢迎对当前的"websocket-ide"项目名提出建议。

[
https://old.reddit.com/r/rust/comments/1gnkws1/buildyourown_ide/
](
https://old.reddit.com/r/rust/comments/1gnkws1/buildyourown_ide/
)
    


### TITLE

这个帖子介绍了一个名为Violin.rs的Rust编写的Minecraft Mod工具。它允许用户使用Rust轻松创建Minecraft Bedrock版Mod。贴子中包含了一个示例代码,展示了如何使用Violin.rs用几行代码创建64种不同的剑。它还提供了一张游戏中剑的截图。该工具由bedrock-crustaceans团队开发,他们计划为工具添加更多功能,比如支持新的方块、生物和内部脚本API。该团队还有兴趣开发一个JavaScript/TypeScript API,以进一步提高工具的可访问性。总的来说,这个工具旨在让用户使用Rust更轻松地为Minecraft Bedrock版开发Mod。

[
https://old.reddit.com/r/rust/comments/1gnhfdr/minecraft_mods_in_rust/
](
https://old.reddit.com/r/rust/comments/1gnhfdr/minecraft_mods_in_rust/
)
    


### TITLE

这个帖子讨论了Rust编程语言标准库中处理一种极端情况的代码。具体来说,当一个Arc(Rust中的引用计数智能指针)被克隆超过最大可能的引用计数数值(92的63次方减1)时,Rust标准库的代码会中止程序执行。

虽然实际达到这种级别的情况几乎是不可能的(即使在多核CPU上并行运行数年也难以完成),但作者对Rust团队处理这种极端情况的代码表示惊叹。他打趣说,在运行了10年的同一分支后,分支预测器突然为最后一次迭代而刷新流水线,会感到多么震惊。

更疯狂的是,理论上如果能在5个时钟周期内完成92的63次方减1次克隆,该函数仍然可能触发未定义行为。

最后,一位用户指出,这种情况在32位系统上实现会容易得多,可能只需几分钟。但即使在16位系统上,触发未定义行为的可能性也很小。

总的来说,这个帖子关注了Rust标准库为了追求健壮性而处理一种极端边缘情况的代码,展现了Rust追求零成本抽象的决心。

[
https://old.reddit.com/r/rust/comments/1gn74mu/rust_std_we_abort_because_such_a_program_is/
](
https://old.reddit.com/r/rust/comments/1gn74mu/rust_std_we_abort_because_such_a_program_is/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

