【Rust日报】2024-10-13


### TITLE

以下是对该存储库的中文总结:

aj是一个简单、可定制且功能丰富的Rust后台任务处理库,基于Actix(Actor模型)。它支持以下主要功能:

1. 多种任务类型:即时任务、计划任务和cron任务。

2. 任务管理:更新、取消任务,获取任务信息。

3. 重试策略:包括间隔重试、指数退避重试,以及自定义重试逻辑。

4. 后端支持:aj可与实现了Backend trait的任何数据库或存储配合使用,包括内存示例和对Redis的本机支持。

5. 处理速度定制:可调整任务扫描周期和同时运行的最大任务数。

6. 有向无环图(DAG):支持设置任务之间的依赖关系。

7. 分布式模式。

8. 仪表板和其他监控API。

该项目基于Apache 2.0或MIT许可证,用户可自行选择。所有有意向提交的代码贡献均默认采用上述双重许可,不附加其他条款。

[https://github.com/cptrodgers/aj
](https://github.com/cptrodgers/aj
)
    


### TITLE

这个帖子讨论了Rust语言中关于内部可变性和自动特性(auto traits)的一个RFC(征求意见案)。该RFC建议引入一个新的`Freeze`特性,用于标记那些不具有内部可变性的类型。rust-lang团队对此有以下几点担忧:

1. `Freeze`这个名称不够清晰,他们更倾向于使用`NoInteriorMutability`这样更具描述性的名称。

2. 引入这个新的自动特性会带来向后兼容性问题。如果将来需要为某个类型添加内部可变性,那么之前标记为`Freeze`的类型就会发生破坏性变更。为了避免这种情况,库作者需要预先使用`PhantomData<UnsafeCell<()>>`标记或使用`Box`包装,这给库作者带来了额外的负担。

3. 他们认为,引入这个特性为了解决一个相对较小的使用场景,所付出的代价(向后兼容性问题和额外负担)有些过高。

因此,rust-lang团队最终决定暂时不将该RFC合并到Rust语言中,而是建议先以外部crate的形式提供这个特性,并通过实际使用评估其价值和影响。

[
https://internals.rust-lang.org/t/exploring-interior-mutability-auto-traits-and-side-effects/13431
](
https://internals.rust-lang.org/t/exploring-interior-mutability-auto-traits-and-side-effects/13431
)
    


### TITLE

以下是对该内容的中文总结:

JSON Resume是一个开源项目,旨在为简历创建一个基于JSON的标准格式。它是由开发者为开发者而设计的。这个标准允许使用JSON格式来表示个人简历信息。

JSON Resume提供了一个主题库,收集了社区贡献的各种简历主题样式供选择。整个项目都是开源的,代码托管在GitHub上,可以自由访问和修改。

JSON Resume的目标是为开发者提供一种标准化、结构化的方式来呈现和共享他们的简历信息,使之更加易于维护和共享。同时,开放的标准格式也方便简历数据与其他系统集成。

[
https://jsonresume.org/
](
https://jsonresume.org/
)
    


### TITLE

这个仓库包含了一个名为`rsume`的工具,旨在简化软件开发人员求职过程中的简历生成。求职过程通常非常繁琐,大多数公司不会回复申请,即使有回复也需要通过多重评估环节才能获得工作。而针对每一次申请定制化简历又非常耗时。`rsume`工具可以根据提供的数据,轻松生成高质量的简历。

目前,该工具只能通过下载或克隆仓库并使用cargo或rustc自行构建二进制文件的方式安装。运行程序需要Google Chrome或Chromedriver实例。

使用`rsume`需要在命令行中指定简历数据文件路径和目标PDF文件路径,可选择指定模板和语言。简历数据应遵循JSONResume架构,可以是JSON或YAML格式。

已知问题包括:当前只有一个可用模板,较短内容可能生成空白第二页,部分章节标题可能与内容分离。未来计划增加更多模板。

[
https://github.com/unexcellent/rsume
](
https://github.com/unexcellent/rsume
)
    


### TITLE

