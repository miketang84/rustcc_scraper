【Rust日报】2024-10-30


### TITLE

以下是对给定内容的中文总结:

这个GitHub仓库包含了一个用Rust语言重写的"Rusty bandwidth"代理服务器项目。这个版本相较于原始的"Bandwidth Hero"代理服务器,在运行时消耗的系统资源大大减少(仅占用15MB内存)。由于只需要运行一个可执行文件,因此更易于自行托管。

默认情况下,该代理服务器使用8080端口,并为缓存分配512MB内存,不过这两个参数都可以通过启动参数进行修改。它支持AVIF和WebP图像压缩模式。由于WebP编码速度更快,作者推荐使用"--webp"参数启用WebP模式。目前还不支持硬件加速编码。

[https://github.com/furdiburd/rusty-bandwith
](https://github.com/furdiburd/rusty-bandwith
)
    


### TITLE

这是一个开源的浏览器扩展程序,名为Bandwidth Hero。它的作用是在浏览网页时压缩页面上的所有图像,从而减少数据流量消耗。它的工作原理是拦截所有图像加载请求,将图像URL发送到数据压缩服务,服务会下载原始图像并将其转换为低分辨率的WebP或JPEG格式,然后返回给浏览器显示。使用该扩展需要自行搭建数据压缩服务,扩展提供了相关文档说明。该项目由Anatoliy Yastreb用爱心制作,托管在GitHub Pages上,使用orderedlist主题。

[
https://bandwidth-hero.com/
](
https://bandwidth-hero.com/
)
    


### TITLE

