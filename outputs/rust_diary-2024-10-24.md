【Rust日报】2024-10-24


### TITLE

这份文档定义了一种在单个 HTTP/2 连接的单个流上运行 WebSocket 协议的机制。它通过扩展 HTTP CONNECT 方法来实现这一点,允许在 HTTP/2 流上建立一个隧道来运行 WebSocket 协议(或任何其他协议)。

主要内容包括:

1. 引入了一个新的 HTTP/2 设置参数 SETTINGS_ENABLE_CONNECT_PROTOCOL,用于启用扩展的 CONNECT 方法。

2. 扩展了 CONNECT 方法,允许指定要连接的协议名称,而不是外部主机。

3. 定义了在扩展的 CONNECT 隧道上引导 WebSocket 协议的过程和示例。

4. 讨论了设计考虑因素、中介问题和安全注意事项。

5. 为新的 HTTP/2 设置参数和 HTTP 升级令牌分配了 IANA 代码点。

该文档使 WebSocket 能够在单个 HTTP/2 连接上运行,从而更高效地利用网络。

[https://datatracker.ietf.org/doc/html/rfc8441
](https://datatracker.ietf.org/doc/html/rfc8441
)
    


### TITLE

这是一个gRPC基准测试结果，比较了各种编程语言和框架在gRPC中的性能表现。测试包括每秒请求数(req/s)、平均延迟、90/95/99百分位延迟、平均CPU利用率和平均内存使用。

总体来看,Rust的实现表现最佳,例如rust_wtx每秒可处理67502个请求,平均延迟12.45毫秒。其次是Go的一些实现,如go_vtgrpc和go_grpc。C++的实现也不错。而Java、Python、Erlang等语言的实现则相对较慢。

这个基准测试为选择合适的gRPC框架和语言提供了很好的参考,可以根据具体需求对性能、资源使用等指标进行权衡。值得注意的是,实际应用场景下的性能可能与基准测试有所差异,但这个测试结果仍然具有一定的指导意义。

[
https://github.com/LesnyRumcajs/grpc_bench/discussions/475
](
https://github.com/LesnyRumcajs/grpc_bench/discussions/475
)
    


### TITLE

这是一篇关于一个新的基于Rust语言开发的HTTP/2框架"wtx"的介绍。该框架是从头实现的RFC7541和RFC9113标准,旨在通过内置的服务器框架和PostgreSQL连接器来支持Web应用开发。该项目支持no_std环境,没有强制依赖,并展现出不错的运行时性能。文章提供了与其他HTTP/2 Web框架的一组基准测试结果比较,展示了wtx在依赖数量、二进制大小和构建时间方面的优势。该框架正在添加对GRPC over HTTP/2的支持。最后作者欢迎大家对基准测试数据的反馈。一些Reddit用户对将HTTP/2和PostgreSQL集成在同一个库中表示疑虑,作者解释PostgreSQL模块是可选的。另有用户询问了WebSocket和HTTP/2流优先级相关的功能实现情况。

[
https://www.reddit.com/r/rust/comments/1et6bdc/a_new_http2_framework_created_from_scratch/
](
https://www.reddit.com/r/rust/comments/1et6bdc/a_new_http2_framework_created_from_scratch/
)
    


### TITLE

这是diesel-rs组织在GitHub上的一个名为"metrics"的开源仓库。该仓库包含了用于收集和分析与Diesel ORM相关的指标和分析数据的代码和资源。主要内容包括:

1. .github/workflows目录下存放GitHub Actions的工作流程配置文件。

2. metrics目录可能包含与指标收集和处理相关的Rust代码。

3. Readme.md是仓库的自述文件,解释了该项目的用途和使用说明。

4. analytics.R可能是用于分析收集到的指标数据的R语言脚本文件。

该仓库旨在通过收集和分析指标数据,帮助diesel-rs团队更好地了解Diesel ORM的使用情况,并根据这些数据进行项目改进和优化。这对于提高Diesel的性能、可用性和开发者体验至关重要。

[
https://github.com/diesel-rs/metrics
](
https://github.com/diesel-rs/metrics
)
    


### TITLE

这是一个来自Reddit的rust子版块的帖子,作者想征求大家对web基准测试(web benchmarks)的建议。具体来说:

