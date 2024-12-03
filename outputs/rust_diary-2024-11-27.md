【Rust日报】2024-11-27


### TITLE

这是一个使用Rust语言和Diesel ORM框架开发的简单Web应用程序的GitHub仓库。该项目名为"requestlist"。它包含以下主要文件和目录:

- migrations/: 用于存放数据库迁移文件
- src/: 存放Rust源代码
- .env_example: 环境变量配置示例文件  
- .gitignore: Git版本控制忽略文件
- Cargo.toml: Rust包管理配置文件
- diesel.toml: Diesel ORM配置文件

根据README文件,这是一个测试Rust和Diesel的项目。仓库目前处于初始阶段,但提供了基本的代码结构和配置文件,以便在此基础上进一步开发Web应用程序。

[https://github.com/bmillham/requestlist
](https://github.com/bmillham/requestlist
)
    


### TITLE

molpipx是一个基于JAX和Rust的开源库,实现了可微分的置换不变多项式(Permutationally Invariant Polynomial,PIP)模型。该库提供了将MSA文件转换为JAX和Rust版本的功能,支持GPU加速。molpipx包含了三种主要的回归模型:线性回归、神经网络和高斯过程,利用自动微分引擎JAX(Python版本)和Enzyme-AD(Rust版本)来模拟化学系统。该库简单易用,为不同分子系统提供了个性化元素。除了提供模型代码,还有教程讲解如何使用不同回归方法、训练模型并进行预测。molpipx旨在成为研究PIP模型及其在化学和材料领域应用的有力工具。

[
https://github.com/ChemAI-Lab/molpipx/
](
https://github.com/ChemAI-Lab/molpipx/
)
    


### TITLE

以下是对该 README 文件的中文总结:

jnv 是一个用于浏览和编辑 JSON 数据的交互式 JSON 查看器和 jq 过滤器编辑器。它受到了 jid 和 jiq 项目的启发。

主要特性包括:

1. 交互式 JSON 查看器和 jq 过滤器编辑器
2. JSON 语法高亮
3. 使用内置的 jaq 直接应用 jq 过滤器,无需用户自行安装 jq

从 v0.3.0 版本开始,该项目从原来使用 libjq Rust 绑定切换到使用 jaq,这是 jq 的克隆版本。这一改变避免了管理 C 相关依赖(如 autoconf 等外部工具)的需要,简化了构建过程。但目前 jaq 还不支持部分过滤器。

jnv 支持多种输入格式,包括文件、标准输入、JSON 或多个 JSON 结构。它还支持过滤器自动补全和提示信息显示。

该 README 还包含了 jnv 的安装方式、示例用法、键盘快捷键映射以及命令行选项的说明。总的来说,这是一个功能丰富的 JSON 数据浏览和编辑工具。

[
https://github.com/ynqa/jnv
](
https://github.com/ynqa/jnv
)
    


### TITLE

根据给出的GitHub链接内容,主要内容如下:

1. jaq是jq (一个JSON数据处理工具)的克隆版本,发音为/ʒaːk/,像Jacques。

2. jaq的目标是:
   - 正确性:提供更正确、可预测的jq实现,同时保持大部分兼容性。
   - 性能:最初创建jaq是因为受到jq 1.6版本长启动时间(约50ms)的困扰,jaq在许多其他基准测试中比jq更快。
   - 简单性:jaq旨在保持简单小巧的实现,以减少Bug和促进贡献。

3. 提供了jaq的安装方式,包括二进制包、Homebrew、Scoop等。

4. 给出了一些jaq的使用示例,展示其语法和功能。

5. 提供了一个jaq、jq和gojq在多个基准测试中的性能评估和结果表格。jaq在大部分基准测试中性能优于jq和gojq。

总的来说,这是一个介绍jaq工具的自述文件,重点是jaq的目标、安装使用方法和与jq、gojq的性能对比。

[
https://github.com/01mf02/jaq?tab=readme-ov-file#performance
](
https://github.com/01mf02/jaq?tab=readme-ov-file#performance
)
    


### TITLE

这是一个名为jqjq的GitHub仓库,它是一个用纯jq语言实现的jq解释器。该项目的主要目的是展示jq语言的表现力、能力和简洁性。

该仓库提供了一个jqjq的包装器脚本,可以直接运行jq代码或交互式REPL。它还支持使用不同的jq实现,如gojq或jaq。除了运行普通的jq代码外,jqjq还添加了一些特殊功能,如eval函数可以动态执行字符串中的jq代码。

该仓库包含了一些示例,展示了jq语言的各种功能,如字面量、运算符、绑定、解构、路径查询、条件语句、循环、模块等。它还列出了jqjq当前支持的jq语法和函数。

