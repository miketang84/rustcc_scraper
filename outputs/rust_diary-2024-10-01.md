【Rust日报】2024-10-01


### TITLE

Rinja是一个基于Jinja模板引擎的Rust模板渲染引擎。它根据用户定义的结构体在编译时从模板生成Rust代码,以保存模板的上下文。Rinja具有以下特点:

1. 使用熟悉和易用的语法构建模板。
2. 受益于Rust的类型系统提供的安全性。
3. 模板代码编译到你的crate中,以获得最佳性能。
4. 可选择内置支持Actix、Axum、Rocket和warp Web框架。
5. 提供调试功能来帮助模板开发。
6. 模板必须是有效的UTF-8编码,并在渲染时产生UTF-8输出。
7. 支持模板继承、循环、条件语句、包含等功能。
8. 支持宏和变量(不可变)。
9. 内置一些过滤器,并能使用自定义过滤器。
10. 使用'-'标记来压缩空白。
11. 可选择性地转义HTML。
12. 允许自定义语法。

[https://rinja.readthedocs.io/en/stable/
](https://rinja.readthedocs.io/en/stable/
)
    


### TITLE

这个项目是一个开源的 SQLite 数据库文件格式可视化工具。SQLite 是一种广泛使用的嵌入式关系型数据库引擎,被广泛应用于移动设备、桌面软件等领域。

该工具可以解析 SQLite 数据库文件的内部结构,并以可视化的形式呈现出各种页面类型的内容,包括表的内部和叶子节点B树页面、索引的内部和叶子节点B树页面、空闲列表页面、溢出页面、记录值溢出、记录头溢出等。它同时支持十六进制、文本、混合等多种展示方式。

该工具不仅对于软件开发人员进行故障排查很有帮助,同时也适用于学习 SQLite 数据库文件格式的目的。它提供了一些预加载的示例数据库,用户也可以上传自己的数据库文件进行分析。此外,它还包含了页面视图、树视图等多种查看模式,以及控制台等功能。

[
https://github.com/torymur/sqlite-repr
](
https://github.com/torymur/sqlite-repr
)
    


### TITLE

Bacon是一个Rust代码后台检查器。它旨在最小化交互,可以在编辑器旁边运行,并在发现Rust代码中的警告、错误或测试失败时通知您。它会优先显示错误而不是警告,并优先显示第一个错误而不是最后一个错误,因此您不必滚动到顶部查找相关内容。您不需要记住命令,因为基本命令都列在底部,其他少量命令可以通过按下h键查看。

您可以通过cargo安装bacon。运行bacon命令在终端中启动它。默认情况下,它会基于cargo check命令监视源目录并显示错误和警告。您也可以通过按t键或运行bacon test命令来启动和监视测试。按c键可查看Clippy警告,按Esc键返回上一个作业。您还可以通过d键在浏览器中打开cargo doc。

您可以在bacon.toml文件中配置和启动所需的作业,如测试、特定目标编译、示例等,并在编码时查看结果。prefs.toml文件允许您定义键绑定或始终以摘要模式或换行模式启动。通过运行bacon --prefs命令可创建默认首选项文件。

总之,bacon是一个方便的Rust代码检查工具,可在后台自动检查代码并及时显示警告和错误,减少手动操作。

[
https://dystroy.org/bacon/
](
https://dystroy.org/bacon/
)
    


### TITLE

总结如下:

这篇博文介绍了作者在开发与Kubernetes交互的Rust应用kty时使用kube-rs库的体验。作者原本更倾向于使用Go语言开发Kubernetes控制器,因为Go生态中有成熟的client-go、controller-runtime和kubebuilder等库。但在尝试使用kube-rs后,作者对Rust生态在与Kubernetes交互方面的能力有了新的看法。

文章重点介绍了kube-rs提供的以下主要功能:

1. 资源CRUD操作API,比client-go更简洁,与controller-runtime类似但用起来更像Rust风格。

2. 动态API,可获取任意资源并解析为JSON值,支持递归获取所有者引用等高级操作。

3. 与Golang的Informer类似,kube-rs提供了Reflector来实时监听资源变化并缓存本地状态。

4. 良好的内存管理,提供了一些优化指南。

5. 利用Rust的宏系统,可以方便地为自定义资源生成所需代码,而不需要复杂的代码生成工具。

