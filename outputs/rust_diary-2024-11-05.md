【Rust日报】2024-11-05


### TITLE

这个GitHub存储库包含了一个Rust crate,名为diffogus,用于计算同一类型的两个实例之间的差异。它具有以下特性:

1. 简单地对基本Rust类型进行比较,包括所有整数类型、浮点数类型。

2. 对实现了Diffable特性的元素的向量进行比较。

3. 对键值对中的值实现了Diffable特性的HashMap进行比较。

4. 对包装了实现了Diffable特性的类型的Option进行比较。

5. 对实现了Diffable特性的结构体的两个实例进行比较。可以手动实现Diffable,或者使用#[derive(Diff)]特性派生。目前只支持具有命名字段的结构体。

6. 使用serde特性标志,可以序列化差异结果。

总的来说,diffogus是一个方便的Rust crate,用于比较同一类型实例之间的差异,支持多种数据类型,并提供了自定义实现和派生的方式。

[https://github.com/funlennysub/diffogus
](https://github.com/funlennysub/diffogus
)
    


### TITLE

这篇文章介绍了如何从头开始构建一个3D终端渲染器。主要内容包括:

1. 总体思路是通过屏幕上的每个像素"发射"一条射线,计算射线与三角形网格的交点,并使用ANSI颜色代码在终端中显示对应三角形的颜色。

2. 使用Rust语言,设置项目结构和依赖库。

3. 定义Ray(射线)结构体,表示射线的起点和方向。

4. 定义Screen(屏幕)结构体,包括宽高、焦距等,用于确定射线的方向。

5. 定义Camera(相机)结构体,表示射线的起点位置。

6. 在render方法中,遍历屏幕上的每个像素,构造对应的射线。

7. 定义Triangle(三角形)和Mesh(网格)结构体,mesh包含多个三角形。

8. 在render中遍历网格中的每个三角形,计算射线与之的交点并在终端显示相应颜色。

总的来说,这是一个从零开始构建简单3D终端渲染器的教程,阐述了实现的基本思路和核心数据结构。

[
https://tagedan.github.io/posts/terminal_rendering.html
](
https://tagedan.github.io/posts/terminal_rendering.html
)
    


### TITLE

这是一个名为TermTrack的终端渲染3D平台/迷宫游戏仓库,专注于速通和自定义关卡创建。游戏需要在终端中运行,目前建议在Windows系统上使用Microsoft Store的Windows Terminal。该仓库提供了Windows和Linux版本的游戏压缩包,用户可以下载解压后直接运行。

游戏关卡是通过文本文件定义的,使用不同的字符表示不同的网格类型,如起点、终点、墙壁、地板、陷阱等。用户可以创建自定义关卡并将其放入专门的文件夹中运行。

该仓库还列出了已知的一些bug,如网络请求错误处理不够完善、排行榜验证存在缺陷、音乐在高负载时会crackling等。未来的计划包括独立的排行榜、敌人音效、关卡编辑器、Discord机器人和3D对象文件加载器等。

如果用户有任何疑问,可以查看GitHub讨论区,或者创建新的讨论主题。开发者也欢迎用户提出新功能的需求,或者fork该项目并提交改进的拉取请求。最后,仓库对一些支持者和使用的工具表示了感谢。

[
https://github.com/TermTrack/TermTrack
](
https://github.com/TermTrack/TermTrack
)
    


### TITLE

这是一个GitHub个人资料页面的概览信息。它显示了用户名为abbfelarb的GitHub账户中最受欢迎的两个公共代码库,分别是flappyMoa(JavaScript项目)和talaBot(也是JavaScript项目)。此外,它还列出了过去一年中该用户每天的代码提交记录。大部分日期都显示为没有贡献,只有在2022年9月29日和9月30日各有4次和7次提交。在2023年4月16日也有2次提交记录。总的来说,这位用户在最近一年里的代码贡献活跃度不是很高。

[
https://github.com/abbfelarb
](
https://github.com/abbfelarb
)
    


### TITLE