该仓库还提供了测试用例,可以使用不同的jq实现运行测试。总的来说,这个项目旨在探索jq语言的深度,并提供了一个可运行的jq解释器实现。

[
https://github.com/wader/jqjq
](
https://github.com/wader/jqjq
)
    


### TITLE

以下是对给定内容的总结:

jq是一个轻量级且灵活的命令行JSON处理器,类似于sed、awk、grep等工具,用于处理JSON数据。它用可移植的C语言编写,没有运行时依赖,允许你轻松地切片、过滤、映射和转换结构化数据。

文档提供了jq的官方文档链接jqlang.github.io/jq,并且可以在jqplay.org在线尝试jq。该页面还提供了从GitHub发布页面下载预构建二进制文件、使用Docker镜像以及从源代码构建的说明。

还介绍了社区支持渠道,如Stack Overflow的jq标签、Discord聊天社区,以及Wiki页面提供的高级主题探讨。

最后,指出jq使用MIT许可证发布,文档使用Creative Commons CC BY 3.0许可,并且使用了一些开源C库decNumber,后者使用ICU许可证。

[
https://github.com/jqlang/jq
](
https://github.com/jqlang/jq
)
    


### TITLE

总结如下:

该项目jaq是一个用Rust编写的JSON处理工具,旨在模仿并提供与jq类似的语法和功能。它的主要目标包括:

1. 正确性 - 提供更加正确和可预测的jq实现,同时保持大部分兼容性。

2. 性能 - 相比jq,jaq在启动时间和许多基准测试中表现更快。

3.简单性 - 追求简单紧凑的实现,以减少错误和方便贡献。

该项目提供了Linux、macOS和Windows的二进制版本,也可通过包管理器如Homebrew和Scoop进行安装,或者从源代码编译。

文中还列举了几个jaq用法示例,并提供了一个性能评估,将jaq与jq和gojq在多个基准测试中的表现进行了对比。总的来说,jaq在很多场景下表现出了更快的速度。

[
https://github.com/01mf02/jaq
](
https://github.com/01mf02/jaq
)
    


### TITLE

以下是对该README的中文总结:

Motor OS是一个专为云环境虚拟化工作负载而设计的简单、快速和安全的操作系统。它完全使用Rust语言构建,支持x64 KVM虚拟机,可在Qemu、Cloud Hypervisor或Alioth VMM上运行。

与Linux相比,Motor OS针对虚拟化环境进行了优化,避免了诸如重复块缓存、重复页表遍历等低效情况。它采用微内核架构,旨在比Linux更简单、更安全,同时提供相当或更好的性能和效率。

Motor OS目前正在积极开发中,并不适合用于敏感工作负载,但可用于试验、研究等目的。它已经具备了基本的引导、内存管理、I/O子系统、网络、文件系统等功能,可运行一些Rust程序。但也还有很多功能有待完善,如异步I/O、更多网络协议支持等。

总的来说,Motor OS作为一个面向虚拟化云工作负载的全新操作系统,具有巨大的潜力,值得关注其未来发展。

[
https://github.com/moturus/motor-os
](
https://github.com/moturus/motor-os
)
    


### TITLE

以下是对该内容的中文总结:

小事放大法则指出,在组织中,人们往往会过度关注trivial(不重要的)问题,而忽视真正重大的事情。作者帕金森以一个委员会关于核电站规划的例子说明了这种现象:该委员会花费大量时间讨论员工自行车棚的细节,而忽视了核电站设计本身这个更加重要和复杂的问题。

这种现象的原因可能在于,人们更容易理解小事,因此会过多地参与讨论;而对于大事项,由于复杂难懂,人们往往投入不足。研究也证实了这种现象的存在:人们倾向于在小决策上花费过多时间,而在重大决策上花费不足。

小事放大现象不仅存在于管理领域,在软件开发等其他领域也有体现。由此衍生出了"自行车棚效应"等相关表述。总的来说,这一法则揭示了人们在决策时常常会产生倾斜和偏差,需要加以警惕和纠正。

[
https://en.wikipedia.org/wiki/Law_of_triviality
](
https://en.wikipedia.org/wiki/Law_of_triviality
)
    


### TITLE

这位开发者是 Rust 新手,但过去主要使用 Python 编程。他正在将一系列旧的 Python 脚本转换为使用 Rust 和 Diesel 库访问现有 MySQL 数据库。他能够实现基本功能,但在处理枚举(Enum)和日期时间(DateTime)这两种 MySQL 类型时遇到了困难,无法找到合适的示例代码。为了简化问题,他在 GitHub 上分享了项目代码,目前只关注一个数据库表。一旦解决了枚举和日期时间的问题,他就会添加其他表,进行查询操作,并使用 Iced 库构建应用程序,最终用 Rust/Iced 取代现有的基于 Python 的网站。他向社区求助,希望能获得相应的指导。

