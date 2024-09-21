【Rust日报】2024-09-21


### TITLE

这个代码库包含了名为PerpetualBooster的梯度增强算法,它不需要像其他梯度增强算法那样进行超参数优化。与AutoML库类似,它有一个预算参数,增加预算参数可以提高算法的预测能力,在看不见的数据上获得更好的结果。

该算法在相同的准确度下,通过不同的预算水平和数据集,可以比普通的梯度增强算法快100倍。文档中给出了在加州住房数据集(回归)和覆盖类型数据集(分类)上的基准测试结果。

代码库提供了Python和Rust两种语言的API接口,可以通过pip或cargo直接安装。作者正在撰写一篇论文,解释该算法是如何防止过度拟合的。该算法的工作原理在博客文章中有初步介绍。

[https://github.com/perpetual-ml/perpetual
](https://github.com/perpetual-ml/perpetual
)
    


### TITLE

这是一个使用Rust编写的开源项目,可以根据OpenStreetMap的地理空间数据在Minecraft Java版中生成现实世界的任何选定位置,具有很高的细节水平。该项目从Overpass API获取原始数据,包括每个元素(建筑物、墙壁、喷泉、农田等)的坐标节点和描述标签。运行时,它会自动完成数据获取、解析、排序、生成Minecraft世界、创建地面层等过程。

用户可以指定目标区域的经纬度范围坐标,生成相应的Minecraft世界。该项目旨在模块化、性能优化、跨平台支持、全面文档说明和用户友好体验。它欢迎所有人的贡献,无论是修复错误、改进性能、添加新功能还是增强文档。该项目使用GPL-3.0许可证。

[
https://github.com/louis-e/arnis
](
https://github.com/louis-e/arnis
)
    


### TITLE

Fjall是一个用Rust编写的基于LSM的可嵌入键值存储引擎。它具有以下特点:

- 线程安全的BTreeMap类API
- 完全安全和稳定的Rust代码
- 支持范围和前缀搜索,支持正向和反向迭代
- 自动后台维护
- 分区(也称为列族),支持跨分区原子语义
- 内置压缩(默认为LZ4)
- 单写入多读取事务(可选)
- 键值分离,支持大blob用例(可选)
- 每个Keyspace是一个单独的逻辑数据库,被分割成多个分区
- 每个分区物理上是单独的LSM树和逻辑集合,但跨分区的写操作是原子的

它不是一个独立服务器、关系数据库或宽列数据库。它对键和值大小有一定限制。Fjall在内部同步以支持多线程访问,并提供了持久性控制等功能。它使用稳定的磁盘格式,并提供迁移路径。该项目还提供了实际示例和文档,欢迎提问、反馈或贡献。项目代码以MIT或Apache-2.0许可开源。

[
https://github.com/fjall-rs/fjall
](
https://github.com/fjall-rs/fjall
)
    


### TITLE

以下是对该内容的中文总结:

与不使用键值对分离的版本相比,启用键值对分离可实现基本线性写入扩展,因为写入放大非常低。

两种垃圾回收配置的性能非常相似,但较积极的垃圾回收可防止临时空间使用过高,同时写入放大率基本相同。 

请注意,图表中的平缓部分是由于工作负载停止写入以截断项目历史记录造成的。

总的来说,该内容比较了启用和不启用键值对分离两种情况下的写入性能表现,并分析了不同垃圾回收配置对性能的影响。结果显示启用键值对分离可大幅提升写入扩展能力,而不同垃圾回收策略对写入放大影响不大。

[
https://fjall-rs.github.io/post/announcing-fjall-2
](
https://fjall-rs.github.io/post/announcing-fjall-2
)
    


### TITLE

这是一个用Rust编写的通用键值对分离存储库的实现,受到了RocksDB的BlobDB和Titan的启发。它提供了线程安全的API、支持通用KV索引结构(如LSM树)、可选的每个Blob压缩、用于热数据的内存Blob缓存(可在多个值日志之间共享以限制内存使用)以及在线垃圾收集。密钥长度限制为65536字节,值长度限制为2^32字节。它具有稳定的磁盘格式,版本1.0.0后的重大变更将导致主版本号升级和迁移路径。所有源代码使用MIT或Apache-2.0许可,贡献也需要使用这两种许可之一。该库旨在作为键值对分离存储的构建块。

[
https://github.com/fjall-rs/value-log
](
https://github.com/fjall-rs/value-log
)
    


### TITLE

