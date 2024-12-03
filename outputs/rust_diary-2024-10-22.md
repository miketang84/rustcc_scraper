【Rust日报】2024-10-22


### TITLE

这是一个用Rust语言重写的经典文本冒险游戏DrugWars的40周年纪念版本。该版本在保持原游戏体验的同时,提供了跨平台支持和更新的终端兼容性。它使用了与原版相同的单键命令,确保了流畅的游戏体验和快速的输入处理。该仓库提供了不同的构建和安装方式,包括普通发行版、LTO优化版本、本地安装等。另外,对于Git Bash和Mintty等终端,提供了特定的运行命令。总的来说,这个项目向怀旧的玩家呈现了一个经典游戏的现代化复刻版本。

[https://github.com/urrickhunt/drugwars-rust
](https://github.com/urrickhunt/drugwars-rust
)
    


### TITLE

以下是对该文章的中文总结:

该文章首先介绍了SOLID设计原则,包括单一责任原则、开放封闭原则、里氏替换原则、接口隔离原则和依赖反转原则。这些原则有助于设计可维护和可扩展的面向对象系统。

然后,文章详细介绍了策略模式这种行为设计模式。策略模式定义了一族算法,将每个算法封装起来,并使它们可在运行时互换。这种模式基于组合的思想,通过将算法的行为与算法使用的客户端代码分离开来,可以灵活地改变算法。

文章以一个简化的交易系统为例,展示了如何在Rust中实现策略模式。系统中有三种不同的执行策略:TWAP、VWAP和POV。通过定义执行策略的公共接口trait,以及具体实现这些策略的结构体,实现了可插拔的策略。上下文OrderExecutor持有策略的引用,通过统一的execute方法调用具体策略的执行逻辑。客户端可以在运行时选择使用何种策略。

总的来说,这篇文章清晰地介绍了SOLID原则和策略模式,并通过实例展示了在Rust中应用该模式的方式,很好地阐释了模式的结构和用法。

[
https://siddharthqs.com/design-patterns-in-rust
](
https://siddharthqs.com/design-patterns-in-rust
)
    


### TITLE

这是一个用C++编写的分析库DataFrame,类似于Python和R中的数据分析库。它可以对数据进行切片、连接、合并、分组等操作,并运行各种统计、总结、金融和机器学习算法。DataFrame还包含了大量分析算法,从基本统计(如均值、标准差、回报率)到更复杂的分析(如关系传播、多项式拟合、任意长度快速傅里叶变换)等,还包括一些交易指标。你可以轻松添加自定义算法。

DataFrame在几乎所有API中都广泛使用了多线程,因此特别适合分析大型数据集。该库遵循了几个原则:支持任何内置或用户定义的类型而无需新代码、避免使用链表等指针追踪、所有列数据在连续内存空间中、避免不必要的数据复制、合理使用多线程、不试图防止垃圾输入等。

作者对DataFrame与Polars(Rust实现的DataFrame库)的性能进行了基准测试。在300万行数据的情况下,C++ DataFrame在数据生成、计算和选择方面的总体时间比Polars和Pandas都快很多。DataFrame还可以处理更大的数据集。最后,作者呼吁赞助该项目,以表示对DataFrame在生产环境中使用的感谢。

[
https://github.com/hosseinmoein/DataFrame
](
https://github.com/hosseinmoein/DataFrame
)
    


### TITLE

这是一个关于Rust编程语言借用检查器(borrow checker)功能的讨论。主要内容包括:

1. 作者认为在某些多线程程序的情况下,借用检查器过于保守,禁止了一些实际上是安全的代码模式。

2. 作者提出借用检查器应该能够进行更精确的分析,利用多线程同步原语的happen-before/happen-after语义来推断数据竞争和生命周期安全性。

3. 有人建议使用作用域线程(scoped thread)作为解决方案,但作者认为这更像是一种技巧,并没有解决借用检查器无法进行多线程分析的根本问题。

4. 也有人提出将数据移动到闭包中、修改后返回赋值给原变量,从而规避借用检查器的限制,但作者认为这样做不太恰当。

5. 讨论反映了Rust社区对编译器分析能力和易用性之间的权衡存在分歧。

总的来说,这是一个关于Rust借用检查器在处理并发程序时缺乏精确分析能力的讨论和反思。

