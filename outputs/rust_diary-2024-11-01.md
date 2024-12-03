【Rust日报】2024-11-01


### TITLE

这是一个包含 TLS 证书和私钥的机密数据。其中包括:

- ApiVersion: v0.16
- Secret名称: test
- Secret类型: nanocl.io/tls  
- 不可变: false
- 数据:
    - 证书 (Certificate)
    - 客户端证书 (CertificateClient)

这些加密数据可能用于建立安全的 TLS 连接。由于包含敏感信息,因此不建议将此类数据公开分享。

[https://docs.next-hat.com/blog/nanocl-0.16
](https://docs.next-hat.com/blog/nanocl-0.16
)
    


### TITLE

以下是对该内容的中文总结:

Nanocl是一个开源的分布式系统,旨在革新开发者的工作流程。它旨在为本地开发提供一个简单的解决方案,解决在处理复杂的微服务架构时常见的CORS和cookie问题。Nanocl还可以轻松部署到生产环境。通过赋予开发者和DevOps专业人员权力,Nanocl有助于优化工作流程、增强安全性并降低运营成本。

Nanocl采用微服务架构,包括多个组件如nstore、ndaemon、nmetrics、nproxy、ncproxy、ndns和ncdns等。它使用Statefile来定义想要的状态,通过nanocl命令应用或删除状态。文档提供了Nanocl的概述、入门指南、CLI参考和守护进程参考。

该项目欢迎各种形式的贡献,包括Bug报告、功能请求、拉取请求和改进文档等。赞助商的支持使该项目得以存活和发展,赞助商将在这里展示名字或Logo。该项目还提供了现场演示,以直观展示Nanocl的功能。

[
https://github.com/next-hat/nanocl
](
https://github.com/next-hat/nanocl
)
    


### TITLE

以下是该博客文章的中文总结:

这篇博文提供了对Rust项目当前26个目标的进展情况的更新。

旗舰目标:
1. 提升异步Rust体验接近同步Rust的水平
- 正在解决"发送边界"问题并支持异步闭包语法
- 进展包括为返回类型注释添加支持、就异步闭包语法达成初步共识等

2. 解决阻碍在稳定版上构建Linux内核的主要障碍  
- 已稳定支持offset_of!宏中的结构字段
- 正在稳定对静态常量中引用的支持
- 正在研究改进内联汇编等功能以满足Linux内核需求

3. Rust 2024版本
- 编辑版本进展顺利,按计划推进
- 完成了所有优先语言项目,将带来许多令人兴奋的新特性
- 三个主要编译器变化已就绪

总的来说,这些目标都取得了不同程度的进展,为Rust的发展做出贡献。

[
https://blog.rust-lang.org/2024/10/31/project-goals-oct-update.html
](
https://blog.rust-lang.org/2024/10/31/project-goals-oct-update.html
)
    


### TITLE

这是一个关于Rust语言稳定性的Pull Request总结。主要内容包括:

1. 将会默认启用`-Znext-solver=coherence`标志,使用新一代trait solver进行隐式负重叠检查。这是rust-lang#114862问题的解决方案,并关闭了rust-lang#121848被还原的问题。

2. 介绍了新一代trait solver的背景,它旨在取代现有的evaluate、fulfill和project实现,并对类型系统产生更广泛影响。

3. 解释了相干性(coherence)和隐式负重叠检查的作用。相干性检查用于检测重叠的trait impl,以确保语言的健全性。隐式负重叠检查验证impl是否与已知或未知的其他impl重叠。

4. 强调了在相干性检查期间,trait solver的行为必须与普通类型检查不同,以保证完整性,即永远不会错误地产生类型错误。

5. 指出这一改变仅稳定了与隐式负重叠检查相关的行为,新trait solver的其他部分还未最终确定设计。

总的来说,这是一个重要的稳定性改进,影响Rust编译器的核心类型系统,旨在提高语言的健全性和可靠性。

[
https://github.com/rust-lang/rust/pull/130654
](
https://github.com/rust-lang/rust/pull/130654
)
    


### TITLE

以下是对给定内容的中文总结:

这篇博客文章讨论了一款名为Tonari的神秘产品。作者认为,尽管听起来像是一句营销口号,但"Tonari不像是技术,而更像是魔术"这句简洁的陈述实际上蕴含了很多内涵。在作者为该产品工作三年的过程中,这是他理解这款与众不同、神秘怪异的产品的最简单方式。虽然这句话并非官方的口号,但作者相信它体现了Tonari的设计理念,使其与其他科技产品有所区别,给人一种魔术般的体验。

[
https://blog.tonari.no/rust-face-detection
](
https://blog.tonari.no/rust-face-detection
)
    


### TITLE

