【Rust日报】2024-10-14


### TITLE

Clavis是一个Rust库,用于在异步流上进行安全加密通信。它利用X25519进行密钥交换,AES-256-GCM-SIV进行加密,确保传输数据的机密性和完整性,同时提供简单高效的API。

该库提供了定义自定义数据包类型的宏,支持无缝的数据序列化和反序列化。它展示了如何建立客户端和服务器之间的加密连接,发送和接收自定义数据包。该库提供了完整的API文档,涵盖了各种模块、结构体和函数,以充分利用Clavis的功能。该项目遵循MIT许可协议。

[https://github.com/pyrohost/clavis
](https://github.com/pyrohost/clavis
)
    


### TITLE

以下是对该文章的中文总结:

本文是一个三部分系列的最后一篇,旨在讲解如何在嵌入式系统上运行 Rust 代码。作者通过将自己的 range-set-blaze 项目移植到 no_std 环境的实践,总结出了9条规则:

1. 确保你的项目可以在 WASM WASI 和浏览器 WASM 环境下运行,因为这些环境与嵌入式环境有类似的约束。

2. 使用 thumbv7m-none-eabi 目标和 cargo tree 命令来识别并修复不兼容 no_std 的依赖项。

3. 在主代码中添加 #![no_std] 和 extern crate alloc;,并将 std:: 替换为 core:: 和 alloc::。

4. 使用 Cargo 特性让主代码有条件地使用 std,以保留文件相关等功能。

5. 理解测试代码始终使用标准库的原因。

6. 创建一个简单的嵌入式测试项目。

7. 使用 QEMU 运行测试。

8. 在 Cargo.toml 中为 WASM 和 no_std 添加关键词和类别。

9. 可选地使用预分配的数据类型避免使用 alloc。

10. 在持续集成测试中添加 thumbv7m-none-eabi 和 QEMU 目标。

通过遵循这些规则,开发者可以顺利地将 Rust 代码移植到资源受限的嵌入式系统中运行。

[
https://medium.com/towards-data-science/nine-rules-for-running-rust-on-embedded-systems-b0c247ee877e
](
https://medium.com/towards-data-science/nine-rules-for-running-rust-on-embedded-systems-b0c247ee877e
)
    


### TITLE

这个仓库提供了一个 try-specialize crate,它在稳定版本的 Rust 上提供了有限的、零开销的泛型上下文专门化。这个 crate 提供了一组全面的 API 来解决各种专门化挑战,减少使用不安全代码的需求。它支持对unconstrained类型、'static类型、引用类型和可变引用类型等进行专门化。

该 crate 的主要用途包括:

1. 将泛型类型特化为任何无生命周期约束的类型。
2. 将'static类型特化为任何其他'static类型。 
3. 将Sized或Unsized类型引用特化为任何无生命周期约束的类型引用。
4. 将Sized或Unsized类型可变引用特化为任何无生命周期约束的类型可变引用。
5. 特化第三方库中的泛型容器类型。
6. 在泛型复合类型中查找特定类型的值。

该 crate 通过类型比较和transmute_copy在确定类型相等时实现特化。它的实现依赖于一些 Rust 标准库未记录的行为。该仓库提供了示例展示了该 crate 的用法。

最后,该仓库比较了一些替代的 crate,并总结了它们在稳定性、API 复杂性、开销等方面的差异。

[
https://github.com/zheland/try-specialize?tab=readme-ov-file#alternative-crates
](
https://github.com/zheland/try-specialize?tab=readme-ov-file#alternative-crates
)
    


### TITLE

这个网站介绍了欧盟资助的一些与互联网和数字身份相关的开源项目。

其中包括:

1. MUSAP项目,由Ammar Bukhari领导,致力于赋予用户灵活管理数字身份的能力。

2. PostmarketOS项目,由Oliver Smith创立,是一个开源操作系统,旨在延长智能手机的使用寿命。 

3. GNU Taler系统,由Taler Systems SA公司开发,是一个保护隐私的电子支付系统,使用存储在用户设备上的数字货币进行交易,并内置了年龄限制功能。

这些项目关注数字身份、设备寿命延长和隐私保护电子支付等领域,体现了欧盟在推进开源、隐私和用户权益方面的努力。

[
https://ngi.eu
](
https://ngi.eu
)
    


### TITLE

这篇文章介绍了由NLnet基金会资助的三个相关开源项目。

