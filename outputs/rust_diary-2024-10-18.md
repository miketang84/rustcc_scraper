【Rust日报】2024-10-18


### TITLE

该资源库提供了一个高级的用户、认证和会话处理系统,旨在用于Axum Web框架,后续可能也会支持Actix Web框架。它的目标是成为Next Auth的基础,但针对Leptos,提供易于设置且功能丰富的解决方案,采用电池内置的方式而非完全可定制化。

主要特性包括:

1. 登录类型:用户名/密码、电子邮件魔法链接、社交登录(OAuth)。
2. 电子邮件相关功能:验证、密码重置。
3. OAuth:易于扩展,实现自定义提供程序。
4. 用户信息获取过程的人体工程学实现。
5. 可选择分离回调路径。
6. 基于Askama的模板,提供基本的登录/注册/账户页面。
7. 内置的社交提供商列表不断增长。
8. 多会话支持。

待办事项包括:

1. 基于功能控制的模板。
2. 可替换的模板(通过返回impl IntoResponse的类型化函数)。
3. WebAuthn多因素认证支持。
4. 文档注释。
5. 测试。
6. 发布。

该crate正在积极开发中,模块层次结构和基本API仍在完善中。对于需要真正自定义的场景,可以考虑使用awesome axum-login或oauth2 crates。但如果只是需要基本的用户登录、密码重置、社交账号绑定和多会话管理等功能,该crate可能适合您。设计方面的Pull Request是受欢迎的。

[https://github.com/StefanTerdell/userp
](https://github.com/StefanTerdell/userp
)
    


### TITLE

以下是对该文章的中文总结:

Rerun是一个为机器人和空间智能构建多模态数据堆栈的公司。在0.19版本中,他们推出了几个新功能:

1. 数据框查询API,允许用户从Rerun录制的数据中读取并运行查询,返回Apache Arrow数据格式,可与熟悉的数据框工具(如Pandas、Polars或DuckDB)集成。这使得从录制中创建训练数据集变得更加容易。

2. 数据框视图,让用户以表格形式查看Rerun中的任何数据。

3. 第一部分原生视频支持,支持MP4文件中的AV1和H.264编码视频,未来将支持视频流和H.264本机查看器。

4. SDK访问Rerun的查询引擎,可以在60fps下对时间感知的实体组件系统数据模型进行查询,支持最新数据和时间对齐等功能。

5. 将机器人数据原语(如时间对齐、空间变换解析、标签查找和视频帧解码)构建到数据库层,简化数据管道。

6. 商业产品即服务,提供ingesting、存储、分析和大规模流式传输数据的托管基础设施,内置可视化调试功能(目前在开发中)。

Rerun呼吁用户尝试新功能并提供反馈意见。他们的目标是为从事空间和体现AI的团队简化数据管道的构建和运营。

[
https://rerun.io/blog/dataframe
](
https://rerun.io/blog/dataframe
)
    


### TITLE

这个仓库是一个简单、快速、轻量级的 Markdown 渲染器和预览器,使用 Rust 语言编写。它有以下主要特点:

1. 静态模式:将 Markdown 文件转换为 HTML,并直接在默认浏览器中打开,无需运行服务器。

2. 服务器模式:运行本地服务器预览 Markdown 文件,并支持实时重新加载。

3. 支持 CommonMark 扩展,如删除线、表格、脚注、任务列表和智能标点符号。

4. 可自定义样式,包括默认 CSS 和可嵌入字体及图标。

5. 支持从源代码构建和通过 crates.io 安装。

6. 使用 Pulldown-Cmark 解析和渲染 Markdown,Warp 运行 Web 服务器,Notify 监视文件变化。

7. 使用 MIT 许可证,欢迎贡献并提出问题或建议。

总的来说,这是一个简便实用的 Markdown 工具,提供了静态和动态两种渲染预览模式。

[
https://github.com/ptrglbvc/omd
](
https://github.com/ptrglbvc/omd
)
    


### TITLE

根据GitHub上torfmaster用户的贡献记录,在过去一年中,他一共只有6次代码贡献。大部分时间他都没有任何贡献。其中有一次是在2022年9月1日做出1次贡献,7月22日和8月12日各贡献2次,8月27日贡献1次。最近的贡献是在2023年2月7日和14日,分别贡献了1次和2次。总的来说,这位用户的代码贡献非常少。

