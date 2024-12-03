【Rust日报】2024-11-19


### TITLE

这个项目是一个基于终端的3D渲染器。它支持加载.obj文件,并在终端中显示3D模型。可以使用不同的字符来表示不同的光照级别,支持相机移动、颜色渲染和八叉树优化。该项目的主要特性包括:

1. 3D渲染功能,可加载.obj文件。
2. 支持使用不同字符表示不同光照强度。
3. 支持相机移动。
4. 支持颜色渲染(目前仅限手动网格,不支持.obj文件)。
5. 可选的八叉树优化,减少渲染时间。
6. 显示三角形数量的功能。

该项目未来将继续改进,计划增加替代光照类型、纹理加载、完善文档注释等功能,并修复一些已知问题。该项目还提供了作为库使用的方式。总的来说,这是一个基于字符的轻量级3D渲染解决方案。

[https://github.com/TageDan/terminal-renderer
](https://github.com/TageDan/terminal-renderer
)
    


### TITLE

这篇文章介绍了如何从头开始构建一个3D终端渲染器。主要内容包括:

1. 使用Rust语言编写代码。

2. 设置项目结构和依赖。 

3. 创建射线(Ray)结构体,表示从屏幕像素发出的射线。

4. 创建Screen结构体,表示屏幕尺寸和聚焦距离(决定视场角)。

5. 创建Camera结构体,表示相机位置。

6. 在render方法中,遍历屏幕上的每个像素,通过Screen、Camera计算射出的射线。

7. 定义三角形(Triangle)和网格(Mesh)结构体,表示三维物体。

8. 在渲染过程中,检测射线与哪个三角形相交,从而确定像素颜色。

9. 使用ANSI转义序列在终端中着色和控制光标。

总的来说,这是一个使用Rust从零开始构建简单3D终端渲染器的过程教程。

[
https://tagedan.github.io/posts/terminal_rendering.html
](
https://tagedan.github.io/posts/terminal_rendering.html
)
    


### TITLE

这是一个基于终端的3D平台/迷宫游戏,专注于速跑和自定义关卡创建。游戏需要一个终端来运行,目前推荐使用Windows Terminal。游戏提供了Windows和Linux两个版本的压缩包进行下载和安装。运行游戏时需要指定一个包含关卡的文件夹。

关卡是通过文本文件来定义的,使用特定字符代表不同的网格类型,如起点、终点、墙壁、地板、陷阱等。每个新的楼层由"sep"行分隔。

已知的一些Bug包括离线时网络请求处理不当导致游戏崩溃、排行榜验证有问题、终端焦点难以regain等。未来的计划包括独立的排行榜、敌人声音、关卡编辑器、Discord机器人和3D对象文件加载器支持。

游戏鼓励用户提出新功能的想法,或者通过fork并提交PR来改进游戏。最后对游戏的创作者和Rust编程语言及终端表示感谢。

[
https://github.com/TermTrack/TermTrack
](
https://github.com/TermTrack/TermTrack
)
    


### TITLE

这是一个关于使用Rust编程语言和Loco框架的介绍。主要内容包括:

1. 使用Loco框架可以让Rust成为你的"超级力量"。Loco提供了简单的请求生命周期、代码生成器、生产力工具等,使得使用Rust变得非常简单。

2. 提供了一些命令行示例,如cargo loco generate scaffold用于生成代码scaffold,cargo loco start用于启动服务器。

3. 有一个ASCII艺术图案,可能代表Loco的标志。

4. 最后列出了一些运行环境的配置,如开发环境、数据库自动迁移、日志记录器禁用、编译级别为debug模式、服务器监听localhost:5150。

总的来说,这是在介绍如何使用Loco框架来提高Rust开发效率和体验。

[
http://Loco.rs
](
http://Loco.rs
)
    


### TITLE

以下是对loco仓库的中文总结:

Loco是一个受Ruby on Rails启发的Rust web框架。它的目标是通过约定优于配置的理念和减少样板代码,提高开发人员的生产力。Loco具有以下主要特性:

1. 约定优于配置:像Rails一样,Loco强调简单性和生产力,减少了配置的需求。它使用合理的默认值,让开发人员可以专注于业务逻辑而不是配置。

2. 快速开发:Loco设计的目标是提高开发效率,通过减少样板代码和提供直观的API,让开发人员可以快速迭代和构建原型。