1. 作者最近为wtx添加了对gRPC连接的基本支持,并展示了令人振奋的运行时性能结果。

2. 作者计划将gRPC作为wtx-bench的第三个测试目标(之前有WebSocket和ServerFramework)。

3. 作者征求大家的意见,是否应该添加更多测试,比如针对不同工作负载(如有丢包情况)的WebSocket测试等。

4. 作者希望大家能推荐其他类似于grpc_bench的基准测试套件。

总的来说,这是一个关于如何改进和扩展wtx-bench基准测试套件的讨论和征求意见的帖子。

[
https://www.reddit.com/r/rust/comments/1ewlq9f/web_benchmarks_suggestions/
](
https://www.reddit.com/r/rust/comments/1ewlq9f/web_benchmarks_suggestions/
)
    


### TITLE

这个Reddit帖子主要讨论了以下几个与Rust语言相关的内容:

1. 一位Rust新手基于Deno的fastwebsockets库编写了一个WebSocket客户端库。

2. 介绍了一个高性能的WebSocket协议Rust实现fastwebsockets。

3. 讨论了使用Rust编写快速WebSocket服务器的好处。

4. 分享了一个Python实现获得了比Node.js或Bun更好的WebSocket性能的方法。

5. 介绍了用于asyncio的快速WebSocket客户端和服务器picows。 

6. 另一个Rust实现的WebSocket库。

7. 讨论了寻找低延迟WebSocket服务器/库的问题。

8. 一个易用且高性能的WebSocket工具包。

9. 介绍了用Rust编写Kubernetes控制器。

10. Symbolica 0.12版本发布。

11. 为Rust添加了8位浮点数类型。

12. Rust的Android支持。

13. Rust学习的好处。

14. 一个基于Rust的类Unix操作系统。

15. 用Rust编写自己的游戏引擎。

16. 讨论了AI的"诅咒"以及Rust如何帮助解决这个问题。

总的来说,这个帖子讨论了Rust在WebSocket、系统编程、游戏开发等领域的应用,以及Rust所具有的高性能、可靠性和生产力等优点。

[
https://www.reddit.com/r/rust/comments/15zz8jm/the_fastest_websocket_implementation/
](
https://www.reddit.com/r/rust/comments/15zz8jm/the_fastest_websocket_implementation/
)
    


### TITLE

这个仓库包含了多种不同的传输实现和与Web技术相关的工具,涵盖了6个IETF RFC(6265、6455、7541、7692、8441、9113)、2个正式规范(gRPC、PostgreSQL)以及其他一些创新想法。主要包括:客户端API框架、数据库客户端、数据库模式管理器、gRPC客户端/服务器、HTTP客户端框架、HTTP服务器框架、HTTP2客户端/服务器、连接池管理器、UI工具、WebSocket客户端/服务器、HTTP/2上的WebSocket等。

该项目注重性能优化,采用了手动矢量化、内存分配优化、减少依赖等多种手段。提供了高层和低层的性能基准测试。支持传输层安全(TLS)加密通信,但需要用户自行选择TLS提供者。给出了不同用例的示例演示。该项目有一些限制,如不支持16位指针长度的系统,需要考虑可能的内存溢出等。

[
https://github.com/c410-f3r/wtx
](
https://github.com/c410-f3r/wtx
)
    


### TITLE

这是一个用于Arch Linux的COSMIC Applet项目仓库。该应用程序可以显示Arch Linux软件包的更新状态,包括pacman、AUR和devel软件包。它的灵感来自两个类似项目。

该项目包含以下主要特性:

1. 原生的COSMIC外观,支持明暗两种模式。
2. 显示pacman、AUR和devel软件包的更新。
3. 支持本地化,可提交翻译文件支持其他语言。
4. 使用模块化API arch-updates-rs,可在其他类似项目中使用。

该仓库提供了构建和安装的说明,可以通过AUR Helper如paru进行安装。已安装后,可以在COSMIC设置中添加该应用小程序。

总的来说,这是一个为Arch Linux用户提供软件包更新状态显示的COSMIC应用小程序。

[
https://github.com/nick42d/cosmic-applet-arch
](
https://github.com/nick42d/cosmic-applet-arch
)
    


### TITLE