[
https://www.reddit.com/r/rust/comments/1g9u0uj/comment/lt97hej/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
](
https://www.reddit.com/r/rust/comments/1g9u0uj/comment/lt97hej/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
)
    


### TITLE

这是一个用Rust语言实现的文档解析和生成库,叫做Shiva。它支持多种文档格式的相互转换,包括纯文本、Markdown、HTML、PDF、JSON、XML、CSV、RTF、DOCX、XLS、XLSX和ODS等。该库提供了一个通用的文档模型(CDM),解析器将文档解析为CDM,生成器从CDM生成目标文档格式。

该库支持解析和生成不同类型文档的标题、段落、列表、表格、图像、超链接、页眉和页脚等元素。它可以用作Cargo依赖库,也可以作为CLI工具和Web服务器使用。该项目欢迎社区贡献,贡献者需要实现相应的Trait接口来支持新的文档类型。该库使用Apache 2.0或MIT协议开源。

[
https://github.com/igumnoff/shiva
](
https://github.com/igumnoff/shiva
)
    


### TITLE

Lapdev是一个自托管的应用程序,可在您自己的服务器或云上启动远程开发环境。它使用开放的Devcontainer规范来定义开发环境代码。Lapdev具有以下主要特点:

1. 易于自托管安装和维护。
2. 可从单台机器扩展到全球服务器集群,实现水平扩展。 
3. 将开发环境作为代码进行标准化,确保一致的环境设置。
4. 加快新手入职速度,无需花费大量时间准备本地环境。

未来,Lapdev计划支持除容器外的更多工作空间类型,如虚拟机和裸机,以及更多操作系统,以支持跨平台桌面应用程序开发。该项目提供了安装步骤和源代码构建指南,并欢迎贡献。

[
https://github.com/lapce/lapdev
](
https://github.com/lapce/lapdev
)
    


### TITLE

Lap.dev 是一个云开发环境服务,可以快速启动高性能的开发环境。它使用游戏级别的 CPU 来提供高单核性能,这对于开发工作非常重要。与其他一些在线开发环境相比,Lap.dev 的单核性能更出色。

Lap.dev 遵循 Devcontainer 开放规范,允许将开发环境定义为代码,确保了一致的开发环境,消除了"在我的机器上是可以工作的"这种问题。它还能加快新开发人员的入职时间,无需在本机花费大量时间准备环境。

在 Lap.dev 上,您可以轻松在不同项目或分支之间切换,无需先暂存或提交当前更改。您可以使用浏览器内置的 IDE,也可以使用 VSCode、JetBrains IDE、Lapce 等您喜欢的编辑器,或者直接通过 SSH 连接到环境中使用 Vim、Emacs 等编辑器。

[
https://lap.dev/
](
https://lap.dev/
)
    


### TITLE

这是一个使用Rust编程语言实现的经典游戏"毒品战争"。作者在Reddit上分享了这个项目的GitHub链接和游戏界面的截图。他欢迎任何反馈意见,并希望大家能享受游戏。这个项目展示了作者使用Rust语言开发游戏的能力,同时也让那些怀念经典游戏的人能重温儿时的乐趣。

[
https://old.reddit.com/r/rust/comments/1g8ymym/drugwars_in_rust/
](
https://old.reddit.com/r/rust/comments/1g8ymym/drugwars_in_rust/
)
    


### TITLE

在这篇文章中,作者探讨了如何使用Rust语言来应用一些设计模式,如策略模式、观察者模式和装饰器模式,来构建算法交易系统。作者表示他将继续添加更多的设计模式示例,并希望听到读者的想法、反馈和见解。文章链接为https://siddharthqs.com/design-patterns-in-rust,详细介绍了在Rust中实现这些设计模式的方法和代码示例。

[
https://old.reddit.com/r/rust/comments/1g9tfpe/exploring_design_patterns_in_rust_with/
](
https://old.reddit.com/r/rust/comments/1g9tfpe/exploring_design_patterns_in_rust_with/
)
    


### TITLE

这个帖子提出了一个关于在 Rust 中实现深层嵌套的面向对象规范的问题。具体场景是,假设有一个使用 UML 描述的旧规范,包含了一些深层嵌套的抽象类,如 A -> B -> C -> D -> E、F、G、H(各种具体类)。每个抽象类可能有1-3个属性和1-3个方法。而且层级关系并非那么线性,因为还有其他类继承自 B、C 和 D。