rav1d是一个跨平台的AV1视频解码器,是dav1d解码器的Rust移植版本,专注于速度和正确性。它是用Rust编写的,可以使用标准的Rust工具链进行构建。

主要特点:

1. 支持x86、x86_64和aarch64架构的稳定版本,arm和riscv64需要nightly编译器。
2. 支持汇编优化、8位、10位和12位色深解码等特性,可通过cargo特性标志进行配置。
3. 可跨平台编译,并提供了多种目标平台的示例编译命令。
4. 使用Meson测试套件进行测试,提供了test.sh辅助脚本运行测试。
5. 提供与libdav1d兼容的C API,可作为libdav1d的替代品使用。

总的来说,rav1d是一个基于Rust编写的高性能AV1解码器,具有跨平台、可配置和测试完备等优点。

[
https://github.com/memorysafety/rav1d
](
https://github.com/memorysafety/rav1d
)
    


### TITLE

这是一个 GitHub 上的拉取请求讨论,主要涉及在 exrs 项目中添加 Rayon 特性的工作。主要讨论点包括:

1. 贡献者 oligamiq 提交了一个拉取请求,在 exrs 项目中添加 Rayon 特性作为可选项。

2. 一些评论者指出,这可能会导致向后不兼容的破坏性更改,因为任何依赖现有 API 的下游代码都可能无法编译。

3. 维护者 johannesvollmer 认为,由于调用 Rayon 的地方很少,可以在运行时根据是否启用 Rayon 特性来选择使用并行或串行处理,从而避免 API 变动。但后来发现有一个公共函数 new_with_thread_pool 暴露了 Rayon 线程池,这使得避免破坏性更改变得不可能。

4. 最后,维护者和贡献者同意,添加 Rayon 特性需要进行主版本升级到 exr 2.0,以保持向后兼容性。贡献者表示在一段时间后会尝试保持公共 API 不变,同时支持运行时选择使用 Rayon 或不使用。

总的来说,这是一个关于在开源项目中添加新特性时,如何权衡向后兼容性和新功能的讨论。

[
https://github.com/johannesvollmer/exrs/pull/241
](
https://github.com/johannesvollmer/exrs/pull/241
)
    


### TITLE

以下是对该内容的中文总结:

该内容是GitHub的一个页面,展示了一位用户awxkee的个人主页。在这个页面上,有几个选项可供操作:

1. 屏蔽用户(Block user) - 可以阻止该用户与你的代码仓库交互或发送通知给你。

2. 报告滥用行为(Report abuse) - 如果该用户有任何不当行为,你可以向GitHub支持团队报告,说明该用户的不当行为。

3. 联系GitHub支持(Contact GitHub support) - 你可以就该用户的行为联系GitHub的支持团队,寻求帮助。

该页面还提供了一个可选的文本框,允许你输入最多100个字符的注释,使用markdown语法。这个注释只有你自己可见。

总的来说,这是GitHub为了让用户能够有效管理其他用户与自己交互的一个功能页面。

[
https://github.com/awxkee
](
https://github.com/awxkee
)
    


### TITLE

这个问题询问了在Kubernetes环境中运行基于Tokio的Rust I/O密集型应用程序的适当设置。具体来说:

1. 对于分配的CPU资源小于1个vCPU(如<100m)的情况下,是否应该始终使用current_thread调度器,或者还有其他需要考虑的因素?

2. 如果有关于Tonic(一个基于Tokio的gRPC库)的优秀文档/书籍推荐,也希望能获得相关推荐。

这些问题涉及到在资源受限的Kubernetes环境中,如何合理配置和优化基于Tokio的Rust应用程序的执行,以获得良好的I/O性能表现。对于相关主题的权威指导资料,作者也在寻求推荐建议。

[
https://old.reddit.com/r/rust/comments/1gk1n4l/a_question_on_tokio_in_kubernetes/
](
https://old.reddit.com/r/rust/comments/1gk1n4l/a_question_on_tokio_in_kubernetes/
)
    


### TITLE

