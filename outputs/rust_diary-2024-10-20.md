【Rust日报】2024-10-20


### TITLE

这个项目是一个用Rust编写的Hive棋盘游戏的终端界面(TUI)实现。它可以通过键盘操作,并包含一个具有挑战性的人工智能对手。游戏包括教程和规则摘要。该项目提供了针对Windows x86_64和Linux x86_64的预编译二进制包。用户可以下载相应的zip文件,解压后即可运行游戏。对于Windows用户,需要先安装Windows Terminal。该项目使用了Unicode和色彩,因此需要终端支持相应的功能。未来可能会添加蜗牛棋子、重播模式等新功能。最后作者声明,该项目并非由Hive游戏的版权所有者Gen42 Games授权或认可。

[https://github.com/N-Maas/hivetui
](https://github.com/N-Maas/hivetui
)
    


### TITLE

这是一个介绍Hive游戏的网页,内容包括以下几点:

1. 这是一个蜂巢主题的策略棋盘游戏,由柱形棋子构成。

2. 游戏简单易学但有深度,适合各个年龄段的玩家。

3. 网页展示了游戏的规则、策略和进阶技巧。

4. 提供了游戏的在线版本可供体验,也有实体版本可购买。

5. 页面底部嵌入了谷歌翻译插件,方便多语种用户浏览。

总的来说,这是一个介绍独特蜂巢棋盘游戏Hive的网站,对于感兴趣的读者来说是不错的游戏入门资源。

[
https://gen42.com/games/hive
](
https://gen42.com/games/hive
)
    


### TITLE

以下是对该内容的中文总结:

Asynq是一个用Go语言编写的分布式任务队列库,支持后台用Redis作为消息代理。它具有以下主要特性:

1. 保证至少执行一次任务
2. 支持任务调度、重试失败任务、自动恢复崩溃worker的任务
3. 支持加权优先级队列和严格优先级队列
4. 低延迟添加任务,因为Redis的写入速度很快
5. 支持任务去重、设置任务超时时间和期限
6. 支持批量聚合多个连续操作的任务组 
7. 灵活的处理器接口,支持中间件
8. 可暂停队列,停止处理队列的任务
9. 支持周期性重复任务
10. 支持Redis集群实现自动分片和高可用
11. 支持Redis哨兵实现高可用
12. 集成Prometheus进行指标监控和可视化
13. 提供Web UI和CLI工具远程监控和控制队列/任务

该库目前处于快速开发迭代中,API可能会有较大变动,直到1.0版本发布后才会保证稳定。文档还提供了快速上手的示例代码,展示如何创建任务、编写任务处理器等。

[
https://github.com/hibiken/asynq
](
https://github.com/hibiken/asynq
)
    


### TITLE

rustc_codegen_clr是一个实验性的从Rust编译到.NET程序集的Rust编译器后端。它允许Rust编译器将Rust代码转换为.NET程序集。这种转换非常高级,保留了类型、字段/变量名等信息。该项目旨在提供一种轻松使用Rust库在.NET中的方式。它带有一个Rust/.NET互操作层,允许您从Rust轻松与.NET代码交互。该项目还将包括从Rust定义.NET类的支持,目前处于大量工作进行中的状态。

目前该项目支持大多数Rust特性(除了async和proc宏),但并非无bug。它可以编译一个基本可工作的Rust标准库版本,但存在许多小bug使得标准库不是100%功能完备。大多数标准库组件约有95%的功能正常工作。因此,您可以编译大量现有的Rust代码,但未必能正常工作。

rustc_codegen_clr仅在Linux x86_64上使用CoreCLR运行时(通常称为.NET运行时)和.NET 8进行了测试。理论上它应该可以在其他平台上工作,但没有保证。对Mono运行时的支持还不如预期。由于不受支持的功能和差异,在Mono上不支持128位整数和经过检查的64位整数运算。对齐分配器和某些内部函数也不受支持。

rustc_codegen_clr根据MIT许可证或Apache 2.0许可证获得双重许可。

[
https://github.com/FractalFir/rustc_codegen_clr
](
https://github.com/FractalFir/rustc_codegen_clr
)
    


### TITLE

这个链接是指向一个在线聊天室的主题页面,主题是关于一个名为"Rust 到 .NET 编译器"的项目。页面上显示了加载错误的信息,无法正确显示聊天记录。看来该链接可能已经过期或出现了技术问题。

[
https://rust-lang.zulipchat.com/#narrow/stream/421156-gsoc/topic/Project.3A.20Rust.20to.20.2ENET.20compiler
](
https://rust-lang.zulipchat.com/#narrow/stream/421156-gsoc/topic/Project.3A.20Rust.20to.20.2ENET.20compiler
)
    