这个帖子讨论了rust中trait对象为何被认为是无大小(unsized)的原因。作者提出了一个疑问:如果我们知道了所有实现该trait的类型,那么不就可以取它们大小的最大值作为trait对象的大小上限吗?在这种情况下,trait对象应该是有确定大小的,不需要分配在堆上。

作者不理解为什么rust中仍然认为trait对象是无大小的。这个疑问反映了对rust语言trait对象实现机制的一些疑惑和不解。回复中解释了rust采用这种设计的一些原因,以及trait对象作为胖指针在运行时动态分派的工作方式。整体上,这个讨论涉及了rust语言对象模型和trait系统的一些深层细节。

[
https://old.reddit.com/r/rust/comments/1gbcgw9/the_size_of_trait_objects/
](
https://old.reddit.com/r/rust/comments/1gbcgw9/the_size_of_trait_objects/
)
    


### TITLE

以下是对原内容的中文总结:

Tokio团队宣布推出了一个名为Toasty的全新异步ORM框架,专为Rust语言打造。Toasty的设计理念是简化使用,支持SQL和NoSQL数据库,目前仍处于早期开发阶段。

Toasty让开发者通过编写schema文件定义数据模型,然后使用CLI工具自动生成与之对应的Rust代码。这样可以很方便地使用生成的代码与数据库交互,比如创建、查询和更新数据。

该项目的出现是因为尽管Rust被定位为系统级编程语言,但它在构建Web应用方面的生产力也变得越来越重要。而目前Rust的ORM库生态还不太完善,使用体验有待改善,因此Toasty应运而生。Toasty的目标是提高开发Web应用时的生产力,优先考虑易用性而非极致性能。

设计Toasty时,团队着重简化了API设计,尽量少用Rust的某些特性如trait和生命周期,以降低使用难度。总的来说,Toasty意在弥补Rust当前ORM库生态的不足,为Web应用开发提供更高效的工具支持。

[
https://tokio.rs/blog/2024-10-23-announcing-toasty
](
https://tokio.rs/blog/2024-10-23-announcing-toasty
)
    


### TITLE

这是iroh 0.27.0版本的发布公告博客。主要更新包括:

1. 修复了一些bug,包括长时间运行节点时出现的棘手问题,以及修复了一个可能导致生产代码错误连接到测试中继服务器的配置选项问题。

2. 优化了与中继服务器的连接保持机制,确保在连接失败时自动重新连接,而不用等待发送数据时才重连。

3. 简化了选择生产环境或测试环境基础设施的逻辑,现在只需设置IROH_FORCE_STAGING_RELAYS环境变量。

4. 简化了在iroh-net库中添加默认Discovery服务的流程,只需调用discovery_n0()方法。

5. 修复了其他一些小问题,添加了一些小功能。完整更改日志见GitHub发布页面。

6. 最后呼吁大家加入Discord频道交流,关注Twitter获取最新动态,并介绍了iroh作为构建点对点设备连接的开源库的用途。

总的来说,这是一个重点修复bug、优化配置选项和简化API使用的版本发布。

[
https://www.iroh.computer/blog/iroh-0-27-0-Squashing-Bugs-And-Taking-Names
](
https://www.iroh.computer/blog/iroh-0-27-0-Squashing-Bugs-And-Taking-Names
)
    


### TITLE

这是一位新手想要学习Rust编程语言的求助帖。他有Python和PyTorch深度学习的开发经验,目的是通过实践来熟悉Rust的开发环境和思维方式。他希望Rust社区的成员能分享一些学习经验、方向指引和有用的资源,帮助他更好地进入Rust的学习之路。

[
https://old.reddit.com/r/rust/comments/1gb9y9l/new_to_rust_and_need_your_experience/
](
https://old.reddit.com/r/rust/comments/1gb9y9l/new_to_rust_and_need_your_experience/
)
    


### TITLE

这篇文章讨论了在企业内核中使用Rust语言的问题。主要内容包括:

1. Rust已被同意继续在Linux内核中实验,未来将有更多Rust代码进入内核。但在实际实施过程中,会遇到一些棘手的细节问题需要解决。

2. nouveau是Linux内核中用于支持NVIDIA GPU的驱动程序,经过多年的逆向工程开发。NVIDIA现在开始支持开源软件对其产品的支持。

