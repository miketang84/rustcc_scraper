【Rust日报】2024-09-25


### TITLE

这是一个使用Rust语言和SDL2库构建的简单Web游戏示例。游戏允许玩家使用方向键移动一个图像,按Enter键将背景切换为黑色,按空格键将背景切换为白色。该项目旨在作为使用emscripten将Rust和SDL2游戏移植到Web的起点。

文档提供了以下主要内容:

1. 构建说明,包括如何构建Web版本和本地版本。
2. 如何自定义项目名称和HTML游戏外壳。
3. 图像资源的使用说明。
4. 另一个链接,介绍了使用Bevy游戏引擎构建传统地牢爬行游戏的教程,包括代码组织、事件处理等内容。

总的来说,这个示例项目展示了如何使用Rust、SDL2和Emscripten将简单游戏移植到Web上运行,并提供了相关的构建说明和定制方法。同时还链接了另一个使用Bevy引擎构建地牢游戏的教程资源。

[https://github.com/ALEX11BR/emscripten-functions/tree/main/examples/simple-game
](https://github.com/ALEX11BR/emscripten-functions/tree/main/examples/simple-game
)
    


### TITLE

以下是对给定内容的中文总结:

该内容讨论了 Bevy 游戏引擎中查询实体组件的方式。`Query<&Position, With<Player>>`允许访问所有同时包含 Position 和 Player 组件的实体,并以只读方式暴露它们的 Position 组件。`Query<(&Position, &mut Transform), Without<Player>>`则允许访问所有包含 Position 和 Transform 组件但不包含 Player 组件的实体,以只读方式暴露 Position 组件,并以读写方式暴露 Transform 组件。这种查询方式使得开发者能够高效地获取和操作游戏中的实体及其组件。

[
https://oneirical.github.io/2-tiles-to-run-around-in/
](
https://oneirical.github.io/2-tiles-to-run-around-in/
)
    


### TITLE

这篇博客文章介绍了如何使用Rust编程语言和Bevy游戏引擎开发传统的roguelike游戏的第一步,即绘制玩家角色。主要内容包括:

1. 解释了传统roguelike游戏的特点和吸引力。

2. 介绍了Bevy的基本概念,如App、实体(Entity)、组件(Component)和系统(System)。

3. 创建了一个新的Rust项目,并在其中添加了Bevy依赖库。

4. 定义了"生物"(Creature)组件包,用于表示玩家和其他生物。

5. 编写了生成玩家角色的system函数。

6. 添加了摄像机系统,让玩家角色可见。 

7. 引入了精灵表(SpriteSheet)和图集(TextureAtlasLayout)的概念,用于管理游戏中所有生物的精灵图像。

8. 将精灵表相关组件整合到"生物"包中。

总的来说,这是一个逐步引导读者使用Bevy开发roguelike游戏的教程,本节着重于绘制和显示玩家角色这一基础。

[
https://oneirical.github.io/1-drawing-the-player/
](
https://oneirical.github.io/1-drawing-the-player/
)
    


### TITLE

本文是作者Alice I. Cecile对于她加入Bevy Foundation工作后的一些体会和反思。主要内容包括:

1. 从爱好到全职工作的转变,工作时间和方式的调整。她采取了较为灵活的工作安排,以完成重点任务和处理大量小任务的平衡。

2. 梦想的工作仍是工作,有其优缺点。优点包括有意义的事业、良好的薪酬和学习机会等;缺点是公开程度高、工作生活界限模糊、仅靠捐赠维持运营等。

3. 不断学习是必须的,要掌握各个领域的知识。

4. 推行Working Group自我组织模式,让贡献者能更好地参与并推动项目进展。

5. 未来计划着手改善Bevy UI解决方案,使其更易用和功能更完善。

6. 总的来说,虽然梦想工作带来了新挑战,但作者对于能为开源项目和游戏开发社区做出贡献而感到自豪和充实。

[
https://bevyengine.org/news/dream-job/
](
https://bevyengine.org/news/dream-job/
)
    


### TITLE

概要:

本文批评了Rust游戏引擎Bevy的一些缺陷。主要批评点如下:

1. Bevy的版本更新太频繁,给项目的代码维护带来了很大困扑。教程和资源很快就过时了。

2. Bevy高度依赖于第三方crate(类库),如果维护者停止更新,将影响整个项目。

3. Bevy的查询系统(Query)虽然高效,但类型变得极其臃肿复杂。同时Rust的借用规则也给操作带来了麻烦,导致代码难以模块化。

4. Bevy贡献者的工作效率太高,版本迭代过快,无法跟上节奏。