该内容讨论了在Rust语言中实现一个高效的环形缓冲区(ring buffer)的需求。作者希望能够在一个线程中连续发送数据到缓冲区,而另一个线程则从缓冲区中接收数据。当缓冲区满时,发送操作会暂停;当缓冲区为空且未被关闭时,接收操作会暂停。

作者考虑直接使用Rust标准库中的mpsc(多生产者单消费者)通道,但由于只有单个生产者线程,因此担心可能会影响性能。他希望能够找到一个现有的高质量环形缓冲区实现,而不必自己从头实现。

作者的最终目标是在多个线程之间实现类似于Unix管道(cat foo | grep bar | sort)的并行处理,每个线程负责一个独立的处理步骤,这种并行处理方式被认为更易于推理和调试,同时也能最大限度利用多核系统的计算能力。

[
https://old.reddit.com/r/rust/comments/1gkhxmq/looking_for_a_good_ring_buffer_implementation/
](
https://old.reddit.com/r/rust/comments/1gkhxmq/looking_for_a_good_ring_buffer_implementation/
)
    


### TITLE

这是一个关于Rust编程语言中一个名为diffogus的crate(库)的介绍。diffogus可以显示两个相同类型实例之间的差异。

该crate支持所有基本类型(整数、浮点数和布尔值)、Option<T>、HashMap<K, T>、Vec<T>(其中T实现了所需的trait)之间的差异对比。你也可以为自己的struct实现diff功能,可以使用Diff派生宏(可选特性),或者手动实现。

此外,你还可以使用serde特性将diff结果序列化为所需的格式。

目前,该crate不是零拷贝的,在某些地方使用了.clone()方法。作者表示对于他的项目来说这不是个大问题,所以在发布之前决定不花时间实现零拷贝。另外,Diff宏目前只支持具有命名字段的struct,对于其他情况,你可能需要手动实现diff功能,不过作者可能会在以后添加对其余情况的支持,或者其他人可以提交PR,作者会审查。

[
https://old.reddit.com/r/rust/comments/1gkd0bc/diffogus_a_diffing_crate/
](
https://old.reddit.com/r/rust/comments/1gkd0bc/diffogus_a_diffing_crate/
)
    


### TITLE

这是一个关于如何加速Rust编译时间的问题。作者正在开发一个类似电子商务的应用程序,该应用程序主要由两个大的crate组成。通过运行`cargo build -p server --timings --release`命令,作者发现这两个crate占用了大约90%的编译时间。

作者知道将代码分割成更多的模块可能会加快编译速度,但他想知道有没有什么启发式方法可以避免大规模的重构,同时又能获得较大的编译加速效果。

目前,代码已经按照主要功能区域(如Customer、Order、Invoice等)分割成了不同的模块,但是这些模块之间存在大量的相互依赖关系,例如Customer模块依赖于其他7个模块。

作者希望获得建议,如何有效地拆分和重组代码,以加快编译速度,同时又不会导致过多的重构工作量。

[
https://old.reddit.com/r/rust/comments/1gkjlkp/how_to_guess_what_to_do_for_speed_up_the_compile/
](
https://old.reddit.com/r/rust/comments/1gkjlkp/how_to_guess_what_to_do_for_speed_up_the_compile/
)
    


### TITLE

这段内容是一个Rust编程语言的问题。作者是一个Rust新手,正在学习Rust的编程。他遇到了一段关于Deref trait的示例代码,无法理解其中的自动dereferencing(解引用)机制是如何工作的。

代码中定义了一个`Selector`结构体,其中包含一个`Vec<T>`存储元素,和一个`current`索引指向当前元素。`Selector`实现了`Deref`和`DerefMut`trait,允许它像指针一样访问当前元素。

作者的疑惑在于,如果`Selector`的泛型参数`T`是`&str`(字符串slice),那么`Deref`trait中的目标类型`Target`也会是`&str`吗?而`deref`方法返回的又是`&&str`(字符串slice的引用)吗?他不理解在这种情况下自动解引用是如何工作的。

总的来说,这是作者对Rust的自动dereferencing特性及其在泛型代码中的应用存在疑惑,希望能获得解答来加深对这一特性的理解。