[
https://github.com/torfmaster
](
https://github.com/torfmaster
)
    


### TITLE

这是Jonathan Behrens(fintelia)在GitHub上的个人主页概览。主要介绍了他的一些工作和项目,包括:

1. 他维护了image-rs和开发了Terra(一个大规模地形渲染库)。

2. 列出了他参与的其他一些开源项目,如ward、RVirt、noria和skyrender等。

3. 展示了过去一年中他在GitHub上的贡献活动统计图表。

4. 提供了对他的项目进行star、fork等操作的入口。

总的来说,这是一个总结了该个人在GitHub上工作、贡献的主页概览。

[
https://github.com/fintelia
](
https://github.com/fintelia
)
    


### TITLE

以下是对Rust 1.80.0版本的中文总结:

1. 稳定了LazyCell和LazyLock类型,用于延迟初始化数据,直到第一次访问时才进行初始化。这类似于之前的OnceCell和OnceLock,但将初始化函数包含在cell内部。

2. Cargo现在对所有cfg名称和值进行检查,包括Cargo.toml中的feature名称以及构建脚本输出的cargo::rustc-check-cfg。这有助于捕获拼写错误或其他配置错误。

3. 模式匹配中的范围现在可以使用独占范围端点,如a..b或..b。这种新语法与Range和RangeTo表达式类型保持一致。同时也增强了穷尽性检查,以防止出现模式匹配间隙。

4. 稳定了一些新的API,包括Default trait的一些实现、IntoIterator和FromIterator实现、LazyCell和LazyLock类型、Duration的一些方法以及NonNull指针的各种操作方法等。

总的来说,这个版本带来了一些实用的新特性和API,并改进了配置检查和模式匹配,使Rust更加安全和方便。

[
https://blog.rust-lang.org/2024/07/25/Rust-1.80.0.html
](
https://blog.rust-lang.org/2024/07/25/Rust-1.80.0.html
)
    


### TITLE

这篇帖子表达了作者想深入理解Rust编译器中生命周期注解背后的机制和规则。具体包括:

1. 编译器如何推导和处理生命周期参数,尤其是在函数调用现场时如何选择最长的生命周期。

2. 对于同一作用域内的多个变量,它们在编译器看来的生命周期结束时间是不是相同的,即使在较低层面上它们会分别被释放。

3. 作者知道掌握这些并不是使用生命周期注解的必要条件,但是有时了解底层机制有助于更好地记住和理解高级抽象概念,就像C++中迭代器失效规则一样。

总的来说,作者希望能找到一些关于Rust生命周期注解在编译器层面上更正式和详细的描述,以更好地理解其内在工作原理。

[
https://old.reddit.com/r/rust/comments/1g6ljge/any_resources_to_learn_how_exactly_lifetime/
](
https://old.reddit.com/r/rust/comments/1g6ljge/any_resources_to_learn_how_exactly_lifetime/
)
    


### TITLE

这篇Reddit帖子提出了两个主要问题:

1. 作者想知道哪些公司有全职参与Rust项目的贡献者,以及他们的职责或工作描述是什么。

2. 作者想知道是否有人设法说服公司为Rust项目贡献者创建了职位。

作者对于公司是否雇佣全职Rust开发人员,以及如何说服公司引入这种职位感兴趣。这反映了Rust作为一种新兴编程语言,受到越来越多关注,一些公司可能已经开始将其用于生产环境。对于那些希望在Rust领域发展职业生涯的人来说,这个帖子提供了一个讨论机会。

[
https://old.reddit.com/r/rust/comments/1g6us2s/which_companies_have_full_time_rust_project_like/
](
https://old.reddit.com/r/rust/comments/1g6us2s/which_companies_have_full_time_rust_project_like/
)
    


### TITLE

以下是对给定网页内容的中文总结:

这是Rust官方博客对Rust 1.82.0版本的发布公告。主要更新包括:

1. cargo工具新增了info子命令,可以显示注册表中包的信息,满足了长期存在的需求。