总的来说,作者对kube-rs的功能给予了很高的评价,认为它集成了client-go、controller-runtime和kubebuilder的优点,在Rust生态中可以轻松构建复杂的Kubernetes控制器应用。

[
https://kty.dev/blog/2024-09-30-use-kube-rs
](
https://kty.dev/blog/2024-09-30-use-kube-rs
)
    


### TITLE

以下是对该内容的中文总结:

这段话讨论了Rust开发人员使用的不同开发环境设置。作者提到,有人使用VSCode浏览项目,而用Vim编辑实际文件。还有庞大的Emacs社区,自身就是一个复杂的话题。作者自己则使用终端编辑器,配合TMux和经过定制的Neovim,以获得良好的开发体验。

总的来说,Rust开发者在选择开发环境时存在很大差异,有人喜欢在不同工具之间切换,也有人钟情于单一但高度定制的工具。不同开发者根据自身喜好和工作习惯选择最佳方案。这种多样性反映了Rust社区的开放与包容性。

[
https://old.reddit.com/r/rust/comments/1ft4fhy/so_what_do_you_use_for_rust_development/
](
https://old.reddit.com/r/rust/comments/1ft4fhy/so_what_do_you_use_for_rust_development/
)
    


### TITLE

这位开发者表达了在使用Rust编写代码时,调试方面仍有很长的路要走。他提到了在显示枚举变量和EcoString时遇到了困难。他想知道使用LLDB调试Rust代码的最佳方式。虽然他知道可以使用dbg!这个方法,但他不希望在代码库中看到大量的dbg!语句。总的来说,他希望能有更好的Rust调试体验。

[
https://old.reddit.com/r/rust/comments/1fttfte/does_anyone_know_the_latest_way_to_use_rust_lldb/
](
https://old.reddit.com/r/rust/comments/1fttfte/does_anyone_know_the_latest_way_to_use_rust_lldb/
)
    


### TITLE

这篇文章提供了三个在线购物和互联网使用时可以省钱的小贴士:

1. 使用浏览器扩展程序(如Capital One Shopping)自动比较价格和应用优惠券,避免在大型网店如亚马逊上支付过高的价格。

2. 使用广告拦截器(如Total Adblock)可以拦截banner广告、弹出窗口、视频前贴片广告等,页面加载速度加快大约两倍,同时也能拦截追踪器和跟踪cookie。高级版的adblock一般每月费用2美元左右。

3. 使用一些可信的付费调查网站(如Branded Surveys),通过填写调查问卷分享对产品和服务的看法,每月可赚20-40美元左右的零花钱。

总的来说,这三个小贴士可以帮助网上购物者和网民节省一些开支,同时获得少量收入。

[
https://old.reddit.com/user/thecouponnerd/comments/1fkmds4/im_a_blogger_who_talks_to_people_about_their/
](
https://old.reddit.com/user/thecouponnerd/comments/1fkmds4/im_a_blogger_who_talks_to_people_about_their/
)
    


### TITLE

以下是对于帖子内容的中文总结:

这是一个关于MongoDB Rust驱动程序的反馈征求帖子。作者是该Rust驱动程序的产品经理,希望听取大家对于该驱动程序的使用体验和意见。他们想了解有哪些需要改进的地方,以及哪些功能非常有用不应该更改。他希望通过征求反馈来帮助规划该驱动程序未来的开发路线图。总的来说,这是一个面向Rust社区开发者的反馈征求,旨在改进MongoDB的Rust驱动程序。

[
https://old.reddit.com/r/rust/comments/1ftp198/mongodb_rust_driver_feedback/
](
https://old.reddit.com/r/rust/comments/1ftp198/mongodb_rust_driver_feedback/
)
    


### TITLE

这篇博客文章介绍了iroh 0.26.0版本的新特性和变更。主要内容包括:

1. 文档(docs)现在默认被禁用,被视为iroh之上的协议层,而非iroh的核心部分。但用户可以在代码中轻松启用。

2. 引入了RemoteInfo,让用户更好地了解每个远程节点的地址来源,例如用户自己提供的地址、iroh自动发现的地址等。用户可以过滤出本地网络中发现的节点。

3. 列出了一些重大的API变更。

4. 修复了许多bug,添加了一些小功能。文章最后提供了iroh的相关链接,包括完整变更日志、即将到来的功能、问题反馈渠道、Discord社区等。

总的来说,这个版本加强了iroh对远程节点连接的管理和了解,并进一步明确了iroh作为底层网络库的定位。