### TITLE

这是GitHub sponsor页面的内容,介绍了一位名为FractalFir的年轻波兰程序员。他对编译器和语言运行时很感兴趣,目前正在开发一个Rust编译器模块rustc_codegen_clr,允许在.NET运行时中编译和运行Rust代码。该页面呼吁赞助FractalFir的开源工作,提供了月度和一次性赞助的选项。总的来说,这是一个开源开发者在GitHub上寻求赞助支持的个人主页。

[
https://github.com/sponsors/FractalFir
](
https://github.com/sponsors/FractalFir
)
    


### TITLE

这个存储库包含一个支持SIMD的CRC算法生成器,以及所有生成的CRC算法的cargo crates。该仓库中的gen_crates.sh脚本使用algos.csv中的算法列表,基于crc-crate-template目录生成每个特定的CRC算法crate。生成的lib.rs文件包含一个单一的crc!(...) 表达式,以及对包含宏逻辑的crc-fast-gen crate的依赖。 

仓库中包含了一些示例,展示了如何更改模板并重新生成crate。版本信息存储在version.txt中,用于crate生成。一般情况下,crate版本与仓库版本同步,但对单个crate的修复可能单独进行版本修补。

该项目的主要目的是性能,因此包含了大量的CRC算法基准测试,根据输入大小(128-64k字节)对SIMD、查表和简单循环性能进行了测试。未来将在一组架构/系列上运行基准测试并发布结果。  

该仓库为每种算法单独生成一个crate,这在Rust生态中是首选做法,因为通常应用只需依赖一两种CRC。单独crate使依赖更加明确,有利于预扩展代码以提高编译速度,方便审查和修复特定算法等。

该仓库使用MIT许可证,并引用了一些CRC相关的参考资源。

[
https://github.com/TobiasBengtsson/crc-fast-rs
](
https://github.com/TobiasBengtsson/crc-fast-rs
)
    


### TITLE

以下是对该内容的中文总结:

随着GPS等定位跟踪设备在智能手机和车辆上的广泛使用,大量地理空间轨迹数据正在被收集和应用于多个领域。MobilityDB提供了必要的数据库支持,用于存储和查询这种地理空间轨迹数据。MobilityDB作为PostgreSQL和PostGIS的一个扩展程序而实现,它实现了永久数据库类型和查询操作,用于管理地理空间轨迹及其随时间变化的属性。如需了解更多信息,可点击"了解更多"。

[
https://mobilitydb.com/
](
https://mobilitydb.com/
)
    


### TITLE

这是一个名为MEOS(Mobility Engine, Open Source)的开源C语言库及其相关API,用于操作时空数据。它是MobilityDB项目的核心组件,MobilityDB是一个建立在PostgreSQL和PostGIS之上的开源地理空间轨迹数据管理和分析平台。

MEOS扩展了ISO 19141:2008标准,用于表示要素的非空间属性的变化。它还考虑到在收集移动数据时,需要表示"时间间隙",即由于信号丢失等原因,在某些时间段内没有收集到观测数据。

MEOS的灵感来自于类似的几何库GEOS(Geometry Engine, Open Source)。MEOS旨在成为其他项目的基础库,例如PyMEOS(Python的MEOS绑定)、MobilityDB(PostgreSQL的扩展)等。其他项目也可以在MEOS之上构建,如Java或C#的MEOS驱动程序,或在其他数据库系统(如MySQL)上实现MEOS。

[
https://libmeos.org/
](
https://libmeos.org/
)
    


### TITLE

这个仓库包含了一个Rust库,提供了对MEOS C库的绑定。MEOS是一个用于时空数据管理和分析的库。这个Rust库使得处理时空数据更加方便,非常适合用于处理移动对象、轨迹和时变地理数据的应用场景。

该库支持MEOS 1.2及更高版本,主要目的是方便创建和操作时间类型,比如带时间戳的地理点、序列和数值等。这些时间数据结构可用于多种用例,如跟踪运动、空间时间查询、距离计算、时间加权平均值计算和相交查询等。

该库提供了多种时间数据类型,如时间几何点(TGeomPoint)、时间浮点数(TFloat)和时间布尔值(TBool)等,并给出了使用示例,如构造轨迹、计算最短距离、检查轨迹是否通过某点等。

该库目前仅支持单线程应用,将来可能会支持多线程。用户需要先在系统上安装MEOS C库,该Rust库会动态链接到系统的MEOS库。该仓库欢迎贡献,目前只实现了MEOS的一个子集。