这个问题是关于如何使用cbindgen工具从Rust结构体生成C结构体的定义。作者有一个Rust本地库，希望为这个库生成C语言绑定。他们使用cbindgen工具生成了C头文件mqtt.h,但是发现生成的头文件中缺少了对应Rust结构体S的C语言定义。他们提供了一个最小可重现的示例代码,并询问如何让cbindgen正确地在mqtt.h头文件中输出S结构体的定义。

[
https://old.reddit.com/r/rust/comments/1ghigtx/generating_c_structs_from_rust_structs/
](
https://old.reddit.com/r/rust/comments/1ghigtx/generating_c_structs_from_rust_structs/
)
    


### TITLE

这是一个探讨Rust编译器和标准库版本耦合程度的问题。作者想知道是否可以使用较新版本的标准库与稍旧版本的编译器一起工作,或者反过来,将较新版本的编译器与略旧版本的标准库一起编译。

作者承认两者之间的兼容性范围可能很小,并猜测官方的保证答案可能是"不可以"。但从理论上来说,如果自行编译标准库的话,或许可以实现这种异构组合。

总的来说,这是一个出于好奇提出的技术探索性质的问题,作者自己也没有实际的使用场景。

[
https://old.reddit.com/r/rust/comments/1ghdnny/is_it_possible_to_use_a_different_version_of_the/
](
https://old.reddit.com/r/rust/comments/1ghdnny/is_it_possible_to_use_a_different_version_of_the/
)
    


### TITLE

根据内容总结,美国网络安全与基础设施安全局(CISA)和联邦调查局(FBI)发布了一份关于软件开发不良做法的报告,其中警告使用像C和C++这样不安全的编程语言开发关键基础设施软件存在严重风险。报告将不良做法分为三类:产品属性、安全功能以及组织流程和政策。

报告鼓励软件制造商避免这些不良做法,并制定降低内存安全漏洞的路线图。从2026年1月1日起,对于使用不安全语言的现有产品,如果没有公布内存安全路线图,将被视为危险做法。报告还规定到那时必须消除管理员账户的默认密码。

这些最后期限标志着从建议转向期望标准的转变。报告还指出,一些"极其危险"的做法仍然存在于关键基础设施中,如默认密码、SQL注入漏洞、缺乏基本入侵检测等。针对开源软件,报告建议特别关注。总的来说,这是美国政府针对关键软件开发的最严厉警告。

[
https://thenewstack.io/feds-critical-software-must-drop-c-c-by-2026-or-face-risk/
](
https://thenewstack.io/feds-critical-software-must-drop-c-c-by-2026-or-face-risk/
)
    


### TITLE

以下是对内容的中文总结:

该帖子来自Nanocl的开发者,分享了该项目在过去一年中的一些新进展。主要更新包括:

1. 引入了"jobs"功能,可以执行一系列命令,例如生成SSL/TLS证书、备份数据库等,并可以单次执行或定期调度。

2. 现在可以为应用和作业添加环境变量,安全地挂载TLS/SSL证书,实现与代理的端到端SSL/TLS通信。

3. Nanocl现已完全支持Docker Desktop,尤其是对macOS用户而言,方便在本地运行和管理应用程序。

4. 还有其他一些增强,详情可查看更新日志和博客文章。

作者对社区的支持表示衷心感谢,这为他带来了一些演讲的机会。下一步计划集成网络mesh功能,欢迎有相关经验的人贡献力量。作者也愿意指导有兴趣学习Rust和云技术的人,可以加入Discord社区。

[
https://old.reddit.com/r/rust/comments/1ghal5w/nanocl_the_rust_developerfriendly_kubernetes/
](
https://old.reddit.com/r/rust/comments/1ghal5w/nanocl_the_rust_developerfriendly_kubernetes/
)
    


### TITLE

这是宣布fjall-rs项目2.3版本的更新说明。主要包括以下几个改进:

1. 提高了随机键插入的写入扩展能力。之前的段合并选择算法较为简单,导致在大数据集下会产生写入瓶颈。新版本改进了该算法,能更合理地选择合并段,大幅提升了随机写入的扩展性能。

2. 默认启用了lz4压缩算法的非安全模式,提高了约5-15%的解压缩速度。

3. 修复了某些工作负载下点读性能下降的问题,之前是由于控制流程错误导致的多余查找。

4. 通过预分配更大的日志文件,减少了小值同步写入时的延迟抖动。

5. 其他一些小的性能优化和bug修复。

总的来说,这个版本主要针对写入扩展性、压缩速度、点读性能等方面做了优化,提升了系统的整体性能表现。

[
https://fjall-rs.github.io/post/fjall-2-3/
](
https://fjall-rs.github.io/post/fjall-2-3/
)
    


### TITLE