3. Nova项目旨在用Rust语言为新的NVIDIA GPU编写新的驱动程序,以更好地应对固件接口的不确定变化。

4. VFIO子系统可让用户空间进程安全访问设备。NVIDIA提交了一个23补丁的vGPU功能,允许云提供商在虚拟机中分配和管理GPU访问。

5. vGPU功能依赖nouveau驱动程序,但Nova开发人员希望vGPU基于Nova。vGPU开发人员表示,vGPU需要在企业内核中广泛回归,而企业内核无法使用Rust代码。

6. 这引发了在企业内核中使用Rust代码的担忧,需要解决Rust代码与遗留C代码共存的问题。

[
https://lwn.net/Articles/993337/
](
https://lwn.net/Articles/993337/
)
    


### TITLE

这位开发者在使用Rust的Cargo工作空间进行项目开发时遇到了一个问题。他的项目包含多个crate,其中有一个核心库存在一些互相冲突的Cargo特性。在使用VSCode的rust-analyzer扩展时,无法为每个crate指定不同的特性集,导致要么所有crate都启用所有特性(会引发大量编译错误),要么只能指定一个通用的特性集(但rust-analyzer似乎会忽略这个设置)。

这给他的工作带来了不便,他不得不临时从工作空间中排除其他crate,只分析核心库代码。他希望能找到一个更好的解决方案,比如让rust-analyzer忽略某些crate或只分析指定的单个crate,而忽略其所在的工作空间环境。他在询问其他开发者是否也遇到过类似的情况。

[
https://old.reddit.com/r/rust/comments/1gbeeia/rustanalyzer_for_workspaces_with_crates_with/
](
https://old.reddit.com/r/rust/comments/1gbeeia/rustanalyzer_for_workspaces_with_crates_with/
)
    


### TITLE

该Reddit帖子介绍了一个新的用Rust编写的Web框架wtx。wtx提供了内置的HTTP/2服务器框架、PostgreSQL连接器和SQL模式管理器,能支持WebSocket通过HTTP/2隧道进行全双工通信。作者声称wtx是最快的Web框架,并提供了多个基准测试结果作为证明。该框架实现了RFC8441规范中定义的WebSocket握手过程。除了wtx,只有deno这个Rust项目也支持WebSocket over HTTP/2这一特性。作者分享了一个代码示例,并欢迎提出相关问题。

[
https://old.reddit.com/r/rust/comments/1gbg2i5/the_fastest_web_framework_can_now_serve_websocket/
](
https://old.reddit.com/r/rust/comments/1gbg2i5/the_fastest_web_framework_can_now_serve_websocket/
)
    


### TITLE

这是一篇关于Rust编程语言的Reddit帖子摘要。作者一直在测试COSMIC桌面环境的alpha版本,但是觉得缺少一些小型的生活质量(QOL)工具,比如查看有多少个Arch Linux系统更新正在等待。为了学习如何使用libcosmic和iced库,作者开发了一个小型应用程序,可以查看待处理的Arch更新数量。他将这个API独立出来作为一个单独的crate(Rust的库/包),希望它可以在其他地方得到使用。该应用的GitHub链接为https://github.com/nick42d/cosmic-applet-arch。

[
https://old.reddit.com/r/rust/comments/1gb6dsf/i_built_an_applet_for_the_new_rust_cosmic_desktop/
](
https://old.reddit.com/r/rust/comments/1gb6dsf/i_built_an_applet_for_the_new_rust_cosmic_desktop/
)
    


### TITLE

总结如下:

这个GitHub仓库包含一个名为Brush的3D重建引擎,它使用高斯拼贴(Gaussian splatting)技术。Brush旨在高度可移植、灵活和快速。它可以在macOS、Windows、Linux、AMD/Nvidia显卡、Android以及网页浏览器上渲染和训练,这是通过使用与WebGPU兼容的技术实现的,能在几乎任何地方运行。

该项目目前仍处于概念验证阶段,尚未实现任何高斯拼贴的扩展,性能还有待优化。它提供了一个实验性的网络演示,可以加载预训练的点云数据和数据集进行训练。支持的数据格式包括synthetic nerf场景数据集的zip文件和COLMAP数据。

在训练过程中,你可以与拼贴点云交互,实时查看训练动态,并将当前渲染结果与训练/评估视图进行比较。它还集成了rerun可视化工具来可视化额外的数据。