1. LibrePCB 2.0 - 这是一款开源的印刷电路板设计自动化软件套件。该项目旨在开发全新的用户界面,并尝试从C++迁移到更安全的Rust编程语言。新版本将提供统一的标签窗口界面、更多的导入/导出功能、性能优化以及其他常被要求的新特性。

2. MOSFET开源SPICE模型验证测试程序 - 该项目旨在为开源工艺设计包(PDK)中的MOSFET紧凑模型建立通用的验证测试程序,以确保模型质量,并支持多种开源电路模拟器。

3. Verilog-A 提炼器 - 这是一个Python工具,可以自动将遗留的SPICE3 C语言器件模型转换为Verilog-A语言,从而提高模型在不同模拟器间的兼容性,加快开发进度。

这些项目旨在推进开源电子设计自动化工具的发展,提高可访问性,并促进学术和产业界的协作创新。

[
https://nlnet.nl/news/2024/20241014-announcing-CommonsFund-call.html
](
https://nlnet.nl/news/2024/20241014-announcing-CommonsFund-call.html
)
    


### TITLE

以下是对该GitHub仓库SniffNet的中文总结:

SniffNet是一款开源的跨平台网络流量监控应用程序,具有直观、可靠的界面。它可以让你舒适地监控互联网流量。主要功能包括:

1. 选择电脑上的网络适配器进行检查。
2. 对观察到的流量应用一组过滤条件。
3. 查看互联网流量的总体统计数据。
4. 实时查看流量强度图表。
5. 最小化时也可监视网络。
6. 将捕获报告导出为PCAP文件。
7. 识别6000多种上层服务、协议、木马和蠕虫。
8. 查找域名、AS号和远程主机所在国家。
9. 识别本地网络连接。
10. 保存您喜欢的网络主机。
11. 实时搜索和检查每个网络连接。
12. 设置自定义通知。
13. 支持多种风格和主题。

该项目还提供了详细的用户手册和故障排除指南。它使用iced这个Rust跨平台GUI库构建界面。该项目感谢所有贡献者和支持者的努力。

[
https://github.com/GyulyVGC/sniffnet
](
https://github.com/GyulyVGC/sniffnet
)
    


### TITLE

utoipa是一个为Rust REST API自动生成OpenAPI文档的crate。它采用代码优先的方法,通过简单的宏注解就能从代码中生成API文档。主要特性包括:

1. 支持OpenAPI 3.1标准,可与多种Web框架集成。
2. 支持泛型类型,能自动从代码用例递归收集模式。
3. 从处理函数参数或特性注解中识别请求体和响应体。
4. 提供多种OpenAPI可视化工具。
5. 支持类型别名定义。
6. 提供大量配置特性,如YAML序列化、单元测试调试等。
7. 开箱即用地支持多种常用类型,如chrono、time、decimal等。
8. 可在运行时通过生成的类型或trait修改OpenAPI定义。

utoipa旨在最大程度简化API文档的生成过程,使开发者能专注于编写实际的API逻辑,而不必操心手动编辑YAML或JSON文件。它追求简洁、简单和高效。

[
https://github.com/juhaku/utoipa
](
https://github.com/juhaku/utoipa
)
    


### TITLE

总结:

Gosub是一个全新的网络浏览器,它拥有自己的网络引擎。这个引擎是一个模块化系统,允许开发者轻松插入自己的组件来定制引擎的功能。这将导致未来浏览器风景更加多样化。通过可插拔的引擎,开发者可以创建自己的渲染管线、自己的JavaScript引擎,而不会受到任何单一公司或组织的议程影响。它支持选择JavaScript引擎、定制渲染管线,并且组件可以像积木一样插拔。

[
https://gosub.io
](
https://gosub.io
)
    


### TITLE

这个仓库包含了Gosub浏览器引擎的代码。Gosub是一个正在开发中的新浏览器引擎项目,旨在提供优化搜索和无限浏览的网关。该引擎包括HTML5标记器/解析器、CSS3标记器/解析器、文档树、连接JavaScript的API、配置存储、网络栈、渲染引擎和JS桥接等组件。

该项目目前处于初期阶段,还没有可用的浏览器,但已经可以解析简单的HTML页面到文档树并做初步渲染。它可以解析HTML5和CSS3文件,并在终端或渲染器中显示结果树。虽然渲染器还无法完全渲染所有内容,但已经可以渲染简单的HTML页面。