[
https://old.reddit.com/r/rust/comments/1gkc62c/doubt_on_deref_coercion/
](
https://old.reddit.com/r/rust/comments/1gkc62c/doubt_on_deref_coercion/
)
    


### TITLE

这段代码讨论了在 Rust 中高效地从一个字节数组中减去常量值的算法。作者提供了一个 `sub_16` 函数,用于从一个包含4个字节的数组中减去16。然而,生成的汇编代码过于庞大和低效。因此,作者在寻求一种更好的方法来执行这种在较小字节集上的操作。这个问题出现在一个需要大量计算彩色光照值的体素世界程序中,所以性能很关键。总的来说,这是一个关于优化字节级运算的 Rust 代码问题。

[
https://old.reddit.com/r/rust/comments/1gkcfai/algorithms_for_efficiently_subtracting_constants/
](
https://old.reddit.com/r/rust/comments/1gkcfai/algorithms_for_efficiently_subtracting_constants/
)
    


### TITLE

这个问题描述了一个使用Rust编程语言和Redis库时遇到的问题。开发者需要实现一个外部库中的trait,其中包含一个读取数据库的函数`read_from_db`。该函数的签名要求返回的Future必须实现`Send + Sync + Future<Output=Result<Vec<Option<LocalValue>>, Error>>`。

然而,当开发者按照如下方式实现`read_from_db`函数时:

```rust
async fn read_from_db(
    &self,
    addresses: Vec<Address>,
) -> Result<Vec<Option<Self::LocalValue>>, Self::Error> {
    let refs: Vec<&Address> = addresses.iter().collect();
    let value = self.clone().connection.mget::<_, Vec<_>>(&refs).await?;
    Ok(value)
}
```

编译器报错说Future不能安全地在线程间共享,因为`mget`返回的Future不是`Sync`。根据Rust的所有权和借用规则,`Sync`trait要求在并行访问数据时不会出现数据竞争。

开发者遇到了这个问题,并请求解决方案。

[
https://old.reddit.com/r/rust/comments/1gkacjh/asynchronous_rust_future_cannot_be_shared_between/
](
https://old.reddit.com/r/rust/comments/1gkacjh/asynchronous_rust_future_cannot_be_shared_between/
)
    


### TITLE

该帖子作者表示,他正在工作中开发一些代码转换工具,他们正在寻找一些真实的公共代码用于演示。他想征求 Rust 社区对于哪些非 Rust 库或应用程序,大家最希望尽快用 Rust 重写的意见和建议。

简而言之,作者希望收集 Rust 社区对于将哪些流行的现有软件或库用 Rust 重写的想法,作为他们代码转换工具的示范对象。这反映了当时 Rust 语言在开源社区的影响力正在扩大,社区希望看到更多软件采用 Rust 编写。

[
https://old.reddit.com/r/rust/comments/1gjunio/which_oss_software_would_you_like_to_see/
](
https://old.reddit.com/r/rust/comments/1gjunio/which_oss_software_would_you_like_to_see/
)
    


### TITLE

这是一篇推荐给程序员们参加一个名为"Everybody.codes"的编程挑战活动的帖子。该活动灵感来自于Advent of Code(AoC),任务风格和难度级别与AoC相似,但每天有三个部分而不是两个。该活动已于今天开始,将持续20天。

与AoC相比,Everybody.codes网站提供了更详细的统计和排行榜。作者认为,这个活动可以作为AoC的预热,因为它提前一个月开始。他希望一些人能加入这个挑战。

[
https://old.reddit.com/r/rust/comments/1gk61ii/ot_everybodycodes_challenge_inspired_by_advent_of/
](
https://old.reddit.com/r/rust/comments/1gk61ii/ot_everybodycodes_challenge_inspired_by_advent_of/
)
    


### TITLE

本文主要讨论了 Rust 中字符串的优化方式,以减少字符串在内存中的占用。文章围绕 spellbook 这个拼写检查库展开,该库需要在内存中存储大量单词词根及相关规则标志。作者提出使用 Box<str> 而不是 String 来表示这些不可变的短字符串,从而避免存储不必要的容量信息,节省内存。