[
https://github.com/MobilityDB/meos-rs
](
https://github.com/MobilityDB/meos-rs
)
    


### TITLE

Canopydb是一个嵌入式键值存储引擎,具有以下特点:

1. 完全事务性,提供有序的映射API类似std::collections::BTreeMap
2. 使用B+树实现,支持前缀和后缀截断
3. 高效处理大值,可选择压缩
4. 高效的I/O利用率,读写放大较低
5. 高效的持久提交,支持可选的预写日志(WAL)
6. 高效的异步持久性,支持后台WAL fsync
7. 使用可选的WAL实现有界恢复时间
8. 提供ACID事务和可序列化快照隔离(SSI)
9. 多版本并发控制(MVCC),读写不相互阻塞
10. 支持大于内存的事务
11. 每个数据库可以有多个键空间,键空间管理完全事务性
12. 每个环境可以有多个数据库,高效共享WAL和页面缓存
13. 支持跨数据库原子提交
14. 优化读取密集型和读取-修改-写入工作负载

该存储引擎针对单个写入者的读取密集型事务键值存储进行了优化设计。如果需要极端写入性能,可以考虑使用Fjall或RocksDB等替代方案。该项目目前处于早期阶段,新版本可能不兼容,不建议用于生产数据。

[
https://github.com/arthurprs/canopydb/
](
https://github.com/arthurprs/canopydb/
)
    


### TITLE

这是一个命令行工具集aid-cli,提供了多种实用功能。主要功能包括:

1. HTTP相关功能:发送HTTP请求,启动HTTP服务器。

2. IP相关功能:查看本地/公网IP,扫描IP地址subnet活跃地址,检测IP地址状态。

3. 端口相关功能:检测端口开放状态,扫描IP地址开放端口。

4. 系统信息功能:查看CPU、内存、磁盘、网络信息,并支持持续监控。

5. JSON解析功能:提取JSON属性值,解码JWT。

6. CSV操作功能:对CSV文件进行SQL查询。

7. 文本操作功能:base64编解码,读取文件行数据。

8. 文件信息功能。

该工具支持跨平台,提供了Windows和Linux/Mac的安装脚本。可通过Cargo从源码构建,也可下载预编译的二进制版本使用。总的来说是一个集合了网络、系统、数据处理等多方面小工具的命令行工具集。

[
https://github.com/Timmoth/aid-cli
](
https://github.com/Timmoth/aid-cli
)
    


### TITLE

这篇内容包含两个部分:

1. 第一部分介绍了 RustyRatracerr 项目,这是一个使用 Rust 语言实现的高性能计算 (HPC) 光线追踪实验。作者在家里搭建了一个 4 节点 32 核心的集群,希望通过这个计算密集型任务来实验高性能计算。项目基于 "Ray Tracing In One Weekend" 教程,同时也在学习 Rust 语言。最后提供了在集群上构建和运行的命令。

2. 第二部分是一个来自 Rust 子社区的帖子,提问者想要将一个实现了某个 trait 的异构集合(可能是 Vec、数组等)作为参数传递给一个函数,但遇到了编译器问题。提问者分享了代码示例,并寻求解决方案或是否有更好的实现方式。

总的来说,这些内容都围绕着 Rust 语言编程,涉及高性能计算应用以及 Rust 语言的泛型编程和 trait 约束等特性。

[
https://github.com/Timmoth/RustyRatracerr
](
https://github.com/Timmoth/RustyRatracerr
)
    


### TITLE

这是一个基于Rust编程语言的命令行界面(TUI)实现的"Hive"棋盘游戏项目。作者将其视为一个业余爱好项目,经过一段时间的努力,该项目已经可以使用了。

这个项目基于ratatui库构建,允许用户定制相当多的设置,并包括了一个相当有挑战性的人工智能对手(如果你还不熟悉这个游戏,作者建议从较低难度级别开始)。

作者认为这个项目可能对一些人来说很有趣和有趣,特别是对于那些对命令行界面游戏感兴趣的人来说。该项目的GitHub链接https://github.com/N-Maas/hivetui也一并提供,其中包括了一些图片。

[
https://old.reddit.com/r/rust/comments/1g84igx/hivetui_implementation_of_hive_board_game_based/
](
https://old.reddit.com/r/rust/comments/1g84igx/hivetui_implementation_of_hive_board_game_based/
)
    


### TITLE

总结如下:

这位新手开发者正在学习Rust语言,希望实现一个后端服务,需要使用任务队列来处理数据获取。他正在寻找一个可靠、基于Redis的解决方案,具有重试和优先级管理等功能,类似于Go语言中的asynq库。他询问是否有类似的Rust crate可供使用。

[
https://old.reddit.com/r/rust/comments/1g7z0oe/why_isnt_there_a_simple_and_efficient_distributed/
](
https://old.reddit.com/r/rust/comments/1g7z0oe/why_isnt_there_a_simple_and_efficient_distributed/
)
    


### TITLE

这篇文章是关于Rust到.NET编译器项目(rustc_codegen_clr)的最新进展。作者一直在重构这个项目,同时也加入了一些新的功能支持:

1. 新增了对夜间浮点数类型f128和f16的支持,尽管f128在某些系统上仍有限制。

2. 支持了内部编译器用于实现async的类型Coroutine,尽管实现可能还不完全正确。

3. 添加了对SIMD(单指令多数据)类型的基本支持,可以将Rust的SIMD类型映射到.NET对应的向量类型,并支持一部分SIMD内部函数。

4. 目前已有96.9%的核心测试用例通过,比两个月前有所提高。作者重构了后端代码,使其更易支持其他虚拟机和运行时。

5. 作者正在撰写第二篇关于panic实现的文章,内容较为深入和详细,因此进展有些缓慢。

6. 由于作者也在上学,项目进度受到一定影响。

总的来说,这个项目持续向前推进,新增了一些重要功能支持,同时作者也在不断重构和优化项目。

[
https://old.reddit.com/r/rust/comments/1g7jnac/rust_to_net_compiler_update_f128_f16_and/
](
https://old.reddit.com/r/rust/comments/1g7jnac/rust_to_net_compiler_update_f128_f16_and/
)
    


### TITLE

这是一个关于生成 CRC (循环冗余校验) 算法的 Rust 项目。作者之前实现了 CRC-24/OPENPGP 的 SIMD (单指令多数据) 版本,发现 CRC 算法具有通用性,因此尝试创建一个通用的 CRC SIMD 算法生成器。

这个周末,作者完成了这个项目,名为 crc-fast-rs。它包含一个用于代码生成的过程宏,以及一些用于基于 CRC 参数生成新 crate 的模板和脚本。目前它支持无反转的 8/16/24/32 位 CRC。使用这个工具可以在几分钟内生成新的 CRC 实现。根据基准测试,SIMD 实现比查表法快 50 倍,比简单循环快 200 倍。

该项目仍有一些待完善的地方,但作者对 Rust 的宏和工具链表示惊喜,尽管是 Rust 新手。作者还询问了社区的意见,因为这个工具最终会生成上百个不同的 CRC crate。

[
https://old.reddit.com/r/rust/comments/1g82uqt/my_weekend_project_a_simd_crc_algorithm_generator/
](
https://old.reddit.com/r/rust/comments/1g82uqt/my_weekend_project_a_simd_crc_algorithm_generator/
)
    


### TITLE

这段内容讨论了在Rust中使用LLDB/GDB调试器可视化工具的方法。作者提到可以使用`#![debugger_visualizer(gdb_script_file = "../foo.py")]`将调试格式化程序嵌入到Rust代码中。如果手动定义了这些格式化程序,这种方式非常有用。

作者询问是否有人使用过类似于`#[derive(Debug)]`这样的过程宏来自动生成Python代码,从而为调试目标提供方便。自动生成调试可视化工具将非常有用。

总的来说,这段内容关注于如何在Rust中方便地使用调试可视化工具,并探讨了自动生成这些工具的可能性,以提高调试效率。

[
https://old.reddit.com/r/rust/comments/1g8bmsi/lldbgdb_visualizers_derive_macro/
](
https://old.reddit.com/r/rust/comments/1g8bmsi/lldbgdb_visualizers_derive_macro/
)
    


### TITLE

该帖子宣布了一个名为meos-rs的Rust库,用于分析时空数据。这个库是基于C语言库MEOS的外部函数接口(FFI)绑定而构建的,MEOS库本身是PostgreSQL数据库扩展MobilityDB的后端。

作者提供了一个使用该库查找两个点之间最近距离的代码示例。作为第一个严肃的库项目,作者还就以下几个方面征求反馈意见:

1. README文档是否可读且有用?
2. -sys(原始绑定)crate是否缺少任何重要功能?
3. 用户API是否足够简单/直观?

作者表示,由于这是他们第一次进行FFI开发,因此非常欢迎任何反馈意见。