这个仓库包含了一个 Rust 语言实现的日志结构合并树(LSM-tree/LSMT)。它提供了一个基本的 LSM 树实现,而不是一个完整的存储引擎。该实现具有以下特性:

- 线程安全的 BTreeMap 类似 API
- 100% 安全和稳定的 Rust 代码
- 支持压缩的基于块的表格
- 支持范围和前缀搜索,支持正向和反向迭代
- 支持分级、(并发)分层和 FIFO 合并策略
- 支持多线程刷新(不可变/密封内存表)
- 使用分区块索引减少内存占用并保持启动时间短
- 支持块缓存将热数据保存在内存中
- 支持 Bloom 过滤器提高点查询性能(可选)
- 支持快照(MVCC)
- 支持键值分离(可选)
- 支持单个删除墓碑("弱"删除)
- 键值大小有限制

该实现提供了多种压缩算法选择(lz4、miniz)和 Bloom 过滤器选项。磁盘格式在 1.0.0 版本后稳定,2.0.0 使用新格式需要手动迁移。所有源代码使用 MIT 或 Apache-2.0 许可证。该仓库还包括一些开发和基准测试的说明。

[
https://github.com/fjall-rs/lsm-tree
](
https://github.com/fjall-rs/lsm-tree
)
    


### TITLE

Dockyard 是一个现代化、快速和用户友好的 Docker 客户端,旨在简化 Linux 系统上的容器管理。它使用 Rust、Tauri 和 React.js 构建,提供了一个时尚的界面和强大的功能,使处理 Docker 容器变得前所未有的简单。

主要特性包括:直观的 UI 界面、实时监控容器状态、容器管理、容器内部终端、日志查看器、高性能、多主题等。

Dockyard 支持在 Linux 系统上安装,提供了 .deb 包和 AppImage 文件的安装方式。用户也可以从源代码构建。该项目欢迎社区贡献,提供了贡献指南。

Dockyard 目前只支持 Linux 系统,但未来计划扩展到 Windows 和 macOS。它的路线图还包括更高级的容器管理功能和镜像管理功能。

Dockyard 使用 MIT 许可证,感谢使用了 Tauri、React.js 和 Rust 等技术,并受到作者个人需求的启发而创建。用户可以在 GitHub 上报告任何问题。

[
https://github.com/ropali/dockyard
](
https://github.com/ropali/dockyard
)
    


### TITLE

以下是内容的中文总结:

这是关于PerpetualBooster v0.4.7版本的发布公告。PerpetualBooster是一种无需调整超参数的梯度增强算法,可简化模型构建过程。此版本带来了多线程支持,大幅提升了性能,同时增加了量化回归任务的功能。

PerpetualBooster类似于AutoML,只需要设置一个"budget"参数来控制模型复杂度,从而在看不见的数据上获得更好的性能表现。使用方式非常简单,只需几行Python代码即可实例化模型、拟合数据。

该软件可通过pip install perpetual命令安装。源代码仓库地址为https://github.com/perpetual-ml/perpetual。

[
https://old.reddit.com/r/rust/comments/1flzpoj/perpetualbooster_improved_multithreading_and/
](
https://old.reddit.com/r/rust/comments/1flzpoj/perpetualbooster_improved_multithreading_and/
)
    


### TITLE

这位开发者正在构建一个包含网页视图组件的桌面应用程序,但在选择最佳实现方式时遇到了困难。他的目标是尽可能轻量级(最终二进制文件尺寸小)。

他尝试过几种方法,包括使用gtk-rs和webkitgtk,但后者似乎已被废弃。他也尝试过cef-rust与Chromium嵌入式框架,但不确定它是否有太多开销。他还考虑过tao,但仍然不确定。

他的要求是:

1. 必须能在Windows上运行,但最好是跨平台的。
2. 应该可以很容易地与现有的Rust代码库集成。

他想知道是否有直接使用操作系统原生网页视图的方式?Windows似乎通过Edge提供了这个功能,但是是否有一个跨平台的crate可以做到在最终二进制文件中几乎不增加任何负担?

[
https://old.reddit.com/r/rust/comments/1flziy0/best_way_to_run_a_webview_in_a_rust_desktop_app/
](
https://old.reddit.com/r/rust/comments/1flziy0/best_way_to_run_a_webview_in_a_rust_desktop_app/
)
    


### TITLE

这个提问者正在使用Rust的Serde库来反序列化一些不同的类型。但是,API响应中包含了很多中间包装器,使得想要的数据嵌套在多层结构中。他们希望能够使用Serde的#[serde(rename)]属性来直接访问嵌套的字段,避免在类型级别上引入这些包装器。

