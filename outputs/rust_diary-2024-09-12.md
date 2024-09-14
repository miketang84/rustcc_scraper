【Rust日报】2024-09-13


### TITLE

总结一下,Kartoffels是一个有趣的游戏,玩家需要使用Rust语言为一个马铃薯实现固件。作者最近将前端从JavaScript迁移到了基于Rust的TUI,集成了Ratatui和russh库,效果相当不错。目前大部分功能都在Rust端实现,这使得测试和通过ssh远程访问成为可能。在下一个版本中,作者计划关注单人模式,引入"挑战"标签,让玩家在多人对战区之外,解决诸如导航迷宫等特定问题。这无疑会让游戏更加有趣。

[https://old.reddit.com/r/rust/comments/1ff7je9/kartoffels_a_robot_combat_arena_v04_released_feat/
](https://old.reddit.com/r/rust/comments/1ff7je9/kartoffels_a_robot_combat_arena_v04_released_feat/
)
    


### TITLE

以下是对该内容的中文总结:

这是由filtra公司发布的2024年8月锈语言工作报告。报告显示,8月份锈语言工作机会总数保持在801个,与7月持平。但使用锈语言的公司数量增加了2家,达到113家。报告列出了使用锈语言最多的前十大公司,其中亚马逊以184个职位排名第一,微软位居第二。科技公司、加密货币公司和生产力软件公司是使用锈语言最多的行业。此外,报告还分析了不同级别锈语言工程师的工作机会数量。总的来说,这份报告全面展示了目前锈语言职位市场的现状。

[
https://filtra.io/rust-aug-24
](
https://filtra.io/rust-aug-24
)
    


### TITLE

这是一个关于Rust语言中名为steady_state的actor框架的介绍。该框架基于actor模型实现异步多线程编程,无需使用trait。actor被监督并在panic时自动重启。有界通道支持批量处理和服务级别协议管理。关闭时可以通过级联机制干净地关闭所有actor。actor可以被分配专用线程或共享线程。内置了遥测功能,可以通过浏览器访问9100端口查看,并为Prometheus提供了指标供抓取。该框架在Crates.io上已经下载量超过10,000次,是一个值得关注的异步并发编程框架。

[
https://old.reddit.com/r/rust/comments/1ffkl91/new_actor_framework_for_reliable_services_with/
](
https://old.reddit.com/r/rust/comments/1ffkl91/new_actor_framework_for_reliable_services_with/
)
    


### TITLE

这个帖子提出了一些关于Rust编译的WebAssembly(WASM)模块中"资源"(resources)的细节问题。作者声明在一个导出的Rust接口中定义了一个名为state的资源,并希望能够从WASM宿主环境中创建这个资源的实例。

作者注意到wit-bindgen不仅生成了导出的构造函数mypkg/myinterface#[constructor]state,还生成了导入的[export]mypkg/myinterface#[resource-new]state和[export]mypkg/myinterface#[resource-drop]state函数。作者理解这些函数属于所谓的"规范内置"(canonical built-ins),但不太清楚它们的用途。

作者发现[resource-new]state函数似乎被编译后的构造函数调用,而在宿主环境中,作者目前只是简单地实现了一个返回参数的函数,该参数似乎是一个"类型索引"(type index)。作者想知道这样做是否正确,以及采用不同实现是否会有收获。

总的来说,这个帖子探讨了WASM模块中资源的细节,特别是wit-bindgen生成的相关函数的用途,以及在宿主环境中处理这些函数的正确方式。

[
https://old.reddit.com/r/rust/comments/1ffcilm/understanding_wasm_resources_and_mapping_to_low/
](
https://old.reddit.com/r/rust/comments/1ffcilm/understanding_wasm_resources_and_mapping_to_low/
)
    


### TITLE

这是一篇关于参加RustConf会议的心得体会。作者表示,虽然这一天同时令人筋疲力尽和有许多领悟。会议将所有使用Rust编程语言的"书呆子"聚集在一起,成为一个有趣的社会实验。

会议的演讲展示了Rust工具(如rustfmt)背后的深思熟虑,介绍了一些新的工具(cargo-mutants和rkyv),并让作者结识了一些只在网上看过头像的人。