2. 将macOS上64位ARM(Apple Silicon)目标aarch64-apple-darwin提升为一级支持目标,表明在这个平台上的工作被完全保证。 

3. 添加了二级支持的Mac Catalyst目标,允许在Mac上运行iOS应用程序,方便测试iOS特定代码。

4. 引入了use<..>语法来精确地控制捕获哪些泛型生命周期参数,修复了之前需要使用"捕获技巧"的问题。

5. 鼓励用户使用rustup工具更新到1.82.0版本,或切换到beta/nightly版本来帮助测试。并请报告任何发现的错误。

总的来说,这个新版本带来了工具改进、平台支持扩展和语言细节完善。

[
https://blog.rust-lang.org/2024/10/17/Rust-1.82.0.html
](
https://blog.rust-lang.org/2024/10/17/Rust-1.82.0.html
)
    


### TITLE



[
https://dirkjan.ochtman.nl/writing/2024/10/18/september-2024-on-github.html
](
https://dirkjan.ochtman.nl/writing/2024/10/18/september-2024-on-github.html
)
    


### TITLE



[
https://old.reddit.com/r/rust/comments/1g6gk7k/introducing_userp_a_batteries_included_user/
](
https://old.reddit.com/r/rust/comments/1g6gk7k/introducing_userp_a_batteries_included_user/
)
    


### TITLE



[
https://old.reddit.com/r/rust/comments/1g6jfjh/announcing_rerun_019_video_and_dataframe_support/
](
https://old.reddit.com/r/rust/comments/1g6jfjh/announcing_rerun_019_video_and_dataframe_support/
)
    


### TITLE

这是一个小型应用程序,可以将Markdown文件转换为HTML文件,并通过本地主机(或使用--host标志指定的任何地址,包括0.0.0.0在本地网络上托管)为用户提供服务。如果文件被修改,它会自动刷新。作者认为它可能对一些人有用,例如在编辑自述文件时预览,或与同一本地网络上的人共享笔记。他打趣地说,也可能有人喜欢看着自己的笔记自慰。这个名为omd的应用已经发布到crates.io和GitHub上,作者希望Rust程序员们能觉得它有用。

[
https://old.reddit.com/r/rust/comments/1g6rpye/made_my_first_rust_project_finally_a_markdown/
](
https://old.reddit.com/r/rust/comments/1g6rpye/made_my_first_rust_project_finally_a_markdown/
)
    


### TITLE

以下是对该文章的中文总结:

这篇文章讨论了在Linux内核中启用自定义智能指针的工作,使它们能够像内置的智能指针一样具有"取消大小限制"和"动态分派"的特殊能力。作者解释了这些特性如何在Rust编译器中实现,以及为什么需要一个新的宏来简化内核中自定义智能指针类型的实现。

文章强调,实现这些特性对于有缺陷的Deref实现可能会破坏Rust所依赖的内存安全保证,特别是与Pin类型(用于标记不可移动的内存分配)相关的情况。因此,新的RFC要求程序员使用unsafe trait作为承诺,保证他们遵守相关文档,不会破坏Pin的行为。

最后,作者提到新的宏目前可以在nightly Rust编译器上使用,但需要进一步测试和文档工作才能稳定下来,预计很快就能完成稳定化过程。总的来说,该工作旨在让Linux内核能够充分利用Rust语言的智能指针特性,同时确保内存安全。

[
https://lwn.net/Articles/992055/
](
https://lwn.net/Articles/992055/
)
    


### TITLE

该文章介绍了一种称为TwoVec的特殊容器,它可以存储两种不同类型的对象,以节省内存空间。主要内容包括:

1. 传统的枚举方式存储不同类型对象会造成内存浪费,因为每个变体都需要填充字节对齐。

2. TwoVec使用一个位域(bitfield)来跟踪每个元素的类型,将两种类型的对象紧凑地存储在同一块内存区域中。

3. 这种设计增加了调试的复杂性,因为错误的索引计算可能会破坏数据或元数据。

4. TwoVec通过精心设计的内存布局,大大减少了存储开销,相比枚举方式可节省大量内存。

5. 代码库已开源,可在crates.io上获取。

6. 虽然设计有些"反直觉",但对于内存敏感的应用场景,这种容器提供了一种节省空间的解决方案。