例如,API响应可能是这样的:

```json
{
    "data": {
        "actualData": {
            "someField": ...,
            "anotherField": ...
        }
    }
}
```

而他们想要的是能够这样定义struct:

```rust
#[derive(Deserialize)]
struct A {
    #[serde(rename(deserialize = "data.actualData.someField"))]
    someField: ...,
    ...
}
```

但是rename属性的语法不支持这种嵌套路径的访问方式。所以提问者想知道是否有办法用Serde实现这一需求,或者是否只能为每个这样的类型手写Deserialize实现。

[
https://old.reddit.com/r/rust/comments/1fm4eat/serde_lifting_values_from_nested_levels/
](
https://old.reddit.com/r/rust/comments/1fm4eat/serde_lifting_values_from_nested_levels/
)
    


### TITLE

这是一个来自Reddit的帖子,主要内容是:

作者想学习Rust编程语言,计划采用每天做一道LeetCode练习题的方式来掌握Rust。他之前使用类似的方法学习了Go语言,工作中也广泛使用Go语言。他想确认用这种每天做算法练习题的方式来学习Rust是否是一个不错的主意。

总的来说,这位开发者正在寻求一种高效的方式来学习Rust语言,他打算通过每天解决LeetCode上的算法问题来实践和掌握Rust语言的语法、特性以及解决问题的思路。这种学习路线之前对他学习Go语言很有效,所以他希望能在学习Rust时复制这个成功的学习模式。

[
https://old.reddit.com/r/rust/comments/1flwxet/does_anyone_use_rust_for_leetcode/
](
https://old.reddit.com/r/rust/comments/1flwxet/does_anyone_use_rust_for_leetcode/
)
    


### TITLE

这是一个使用Rust语言编写的开源项目,旨在从真实世界生成任何位置到Minecraft游戏中,包括街道、建筑物、公园等。作者之前使用Python语言开发过该项目,最近将其移植到Rust语言,以便更深入地学习Rust。作者希望能获得关于代码质量的反馈,并渴望该项目能够不断发展壮大。该项目的GitHub链接为https://github.com/louis-e/arnis,同时还附有一张预览图片展示了生成的城市场景。

[
https://old.reddit.com/r/rust/comments/1flid4l/minecraft_real_life_city_generator_written_in/
](
https://old.reddit.com/r/rust/comments/1flid4l/minecraft_real_life_city_generator_written_in/
)
    


### TITLE

这是一个关于2024年Minecraft生成设计比赛的获奖作品的代码仓库。该项目旨在在现有的Minecraft地图中生成一个具有适应性、功能性、富有表现力的叙事和美学的定居点。与大多数只生成静态村庄实例不同,该生成器会模拟村庄的建造过程,并在Minecraft中重放这一过程。

运行重放只需要Minecraft,不需要任何模组或外部程序。该代码使用Rust编写,旨在直接与世界进行交互而不是通过http接口,从而提高运行速度。代码中没有严格的框架和生成器分离、缺乏文档注释、没有注重可维护性和最佳实践。

该模拟使用ECS架构,将物体(如村民或树木)组成数据组件(如位置或树木)和实现行为的系统。方块和一些辅助数据以栅格格式存储。为了建造建筑物,村民需要运送物资到施工现场并放置方块。每个模拟刻都对应一个游戏刻,世界的变化会被记录为Minecraft命令以供重放。

重放可以暂停或快进。模拟是伪随机的但具有确定性(有利于调试)。代码目前还没有进行许多优化,但是世界加载已经并行化,nbt编码和gzip压缩被卸载到工作线程中。该仓库提供了运行生成器的说明。

[
https://github.com/SpecificProtagonist/frightful_hobgoblin
](
https://github.com/SpecificProtagonist/frightful_hobgoblin
)
    


### TITLE

这位Rust程序员在GitHub设置双重身份验证时,获得了一系列备用代码,这些代码都是十六进制数字格式。为了方便查看,他决定编写一个小程序来将这些十六进制数字转换成十进制数字。

他用Rust编写了一个简单的程序,首先从文件中读取代码,然后将代码按行分割成向量。接着遍历向量,将每个十六进制代码转换为十进制数字,并将结果打印出来。

虽然这个程序很小,但对他来说是一个重大胜利,因为这是他第一次使用编程来解决个人问题。这让他更加欣赏编程语言的强大,即使是最小的程序也能节省大量时间和精力。