3. ORM集成:使用强大的实体对象来建模业务,无需编写SQL。可以直接在实体上定义关系、验证和自定义逻辑。

4. 控制器:处理Web请求参数、主体、验证,并呈现与内容相关的响应。使用Axum框架获得最佳性能、简单性和可扩展性。

5. 视图:Loco可以集成模板引擎,从模板生成动态HTML内容。

6. 后台作业:在后台使用基于Redis的队列或线程执行计算或I/O密集型作业。

7. 调度器:简化传统的crontab系统,更优雅地安排任务或shell脚本。

8. 邮件发送:通过现有的后台工作基础设施在后台发送邮件。

9. 存储:方便操作文件,支持内存、磁盘或云服务如AWS S3。

10. 缓存:提供缓存层以提高应用程序性能。

Loco还提供了创建新应用的命令行工具,可以生成包含所需组件的应用程序框架代码。总的来说,Loco旨在提供类似Rails的开发体验,同时利用Rust语言的优势。

[
https://github.com/loco-rs/loco
](
https://github.com/loco-rs/loco
)
    


### TITLE

以下是对给定内容的中文总结:

这篇文档介绍了如何开始使用tracing-tape工具进行应用程序的跟踪。主要步骤包括:

1. 将tracing、tracing_subscriber和tracing-tape-recorder三个crate添加到应用程序中。

2. 创建带有TapeRecorder层的subscriber,并将其设置为默认subscriber。这样所有的跟踪就会被记录到一个名为{crate_name}-{timestamp}.tape的文件中。

3. 使用#[tracing::instrument]属性和trace_span!宏在代码中插入跟踪点。

4. 使用Trace Deck GUI工具查看生成的跟踪文件。该工具提供了时间线、日志消息、调用点等多个视图,以及一些交互式的功能,如缩放、选择等。

5. 在Trace Deck中,可以查看跟踪的细节,包括源代码位置、字段信息、跨度统计数据等。还可以对跨度持续时间进行绘制。

总的来说,这个过程使得在Rust应用程序中添加跟踪变得很方便,而Trace Deck则提供了一个友好的GUI界面来查看和分析跟踪数据。

[
https://github.com/soehrl/tracing-tape/wiki/Getting-Started
](
https://github.com/soehrl/tracing-tape/wiki/Getting-Started
)
    


### TITLE

这个项目提供了一个简单的方法来调试和分析分布式Rust应用程序的执行情况。它基于 tracing 创建,可以将应用程序的追踪事件记录到磁盘文件中,然后使用一个GUI工具 trace-deck 来可视化查看这些记录文件。

要使用它,只需在应用程序中添加几个依赖项,并用一些代码初始化记录器。运行应用程序时就会在当前目录下生成以时间戳命名的 .tape 文件,里面记录了应用执行过程中的追踪事件。之后就可以用 trace-deck 工具直接打开这些文件查看可视化的执行情况。

这个工具的优点是无需复杂的设置和配置,就可以快速地对应用进行追踪分析,并支持同时加载多个记录文件。但目前还存在一些已知问题,比如无法配置记录器、大文件加载慢、偶尔会导致延迟等。总的来说是一个方便的分布式系统调试和分析工具。

[
https://github.com/soehrl/tracing-tape
](
https://github.com/soehrl/tracing-tape
)
    


### TITLE

这是一个Rust库项目的概述,主要介绍了tracing框架及其用法。tracing是一个用于在Rust程序中收集结构化的基于事件的诊断信息的框架。

该框架可用于应用程序和库。在应用程序中,可通过tracing-subscriber等收集器实现来记录跟踪事件。在库中,开发者只需使用tracing提供的宏和类型来收集可能对下游使用者有用的信息。

文档提供了一些代码示例,展示了如何初始化收集器、使用跨度(span)和事件(event)等核心概念在应用程序和库中记录诊断信息。总的来说,tracing旨在帮助Rust开发者方便地将结构化日志记录集成到他们的程序中。

[
https://github.com/tokio-rs/tracing
](
https://github.com/tokio-rs/tracing
)
    


### TITLE

该资源库是一个用Rust语言编写的简单快速的Halo Infinite游戏数据反序列化库。它主要功能包括:

1. 加载Halo Infinite游戏中的Module文件,这些文件存储了游戏中所有资源如模型、纹理、元数据等。

2. 从Module文件中加载特定的tag文件,获取tag数据流和信息。

3. 通过derive特性,可以将tag数据直接反序列化到自定义的数据结构中,方便后续处理。

4. 提供了一些示例代码展示如何使用该库加载Module文件、tag文件以及自定义数据结构。

5. 该项目受到了其他一些游戏文件相关工具如libinfinite、Reclaimer等的启发。

总的来说,这是一个方便Rust开发者访问和处理Halo Infinite游戏数据的实用库。

[
https://github.com/Surasia/infinite-rs
](
https://github.com/Surasia/infinite-rs
)
    


### TITLE

dynosaur是一个Rust编程语言的crate,它允许你在具有异步函数和返回impl Trait的trait上使用动态分发。它提供了一个[dynosaur::dynosaur(DynNext)]属性宏,用于生成一个叫DynNext的类型,可以像使用dyn Trait那样使用该类型。当动态分发时,返回impl Trait的方法会将其返回类型装箱,而静态分发时则不会。该crate使用Apache 2.0或MIT许可证授权。

[
https://github.com/spastorino/dynosaur
](
https://github.com/spastorino/dynosaur
)
    


### TITLE

根据这位Python后端程序员的描述,他已经学习了Rust的基础知识,并且能够使用Rust编写简单的Web应用程序(如Axum、Rocket)和命令行工具。他非常热爱Rust,希望将来能够找到一份Rust相关的工作。

他正在寻求建议,应该继续深入学习Rust的哪一方面。他提到有很多可以选择的领域,但目前还无法确定。不过,他表示不想涉及Web3和区块链相关的内容,因为他认为那些都是骗局。

最后,他为自己的英语水平不太好而道歉。

[
https://old.reddit.com/r/rust/comments/1gusxz0/recommend_next_thing_to_learn_in_rust/
](
https://old.reddit.com/r/rust/comments/1gusxz0/recommend_next_thing_to_learn_in_rust/
)
    


### TITLE

该文章描述了作者正在尝试编写一个CPU模拟器。他目前正在遍历一个包含指令的向量,指令是一个枚举类型,包括NOP(空操作)、MOV(移动数据)等。寄存器ID被编码在一个字节中,包含寄存器编号和访问方式(低字节、高字节、字)。在他的机器上,该模拟器每秒可执行约3亿条指令。

作者考虑编写某种即时(JIT)编译器,但由于需要精确地一条指令一条指令地执行模拟CPU指令(为了并行多个CPU并保持同步),所以JIT编译器不适用。他在寻求编写此类模拟器的一些技巧和更高效的方法。

[
https://old.reddit.com/r/rust/comments/1gv2pia/cpu_emulator/
](
https://old.reddit.com/r/rust/comments/1gv2pia/cpu_emulator/
)
    


### TITLE

这是一个关于作者重建终端渲染器的经历。最初,作者想为自己的游戏TermTrack创建一个简单的终端渲染器,并打算写一系列博客文章教大家如何做。在实现对象加载时,作者被较大对象渲染时的延迟所困扰,于是开始优化。大约一周后,作者完成了一个用于终端的3D文件查看器。

新的渲染器比游戏中使用的旧渲染器速度更快,所以作者打算将游戏更新为使用新渲染器。

这也是作者第一次实现八叉树,一开始对此有些担心,因为广为流传八叉树在Rust中由于递归性质而较难实现。不过最终并没有太多麻烦,作者有些草率地实现了它。文中,作者欢迎有经验的人提供反馈意见。

[
https://old.reddit.com/r/rust/comments/1gv5u7q/terminal_renderer/
](
https://old.reddit.com/r/rust/comments/1gv5u7q/terminal_renderer/
)
    


### TITLE

这位Reddit用户想知道是否有公司或项目正在生产环境中使用Loco.rs这个rust编写的后端框架。由于Loco.rs相对较新,他希望能听到一些使用经验,比如它的稳定性、性能表现以及生态系统的成熟度等,以帮助他决定是否在自己的项目中采用这个框架。他向那些有实际使用经验的人征求意见和见解。