根据GitHub上的Pull Request描述,这次更新做了以下几点改动:

1. 增加了记住用户--mode选择的功能,这样用户就不需要每次都指定mode参数了。

2. 为diff.rs的inspect模式添加了基本支持,通过从当前版本加载自身的差异来实现,这样inspect模式的mode选择也可以被记住。

3. 如果xfbs/diff.rs#24被合并,需要将diff.rs的inspect行为改为使用新的browse端点。 

4. 将默认模式从sourcegraph改为了diff.rs。之前sourcegraph经常不可靠,因此暂时将diff.rs设为默认模式以获得更好的体验。

总的来说,这个更新主要是增强了mode选择的用户体验,并将默认模式临时切换到了较为可靠的diff.rs。

[
https://github.com/mozilla/cargo-vet/pull/633
](
https://github.com/mozilla/cargo-vet/pull/633
)
    


### TITLE

总结如下:

这位开发者在Windows环境下尝试使用Rust的libesedb-sys包时遇到了编译问题。在添加libesedb-sys依赖并尝试编译时,出现了缺少内存管理函数(如memory_allocate、memory_set、memory_free)声明的错误。

看起来缺少包含这些函数声明的必要头文件。由于在Windows环境下无法运行./configure或make命令,开发者不确定如何正确设置C库的构建环境。

他希望得到指导,是否应该使用Cygwin或MinGW,以及需要采取哪些额外步骤才能在Windows上正确配置libesedb-sys并成功编译。他在寻求相关的帮助和建议。

[
https://old.reddit.com/r/rust/comments/1g2xffv/issues_compiling_libesedbsys_in_rust_on_windows/
](
https://old.reddit.com/r/rust/comments/1g2xffv/issues_compiling_libesedbsys_in_rust_on_windows/
)
    


### TITLE

这是一个GitHub上的Pull Request,主要内容如下:

1. 将OpenSSL库替换为Rustls库。

2. 将async_tungstenite依赖库的版本降级到0.24。

3. Pull Request已被合并到main分支。

4. 这个更改通过了所有9项状态检查。

5. 一些贡献者对此次更改做出了正面反馈(点赞等)。

6. 有人提到这个Pull Request可能与另一个Issue #18891关于支持RISC-V架构的相关。

总的来说,这是一个将加密库从OpenSSL切换到Rustls,并调整了一个WebSocket依赖库版本的代码更改。

[
https://github.com/zed-industries/zed/pull/19104
](
https://github.com/zed-industries/zed/pull/19104
)
    


### TITLE

这是一个介绍了一个名为 AJ 的 Rust 库的帖子。AJ 是一个用于处理后台任务的库,它专注于简单的接口,同时提供了高度的可定制性和许多功能,如重试、执行前/后操作、调度、cron、更新、取消等。作者刚刚发布了 0.6.2 版本,并分享了一些使用示例。作者提到正在致力于编写更多测试、示例,并改进宏以简化使用。最后,作者邀请大家查看代码库并提供反馈,以帮助改进这个库。

[
https://old.reddit.com/r/rust/comments/1g2lpmd/aj_simple_customizeable_and_featurerich/
](
https://old.reddit.com/r/rust/comments/1g2lpmd/aj_simple_customizeable_and_featurerich/
)
    


### TITLE

以下是对给定内容的中文总结:

这段内容讨论了在Rust编程语言中使用`PhantomData`时的一些惯例做法。

1. 假设正确的做法是`PhantomData<T>`
2. 但更常见的做法是`PhantomData<fn() -> T>`
3. 较不常见的做法是`PhantomData<fn(T)>`
4. 前两种做法适用于多个泛型参数的情况
5. 第三种做法不需要使用元组语法,因为它涉及函数参数

总的来说,这些都是在Rust中使用`PhantomData`的不同方式,目的是在编译时强制类型安全性,尽管它们看起来有些冗余和不直观。作者提出了这个问题,可能是为了讨论这些惯例使用方式的合理性和必要性。