以下是对该内容的中文总结:

Sycamore v0.9.0 是该反应式Rust UI框架发布的最大更新版本。它包括大量新功能和改进,主要包括:

1. 全新网站和域名sycamore.dev,界面和文档得到重新设计。

2. 反应性系统升级到v3版本,通过消除生命周期和cx参数,简化了反应式数据类型的使用。

3. View系统升级到v2版本,移除了GenericNode和Html特性,减少了代码开销。

4. 社区大幅增长,GitHub星标从1000增至2800,Discord成员从350增至626人,下载量达到15.1万次。

5. 发布了迁移指南,以帮助从v0.8版本升级。

6. 文档进行了重写和更新,增加了介绍性内容来帮助新手入门。

总的来说,这是一个重大升级,提高了Sycamore的易用性和开发效率,同时社区持续快速增长。

[
https://sycamore.dev/post/announcing-v0-9-0
](
https://sycamore.dev/post/announcing-v0-9-0
)
    


### TITLE

以下是对内容的中文总结:

这个帖子提出了一个关于Rust语言中SIMD(单指令多数据)特性的问题。作者对于编写一个快速的Python包很感兴趣,希望能够利用Rust的SIMD功能。

Rust目前有两种SIMD crate(包),一种是平台特定的std::arch,另一种则是可移植的std::portable_simd。除了可移植性之外,使用std::arch是否有任何其他缺点?如果有,std::portable_simd什么时候能进入稳定版本?

作者希望能够了解在编写跨平台代码时,选择这两种SIMD包的利弊,以及可移植SIMD包的开发进展,从而做出正确的技术选择。

[
https://old.reddit.com/r/rust/comments/1ghb8lf/when_can_we_expect_portablesimd_to_be_in_stable/
](
https://old.reddit.com/r/rust/comments/1ghb8lf/when_can_we_expect_portablesimd_to_be_in_stable/
)
    


### TITLE

根据Reddit帖子内容,总结如下:

原帖子作者面临一个艰难的项目,需要求助。他在像Upwork这样的平台上寻找合作者,但大多都是一些与"人工智能、区块链、加密货币"等概念相关的项目,与他的需求不符。

因此,作者向Reddit求助,希望能找到合适的Rust程序员参与他的项目。这个帖子在Rust社区引起了热烈反响,作者收到了大量私信,不过由于反馈过多,他暂时将此事搁置。

总的来说,这是一个寻求Rust程序员合作伙伴的请求,并获得了Rust社区的大量关注和回应。

[
https://old.reddit.com/r/rust/comments/1gh31df/where_to_hire_rust_nerds/
](
https://old.reddit.com/r/rust/comments/1gh31df/where_to_hire_rust_nerds/
)
    


### TITLE

这是一位软件工程师的困扰。最初他学习了Rust编程语言,但为了找到工作,不得不转向学习.NET和面向对象编程(OOP)。经过几个月的学习,他最终找到了一份Python/JS开发人员的工作。

虽然学习OOP帮助他应对了面试,但他的内心仍然渴望使用Rust编程。然而,他所在的国家完全没有Rust相关的工作机会,所以未来的工作只能是远程或自由职业。他并不介意远程工作,但担心很难找到Rust相关的工作机会。

他现在在权衡是继续从事.NET开发工作,还是追随内心的热情,专注于Rust并寻求远程或自由职业的机会。他希望能得到大家的建议和意见。

[
https://old.reddit.com/r/rust/comments/1ghg1da/should_i_stick_to_rust/
](
https://old.reddit.com/r/rust/comments/1ghg1da/should_i_stick_to_rust/
)
    


### TITLE

这篇文章宣布了 Rust 编译器团队的重组和团队成员的贡献认可。主要内容包括:

1. 编译器团队合并了 RFC 3599,对团队结构进行了重组,以确保能够支持 Rust 编译器的长期维护。

2. 随着 Rust 项目的发展,编译器贡献量大幅增加,需要更多人参与维护工作,因此重组团队至关重要。

3. 团队成员分为"维护者"和"成员"两种角色。为团队做出持续贡献一年以上的成员可选择成为"维护者"。

4. 文章列出并认可了团队中主要贡献者的工作,包括改进诊断、优化、代码生成、特性实现等方面的贡献。

5. 重组旨在确保团队流程高效,并适当认可每个成员的贡献,在赋予身份地位与保持高效工作之间取得平衡。

总的来说,这是一次旨在应对团队发展需求、提高运作效率的重组,同时也是对长期贡献者工作的认可和肯定。

[
https://blog.rust-lang.org/inside-rust/2024/11/01/compiler-team-reorg.html
](
https://blog.rust-lang.org/inside-rust/2024/11/01/compiler-team-reorg.html
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