他鼓励正在学习编程语言的人要坚持下去,因为你会惊讶于它能以意想不到的方式提供帮助。他对自己使用Rust编程语言解决这个小问题而感到高兴。

[
https://old.reddit.com/r/rust/comments/1fm62lv/for_the_first_time_ever_rust_helps_me_with_my/
](
https://old.reddit.com/r/rust/comments/1fm62lv/for_the_first_time_ever_rust_helps_me_with_my/
)
    


### TITLE

Fjall 2.0是一个可嵌入的基于LSM树的Rust键值存储引擎,这是一个重大更新,为未来2.x版本奠定了基础。主要新特性是(可选的)键值分离,受RocksDB的BlobDB和Titan的启发,由新发布的crate value-log提供支持。键值分离旨在处理较大的值用例,允许在线可调垃圾收集,从而实现低写放大。该版本的完整博客文章、代码仓库和Discord链接也一并提供。

[
https://old.reddit.com/r/rust/comments/1fm22bg/just_released_fjall_20_an_embeddable_keyvalue/
](
https://old.reddit.com/r/rust/comments/1fm22bg/just_released_fjall_20_an_embeddable_keyvalue/
)
    


### TITLE

这位开发者是出于个人需求开发了一款名为Dockyard的开源Docker桌面客户端应用程序。几年前,他需要在Linux机器上管理Docker容器,但当时官方的Docker桌面软件不支持Linux系统,而来自开源社区的替代方案也不尽人意。因此,他着手构建Dockyard,旨在提供一款以Linux为先的简单设计、易于使用的工具。

Dockyard使用Rust和Tauri框架开发,目前支持Linux和macOS系统。你可以从Github的发布页面下载它。开发者希望大家体验并反馈意见,如果喜欢这个项目,不要忘记为它加星支持。他附上了一张Dockyard的预览图片,展示了其简洁直观的用户界面。

总的来说,这是一个基于开发者个人需求而诞生的实用开源项目,旨在提供一种简单管理Docker容器的体验。

[
https://old.reddit.com/r/rust/comments/1flwlwl/meet_my_open_source_project_dockyarda_docker/
](
https://old.reddit.com/r/rust/comments/1flwlwl/meet_my_open_source_project_dockyarda_docker/
)
    


### TITLE

以下是对该内容的中文总结:

这是一个关于Rust语言中原子引用计数(Arc)的问题。提问者表示他一直在努力理解Arc的作用,知道它意味着原子引用计数变量,但不明白普通引用计数和原子引用计数的区别。

他阅读的资料说原子操作是"一次性"的,而不是分成多个操作。但在这种情况下,原子操作能实现什么?

总的来说,这个问题探讨了Rust中Arc与普通引用计数的不同之处,以及原子操作在多线程编程中的意义和作用。提问者试图加深对这个概念的理解。

[
https://old.reddit.com/r/rust/comments/1fm3697/what_do_atomic_operations_accomplish/
](
https://old.reddit.com/r/rust/comments/1fm3697/what_do_atomic_operations_accomplish/
)
    


### TITLE

这篇博客文章介绍了rustc_codegen_gcc项目最新进展中遇到的一些挑战。主要涉及以下几个方面:

1. 与上游Rust编译器同步的困难。rustc_codegen_gcc需要与Rust编译器保持同步,以获取最新特性和支持。但每次同步时,都可能会导致一些测试失败,需要花费大量时间调查和修复。

2. AVX-512内在函数支持的挑战。Rust增加了100多个新的AVX-512内在函数,但GCC和LLVM之间的映射关系不完全一致,需要人工维护映射关系,是一个耗时的过程。

3. GCC后端的Bug。在使用Intel语法时,GCC后端存在一个Bug,生成了无效的汇编代码,需要定位和修复。

4. 目标特性启用的问题。有一个错误暗示rustc_codegen_gcc在函数级别启用目标特性时出现了问题,需要进一步调查。

5. 优先级调整的考虑。作者建议可能需要调整优先级,暂时推迟最新AVX-512内在函数的支持,先集中精力解决其他更重要的特性,以加快同步进度。同时,也可以将一些相对简单的任务分配给新的贡献者。

总的来说,rustc_codegen_gcc项目目前在与上游同步方面面临一些技术挑战,作者正在探索解决方案和优先级调整,以推进项目进展。

[
https://blog.antoyo.xyz/development-rustc_codegen_gcc
](
https://blog.antoyo.xyz/development-rustc_codegen_gcc
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