[
https://old.reddit.com/r/rust/comments/1g2f2al/why_do_people_do_this_in_phantomdata/
](
https://old.reddit.com/r/rust/comments/1g2f2al/why_do_people_do_this_in_phantomdata/
)
    


### TITLE

这位开发者目前正在面临一个问题。他有两个使用Tauri框架的项目,这两个项目需要共享大量本地Rust代码。为了解决这个问题,他在根目录下创建了一个共享的Rust库,并将两个项目作为子模块集成到根目录中。

目录结构如下:
- 根目录包含一个Cargo.toml文件和三个子目录:shared_lib、project1/src-tauri和project2/src-tauri。
- shared_lib目录是共享的Rust库。
- project1和project2目录分别包含了每个项目的Tauri配置、前端代码等。

他对这种模式和结构是否合理表示疑虑,并希望获得一些最佳实践的建议。目前他可以在每个项目目录中运行和调试没有问题,但不确定如何从根目录直接运行和调试项目的最佳方式。他期望能获得相关建议。

[
https://old.reddit.com/r/rust/comments/1g368lo/about_tauri_and_workspace_problem/
](
https://old.reddit.com/r/rust/comments/1g368lo/about_tauri_and_workspace_problem/
)
    


### TITLE

这是一个关于Rust编程语言中零成本单元(zero-cost cell)的想法的讨论。作者提出了一个名为TaintedCell的设计,类似于之前提出的phlopsis的想法,但使用了LocalToken来解决"静态"问题。作者希望能够得到反馈,看看这个API在哪些地方还是不安全的,或者在实践中是否有用。尽管需要到处传递LocalToken令人有些困扰,但作者认为它在缓存、内联字符串等场合仍有一些很酷的用例。总的来说,这是一个关于如何在Rust中实现内部可变性的有趣想法和讨论。

[
https://old.reddit.com/r/rust/comments/1g2pali/idea_taintedcell_yet_another_safe_zerocost/
](
https://old.reddit.com/r/rust/comments/1g2pali/idea_taintedcell_yet_another_safe_zerocost/
)
    


### TITLE

该问题描述了一种PHP解释器在Rust中的实现思路。在生成完整的抽象语法树(AST)之后,作者提出了一个额外的优化步骤的想法。这个优化步骤包括遍历AST,生成一个优化版本。

作者提出了两种可能的优化方法:

1. 如果系统检测到一个变量只被使用一次,它可以建议避免克隆该值,而是直接将其移动到所需位置。

2. 如果一个函数被多次调用,在第一次调用之后,它可以建议将该函数的函数体(由AST生成的语句)复制以便重用。

作者寻求反馈,想知道这些优化想法是否合理,以及这种优化方法是否可行。作为解释器开发的新手,作者希望获得社区的建议和意见。

[
https://old.reddit.com/r/rust/comments/1g3692u/does_this_optimization_make_any_sense_is_it_a/
](
https://old.reddit.com/r/rust/comments/1g3692u/does_this_optimization_make_any_sense_is_it_a/
)
    


### TITLE

这篇博文宣布了 Deno 2.0 的重大更新。主要内容包括:

1. Deno 2.0 向后兼容 Node.js 和 npm,可以无缝运行现有的 Node 应用程序,并支持逐步采用 Deno 的工具链。

2. 新增了 deno install、deno add 和 deno remove 命令,作为包管理器功能。

3. 推出了现代化的 JavaScript 注册表 JSR,支持以 TypeScript 源码的形式发布模块。

4. 标准库现已稳定,涵盖数据操作、网络等多种工具模块。

5. 支持私有 npm 注册表、工作区和单体应用。 

6. 引入了长期支持(LTS)版本,每6个月发布一个 LTS 版本,backport 重要的bug修复。

7. 持续改进现有功能,如格式化、lint、测试、编译、服务器等。

8. 针对企业用户,推出了 Deno for Enterprise 计划,提供优先支持和直接访问工程师的服务。

总的来说,Deno 2.0 加强了与 Node.js 生态系统的兼容性,同时保留了 Deno 的简单、安全和一体化特性,旨在推动大规模 Web 开发。