尽管作者对Bevy有诸多不满,但他打算通过夸张错误的方式,引发网友的反馈和建议,间接获得提高代码质量的机会。总的来说,这篇文章是一种诙谐而巧妙的方式,批评同时也渴望改进。

[
https://oneirical.github.io/bevyrage/
](
https://oneirical.github.io/bevyrage/
)
    


### TITLE

这是一个使用Rust编写的游戏引擎项目,名为Vent-Engine。该引擎的目标是非常快速和用户友好。

主要目标包括:

1. 使用Rust语言编写,尽量避免外部语言绑定。
2. 通过使用Vulkan本地API实现高性能优化。
3.设计上注重易用性。 
4. 支持跨平台(Windows、Linux等)。

该项目目前处于高度开发阶段。要运行该引擎,需要先安装Rust编译器和支持Vulkan的GPU。然后克隆代码仓库并运行cargo命令进行编译和运行。

该项目欢迎任何形式的贡献。目前已经支持Windows和Linux平台,对于其他平台的支持状态则不确定。

该项目使用Apache 2.0许可证。

[
https://github.com/ventengine/Vent-Engine
](
https://github.com/ventengine/Vent-Engine
)
    


### TITLE

以下是对https://github.com/monadbobo/skiplist-rust这个Rust语言实现的跳表(SkipList)数据结构项目的中文总结:

这个项目提供了一个并发、无锁的跳表实现,可用于高效的键值存储和检索。主要特性包括:

- 无锁并发操作
- 高效的插入和查找
- 支持迭代遍历
- 可配置最大高度和分支因子
- 使用安全的Rust编写,只有少量unsafe代码

使用方式是在Cargo.toml中添加依赖,然后在Rust代码中使用SkipList相关API进行键值插入、检查存在性、遍历等操作。

该项目的API包括SkipList的创建、插入键值、检查键是否存在、获取迭代器等;SkipListIterator则提供了创建、检查有效性、获取当前键值、前移后移、查找目标键值等操作。

性能方面,该实现旨在提供与LevelDB的SkipList相似的性能特征,使用原子操作实现并发访问,并采用类似的概率平衡策略。

该项目欢迎贡献,接受Pull Request。使用MIT协议开源。

[
https://github.com/monadbobo/skiplist-rust
](
https://github.com/monadbobo/skiplist-rust
)
    


### TITLE

这是一个使用Rust语言和SDL2库构建的简单网页游戏示例,是作者emscripten-functions项目的一部分。该项目旨在为使用Rust构建Web游戏并与C库集成提供一个起点,尤其是那些在wasm32-unknown-unknown环境下无法正常工作的C库。通过这个简单的游戏示例,开发者可以了解如何使用Emscripten将Rust代码编译为WebAssembly,并与SDL2等C库一起工作,从而在Web浏览器中运行游戏或其他应用程序。

[
https://old.reddit.com/r/rust/comments/1fplvv0/a_simiple_minimal_working_sdl2_rust_game_with/
](
https://old.reddit.com/r/rust/comments/1fplvv0/a_simiple_minimal_working_sdl2_rust_game_with/
)
    


### TITLE

这段代码展示了Rust中使用`Rc`和`RefCell`来实现内部可变性的一个例子。作者希望在克隆变量后,能够分别修改克隆变量的值,而不影响原始数据源。

代码中,`Rc`(Reference Counted)类型用于多个所有权,而`RefCell`则提供内部可变性。初始值为100,通过`Rc::clone`创建了两个克隆变量`temp1`和`temp2`。代码修改了`temp1`的值加100,期望`temp2`保持原值100,但实际上`temp2`的值也被修改为200。

作者想要的是一种方法,能在克隆变量后分别修改克隆变量的值,而不影响原始数据源的值。这在Rust的所有权和借用规则下需要特别的数据结构和操作方式,代码中使用`Rc`和`RefCell`的方式无法实现这一需求。

[
https://old.reddit.com/r/rust/comments/1fpn1xs/how_can_i_clone_one_variable_multiple_times_with/
](
https://old.reddit.com/r/rust/comments/1fpn1xs/how_can_i_clone_one_variable_multiple_times_with/
)
    


### TITLE

以下是对该内容的中文总结:

在2024年内核维护者峰会上,Miguel Ojeda主持了一场关于在内核中使用Rust语言的讨论。尽管Rust在内核中的进展一直处于实验阶段,但已有越来越多的内核代码使用Rust编写。一些开发者对将新功能合并到内核的缓慢速度表示frustration,对这个项目的未来也存在不确定性。

会议讨论了以下几个主题:

1. 内核子系统维护者需要保持灵活性,因为有时需要改变一些核心API以适应Rust代码。