[
https://old.reddit.com/r/rust/comments/1g8bcky/meosrs_spatiotemporal_analysis_in_rust_feedback/
](
https://old.reddit.com/r/rust/comments/1g8bcky/meosrs_spatiotemporal_analysis_in_rust_feedback/
)
    


### TITLE

总结如下:

CanopyDB是一个用Rust编写的轻量级、高效的事务性键值存储引擎。它针对读取密集型和读写修改型工作负载进行了优化,采用MVCC设计和可选WAL(预写日志),可以比类似的替代品提供更好的写入性能和空间利用率,适用于更广泛的使用场景。

主要特点包括:

1. 完全事务性API,支持可串行化快照隔离的单写入者模式。
2. 类似BTreeMap的API,易于与Rust代码集成。 
3. 高效处理大值,并支持透明压缩。
4. 每个数据库可有多个键空间,键空间管理完全事务化。
5. 每个环境可有多个数据库,高效共享WAL和页面缓存。
6. 支持跨数据库原子提交,在多个数据库间保持一致性。
7. 可定制持久性,从同步提交到定期后台fsync。

CanopyDB在读取性能方面表现出色且稳定,写入性能和空间放大率也不错,有时可与基于LSM树的设计相媲美。该项目始于2020年,是作者对LMDB的一些缺陷(如最大键长510B、强制同步提交等)的不满而酝酿的实验性项目,经过多次重写。该作者希望这个项目能够为他人提供帮助。

[
https://old.reddit.com/r/rust/comments/1g7w628/canopydb_lightweight_and_efficient_transactional/
](
https://old.reddit.com/r/rust/comments/1g7w628/canopydb_lightweight_and_efficient_transactional/
)
    


### TITLE

这位开发者分享了他学习 Rust 的方式。一年前,他尝试构建一个简单的光线跟踪器项目,但感觉更多是学习了光线跟踪的原理,而没有真正学到很多 Rust 语言知识。

这一次,他采取了一种不同的方法。他决定构建一系列命令行工具,每个工具的范围都比较小,但涉及到广泛的crate、内置函数和语言特性。他计划在工作中使用这些工具,尽管标准工具可能会做得更好,但使用自己构建的工具是一种很好的学习动力。另外,他可以持续地为这个项目添加新功能,以学习更多新知识。

目前,这个项目已经涉及到网络、HTTP请求、文件操作、正则表达式、解析等多个方面。他建议其他想学习 Rust 的新手也可以尝试构建自己的工具集,来获取灵感和动力。最后,他欢迎任何反馈,因为他在 Rust 的学习之路上还很早。

[
https://old.reddit.com/r/rust/comments/1g86tyv/how_im_learning_rust/
](
https://old.reddit.com/r/rust/comments/1g86tyv/how_im_learning_rust/
)
    


### TITLE

作为一名有C/C++背景的开发人员,在使用Rust时,往往会习惯从头开始编写代码,而忽视了Rust生态系统中丰富的crate(包/库)资源。这种情况源于以下几个原因:

1. 开发者不太习惯浏览代码库去发现所需的crate,往往依赖Google搜索结果。

2. 在C/C++开发中,通常只需要选择一些大型子系统的库,其余代码自行编写。

3. 对于嵌入式开发,有些crate可能不完整、优化不足或被弃置,导致开发者对crates.io的导航感到困难。

4. 有C/C++背景的开发人员往往倾向于从头开发所需代码,而不太习惯复用现有库。

总的来说,对于C/C++转Rust的开发人员来说,如何高效地发现和使用Rust生态系统中优秀的crate是一个需要适应的地方。欢迎所有人就这个话题提供意见和建议。

[
https://old.reddit.com/r/rust/comments/1g80u99/people_with_cc_background_who_successfully_got/
](
https://old.reddit.com/r/rust/comments/1g80u99/people_with_cc_background_who_successfully_got/
)
    


### TITLE

该文章主要讨论了异步代码和阻塞代码在Rust语言中的抽象泄漏问题。

作者认为,与JavaScript等其他高级语言不同,在Rust中,阻塞代码才是真正的"抽象泄漏",因为它们无法很好地与异步代码交互。相比之下,异步代码可以通过 block_on 等机制从阻塞代码中调用,反之则比较困难。

文章还列出了一个表格,对比了异步代码和阻塞代码相互调用的难易程度。同时也提到了一些特殊情况,如GUI代码虽然使用阻塞语义但表现类似异步代码。

总的来说,作者认为在Rust中,异步代码具有更好的可组合性和可扩展性,而阻塞代码才是真正的"抽象泄漏"。

[
https://notgull.net/blocking-leaky
](
https://notgull.net/blocking-leaky
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