该仓库提供了多个二进制文件用于测试不同组件,如配置存储、CSS解析器、HTML解析器、渲染器和JS运行环境等。项目还提供了基准测试套件和测试工具。

此外,该引擎还可以编译成WebAssembly版本,允许在浏览器中运行。该项目欢迎新的贡献者加入,目前可以通过Zulip聊天室与开发者交流。

[
https://github.com/gosub-io/gosub-engine
](
https://github.com/gosub-io/gosub-engine
)
    


### TITLE

该内容介绍了一个名为Clavis的Rust库,用于通过加密的AES流实现简单的数据包通信。该库使用AES加密算法传输数据包,并提供了密钥交换和数据包发送接收的功能。

作者声称该库性能不错,密钥交换平均耗时100微秒,同设备间单向传输数据包平均耗时50微秒左右。不过基准测试使用了Duplex流,结果可能有些不稳定。

文中还提供了一个使用示例,展示了如何定义自定义数据包类型、建立加密连接、发送和接收数据包等。

最后,作者表示这个库目前还是非常初期的工作,欢迎其他人为它修复bug、增强安全性以及提供其他改进建议。

[
https://old.reddit.com/r/rust/comments/1g3okjo/introducing_clavis_an_encrypted_dead_simple/
](
https://old.reddit.com/r/rust/comments/1g3okjo/introducing_clavis_an_encrypted_dead_simple/
)
    


### TITLE

这是一个关于将 Rust 语言移植到嵌入式系统的经验分享。作者总结了移植过程中的一些优点和缺点。

优点包括:
1. "如果编译通过,它就可以工作"的原则在嵌入式系统上依然有效,不像在 WebAssembly(WASM)上。
2. 在移除 no_std 特性后,很多项目仍然可以通过 core 和 alloc 来使用。
3. 可以通过 QEMU 模拟器在没有实际硬件的情况下开发嵌入式系统。

缺点包括:
1. #[test] 属性无法正常工作,需要创建一个新的子项目来编写测试用例。
2. 寻找与 no_std 兼容的依赖库可能有一定挑战,不过 cargo tree 命令可以帮助解决这个问题。

作者还撰写了一篇免费文章,分享了使用模拟器设置测试环境以及其他技巧,题为"Running Rust on Embedded Systems 的 9 条规则"。总的来说,作者发现使用 Rust 开发嵌入式系统是一种有趣的体验,并已在多个微控制器项目中使用了 Rust。

最后,作者询问是否有更好的方式来测试嵌入式 Rust 代码。

[
https://old.reddit.com/r/rust/comments/1g3i5uh/porting_rust_to_embedded_systems/
](
https://old.reddit.com/r/rust/comments/1g3i5uh/porting_rust_to_embedded_systems/
)
    


### TITLE

这是一篇介绍Rust编程语言中一个名为`try-specialize`的库的帖子。该库提供了一种在稳定版Rust中进行特化的方法,可以在未约束类型、`LifetimeFree`类型、`'static`类型、引用类型和可变引用类型之间进行特化。该库确保特化在编译时执行,并在优化级别大于等于1时完全优化,不会产生运行时开销。

帖子展示了一些使用该库进行特化的简单示例,并指出虽然特化在某些优化情况下很有用,但在大多数情况下使用trait会更加惯用。最后,作者诚挚地征求社区的反馈意见。

总的来说,这个帖子介绍了一种在Rust中实现零开销类型特化的库,为Rust程序员提供了一种新的优化工具。

[
https://old.reddit.com/r/rust/comments/1g3dipy/tryspecialize_a_crate_for_limited_zerocost/
](
https://old.reddit.com/r/rust/comments/1g3dipy/tryspecialize_a_crate_for_limited_zerocost/
)
    


### TITLE

这位朋友正在学习Rust编程语言。虽然他正在阅读相关书籍并做练习,但认为如果能够构建一些自己感兴趣的东西(比如游戏)会让学习过程更有趣。他决定从头开始构建一个游戏引擎,尽管这可能是一个漫长而艰难的过程。作为一名数据开发人员,他并不介意投入大量时间而"一无所获"的挫折感。他只是希望通过一个更具体的项目来学习这门语言。因此,他向拥有更多经验的人请教相关的书籍、视频教程等资源,以指导他在Rust中开发游戏引擎。