2. 人们和公司希望在内核中投资使用Rust,但对其未来存在疑虑。需要就Rust在内核中的目标达成共识。

3. Rust工具链和支持的可用性。一些发行版还无法提供合适的Rust编译器来构建内核代码。

4. Rust社区正在为内核开发者提供专家支持和帮助。

5. 管理预期,将Rust合并到内核是一个漫长的过程。

6. 在C代码一侧进行重构以适应Rust,这可能会破坏Rust代码。

7. Rust在内核中虽然还处于早期阶段,但前景光明,可以消除整个类别的错误。开发者应继续努力推进而不要过于谨小慎微。

8. 将第一个真正的Rust驱动程序合并将是一个转折点,之后进度会加快。

9. 虽然还有一些阻碍,但越来越少的内核特性与Rust不兼容了。

总的来说,尽管过程缓慢,但Rust在内核中的未来是肯定的,只是需要数年时间达到生产水平。

[
https://lwn.net/SubscriberLink/991062/b0df468b40b21f5d/
](
https://lwn.net/SubscriberLink/991062/b0df468b40b21f5d/
)
    


### TITLE

这个代码库主要探讨了 Rust 中的所有权和借用规则,以及在堆分配数据时如何使用引用(& T)、可变引用(& mut T)、Rc(引用计数)和 RefCell(内部可变性)等机制。具体内容包括:

1. 解释了 Rust 的所有权规则,每个值必须有一个所有者,当所有者离开作用域时,值会被丢弃。

2. 对比了不可变引用(&T)、可变引用(& mut T)、Rc(引用计数智能指针)和 RefCell(内部可变性)在不同场景下的使用方式和优缺点。

3. 展示了使用不可变引用读取堆上数据、使用可变引用修改堆上数据,以及使用 Rc 实现多个所有者共享数据的示例。

4. 解释了 RefCell 与 Rc 的组合使用,可以实现多个所有者共享可变数据,并给出了二叉树等复杂数据结构的示例代码。

5. 通过一系列简单和复杂的例子,帮助读者掌握 Rust 所有权和借用的概念,并学习使用安全的方式操作堆上的数据。

总的来说,这个代码库系统地介绍了 Rust 所有权、借用等核心概念,有利于读者深入理解和掌握 Rust 内存安全管理的方法。

[
https://github.com/tracyspacy/mastering_rust
](
https://github.com/tracyspacy/mastering_rust
)
    


### TITLE

这位开发者正在用Rust语言开发第二个项目,一个可以下载、搜索和播放音乐的音乐播放器/管理器。他打算在Rodio库中实现自动增益控制(AGC)功能。但当查看Rodio项目的源代码时,他感到一筹莫展,对于代码中的方法、特征等感到困惑,就像一个不会游泳的孩子被扔进游泳池。他询问这种困惑的感觉对于开发者来说是否正常。

另外,由于他之前没有使用过Github进行协作开发,对于如何发起拉取请求(Pull Request)的礼仪也不太清楚,比如是否必须在代码处于可运行状态时才能发起拉取请求等。

最后,他提到Rust是他学习的第一门"正式"编程语言,在此之前只做过一些Bash脚本和Windows批处理/PowerShell脚本编写,可以说是一下子就跳进了编程的深渊,但他还是"有点喜欢"这种感觉。

[
https://old.reddit.com/r/rust/comments/1fpc0ia/feel_like_i_know_nothing/
](
https://old.reddit.com/r/rust/comments/1fpc0ia/feel_like_i_know_nothing/
)
    


### TITLE

这个帖子讨论了在Rust编程语言中访问UTF-8字符串中单个字符的方式。作者指出,在Python中可以直接使用索引来访问字符串中的单个字符,例如'你好'[1]会返回'好'。但在Rust中,需要使用更加繁琐的方式,即'你好'.chars().nth(1).unwrap()。作者希望能找到一种更简洁的方式在Rust中访问UTF-8字符串中的单个字符。

[
https://old.reddit.com/r/rust/comments/1fpmvso/finding_neat_way_to_access_utf8_characters/
](
https://old.reddit.com/r/rust/comments/1fpmvso/finding_neat_way_to_access_utf8_characters/
)
    


### TITLE

这篇文章讨论了学习使用Bevy游戏引擎的一些困难。作者指出,Bevy官方文档缺乏对引擎各种高级功能的充分介绍,导致初学者很容易写出低效的代码。为了解决这个问题,作者正在撰写一个快速入门指南,旨在通过实例代码的方式,全面介绍Bevy的各种特性,包括动画系统、事件监听器、自定义查询结构等。作者已经完成了前3章的内容,并且第3章是在与Bevy全职开发人员Alice的指导下完成的。作者希望该指南能够帮助初学者快速掌握Bevy,避免重复前人的错误,并欢迎广大网友对内容提出批评修改建议,使指南质量不断完善。