总的来说,这是一种通过位级编码和细致的内存管理,在空间利用和性能之间寻求平衡的有趣尝试。

[
https://walnut356.github.io/posts/twovec-a-very-silly-container/
](
https://walnut356.github.io/posts/twovec-a-very-silly-container/
)
    


### TITLE

这是一期播客节目总结,主要内容是采访了 Zed 文本编辑器的开发人员 Conrad Irwin。Zed 是一款全新的开源文本编辑器,完全使用 Rust 语言编写,具有扩展性强、运行速度快、轻量级等优点。

Conrad 之前参与过知名高效率邮件客户端 Superhuman 的开发。他热衷于开源软件,是 Rust 语言的拥趸。在这一期节目中,Conrad 分享了 Zed 的愿景,即从零开始重建文本编辑器,以实现新一代的文本编辑体验。

除了编辑器本身,Zed 团队还自主开发了 GUI 工具包、集成了高级解析技术如 tree-sitter,并加入了多人实时协作编辑等新功能。Zed 着力于性能优化、注重细节,旨在打造一款真正面向未来的文本编辑器。该节目还介绍了 Conrad 的背景、Rust 生态中的相关工具库等其他信息。

[
https://corrode.dev/podcast/s03e01-zed/
](
https://corrode.dev/podcast/s03e01-zed/
)
    


### TITLE

以下是对该Reddit帖子的中文总结:

图像crate的新版本0.25.4刚刚发布!此版本的亮点包括:

1. 由fintelia贡献的各种优化,使无损WebP图像解码速度提高了2至2.5倍。

2. torfmaster贡献了一种近似但更快的模糊实现。

3. 现在支持方向元数据,因此可以以正确的旋转方式显示照片(由fintelia和原帖作者共同实现)。

此外,还修复了解码动画APNG和WebP图像的一些错误,以及其他一些小的改进。

值得注意的是,从元数据中读取的图像方向目前还没有在加载图像时自动应用,因为这会导致破坏性更改。但API使得正确处理方向变得非常简单。作者对实现方式和避免引入复杂依赖关系的做法感到满意。

[
https://old.reddit.com/r/rust/comments/1g6mx4b/image_v0254_brings_faster_webp_decoding/
](
https://old.reddit.com/r/rust/comments/1g6mx4b/image_v0254_brings_faster_webp_decoding/
)
    


### TITLE

这个帖子提醒Rust程序员注意到自Rust 1.80版本以来,size_of和align_of函数已经包含在prelude中了。这意味着你不需要像以前那样输入std::mem::size_of,只需输入size_of即可。帖子还指出,size_of_val和align_of_val这两个函数也同样被加入了prelude。由于这一变化没有在1.80版本的发行说明中提及,很多人可能都错过了这一更新。

[
https://old.reddit.com/r/rust/comments/1g6s4ml/psa_size_of_and_align_of_are_in_the_prelude_since/
](
https://old.reddit.com/r/rust/comments/1g6s4ml/psa_size_of_and_align_of_are_in_the_prelude_since/
)
    


### TITLE

这位作者最初认为 Rust 是个难以掌握的编程语言,远远超出了自己的能力范围。但在一次冲动之下,他决定尝试学习 Rust。虽然一开始确实挣扎不已,不仅要学习函数式编程、Rust 的类型系统,还需要用非面向对象的方式编写代码,但最终他意识到自己完全有能力掌握 Rust。

这次经历让他明白,没有什么是太难学的。之后他尝试了自己以前认为无法掌握的很多事物,比如盲打、Neovim 等。从前他会被使用 Rust、Neovim 和分体键盘的程序员给吓到,但现在他自己就成了其中一员。

作者感到自己几乎可以学习任何东西。即使最开始看起来很神秘,只要持之以恒研究,最终就会发现那只是一些代码而已。这次学习 Rust 的经历使他打消了恐惧,勇于尝试之前自认为太难的东西。

[
https://old.reddit.com/r/rust/comments/1g6kavw/learning_rust_was_the_best_thing_i_ever_did/
](
https://old.reddit.com/r/rust/comments/1g6kavw/learning_rust_was_the_best_thing_i_ever_did/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