[
https://www.iroh.computer/blog/iroh-0-26-0-Say-Hello-to-Your-Neighbors
](
https://www.iroh.computer/blog/iroh-0-26-0-Say-Hello-to-Your-Neighbors
)
    


### TITLE

这个帖子提出了一个关于Rust编程语言中错误处理的问题。作者对Rust的错误处理方面很感兴趣,想知道是否有办法禁止在代码中使用`panic`、`unwrap`或`expect`等可能导致程序崩溃的函数,而只允许在程序的某个单一位置使用这些函数。

作者的想法是强制采用适当的错误传递实践,避免在代码中随意使用可能导致崩溃的函数。这种做法可以提高程序的健壮性和可靠性,有助于更好地控制错误处理流程。

总的来说,这个问题反映了作者对Rust错误处理机制的重视,希望能够在编译或其他方式上强制执行一种规范的错误处理方式。

[
https://old.reddit.com/r/rust/comments/1ftkig8/is_there_a_way_to_enforce_prohibiting_usage_of/
](
https://old.reddit.com/r/rust/comments/1ftkig8/is_there_a_way_to_enforce_prohibiting_usage_of/
)
    


### TITLE

根据该Reddit帖子,现有的基准测试显示,rinja是目前最快的Jinja模板引擎。该帖子提供了一个基准测试链接,显示rinja的渲染速度最快。

值得注意的是,Rust语言中唯一可与rinja相媲美的Jinja模板引擎是askama,因为askama在编译时就完成了解析,因此只剩下渲染部分,这就是为什么askama和rinja比其他引擎快得多的原因。

如果想了解更多关于rinja的信息,可以查看他们的书籍或者博客文章,该帖子都提供了相关链接。

[
https://old.reddit.com/r/rust/comments/1ftx5iv/new_fastest_jinja_template_engine_in_rust/
](
https://old.reddit.com/r/rust/comments/1ftx5iv/new_fastest_jinja_template_engine_in_rust/
)
    


### TITLE

这篇文章介绍了用Rust语言开发一个Game Boy模拟器时CPU指令的相关内容。主要包括:

1. 回顾了CPU寄存器的数据结构,以及使用声明式宏来避免重复代码。

2. 添加了一些寄存器的实用函数,如获取并增加PC寄存器值的函数。

3. 介绍了标志位寄存器,使用枚举类型来表示不同的标志位。

4. 定义了指令的数据结构,包括操作码、名称、周期数、大小、修改的标志位以及执行函数等属性。

5. 解释了Rust中的生命周期概念,并使用静态生命周期来确保指令表在程序运行期间始终可访问。

6. 声明了两个常量指令表,分别存储主要指令和CB子集指令。

总的来说,这一部分着重介绍了CPU指令在代码中的具体表示和处理方式,为开发Game Boy模拟器的核心部分做准备。

[
https://medium.com/@wolferxy/rust-adventure-to-develop-a-game-boy-emulator-part-3-cpu-instructions-d6d1d727026f
](
https://medium.com/@wolferxy/rust-adventure-to-develop-a-game-boy-emulator-part-3-cpu-instructions-d6d1d727026f
)
    


### TITLE

这是一个将SQLite数据库文件格式可视化的项目。作者最初是为了更深入地研究SQLite的内部结构而开始这个项目,但后来它演变成了一个功能齐全的UI应用程序。

该项目使用Rust编程语言和Dioxus库构建,界面则采用了Tailwind和Daisy。作者对使用这些技术表示非常满意。

你可以在提供的链接中查看这个可视化SQLite文件格式的在线演示,以及查看项目的GitHub代码仓库。作者很高兴分享这个有趣的项目。

[
https://old.reddit.com/r/rust/comments/1ftua0w/visual_representation_of_ondisk_sqlite_file_format/
](
https://old.reddit.com/r/rust/comments/1ftua0w/visual_representation_of_ondisk_sqlite_file_format/
)
    


### TITLE

这篇文章是关于 Rust 的一个名为 "Cargo Watch" 的工具的现状。作者 @passcod 表示,由于缺乏时间和动力,他无法继续积极维护这个工具及其相关依赖项。他认为该工具已经过于老旧,设计不太理想,建议用户转而使用更现代的替代品 "Bacon"。