对于那些由于工作、学习或经济原因无法亲自到场的人,作者在这篇文章中分享了当天所学到的一些体会,为无缘亲临会场的人提供了一个窗口。

[
https://old.reddit.com/r/rust/comments/1fesayw/my_day_at_rustconf/
](
https://old.reddit.com/r/rust/comments/1fesayw/my_day_at_rustconf/
)
    


### TITLE

这位程序员是一名研究生,正在学习机器学习。他最近开始学习Rust语言,尽管之前有一些C#、Java和Python的编程经验,但对底层编程语言还是新手。

他决定直接使用Rust语言编写一个用于板游的搜索算法,这是他的一门课程项目。在编码过程中,他遇到了一些Rust语言的特性,如生命周期、`dyn`关键字、双重引用和解引用,虽然编译器给出了有用的提示,但他并不完全理解这些概念。他一直在为每个结构体和实现对编写单元测试,以确保获得预期的行为。

他担心自己正在学习的过程中,可能会遇到一些隐藏的错误,导致整个程序崩溃或者影响到计算机的正常运行,尤其是在使用底层编程语言的情况下。他想知道最坏的情况会是什么,以及可能会遇到哪些隐藏的错误。他认为自己对底层编程语言的担心可能过于夸大了。总的来说,这是他第一次使用底层编程语言,并直接编写了一个相当复杂的搜索算法,所以对可能出现的问题感到担心。

[
https://old.reddit.com/r/rust/comments/1ff0msk/whats_the_worst_that_can_happen_if_the_code/
](
https://old.reddit.com/r/rust/comments/1ff0msk/whats_the_worst_that_can_happen_if_the_code/
)
    


### TITLE

该帖子是一位Rust开发者在开发本地优先Web应用时遇到的问题。他最初使用automerge来处理CRDT(状态基数据类型)数据,但遇到了一些严重的可用性问题。他认为automerge在Rust方面的API使用起来过于困难,问题反馈也没有得到解决,文档链接也出现404错误,感觉该项目在Rust方面已被放弃。因此,他考虑使用其他CRDT库loro作为替代,并征求其他人对loro或其他CRDT库的看法和建议。

[
https://old.reddit.com/r/rust/comments/1ffbspu/asking_for_thoughts_on_automerge_loro_other_crtd/
](
https://old.reddit.com/r/rust/comments/1ffbspu/asking_for_thoughts_on_automerge_loro_other_crtd/
)
    


### TITLE

这个仓库包含了一个名为"kartoffel"的机器人入门程序包。用户可以克隆这个仓库,运行./build命令,然后将生成的./kartoffel文件上传到游戏中(或者通过ssh kartoffels.pwy.io访问)。这个程序包提供了一个机器人框架,主要用于游戏教程,但是在实际战场环境下可能无法生存太久。

创建一个"kartoffel"时,用户实现的是一个固件,因此无法访问标准库(如std::fs、println!()等)。不过,可以通过serial_send()进行通信,并且可以使用alloc crate提供的vec或format!()等高级功能。该机器人拥有64kHz的CPU和128kib的RAM。

该作品采用CC0 1.0通用许可,作者已将其权利在全球范围内完全开放,允许他人任意复制、修改、分发和表演该作品,包括用于商业目的。

[
https://github.com/patryk27/kartoffel
](
https://github.com/patryk27/kartoffel
)
    


### TITLE

这是一段在终端里运行某个程序或游戏的记录视频。视频展示了一个有文字和图形元素的界面,有分数、状态等显示,似乎是某种模拟或互动程序。屏幕上有一些指示,如暂停、帮助、机器人等功能按键。整体界面设计简洁、视觉直观,给人一种赛博朋克的科技感。视频最后提供了一些分享、下载、嵌入等选项。

[
https://asciinema.org/a/EW4DDQocLzK4WwIPE6NbL1jdo
](
https://asciinema.org/a/EW4DDQocLzK4WwIPE6NbL1jdo
)
    


### TITLE