[
https://old.reddit.com/r/rust/comments/1guycdf/who_is_using_locors_in_production/
](
https://old.reddit.com/r/rust/comments/1guycdf/who_is_using_locors_in_production/
)
    


### TITLE

这是一篇关于Rust编程语言中一个名为"tracing-tape"的新发布的crate(软件包)集合的介绍。该项目包括一个简单的二进制格式、一个易用的追踪记录器和一个支持多进程的追踪查看器。

该项目的主要设计目标是:
1. 简单性 - 记录追踪应该非常简单,不需要复杂的设置。
2. 低开销 - 写入追踪应该非常快,以便在高性能实时应用中使用。 
3. 多进程支持 - 调试客户端-服务器应用程序可能很棘手,追踪查看器允许同时加载多个应用程序生成的追踪文件并正确对齐它们。

该项目目前还处于早期开发阶段,记录器还无法配置。但作者希望它对一些人有用。作者欢迎任何反馈意见。

[
https://old.reddit.com/r/rust/comments/1gv0arl/tracing_tape_simple_application_tracing/
](
https://old.reddit.com/r/rust/comments/1gv0arl/tracing_tape_simple_application_tracing/
)
    


### TITLE

该Reddit帖子是作者分享了他的第一个Rust crate项目"infinite-rs"。这个crate能够读取(反序列化)视频游戏"Halo Infinite"使用的二进制资产格式。作者花了几个月的时间来编写这个实践项目,旨在学习Rust语言。

虽然这个库的大部分逻辑只是读取二进制数据并相应地解压缩,但作者在尝试抽象化以创建可用的API时遇到了一些问题,目前的API不太友好。他分享了GitHub仓库链接和文档链接,并希望能够获得建议来改进代码质量。总的来说,这是一个作者为了学习Rust而编写的实践项目,并公开分享以寻求反馈和改进。

[
https://old.reddit.com/r/rust/comments/1guw9pw/my_first_rust_crate_infiniters/
](
https://old.reddit.com/r/rust/comments/1guw9pw/my_first_rust_crate_infiniters/
)
    


### TITLE

这个Reddit帖子讨论了Rust语言中AsyncDrop特性在nightly版本中的引入。作者表达了对此的兴奋,因为这可能最终实现了类似于thread::scope的安全异步等效功能,用于异步上下文。

帖子提供了AsyncDrop函数和AsyncDrop trait的链接,并询问大家对该特性有何期待,是否已在库中发现有趣的用法(无论是在nightly标志或功能开关后面)。

作为一名结构化并发的狂热爱好者,作者对AsyncDrop的加入感到非常兴奋,并希望能够安全地在异步上下文中实现类似thread::scope的功能,尽管需要考虑futures可能在完成之前被放弃的情况。

总的来说,这个帖子讨论了AsyncDrop在Rust语言中的引入,以及其可能带来的影响和应用前景,并征求社区对此的看法和反馈。

[
https://old.reddit.com/r/rust/comments/1gvblzb/asyncdrop_is_in_nightly_what_now/
](
https://old.reddit.com/r/rust/comments/1gvblzb/asyncdrop_is_in_nightly_what_now/
)
    


### TITLE

这个提问涉及到Rust编程语言中str和OsStr类型的关系。

首先解释了OsStr类型用于存储与平台相关的字符串表示形式,例如在Windows系统上是UCS-2编码。

然后提出了一个疑问:str类型实现了AsRef<OsStr>特性,这意味着str底层的字节序列可以直接被视为OsStr。作者不理解这是如何实现的,因为str通常是UTF-8编码,而OsStr在不同平台可能使用不同的编码方式。

因此,这个问题探讨了不同字符串类型之间的编码转换,以及Rust标准库是如何处理这种转换的。这对于处理涉及多个平台的字符串操作是很重要的一个问题。

[
https://old.reddit.com/r/rust/comments/1guwjho/why_does_str_implement_asrefosstr/
](
https://old.reddit.com/r/rust/comments/1guwjho/why_does_str_implement_asrefosstr/
)
    


### TITLE

这是一个关于Rust编程语言的帖子,作者正在开发一个名为dynosaur的crate(Rust代码包),它可以扩展普通的Rust traits(类似接口),支持包含async fn或-> impl Trait方法的动态分发。作者提供了一个示例代码,展示了如何使用该crate。