尽管如此,Cargo Watch 仍然可以正常使用,如果有人有足够的动力,也可以接手维护。作者回顾了这个项目的历史,曾经它的一部分代码甚至被剥离出来成为了 Notify 库。他对这个项目产生了一些美好的回忆,并表示虽然过去推广不力,但还是有人自发发现并使用了它。

最后,作者重申建议使用 Bacon 或 watchexec 这些更新更好的替代品,但同时也没有完全放弃 Cargo Watch。

[
https://old.reddit.com/r/rust/comments/1ftc7cj/cargo_watch_is_on_life_support/
](
https://old.reddit.com/r/rust/comments/1ftc7cj/cargo_watch_is_on_life_support/
)
    


### TITLE

这是一篇关于使用Rust语言编写Kubernetes控制器的帖子。作者之前曾尝试使用JavaScript、Python和Java等其他语言编写Kubernetes控制器和CLI,但最终都放弃了,回归到使用Golang。这一次,作者决定尝试使用Rust,结果发现体验非常棒。作者推荐了kube-rs这个库,它使得用Rust编写Kubernetes控制器变得很好。作者还分享了一篇自己的相关博文,对此做了更详细的介绍。总的来说,这篇帖子倡导使用Rust来开发Kubernetes控制器,并且分享了作者的正面体验。

[
https://old.reddit.com/r/rust/comments/1ftswk6/write_your_next_kubernetes_controller_in_rust/
](
https://old.reddit.com/r/rust/comments/1ftswk6/write_your_next_kubernetes_controller_in_rust/
)
    


### TITLE

以下是对该博客文章的中文总结:

这篇博文总结了 Cargo 1.82 开发周期中所发生的事情。主要内容包括:

1. 介绍了本周期的插件 cargo-show-asm,它可让你查看 Rust 代码编译后的汇编代码、WASM、LLVM-IR 或 MIR。

2. cargo info 功能已合并,但暂时移除了显示所有者的功能。

3. 作为 Google 夏季代码项目的一部分,正在为 Cargo 贡献改进 shell 自动补全功能的工作。

4. MSRV(最低支持 Rust 版本)感知功能进展中,Cargo 现在会报告选择了与工作区成员 MSRV 不兼容的依赖版本的情况。仍在探索理想的解决方案。

5. 增加了 cargo publish --workspace 支持一次发布多个包的功能。

6. 新增 cargo::error 构建脚本指令,可让构建脚本发出错误信息。

7. 完善了 cargo update --precise 对预发布版本的版本要求匹配规则。  

8. 继续将测试迁移到快照测试框架 snapbox。

9. 讨论了一些其他设计,如构建探针、检测未使用的依赖项等。

总的来说,这个开发周期着重于改进 Cargo 的功能、可用性和生产力。

[
https://blog.rust-lang.org/inside-rust/2024/10/01/this-development-cycle-in-cargo-1.82.html
](
https://blog.rust-lang.org/inside-rust/2024/10/01/this-development-cycle-in-cargo-1.82.html
)
    


### TITLE

谷歌透露,作为其安全设计方法的一部分,转向使用诸如Rust等内存安全语言,导致Android中发现的内存安全漏洞的百分比从6年前的76%下降到24%。谷歌表示,对新功能采用安全编码不仅可减少整体代码库的安全风险,而且使转换更加"可扩展和经济高效"。随着一段时间后新的不安全内存开发放缓,新的安全内存开发占主导地位,内存安全漏洞往往会下降。

即便新增不安全内存代码,内存安全漏洞的数量也往往会下降,这一矛盾是由于漏洞会呈指数级衰减,研究发现大量漏洞往往存在于新代码或最近修改的代码中。谷歌自2019年起优先将新开发转向内存安全语言,导致Android中发现的内存安全漏洞从2019年的223个下降到2024年的不到50个。

谷歌还注重提供Rust、C++和Kotlin之间的互操作性,而不是重写代码,作为采用内存安全语言和最终消除整个漏洞类的实用且渐进的方法。通过在新代码中采用安全编码,可利用漏洞的固有衰减,从而提高整体代码安全性并提高安全设计的有效性。谷歌还与Arm合作,发现了Pixel驱动程序代码和Arm GPU固件中的多个缺陷。

[
https://thehackernews.com/2024/09/googles-shift-to-rust-programming-cuts.html?m=1
](
https://thehackernews.com/2024/09/googles-shift-to-rust-programming-cuts.html?m=1
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