[
https://deno.com/blog/v2.0
](
https://deno.com/blog/v2.0
)
    


### TITLE

该Reddit帖子的作者介绍了一个名为rsume的Rust工具,用于根据JSONResume格式的数据生成样式化的简历。作者最初使用Python开发该工具,但由于对Python的数据解析和HTML到PDF转换效率不满意,最终决定使用Rust语言重写。rsume是作者的一个较大的Rust项目,目前只有一种设计风格,但未来计划添加更多风格。帖子中还附有一张使用该工具生成的简历示例图。作者欢迎反馈意见和贡献。

[
https://old.reddit.com/r/rust/comments/1g2uvvl/generating_templated_resumes_in_rust/
](
https://old.reddit.com/r/rust/comments/1g2uvvl/generating_templated_resumes_in_rust/
)
    


### TITLE

这篇文章介绍了如何用Rust的Axum Web框架来替代Nginx做反向代理服务器。文章首先简单介绍了Axum的基本用法,如何创建路由、处理器等。然后展示了如何使用Axum托管一个简单的静态网站,只需几行代码。

接着,文章阐述了如何使用Axum托管两个不同子域名的静态网站,通过解析主机名来路由到不同的服务。代码通过判断是否提供"--production"参数来决定使用实际域名还是本地测试域名。

最后,作者将路由逻辑抽取到一个单独函数中,使main函数更加清晰,并使用BoxCloneService来简化Service trait的使用。

总的来说,这篇文章向读者展示了如何使用Axum框架编写功能丰富但代码简洁的Web服务,以替代配置复杂的Nginx反向代理。Axum利用Rust的高性能特性,同时提供了清晰直观的API,是构建Web服务的不错选择。

[
https://felix-knorr.net/posts/2024-10-13-replacing-nginx-with-axum.html
](
https://felix-knorr.net/posts/2024-10-13-replacing-nginx-with-axum.html
)
    


### TITLE

这是一封来自 Rust 社区的邮件,作者正在开发一个名为 diff.rs 的工具。该工具可以比较 crates.io 上发布的 crate 版本之间的差异,因为发布的文件可能与存储库中的文件不完全匹配。diff.rs 完全使用 Rust 编写,可在浏览器中运行。

作者最近对该工具进行了一些改进,现在需要社区的帮助来实现新功能和修复一些 bug。如果你对使用 Rust 进行前端开发感兴趣且勇于尝试新事物,欢迎查看开放的 issue 并尝试解决其中的任务。作者将把贡献者列入 diff.rs/about 页面。作者欢迎任何新的想法或用例,只要他能够维护相关代码即可。

总的来说,这是一个面向 Rust 社区的贡献邀请,旨在改进 diff.rs 这一实用工具。

[
https://old.reddit.com/r/rust/comments/1g2rdrk/help_wanted_for_diffrs_and_new_design/
](
https://old.reddit.com/r/rust/comments/1g2rdrk/help_wanted_for_diffrs_and_new_design/
)
    


### TITLE

该帖子讨论了Tonic(一个Rust语言的gRPC框架)在某些多核基准测试中表现不佳的原因。作者认为,这可能是由于语言障碍导致的初始理解出现了问题。

通常情况下,Tonic能够在多个线程上独立运行,而不会对性能造成额外的影响。但在特定的基准测试中,Rust的性能受到影响的一个原因是,accept循环直接运行在block_on调用中。这可能会导致性能问题,因为tokio::spawn任务永远不会在block_on任务所在的同一个线程上运行,这意味着所有连接在被处理前都必须转移到另一个Tokio线程。

将accept循环移动到一个spawned任务中可以帮助解决这个问题,因为这样就有可能在不必将任务移动到另一个线程的情况下执行任务。

[
https://users.rust-lang.org/t/why-tonic-not-good-in-beckmark-of-multi-core/70025/6
](
https://users.rust-lang.org/t/why-tonic-not-good-in-beckmark-of-multi-core/70025/6
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