[
https://old.reddit.com/r/rust/comments/1fpnjl0/giving_bevy_the_quickstart_guide_it_deserves/
](
https://old.reddit.com/r/rust/comments/1fpnjl0/giving_bevy_the_quickstart_guide_it_deserves/
)
    


### TITLE

总结:

这是一位Rust程序员分享了自己使用Rust编写的游戏引擎项目。他最近在YouTube上发布了一个视频,介绍了自己制作的这个完全使用Rust编写的游戏引擎。该视频投入了大量精力制作,作者希望能获得反馈意见。他还提供了该游戏引擎的GitHub仓库链接,供感兴趣的人查看源代码。通过这个项目,他展示了Rust在游戏引擎开发领域的应用,并分享了自己的经验和作品。

[
https://old.reddit.com/r/rust/comments/1fp7ti8/i_made_a_youtube_video_about_making_my_own_engine/
](
https://old.reddit.com/r/rust/comments/1fp7ti8/i_made_a_youtube_video_about_making_my_own_engine/
)
    


### TITLE

以下是对该内容的中文总结:

Oku是一款重新让网页浏览变得有趣的浏览器。它允许你创建并发布网站、直接与朋友分享文件、将个人数据存储在你控制的设备上,并在你的设备之间同步文件。

Oku引入了"Replica"的概念,这是一种可以在线共享的虚拟驱动器,可以包含照片、视频或文档等任何内容。你可以在Replica中放置一个网站,并与世界分享。

Oku让网站能够请求读写权限,将你的数据存储在你自己的设备上的Replica中,而不是存储在他们控制的服务器上。只拥有只读权限的人只能查看而不能编辑你的Replica。

通过Replica,你可以在互联网上独立于社交媒体平台,建立自己的空间。每次访问一个新的Replica时,你的浏览器会保存其ID,当Replica更新时,拥有相同ID的人都会看到最新版本。

Oku基于WebKit内核,是谷歌Chrome的真正替代品,而不是衍生品。它是免费开源软件,欢迎在GitHub上贡献代码。

[
https://okubrowser.github.io/
](
https://okubrowser.github.io/
)
    


### TITLE

这篇博文概述了谷歌消除安卓系统内存安全漏洞的努力和进展。主要内容包括:

1. 内存安全漏洞一直是软件安全的主要威胁。谷歌认为,通过安全编码(Safe Coding)的方式,即过渡到使用内存安全语言,可以从根源上消除这一类漏洞,并构建高保证的软件。

2. 模拟显示,随着时间推移,即使代码库中存在内存不安全的代码,只要新代码使用内存安全语言,漏洞数量也会显著下降。这是因为漏洞会随时间呈现指数级衰减。

3. 在实践中,自2019年起,安卓团队开始优先使用内存安全语言进行新的开发。尽管大部分代码仍然是不安全的,但内存安全漏洞的总数和比例都持续下降。2019年内存安全漏洞占76%,而2024年只占24%。

4. 消除内存安全漏洞的策略经历了四个阶段:1)被动修补;2)主动缓解;3)主动发现;4)高保证预防(安全编码)。安全编码不仅改变了技术,更改变了安全方法,可以大规模、持续、高效地提供保证。

5. 采用安全编码可以打破无休止的攻防对抗循环,使高保证内存安全成为常规,并提高开发人员生产力。

[
https://security.googleblog.com/2024/09/eliminating-memory-safety-vulnerabilities-Android.html?m=1
](
https://security.googleblog.com/2024/09/eliminating-memory-safety-vulnerabilities-Android.html?m=1
)
    


### TITLE

本文总结了Bevy Foundation获得501(c)(3)公益慈善机构地位的重要性和影响。这一地位使得Bevy Foundation在美国可获得免税待遇,捐赠也可在美国申请扣除。同时也使其能参与更多公益项目及资助机会。

文中还介绍了一个新的捐款平台Every.org,其收费低于之前使用的Stripe平台,有利于提高捐款效率。

此外,Bevy Foundation还推出了新的电子邮件地址,方便与支持者和捐助者的交流。

总的来说,这一501(c)(3)地位不仅提高了Bevy Foundation的公信力,也为其带来了更多资金来源,有利于推动Bevy引擎的发展。因此文中呼吁读者为Bevy Foundation捐款,支持其持续运营。

[
https://bevyengine.org/news/bevy-foundation-501c3/
](
https://bevyengine.org/news/bevy-foundation-501c3/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