steady_state是一个用于在Linux系统上构建长期运行、低延迟的基于actor模型的服务框架。它提供了可视化的遥测功能,用于监视actor之间的数据流。steady_state使用了类似于Erlang的监督者模式来防止panic,并使用非阻塞的异步环形缓冲区进行actor之间的通信。这个crate最新版本是0.0.21,遵循MIT许可协议。它托管在GitHub上,总计有10,181次下载,最近90天的下载次数在统计图表中有显示。使用cargo add steady_state或在Cargo.toml中添加steady_state = "0.0.21"即可将其集成到您的Rust项目中。

[
https://crates.io/crates/steady_state
](
https://crates.io/crates/steady_state
)
    


### TITLE

以下是对博文内容的中文总结:

作者参加了Rust会议,分享了当天所听的一些精彩演讲。

1. cargo-mutants工具能对测试代码进行随机更改,帮助检查代码健壮性。

2. 两个演讲分别介绍了用Rust构建MongoDB驱动和实时公交车地图应用的经验,深入探讨了数据库和前端应用的连接。

3. rustfmt代码格式化器背后的工作原理很复杂,需要生成中间表示并完全匹配编译器的代码解析。

4. rkyv是一个零拷贝反序列化框架,在现场发布了2.5年来的新版本。它能高效地将数据序列化为字节slice,但需处理指针、字节序等边缘情况。

5. 演讲者赞赏了Rust社区的才华和热情,即使有时会感到"行业术语"让人有点迷惑。

总的来说,作者对Rust会议上展示的创新技术和项目有了新的认识和体会。

[
https://oneirical.github.io/rustconf1/
](
https://oneirical.github.io/rustconf1/
)
    


### TITLE

这篇帖子总结了 Reddit 上 Rust 编程语言相关社区(/r/rust)的一些热门讨论主题和新闻动态。主要包括:

1. Rust 到 .NET 编译器项目进展,包括通过 GSoC 项目后续计划,以及最新进展已通过 95% 的单元测试。

2. 一些 Rust 相关的开源项目和代码库发布,如 axum Web 框架、ratatui 的 GPU 后端、Excel 文件创建库 rust_xlsxwriter、数据可视化库 Plotlars 等。

3. Rust 语言特性和优化的讨论,如利用 Rust 类型系统进行更好的优化、内存安全的定义等。

4. Rust 作为第一语言学习的体会分享,以及如何向 C++ 开发者介绍 Rust 的问题。

5. Rust Web 框架的选择建议以及 Rust 面试可能会被问到的问题。

6. 全 Rust 原生深度学习框架 Burn 0.14.0 版本发布等新闻动态。

总的来说,这是一个关于 Rust 语言、生态系统、学习资源和社区动态的综合性内容汇总。

[
https://www.reddit.com/r/rust/s/OONTQpZYEj
](
https://www.reddit.com/r/rust/s/OONTQpZYEj
)
    


### TITLE

这段代码展示了如何使用Rust中的automerge库创建一个联系人列表。代码首先创建了一个新的AutoCommit对象,然后在其根对象中插入一个名为"contacts"的列表对象。接下来,代码在这个列表中插入两个对象,每个对象表示一个联系人,并给每个联系人对象设置"name"和"email"键值对。最后,代码将整个文档序列化为字节向量,以便存储或传输。这个例子展示了如何使用automerge库创建和修改结构化的文档数据。

[
https://docs.rs/automerge/latest/automerge/
](
https://docs.rs/automerge/latest/automerge/
)
    


### TITLE

这段代码演示了如何使用 Rust 的 loro 库来创建和操作 JSON 格式的数据。它首先创建了一个新的 `LoroDoc` 对象 `doc`。然后它在 `doc` 中插入一个名为 `"map"` 的新映射 (map)。接着，它向这个 `map` 中插入了几个键值对，包括字符串、布尔值和 `null` 值。它还演示了如何删除一个键值对。

之后，代码在 `map` 中插入了一个新的列表 (list) 容器 `"list"`，并向其中添加了一个字符串和一个数字。同样，它还插入了一个新的文本 (text) 容器 `"text"`，并插入了一个字符串 `"Hello world!"`。

最后，代码使用 `doc.get_deep_value().to_json_value()` 将整个文档转换为 JSON 值，并使用 `assert_eq!` 来验证转换后的 JSON 值是否与预期的 JSON 相同。

[
https://docs.rs/loro/latest/loro/index.html
](
https://docs.rs/loro/latest/loro/index.html
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