接着作者设计实现了一个名为 UmbraString 的新类型,它在 Box<str> 的基础上进一步压缩存储,将字符串内容和相关的标志位存储在一起。这种"德语字符串"表示法能最小化内存占用。

文章还分析了一些可能会影响优化效果的缺陷,如指针对齐等问题。最后,作者总结了 UmbraString 相较于其他方案的内存节省效果,并对该项目贡献者表示感谢。

总的来说,这是一篇关于 Rust 字符串优化的实践型文章,阐述了作者在 spellbook 库中优化字符串表示的尝试和思路。

[
https://the-mikedavis.github.io/posts/german-string-optimizations-in-spellbook
](
https://the-mikedavis.github.io/posts/german-string-optimizations-in-spellbook
)
    


### TITLE

这是一篇关于使用Rust语言创建3D终端渲染器的博客文章。作者最近发布了一款名为TermTrack的游戏,受到了一些关注。有人想知道如何制作类似的游戏,于是作者写了这篇博客文章,介绍了如何用Rust创建一个简化版的3D终端渲染器作为起点。文章链接是https://tagedan.github.io/posts/terminal_rendering.html,对于感兴趣的人可以查看了解更多细节。

[
https://old.reddit.com/r/rust/comments/1gk8fk9/how_to_create_a_3dterminal_renderer_using_rust/
](
https://old.reddit.com/r/rust/comments/1gk8fk9/how_to_create_a_3dterminal_renderer_using_rust/
)
    


### TITLE

这是关于Rust编程语言中用于图像处理的领先crate "image"的最新版本0.25.5的更新说明。主要更新包括:

1. 大幅改进了对AVIF图像格式的解码能力,现在支持10位和12位AVIF图像,并修复了许多AVIF解码方面的bug。这些改进由@awxkee贡献。

2. rayon特性现在可以正确切换AVIF编码时是否使用并行处理。但对于EXR格式,由于会破坏后向兼容性,所以暂时没有切换并行处理的选项。

3. 现在可以识别极少使用的".jfif"作为JPEG文件扩展名。

4. AVIF解码目前仍然依赖于C语言库dav1d,而不是Rust版本rav1d,因为rav1d还没有提供Rust API。未来希望能够迁移至rav1d以摆脱对dav1d的依赖。

总的来说,这个版本主要侧重于增强对AVIF格式的支持和解码能力,同时修复了一些bug,并对少数其他特性做了完善。

[
https://old.reddit.com/r/rust/comments/1gk3v7i/image_v0255_brings_much_improved_avif_decoding/
](
https://old.reddit.com/r/rust/comments/1gk3v7i/image_v0255_brings_much_improved_avif_decoding/
)
    


### TITLE

这篇文章介绍了一个名为 MinPin 的新的 Pin 提案,其目标是以一种"最小破坏性"的方式将 Pin 集成到语言中,同时保持完全向后兼容性。与 Overwrite 提案不同,MinPin 没有试图让 Pin 和 &mut 很好地协作,但它为将来添加 Overwrite 留下了空间。

MinPin 的主要设计决策包括:

1. 使用 pinned 关键字获取 Pin 变体,如 pinned &mut T 等同于 Pin<&mut T>。
2. Drop trait 修改为 fn drop(pinned &mut self)而不是 fn drop(&mut self)。
3. 对于实现了 Unpin 的字段类型,始终允许投影。
4. 对于未实现 Unpin 的字段类型,投影规则取决于结构体是否实现 Unpin 及是否有 fn drop(&mut self)方法。
5. 引入 struct Unpinnable<T> 始终实现 Unpin。

文章还列出了一些设计原则,如 Pin 是语言的一部分、Pin 应有零概念成本、应该显式使用等。最后,作者回答了一些常见问题,比较了 MinPin 与 UnpinCell 提案之间的异同。

[
https://smallcultfollowing.com/babysteps/blog/2024/11/05/minpin/
](
https://smallcultfollowing.com/babysteps/blog/2024/11/05/minpin/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