[
https://old.reddit.com/r/rust/comments/1h1kzdp/rust_diesel_and_mysql_types_help/
](
https://old.reddit.com/r/rust/comments/1h1kzdp/rust_diesel_and_mysql_types_help/
)
    


### TITLE

Pipelight是一个基于Rust编写的命令行工具,用于自动化重复性任务。它允许使用TypeScript、TOML、HCL和YAML定义管道,并支持通过Git钩子、文件更改等方式触发执行。Pipelight提供了方便的对象API和助手API来定义管道步骤,还支持各种配置语言。它提供详细的日志输出,便于监控管道状态和子进程输出。Pipelight可在MacOS和Windows WSL上安装,具有初始化和运行示例管道的命令。该项目处于持续开发中,计划支持更多配置语言、重构JavaScript辅助功能为Rust WebAssembly、添加TUI界面等。Pipelight遵循GNU GPLv2许可协议,项目所有者欢迎通过Discord、Telegram或电子邮件提供支持和改进建议。

[
https://github.com/pipelight/pipelight
](
https://github.com/pipelight/pipelight
)
    


### TITLE

对不起,您提供的链接似乎有问题,无法访问该网页内容。该链接返回了一个404错误,表示请求的文件未找到。没有可供总结的具体内容。如果您可以提供一个有效的链接,我会很乐意帮您总结相关内容。

[
https://this-week-in-rust.org/blog/2024/11/20/this-week-in-rust-575/
](
https://this-week-in-rust.org/blog/2024/11/20/this-week-in-rust-575/
)
    


### TITLE

这是一个关于学习Vulkan和Rust的问题。作者是一位熟练的C++程序员,目前在业界从事C++开发工作。他对Rust语言的优势很感兴趣,希望能够学习Rust,同时也想在学习Rust的过程中探索计算机图形学等他之前从未接触过的领域。

他询问是否应该同时学习Vulkan和Rust,如果是,有哪些推荐的crate、教程和视频资源。另一个选择是先学习Rust,之后再用C++学习Vulkan。他希望大家能就这两种路径提供一些建议。

[
https://old.reddit.com/r/rust/comments/1h1jj8d/learning_vulkan_and_rust/
](
https://old.reddit.com/r/rust/comments/1h1jj8d/learning_vulkan_and_rust/
)
    


### TITLE

这是一个关于Rust语言中bindgen工具在不同操作系统下对长整型数据类型处理不一致的问题。

具体来说,作者从Java类生成的C头文件中包含64位的长整型常量。在Linux系统下,bindgen可以正确地将这些常量表示为64位的Rust整型;但在Windows系统下,bindgen却将它们表示为32位整型,因为Windows中long类型就是32位的。

与此同时,bindgen对函数签名中的长整型却正确地使用了64位整型。作者想知道如何让bindgen在Windows下也能用64位整型来表示这些从C头文件生成的64位常量,因为bindgen目前使用了std::os::raw::c_ulong类型,该类型在Windows下是32位的。

总之,这是一个bindgen工具在跨平台情况下对数据长度类型处理不一致的问题,需要解决方案来统一不同系统下的行为。

[
https://old.reddit.com/r/rust/comments/1h1g8nr/is_bindgen_behaving_correctly_in_this_case/
](
https://old.reddit.com/r/rust/comments/1h1g8nr/is_bindgen_behaving_correctly_in_this_case/
)
    


### TITLE

这篇文章介绍了Rust中的线性类型(Linear Types)概念和使用。线性类型是一种资源类型,确保只能被使用一次,从而避免了并发访问和重复释放等问题。文中使用了一个`UseOnce`的线性类型作为示例,展示了如何在Rust中实现和使用线性类型。当尝试多次使用同一个`UseOnce`实例时,程序会panic并报错,因为线性类型必须只被使用一次。通过这种方式,线性类型可以确保资源的唯一所有权和生命周期安全,在某些场景下可以提高程序的正确性和性能。

[
https://geo-ant.github.io/blog/2024/rust-linear-types-use-once/
](
https://geo-ant.github.io/blog/2024/rust-linear-types-use-once/
)
    


### TITLE

这是关于一个名为Fyrox的游戏引擎的更新计划。下一个版本0.35将是1.0大版本发布前的最后一个小版本更新。开发团队正在征集用户对1.0版本的新功能和改进建议。由于1.0版本需要大量的打磨工作,新功能集合必须被"冻结",在发布1.0版之前将不再添加其他新功能。开发团队欢迎任何建议。这份总结反映了开发团队在发布重大版本之前,希望获取用户反馈并冻结功能集的计划。