[
https://old.reddit.com/r/rust/comments/1g3hd2z/learning_rust_dev_game_engine/
](
https://old.reddit.com/r/rust/comments/1g3hd2z/learning_rust_dev_game_engine/
)
    


### TITLE

这篇文章讲述了作者在学习 Rust 编程语言的过程中,花费了 382 个小时并创建了一个 Rust 游戏引擎,可以渲染出一个 PBR 着色的猴子模型。作者从零开始学习 Rust,期间完成了多个 Rust 项目,包括游戏引擎、预算应用程序、复杂解码器等。

文章总结了作者在学习 Rust 过程中的一些体会,比如 Rust 的所有权模型和借用检查器并不像人们说的那样难以使用,cargo 工具使测试过程变得非常顺畅,枚举类型和模式匹配让编程感觉有超能力,错误处理比 C# 更优雅。总的来说,Rust 让作者感到更容易上手,代码更加显式和明确,无需过多地在脑中保留隐式假设,让编程过程更加轻松。

[
https://medium.com/@ryanrothweiler/i-spent-382-hours-learning-rust-and-all-i-got-was-this-shiny-monkey-0f3163e06921
](
https://medium.com/@ryanrothweiler/i-spent-382-hours-learning-rust-and-all-i-got-was-this-shiny-monkey-0f3163e06921
)
    


### TITLE

这位发帖人想要制作一个跨平台的原生GUI程序,包含一些输入框、下拉菜单、按钮和一个嵌入式终端窗口用于输出命令的结果。他提出了以下限制条件:

1. 程序需要在Linux和Windows上顺利运行,因为他要为一些不太熟悉技术的亲戚制作这个程序。
2. 他不想学习Web技术,所以排除了像Tauri等基于Web的框架。
3. 需要有良好的文档和教程,因为他不太喜欢GUI编码。
4. 希望使用稳定的框架,不希望频繁面临破坏性更改。
5. 希望能够从Linux交叉编译到Windows,所以原生C/C++绑定可能不太合适。

他在areweguiyet.com上查看了可用选项,最终将选择范围缩小到slint或fltk。他列出了这两个选项的一些利弊,例如fltk是成熟技术但缺少内置终端小部件,而slint则相对较新但有更严格的许可证限制。

最后,他希望在做出选择之前,先尝试在Windows上交叉编译这两个框架的一些示例项目,了解一下难度如何。他也征求大家的其他建议或对于这两个选择的推荐意见。

[
https://old.reddit.com/r/rust/comments/1g3kh2d/what_nonweb_gui_framework_to_use/
](
https://old.reddit.com/r/rust/comments/1g3kh2d/what_nonweb_gui_framework_to_use/
)
    


### TITLE

这是一篇关于Rust基金会宣布2024年度研究员的消息。Rust基金会的社区赞助计划旨在通过财政资助、差旅津贴和培训支持等方式，资助Rust编程语言的维护者、社区成员和组织者。该计划每年都会向为Rust项目做出重大贡献的活跃成员授予研究员奖项。

2024年度的研究员分为三类:社区研究员(为期12个月,资助建立Rust社区的工作)、项目目标研究员(为期6个月,资助Rust项目目标相关工作)和项目研究员(为期12个月,资助Rust项目团队和工作组的工作)。

本文宣布了2024年度研究员的名单,包括12位社区研究员、5位项目目标研究员和6位项目研究员,他们分别来自世界各地,将在一年的任期内通过组织社区活动、改进工具、提升性能、完善文档等方式为Rust生态系统的发展做出贡献。

[
https://foundation.rust-lang.org/news/announcing-the-rust-foundation-s-2024-fellows/
](
https://foundation.rust-lang.org/news/announcing-the-rust-foundation-s-2024-fellows/
)
    


### TITLE

这是一个令人振奋的消息!作者的基于Rust编写的开源项目Sniffnet(一个网络监控工具)获得了NLnet基金会的资助。NLnet基金会支持为开放互联网做出贡献的组织和个人,资助旨在通过开放硬件、开放软件、开放标准、开放科学和开放数据来修复互联网的项目。

这一资助来自NGI Zero Commons Fund,旨在帮助交付、成熟和扩展整个技术领域的新互联网公共事业,从自由硅到中间件,从P2P基础设施到方便的终端用户应用程序。NGI Zero Commons Fund得到了欧盟"下一代互联网"计划的资助,该计划旨在为第三代及以后的互联网重新设想和重新构建,塑造一个以价值为中心、人性化和包容性的社会。