这是一个用Node.js编写的代理服务器,主要功能是压缩传输的图像到低分辨率,以节省网络带宽。它是一个开源项目,托管在GitHub上(https://github.com/ayastreb/bandwidth-hero-proxy)。

该代理服务器可以部署到云平台(如Heroku)上运行。通过向代理发送图像URL,代理会下载原始图像,压缩成低分辨率,然后返回压缩后的图像。这样可以大幅减小图像的传输流量,适合在带宽有限的网络环境下加速网页加载。

这个项目提供了源代码,使用说明,部署步骤等,是一个对网站性能优化有益的开源解决方案。

[
https://github.com/ayastreb/bandwidth-hero-proxy
](
https://github.com/ayastreb/bandwidth-hero-proxy
)
    


### TITLE

该github存储库crud_routers是一个自动生成CRUD路由的库,与ORM和Web框架无关。它受fastapi-crudrouter的启发而开发。该库可以与Axum和Diesel等Web框架和ORM配合使用,只需几行代码就可生成所需的CRUD路由。

主要特性包括:

1. ORM无关性,支持Diesel和Sea-ORM,可扩展支持其他ORM
2. Web框架无关性,支持Axum和Actix,可扩展支持其他框架
3. OpenAPI支持,可生成API规范文档
4. 自动分页支持
5. 可选择禁用特定的CRUD路由
6. 可设置URL前缀和API标签

项目还计划添加中间件支持和创建文档网站等功能。总的来说,crud_routers可以极大简化Web应用的CRUD路由开发,提高开发效率。

[
https://github.com/furkan-guvenc/crud_routers
](
https://github.com/furkan-guvenc/crud_routers
)
    


### TITLE

这是一个名为fastapi-crudrouter的Python包,它是FastAPI的扩展。它可以自动为您的模型生成和记录CRUD(创建、读取、更新、删除)路由,从而节省重复编写通用CRUD路由的时间。主要功能包括:

1. 基于传入的模型和可选的数据库连接,快速生成完整的CRUD路由。
2. 支持自动分页、自定义创建和更新模式、动态生成模式等高级功能。
3. 与FastAPI依赖注入机制集成,支持内存、SQLAlchemy、数据库、Gino、Ormar、Tortoise ORM等后端。
4. 根据OpenAPI规范自动为生成的路由生成文档。

总的来说,这个包旨在提高FastAPI应用程序的开发效率,加快原型设计和迭代速度,减少重复工作量。它被设计为高性能、经过测试且可用于生产环境。

[
https://github.com/awtkns/fastapi-crudrouter
](
https://github.com/awtkns/fastapi-crudrouter
)
    


### TITLE

总结:

这个项目是一个名为Managarr的终端用户界面(TUI)和命令行界面(CLI)工具,用于管理家庭媒体服务器(HTPC)软件。它用Rust语言编写,支持Radarr、Sonarr、Readarr、Lidarr、Prowlarr、Whisparr、Bazarr和Tautulli等软件。

主要功能包括:

- 查看媒体库、下载、收藏及黑名单
- 查看影片/电视剧详情、历史、文件信息、演员等
- 搜索库、添加/删除影片/剧集
- 手动搜索、刷新、扫描磁盘
- 编辑影片/收藏夹/索引器
- 管理标签、根目录、黑名单
- 查看日志、任务队列、事件、更新
- 手动触发计划任务

该工具可通过Cargo或Docker安装。除了TUI界面外,还提供了CLI命令,方便自动化任务和脚本集成。它使用默认设置连接各服务器,但需要输入API令牌等配置。总的来说,这是一个方便集中管理多个HTPC软件实例的工具。

[
https://github.com/Dark-Alex-17/managarr
](
https://github.com/Dark-Alex-17/managarr
)
    


### TITLE

以下是对Reddit帖子的总结:

这是一个关于Rust编程语言的帖子。一位年轻的程序员自豪地分享了他开发的一个名为Ygen的项目,这是一个独立于LLVM的IR(中间代码表示)编译器。

其他用户对他年纪轻轻就能完成如此令人印象深刻的项目表示祝贺。用户提出了一些疑问,比如Ygen为什么偏离LLVM IR的设计,以及是否计划支持全部的LLVM IR节点。

开发者Cr0a3解释说,Ygen的IR在某些地方简化了LLVM IR的设计,合并了一些节点。他支持55%的LLVM IR节点功能,但不一定采用相同的实现方式。

用户建议Cr0a3可以进一步模仿LLVM IR,这样就能借助LLVM的测试用例来测试Ygen,甚至尝试编译一些简单的Rust代码。Cr0a3表示这是个好主意,他经常通过比对LLVM输出来测试Ygen,并考虑增加解析LLVM IR到Ygen IR的支持。

总的来说,这是一个令人鼓舞的项目,体现了开发者的热情和能力,同时也得到了社区的认可和建议。

[
https://www.reddit.com/r/rust/comments/1gctr27/ygen_now_landed_in_godbolt/
](
https://www.reddit.com/r/rust/comments/1gctr27/ygen_now_landed_in_godbolt/
)
    


### TITLE

这个仓库包含了ygen项目的源代码。ygen是一个用于构建现代编译器的工具包,使用了类似LLVM的API。它的主要目标是简单性,API与LLVM非常相似,易于上手且无需安装dll或构建。该项目目前处于早期开发阶段,仅适用于玩具编译器。

该仓库提供了一个简单的示例,展示了如何使用ygen来构建一个add函数。目前ygen支持x86-64和正在开发的wasm架构。该项目由Cr0a3拥有,并根据Apache 2.0许可证授权。

[
https://github.com/Cr0a3/ygen
](
https://github.com/Cr0a3/ygen
)
    


### TITLE

这个项目是一个使用 Tauri、Rust 和 Svelte 构建的现代跨平台系统监视器,名为 NeoHtop。它具有以下主要功能:

1. 实时监控进程
2. 跟踪 CPU 和内存使用情况
3. 美观的现代 UI,支持深色/浅色主题
4. 进程搜索和过滤
5. 固定重要进程
6. 进程管理(杀死进程)
7. 按任意列排序
8. 自动刷新系统统计数据

技术栈包括前端使用 SvelteKit 和 TypeScript,后端使用 Rust 和 Tauri,样式使用 CSS 变量进行主题化,图标使用 FontAwesome。

开发环境需要 Node.js (v16 或更高版本)、Rust (最新稳定版)和 Xcode 命令行工具(仅适用于 macOS)。安装依赖、运行开发模式、构建生产版本和构建通用 macOS 二进制包的命令也在总结中给出。

[
https://github.com/Abdenasser/neohtop
](
https://github.com/Abdenasser/neohtop
)
    


### TITLE

NeoHtop是一款现代的、跨平台的系统监视器软件,具有以下特点:

1. 实时监控系统进程,资源使用率低。
2. 拥有美观的用户界面,支持自动深色/浅色主题切换。
3. 智能搜索功能,可以快速过滤查找进程。
4. 进程置顶功能,可以固定重要进程便于监控。
5. 使用Rust语言开发,性能优化,内存占用低。
6. 可查看并管理进程的详细信息。

该软件适用于macOS系统,提供Intel和Apple Silicon芯片的版本,安装使用简便。总的来说,NeoHtop是一款资源占用低、现代化界面且功能丰富的进程监视工具。

[
https://abdenasser.github.io/neohtop/
](
https://abdenasser.github.io/neohtop/
)
    


### TITLE

该帖子询问如何编写一个Rust宏来生成一些方法,这些方法封装了结构体中的某个字段的操作。具体来说,原贴提供了一个 `User` 结构体的示例,其中包含一个 `history` 字段。原贴希望通过某种方式(可能是一个宏 `add_history_code!`)自动为 `User` 实现几个与 `history` 相关的方法,如 `date_created()`、`date_last_updated()` 和 `update()`。这些方法本质上是对 `history` 字段的封装,但需要在多个结构体中重复实现,因此希望通过宏来自动生成。

[
https://old.reddit.com/r/rust/comments/1gfsdz2/how_to_pass_a_struct_field_into_a_macro/
](
https://old.reddit.com/r/rust/comments/1gfsdz2/how_to_pass_a_struct_field_into_a_macro/
)
    


### TITLE

该内容介绍了一个名为 Bandwidth Hero 的浏览器扩展程序,它的目的是实时对访问的网站图片进行编码,从而节省带宽,在使用移动数据时特别有用。这个扩展程序的公共实例已经停止运行,其基于 Node.js 的服务器端代码效率低下,在无任何负载时就占用了 1.5GB 内存。

作者决定用 Rust 语言重写这个服务器端,目标是提高速度、降低资源占用,使其能够运行在低端设备或廉价云服务器上。现有的唯一选择是自行托管后端服务器,否则这个扩展程序就无法使用。如果有足够需求,作者可能会提供一个公共实例。

该项目目前版本为 2.0.2,之前尝试使用 Avif 代替 WebP 格式未果,最近进行了大规模重写。作者希望获得反馈和错误报告,以帮助项目进一步改进。项目地址是 https://github.com/furdiburd/rusty-bandwith。

[
https://old.reddit.com/r/rust/comments/1gfqi84/rusty_bandwidth_the_original_bandwith_hero/
](
https://old.reddit.com/r/rust/comments/1gfqi84/rusty_bandwidth_the_original_bandwith_hero/
)
    


### TITLE

这是一个介绍名为crud_routers的Rust语言库的帖子。该库可自动生成CRUD(创建、读取、更新、删除)路由,功能类似于Python的fastapi-crudrouter。它不仅限于某个特定的Web框架,支持Axum和Actix等API服务器,也支持Diesel和Sea-orm等ORM用于数据库操作。借助于utoipa,它还支持OpenAPI,可以快速生成包含所有CRUD操作的Swagger页面。尽管这是0.1版本,但已经非常实用。作者邀请大家试用这个库并提供反馈意见。该库的GitHub仓库链接是https://github.com/furkan-guvenc/crud_routers。

[
https://old.reddit.com/r/rust/comments/1gfqicr/introducing_crud_routers/
](
https://old.reddit.com/r/rust/comments/1gfqicr/introducing_crud_routers/
)
    


### TITLE

总结如下:

作者在近3年时间内使用Rust语言开发了一个名为Managarr的项目,这是他的第一个Rust项目,旨在帮助他更好地学习这门语言。他说通过这个项目,他意识到Rust是他最喜欢的编程语言。

Managarr是一个终端界面(TUI)和命令行界面(CLI)工具,用于管理Servarr实例。目前的alpha版本仅支持Radarr。该版本还没有实现所有功能,如管理质量配置文件或质量定义等。

作者分享了几张Managarr的截图,展示了它的界面和功能。除了TUI界面外,Managarr还可以作为Radarr的CLI使用,比如搜索新电影或通过TMDB ID添加新电影等。文中给出了几个CLI命令示例。

总的来说,这是作者基于Rust语言开发的第一个可用项目Managarr的alpha版本发布,展示了TUI和CLI两种界面管理Radarr实例的功能。

[
https://old.reddit.com/r/rust/comments/1gfzbju/i_present_managarr_a_tui_and_cli_to_manage_your/
](
https://old.reddit.com/r/rust/comments/1gfzbju/i_present_managarr_a_tui_and_cli_to_manage_your/
)
    


### TITLE

总结:

这位Reddit用户一直在用Rust书籍辅助学习Rustlings练习。他认为在开始自己的Rust项目之前,这种结合Rustlings练习和Rust书籍的学习方式是最佳途径。这种学习方式让他感到有趣实用、实践性强且专注,是一种非常好的Rust入门方法。

[
https://old.reddit.com/r/rust/comments/1gfvf58/rustlings_the_rust_book_for_the_win/
](
https://old.reddit.com/r/rust/comments/1gfvf58/rustlings_the_rust_book_for_the_win/
)
    


### TITLE

这位用户正在寻找一些关于Rust编程语言的高质量技术深入探讨/教程视频。与Java和Python相比,这样的Rust视频资源似乎较少。虽然有官方的Rust频道,但那里的演讲大多来自于聚会等,质量相对不如Java的DevOps会议和Python的pycon/pydata大会。用户希望能获得一些针对Rust编程语言的深度技术分享视频的推荐。

[
https://old.reddit.com/r/rust/comments/1gfse67/good_technical_deep_dive_talks_on_rust_on_youtube/
](
https://old.reddit.com/r/rust/comments/1gfse67/good_technical_deep_dive_talks_on_rust_on_youtube/
)
    


### TITLE

这是一封来自Rust社区的邮件,作者 Cr0a3 目前正在开发一个名为ygen的新代码生成库。ygen受到LLVM的启发,目前已经支持55%的LLVM IR节点。该库目前支持x64和WebAssembly(WIP)后端。

作者希望能够与社区合作一起实现一些优化,特别是Mem2reg优化和循环分析及优化。由于优化方面的知识存在缺口,作者希望能够在实现优化的同时也学习相关的知识。为了方便沟通和协作,作者创建了一个Discord服务器,欢迎感兴趣的人加入讨论和贡献代码。

[
https://old.reddit.com/r/rust/comments/1gfjwu9/ygen_is_looking_for_contributors/
](
https://old.reddit.com/r/rust/comments/1gfjwu9/ygen_is_looking_for_contributors/
)
    


### TITLE

这位Reddit用户分享了他用Rust和Tauri构建的第一个项目 - 一个为macOS设计的原生任务管理器neohtop,旨在带来类似于htop的桌面体验。该应用具有进程监控、CPU/内存使用情况查看、原生黑暗模式UI等功能。他表示使用Rust和Tauri的开发体验很棒,性能出色,尽管经历了与Apple认证过程有趣的挣扎。他使用了Rust、Tauri、Svelte和sysinfo等技术栈。最后,他分享了GitHub仓库和网站链接,并希望听到大家的反馈,也想知道其他人使用Tauri开发桌面应用的经历如何。

[
https://old.reddit.com/r/rust/comments/1gfzrdx/my_first_attempt_at_tauri/
](
https://old.reddit.com/r/rust/comments/1gfzrdx/my_first_attempt_at_tauri/
)
    


### TITLE

这篇文章讲述了一个名为Barbara的工程师在尝试将她的异步Rust Web服务从使用epoll驱动转换为io_uring驱动时遇到的问题。主要内容如下:

1. Barbara最初编写了一个简单的TCP服务器示例,使用monoio这个支持epoll和io_uring的运行时库。代码看起来与使用Tokio时没什么区别。

2. 为了增加处理超时的功能,Barbara使用了select!语句,允许服务器在一定时间无连接时执行其他任务。这在io_uring下也工作正常。

3. 但是当Barbara真正将服务部署到生产环境后,发现有些客户端请求从未被处理。进一步调查发现,尽管客户端线程能正常连接,但服务器主线程一直在触发超时,无法正常接收新连接。

4. 文章指出,这是因为使用io_uring驱动程序时,存在潜在的TCP连接泄漏问题,而在epoll下则没有这个问题。而且这是所有io_uring运行时库都存在的通病。

5. 最后作者呼吁Rust社区重视这个问题,因为它违背了Rust"安全无数据竞争"的保证,可能会导致性能下降和连接受限等严重后果。

[
https://tonbo.io/blog/async-rust-is-not-safe-with-io-uring
](
https://tonbo.io/blog/async-rust-is-not-safe-with-io-uring
)
    


### TITLE

该博客文章总结了作者在将一个现有的 Rust 项目从较旧的 Rust 版本重写为最新版本时的一些经验教训。主要内容包括:

1. 明确项目的目标和期望,对比重写和渐进式重构的利弊。

2. 利用最新的 Rust 编译器版本及相关工具(如 miri)来检测代码中的未定义行为(Undefined Behavior)。

3. 审视代码库中的技术债务,确定需要重构的部分。逐步重构关键部分,而非一次性全部重写。

4. 注意处理好并行安全(data race)、锁顺序deadlock等多线程相关的问题。

5. 合理使用异步编程模型,避免过度同步等反模式。

6. 密切关注编译器升级的变化,及时采纳新的语言特性和最佳实践。

7. 重写过程中注重测试覆盖率,并保持向后兼容性。

8. 与社区保持沟通,促进经验交流和最佳实践的分享。

文章还分享了一个实际的未定义行为示例,体现了 miri 工具在发现这类潜在 bug 中的重要作用。

[
https://gaultier.github.io/blog/lessons_learned_from_a_successful_rust_rewrite.html
](
https://gaultier.github.io/blog/lessons_learned_from_a_successful_rust_rewrite.html
)
    


### TITLE

该Reddit帖子提出了一个疑问:为什么涉及到的高质量工具几乎都是用Rust编写的?作者表示,在尝试各种计算机工具的过程中,每当发现一个体验良好、流畅、漂亮的工具时,几乎都是用Rust编写的,如Dufs文件服务器和GlazeWM窗口管理器。作者怀疑是否是Rust语言吸引了优秀的开发者,还是仅仅是一个幸运的巧合。该帖子反映了Rust在制作高质量工具方面的突出表现,引发了人们对Rust语言和社区的思考。

[
https://old.reddit.com/r/rust/comments/1gfs7zz/sincere_question_what_is_up_with_all_these_great/
](
https://old.reddit.com/r/rust/comments/1gfs7zz/sincere_question_what_is_up_with_all_these_great/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