[
https://old.reddit.com/r/rust/comments/1h12ors/fyrox_game_engine_aims_to_release_version_10_in/
](
https://old.reddit.com/r/rust/comments/1h12ors/fyrox_game_engine_aims_to_release_version_10_in/
)
    


### TITLE

这是一个关于使用Rust语言中实验性的std::autodiff模块进行自动微分的分享。作者所在的小组发布了第一个使用该模块的应用程序molpipx。自动微分可以应用微积分中的链式法则来计算代码的梯度/导数。之所以使用Rust的autodiff,是因为Python/JAX需要Just-In-Time(JIT)编译才能达到好的运行时性能,但JIT编译时间却极其漫长,有时甚至需要数小时或数天。相比之下,Rust的autodiff虽然编译时间约30分钟也不算短,但至少只需编译一次,而且作者正在努力进一步改善编译时间。目前Rust版本的功能还比不上Python/JAX版本,但一旦作者将autodiff完全上游(目前有两个开放的PR,链接在原文中),他将添加更多功能、基准测试和使用说明。

[
https://old.reddit.com/r/rust/comments/1h1b1po/using_stdautodiff_to_replace_jax/
](
https://old.reddit.com/r/rust/comments/1h1b1po/using_stdautodiff_to_replace_jax/
)
    


### TITLE

这篇文章介绍了一个用Rust编写的名为jaq的jq克隆版本2.0的发布。jq是一种用于处理JSON数据的函数式编程语言。与之前的稳定版本相比,jaq 2.0添加了对jq模块系统和许多语法结构的支持,因此现在可以运行用jq语言本身编写的jq解释器jqjq。

作者还改进了jaq的性能,使其成为已知最快的jq实现,并提供了基准测试结果。对于Rust开发人员来说,一个有趣的特性是可以将jaq嵌入到自己的应用程序中,从而使用jq语言作为类似Lua的脚本语言。

与jq不同,jaq不仅可以处理JSON,还可以处理自定义数据类型,只需为自定义数据类型实现ValT trait即可使用jq过滤器进行处理。

最后,作者提供了一个使用WebAssembly编译的jaq的在线playground供读者试用。

[
https://old.reddit.com/r/rust/comments/1h1an20/jaq_20_jq_clone_released/
](
https://old.reddit.com/r/rust/comments/1h1an20/jaq_20_jq_clone_released/
)
    


### TITLE

这篇帖子的作者最初从事C++开发,包括Windows GUI、COM/OLE、数据库中间件、低延迟交易系统等。后来在一家科技公司从事操作系统级别的C++开发,并参与Linux内核的C语言开发工作。

随着C++的发展,引入了许多来自现代语言(如Rust)的新特性,但语法仍然陈旧,使其变得丑陋。作者渴望尝试新事物,于是利用周末时间用Rust编写了一个新的操作系统MotorOS。

作者发现,越来越多的公司开始使用Rust并招聘Rust工程师,包括一些大型科技公司正在将代码库迁移到Rust。因此,尽管有人说Rust工作机会不多,但实际上在湾区有很多Rust工作机会,而且待遇不错。

最终,作者依靠自学的Rust知识和过去的领域经验,在一家不错的公司获得了一份Rust软件工程师的工作,薪酬也有可观的提升。总的来说,是出于兴趣爱好和职业发展的双重原因,作者告别C++转投Rust怀抱。

[
https://old.reddit.com/r/rust/comments/1h19myr/goodbye_c_rust_is_the_future/
](
https://old.reddit.com/r/rust/comments/1h19myr/goodbye_c_rust_is_the_future/
)
    


### TITLE

该帖子作者表示,他经过3年的Rust编程经验后,最近开始了一份C++的工作。虽然他认为Rust作为一门编程语言优于C++,但由于缺乏足够的采用率和就业机会,他不得不转投C++。文中列举了几个例子,说明即使一些技术更优秀,但如果缺乏足够的用户群和企业推广支持,最终也难以大规模被接受和使用。作者认为Rust目前的就业机会大多局限于一些特殊领域,普通的Rust工程师很难找到合适的工作。他并不否认Rust的优点,但指出必须要有大规模采用才能成功,否则就会像历史上许多优秀但最终没有被广泛接受的技术一样,难以生存下去。该帖引起了热烈讨论,作者也意识到自己的观点可能有失偏颇。

[
https://old.reddit.com/r/rust/comments/1h15md8/goodbye_rust_i_wish_you_success_but_im_back_to_c/
](
https://old.reddit.com/r/rust/comments/1h15md8/goodbye_rust_i_wish_you_success_but_im_back_to_c/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