该项目可在Windows/macOS/Linux等平台编译运行,也可以在网页和Android手机上运行。它由多个Rust crate组成,分别负责不同的功能,如渲染、训练、数据集导入等。内核采用"稀疏"风格编写,只对可见的高斯体进行操作。它使用基于FidelityFX的GPU基数排序,并针对WebGPU的一些限制做了优化。

总的来说,Brush是一个跨平台的3D重建引擎,具有灵活性和可移植性,尽管仍在开发中。

[
https://github.com/ArthurBrussee/brush
](
https://github.com/ArthurBrussee/brush
)
    


### TITLE

这个存储库介绍了一个 Rust 编译器插件 roxygen,它允许你为函数的参数添加文档注释。这是目前 Rust 中不支持的功能,会导致编译错误。使用 roxygen 插件后,可以像这样为参数添加文档:

```rust
#[roxygen]
/// 对图像行求和
fn sum_image_rows(
    /// 按行主序存储的图像数据
    image_data: &[f32],
    /// 图像行数  
    nrows: u32,
    /// 图像列数
    ncols: u32,
    /// 存储求和结果的缓冲区,必须有nrows * ncols个元素的空间
    sums: &mut [f32]
) -> Result<(), String> {
    todo!()
}
```

插件会将参数文档合并到函数顶层文档中。可以使用 #[arguments_section] 属性自定义参数文档段落的位置。

作者认识到标准库没有采用这种参数文档风格,并建议根据实际需求权衡是否使用该插件,考虑使用更少参数、更有描述性的参数名、类型等方式传达意图。

最后,作者认为该宏的编译时间影响不大,因为它只做了一些文档标记的轻量解析和重新排列,不会引入额外的代码复杂度。

[
https://github.com/geo-ant/roxygen
](
https://github.com/geo-ant/roxygen
)
    


### TITLE

在Loro 1.0版本中,内部实现了一个简单的LSM(日志结构合并树)引擎。LSM是一种常用于实现键值数据库的数据结构,Loro 1.0的设计受其很大启发。

Loro的存储实现使用了键值数据库的get、set和range操作作为基本操作。例如,Loro将历史记录存储为一系列ChangeBlock,每个ChangeBlock序列化约4KB。每个ChangeBlock使用其第一个OpId作为键,ChangeBlock的序列化二进制数据作为值,存储在内部的LSM引擎中。在简单的LSM引擎实现中,每个块在序列化期间会被压缩,只有在实际检索相应的值时才会解压缩。这使得新数据格式的导入速度比以前快100倍,内存使用也更低。

因此,在Loro 1.0中:
- 导入时会检查数据完整性
- Loro内部以块的形式存储历史记录(History/OpLog)和状态(DocState),并根据需求加载相应的块
- Loro所基于的Eg-walker算法允许文档在没有完整的CRDT元信息的情况下开始编辑,因此可以轻松地与延迟加载行为协同工作

延迟加载的价值在于,在许多使用场景中,我们不需要完全加载文档的历史记录和状态。

导入和导出新版本时会发生以下情况:
- 首先导入旧版快照
- 接收的更新可能包含并行编辑,因此需要加载旧版本的部分相关并行编辑历史以构造CRDT和完成diff计算
- Loro内部会加载并解析相应块的数据以获取相应历史,此时不会发生完整文档解析
- 完成diff计算后,需将其应用到相应的States
- Loro会内部加载并解析相应的状态,此时也不会发生完整文档解析
- 导出时,未受影响的历史块或状态块会按原样导出
- 受影响的块将被序列化以覆盖原始块,然后导出
- 导出过程中,内部使用类似SSTable的方法进行最终导出

在整个过程中,只需解析:
- 每个存储块的元信息
- 需要读取的块将被解压缩
- 将受更新影响的历史块/状态块

我们仍然需要将整个文档blob加载到内存中。但是,当前架构已经实现了基于块的内部加载和存储,这使我们在未来更容易实现从磁盘真正基于块的检索和保存。理论上是可行的,但我们会评估是否有实际场景需要这种能力。对于大多数文档,Loro当前的性能已经相当不错了。

[
https://loro.dev/blog/v1.0
](
https://loro.dev/blog/v1.0
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