与#[async_trait]这个crate不同,dynosaur允许trait支持完全静态分发,而无需boxing的开销。但是,在使用动态分发时,返回的futures会被boxing。

作者呼吁大家测试并反馈这个crate,以便他可以正式在Rust博客上宣布该项目。他希望大家尝试使用dynosaur,并将发现的任何问题提交到该项目的GitHub仓库。

[
https://old.reddit.com/r/rust/comments/1gv47xa/call_for_testing_use_async_fn_in_dyn_traits_with/
](
https://old.reddit.com/r/rust/comments/1gv47xa/call_for_testing_use_async_fn_in_dyn_traits_with/
)
    


### TITLE

以下是对该进展报告的中文总结:

这份进展报告介绍了rustc_codegen_cranelift项目在2024年11月的主要进展。自上次报告以来,该项目已经有383次代码提交。主要成就包括:

1. ABI兼容性方面有重大改进,部分修复了Rust ABI中多值返回时的问题,并改进了对ARM64、RISC-V64和S390X体系结构的支持。

2. @dpaoliello和@ChrisDenton为Windows实现了原始动态库支持,这是将cg_clif作为rustup组件在Windows上分发的最后障碍。

3. @beetrees的工作解决了在ARM64 macOS上调用可变参数函数的问题,cg_clif现在可以作为rustup组件在ARM64 macOS上分发了。

4. 项目首次运行rustc-perf基准测试套件,结果显示许多基准测试的性能较差,需要进一步优化。

5. 其他改进包括对Mach-O目标文件格式的支持增强、ARM64 macOS目标特性的静态启用等。

总的来说,该项目在ABI兼容性、Windows和macOS支持等方面取得了长足进展,但性能方面仍有提升空间。

[
https://bjorn3.github.io/2024/11/14/progress-report-nov-2024.html
](
https://bjorn3.github.io/2024/11/14/progress-report-nov-2024.html
)
    


### TITLE

这位提问者是一名Rust初学者,自学Python编程。他最近发现可以将更快的Rust程序集成到更简单的Python脚本中,对此过程的底层机制很感兴趣。

他理解Python本质上是将多个C++文件"粘合"在一起的接口,因此可以用C++编写与Python的C++文件交互的函数。但他不确定Rust是如何实现这一点的,是否需要将Python对象转换为Rust数据类型?他知道可以将Rust程序编译为可以接受和处理命令行参数的二进制文件,不确定Python是否本质上也是这样做的。

他希望能够得到大家对此的看法,如果有任何好的相关文章推荐,也很乐意阅读。

[
https://old.reddit.com/r/rust/comments/1gv508j/how_exactly_does_python_and_rust_work_together/
](
https://old.reddit.com/r/rust/comments/1gv508j/how_exactly_does_python_and_rust_work_together/
)
    


### TITLE

这是一个在线Rust编程实践网站,它提供了以下主要功能:

1. 在浏览器中集成了Rust编译器,无需本地安装环境即可编写和运行Rust代码。

2.提供了大量的Rust编程问题供练习,数量非常多,并且会持续添加新题目。

3.提供了分步骤的Rust教程和课程,可以循序渐进地学习Rust编程语言。

4.允许订阅以获取网站的最新消息和更新。

总的来说,这是一个面向Rust编程语言学习者和爱好者的在线实践平台,集成了编译器、编程题库和教学资源,旨在提供一站式的Rust学习和实践体验。

[
https://www.rustfinity.com/
](
https://www.rustfinity.com/
)
    


### TITLE

这是一篇来自Sean McArthur的博客文章,主要讨论了将Rust编程语言中的hyper HTTP库集成到curl项目中的进展。文章指出,由于curl广泛应用于互联网,将其与内存安全的hyper结合可以提高互联网的安全性。自2020年以来,作者和Daniel一直在努力将hyper集成到curl中,目前工作已接近完成。但是,由于缺乏支持的供应商或发行版想要启用并使用这一后端,该功能有可能在2025年初被弃用和移除。作者呼吁有意启用此功能的组织或个人作为"冠军"支持者,积极参与测试并提供反馈,从而帮助完成这一改进,为互联网带来更高的安全性。

[
https://seanmonstar.com/blog/hyper-in-curl-needs-a-champion/
](
https://seanmonstar.com/blog/hyper-in-curl-needs-a-champion/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