这是作者进一步改进和扩展项目的绝佳机会,也是一个强有力的信号,证明有出色的组织存在来推进开源事业的发展!

[
https://old.reddit.com/r/rust/comments/1g3k6i2/my_rustbased_project_will_be_supported_by_the/
](
https://old.reddit.com/r/rust/comments/1g3k6i2/my_rustbased_project_will_be_supported_by_the/
)
    


### TITLE

这篇文章讨论了生成器的精确设计仍然是一个有争议的话题。作者并没有试图在这里阐述一个真正的设计或表达任何立场,主要想展示我们可以在不明确提及固定(pinning)的情况下,以一种人性化的方式创建一个介于"必须固定"类型(如生成器)和"非固定"接口(如迭代器)之间的桥梁。作者认为,支持跨越生成点的借用对于人性化设计是必要的,就像在未来中是如此一样。文章还讨论了投影(projection)、Pin引用和ChatGPT在总结这篇文章时遇到的挑战等相关概念。总的来说,这是一篇思维引人入胜的文章,探讨了Rust编程语言中生成器设计的一些问题和可能的解决方案。

[
https://smallcultfollowing.com/babysteps/blog/2024/10/14/overwrite-and-pin/
](
https://smallcultfollowing.com/babysteps/blog/2024/10/14/overwrite-and-pin/
)
    


### TITLE

该帖子宣布了Rust编程语言的utoipa库的5.0.0版本发布,这是自上次发布以来最大的一次升级。新版本带来了一些重大改进和新功能,包括:

1. 完全支持泛型类型,不再需要使用旧的aliases属性。

2. 自动从使用中收集模式,无需再使用components(schemas(...))属性来声明模式。

3. 通过utoipa-config库支持Rust类型别名,提供了utoipa的构建配置选项。

4. utoipa-axum库将axum和utoipa更紧密地集成,减少了重复代码。

5. nest(...)属性支持OpenAPI的嵌套定义。

6. 在#[utoipa::path(...)]中支持定义多种HTTP方法。

7. 改进了对元组和数组的OpenAPI定义支持,借助了OpenAPI 3.1。

8. 完全符合OpenAPI 3.1规范,遵循JSON模式规范,提供了更好的类型定义支持。

总的来说,这个新版本提供了更好的泛型支持、自动模式收集、Rust别名支持、与axum的集成等多个升级,使utoipa库在生成OpenAPI文档方面更加强大和方便。

[
https://old.reddit.com/r/rust/comments/1g3kevi/utoipa_500_release_compile_time_openapi_for_rust/
](
https://old.reddit.com/r/rust/comments/1g3kevi/utoipa_500_release_compile_time_openapi_for_rust/
)
    


### TITLE

这是一个关于开源浏览器引擎 Gosub 的介绍。一年前,一个小团队开始用 Rust 语言从头开始编写这个浏览器引擎。他们的主要目标是创建一个高度模块化的引擎,允许其他开发者在此基础上构建自己的浏览器。

尽管团队人数很少,但他们在过去一年里取得了很大进展,能够渲染一些简单的网页。虽然与 Servo 或 Ladybird 等项目相比还有一定差距,但他们认为存在多种不同的浏览器引擎是很重要的,这也是从头开始这个项目的原因。

该团队正在寻找热情的开发者,一起讨论、探索和开发 Gosub 项目。项目代码库位于 https://github.com/gosub-io/gosub-engine 和 https://gosub.io。

[
https://old.reddit.com/r/rust/comments/1g3gw0l/gosub_an_open_source_browser_engine_written_in/
](
https://old.reddit.com/r/rust/comments/1g3gw0l/gosub_an_open_source_browser_engine_written_in/
)
    


### TITLE

总结如下:

KDE社区正在努力作为KDE Goals计划的一部分,改进KDE中对Rust编程语言的支持。他们正在为KDE实现Rust绑定,并改进对除C++之外第三方应用程序的构建支持。

2023年10月20日星期日,KDE Goals团队将在UTC时间18:00举行一次在线问答环节。您可以在此主题下提出任何相关问题,这些问题将确保得到回答。该活动旨在让社区更好地了解和讨论KDE对Rust的支持计划。

[
https://old.reddit.com/r/rust/comments/1g3ni5y/rust_support_in_kde/
](
https://old.reddit.com/r/rust/comments/1g3ni5y/rust_support_in_kde/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