作者想知道在 Rust 中用惯用的方式来实现这种嵌套的面向对象规范应该如何做。Rust 作为一种系统编程语言,并不直接支持传统面向对象编程中的类和继承等概念,但它提供了其他模拟面向对象的方式,如结构体、trait 等。因此这个问题旨在探讨如何用 Rust 的方式优雅地实现类似需求。

[
https://old.reddit.com/r/rust/comments/1g9j8k7/implementing_a_deeplynested_oo_specification_in/
](
https://old.reddit.com/r/rust/comments/1g9j8k7/implementing_a_deeplynested_oo_specification_in/
)
    


### TITLE

这个问题讨论了在使用WebGPU API时不同GPU缓冲区的作用和性能影响。作者正在学习WebGPU教程,对于顶点缓冲区、索引缓冲区和存储缓冲区等不同类型的缓冲区的用途感到疑惑。

作者认为,CPU可以向GPU发送任意数据缓冲区,GPU着色器可以以任何方式解释这些数据。比如可以将索引缓冲区的数据当作顶点数据,或者将存储缓冲区的数据当作索引使用。教程中甚至将顶点数据和颜色数据一起存储在顶点缓冲区中。

作者的疑问是,这些不同类型的缓冲区在GPU端是否真的有特殊含义,使用正确的缓冲区类型对性能是否会有影响。总的来说,这是一个关于如何正确使用GPU缓冲区资源的技术性问题。

[
https://old.reddit.com/r/rust/comments/1g9pzsh/need_help_understanding_the_types_of_wgpu_gpu/
](
https://old.reddit.com/r/rust/comments/1g9pzsh/need_help_understanding_the_types_of_wgpu_gpu/
)
    


### TITLE

这篇Reddit帖子讨论了Rust编程语言在数据框架库性能方面的表现。Rust一直被宣传为比C++更安全且性能相当,但是根据C++ DataFrame项目中的基准测试,Rust编写的Polars数据框架库在某些情况下明显比C++ DataFrame慢。

作者表达了疑惑,认为Rust应该能与C++的性能相当,但测试结果显示Polars在某些基准测试中的性能却落后于C++ DataFrame。帖子提出了对Polars与C++ DataFrame性能的比较,并链接了相关的C++ DataFrame GitHub项目。

总的来说,这个讨论集中在Rust语言实际性能是否如宣传的那样能与C++相媲美,特别是在数据密集型应用场景下的表现。

[
https://old.reddit.com/r/rust/comments/1g9c9jy/polars_is_faster_than_pandas_but_seems_to_be/
](
https://old.reddit.com/r/rust/comments/1g9c9jy/polars_is_faster_than_pandas_but_seems_to_be/
)
    


### TITLE

这是一篇关于Rust语言借用检查器(borrow checker)在处理多线程程序时的局限性的讨论。作者提出了一种常见的多线程编程模式,即在主线程中创建一个共享数据,然后派生一个新线程来修改该数据,之后在主线程中等待新线程结束并读取修改后的数据。

作者认为,从程序语义上看,这种模式是安全的,不存在数据竞争或生命周期违例的问题。因为新线程在主线程结束前必须终止(由于join()调用),所以主线程的数据生命周期足以覆盖新线程的使用。同时,主线程读取数据的时间点一定晚于新线程完成数据修改,因此不存在数据竞争。

但是,Rust的借用检查器目前无法对这种多线程场景进行推理,因此会禁止这种代码模式。作者认为,现代语言的内存模型都基于happens-before/happens-after关系来推理多线程安全性,借用检查器理论上也可以在编译期对此进行推理,而不需要运行时的推理。

总的来说,这是一篇关于扩展Rust借用检查器以支持更复杂多线程场景的讨论和建议。

[
https://old.reddit.com/r/rust/comments/1g9u0uj/rust_borrow_checker_should_be_capable_of_flow/
](
https://old.reddit.com/r/rust/comments/1g9u0uj/rust_borrow_checker_should_be_capable_of_flow/
)
    


### TITLE

总结:

Shiva是一个用Rust编写的开源项目,旨在成为一个通用的文档解析和转换工具。该项目于2024年3月启动,经过数月的持续开发,现已支持包括HTML、Markdown、纯文本、PDF、JSON、CSV、RTF、DOCX、XML、XLS、XLSX、ODS和Typst在内的广泛文件格式。

Shiva的目标是为已有的Java编写的Apache Tika和Haskell编写的Pandoc等工具提供一种简单高效的替代方案,以应对日益增长的数字文档多样性和复杂性。该项目仍在快速发展中,有很多工作要做,但目前的进展令人鼓舞。

作者对所有为增加新格式支持做出贡献的人表示了巨大的感谢。他欢迎大家查看项目库,提供反馈或进行协作,以推动Shiva项目取得更大的成就。

[
https://old.reddit.com/r/rust/comments/1g9e44z/shiva_a_new_project_an_alternative_to_apache_tika/
](
https://old.reddit.com/r/rust/comments/1g9e44z/shiva_a_new_project_an_alternative_to_apache_tika/
)
    


### TITLE

以下是对该内容的中文总结:

Lapdev是一个新的类似于Codespaces/Gitpod的远程开发环境服务,由Lapce团队构建。该团队意识到Rust编译速度较慢可能是其唯一的缺点,因此在构建Lapdev时着重考虑了单核CPU性能。Lapdev使用AMD 7950X3D处理器,这是当前单核性能最快的CPU之一,因此如果你的机器性能较低,使用Lapdev应该可以提高Rust编译速度。Lapdev提供30小时的免费使用时间,足以支持业余项目的开发而无需付费。值得一提的是,Lapdev本身也是用Rust编写的。该项目的源代码托管在GitHub上。

[
https://old.reddit.com/r/rust/comments/1g9ruge/lapdev_a_remote_dev_env_that_you_can_compile_rust/
](
https://old.reddit.com/r/rust/comments/1g9ruge/lapdev_a_remote_dev_env_that_you_can_compile_rust/
)
    


### TITLE

以下是对该内容的中文总结:

本月进展令人兴奋,主要包括以下几个方面:

1. 树合并(正在进行中)
已经完成了blob合并,包括文本内容合并,现在开始着手处理更加复杂的实际树合并问题。引入了重命名跟踪功能后,特别是处理干净的目录重命名时,合并难度加大。参考了Git默认的合并算法merge-ORT,利用基线测试和调试打印等手段,一步步解决了这个问题。

2. Blob合并改进
发现之前的blob合并算法在某些情况下没有满足交换运算的特性,并修复了这个重大缺陷。引入了fuzzer对算法进行更多测试。

3. gix merge file 命令
新增了用于blob内容合并的gix merge file命令行工具。

4. 社区建设
Gitoxide项目现在由GitoxideLabs小组所有,并在Open Collective上设立了fiscally hosted。准备向Sovereign Tech Fund申请资助。

5. GitMerge 2024大会
作者在柏林的这次大会上做了关于gitoxide的演讲,结识了一些Git社区的重要人士。

6. 优化提交遍历
合并了一个PR,通过"oldest-first"遍历顺序优化,显著提高了遍历速度。

7. gix cat命令
针对git cat-file命令的UX问题,新增了gix cat命令。

[
https://github.com/GitoxideLabs/gitoxide/discussions/1641
](
https://github.com/GitoxideLabs/gitoxide/discussions/1641
)
    


### TITLE

这篇博客文章介绍了 Rustls TLS 库在性能方面优于 OpenSSL 和 BoringSSL。Rustls 是一个内存安全且高性能的 TLS 实现。文章分享了 Rustls 在握手和吞吐量测试中的优异表现。测试方法是在同一硬件和资源限制下对不同库进行评估。

Rustls 不仅在性能方面领先,而且提供了 C 和 Rust API、FIPS 支持、量子密钥交换等功能。该库已准备好用于生产,文章鼓励大家尝试使用。

文章最后感谢了 AWS、Intel 等合作伙伴,以及资助机构的支持,使 Rustls 能够取得如此出色的性能。总的来说,Rustls 凭借内存安全和卓越性能,可以成为替代 OpenSSL 等 C 语言 TLS 库的绝佳选择。

[
https://www.memorysafety.org/blog/rustls-performance-outperforms/
](
https://www.memorysafety.org/blog/rustls-performance-outperforms/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

