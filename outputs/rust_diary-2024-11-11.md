【Rust日报】2024-11-11


### TITLE

CurVis是一个用Rust编写的光线追踪器,它能够模拟在曲率时空中光线的传播,并根据广义相对论方程渲染曲率时空的图像和视频。它的主要功能包括:

1. 渲染像虫洞等曲率时空物体的图像和视频。
2. 支持通过命令行界面设置渲染参数,如相机位置、分辨率、视场角等。
3. 支持自定义时空度规和参数。
4. 提供了一些示例相机路径文件,用于渲染视频时指定相机在时空中的运动轨迹。

该项目目前还存在一些已知问题和局限性,如只支持球对称时空度规、视频渲染有问题、存在边界伪影等。未来的愿景包括支持黑洞渲染、多线程加速等。总的来说,CurVis为在曲率时空中可视化光线传播提供了一种有趣的方式。

[https://github.com/fragarriss/CurVis
](https://github.com/fragarriss/CurVis
)
    


### TITLE

总结如下:

GGRS(Good Game Rollback System)是一个用纯安全的Rust语言重新实现的GGPO网络SDK。它摒弃了原来的基于回调函数的API,采用了更简单直观的控制流程。GGRS会返回一系列需要用户满足的请求,而不是注册回调函数。

该项目目前处于早期阶段,但多人游戏和观战功能较为稳定。它提供了两个基于浏览器的在线演示Demo,分别使用了macroquad和bevy框架。此外,还有一些游戏项目在使用GGRS。

GGRS有详细的Wiki、示例和文档供开发者入门。开发者们正在努力扩展GGRS的功能,如为Bevy游戏引擎开发插件、支持WebRTC网络传输、为Godot引擎封装接口等。GGRS采用双重许可MIT和Apache 2.0协议。

总的来说,GGRS是一个使用Rust语言开发的新型回滚网络库,为游戏开发提供了良好的网络同步支持。

[
https://github.com/gschup/ggrs
](
https://github.com/gschup/ggrs
)
    


### TITLE

这个GitHub仓库包含了一个名为Matchbox的项目,旨在为Rust语言的原生应用和WebAssembly应用程序实现类似UDP的不可靠、无序的点对点网络连接,以支持低延迟的多人在线游戏。

Matchbox包括几个主要部分:

- matchbox_socket: 一个封装了WebRTC的Socket抽象层,支持不可靠和可靠的数据通道。
- matchbox_signaling: 一个信令服务器库及示例。
- matchbox_server: 一个现成的全网状信令服务器。
- bevy_matchbox: 将matchbox_socket集成到Bevy游戏引擎中。

该仓库还提供了一些使用示例,包括一个简单的通信循环、使用Bevy和GGRS的浏览器游戏demo等。

Matchbox的工作原理是先通过信令服务器协调对等体之间的连接,之后对等体就可以直接相互发送数据,不再经过信令服务器。

该项目欢迎贡献和问题反馈,采用双重MIT和Apache 2.0许可证。感谢Ernest Wong的Dango Tribute实验对该项目的启发。

[
https://github.com/johanhelsing/matchbox
](
https://github.com/johanhelsing/matchbox
)
    


### TITLE

这篇文章介绍了用Rust编程语言开发一个NES模拟器的教程系列。NES是上世纪80年代和90年代最受欢迎的游戏平台之一,硬件相对简单但却诞生了很多经典游戏。作者选择Rust语言是因为它是一种现代化的系统编程语言,性能出色,能够满足硬件模拟的低级需求。教程将分阶段构建,每个阶段都有可见和可玩的目标,从而使得整个过程更有趣。作者假设读者已具备一定的Rust语言、位运算、计算机系统等基础知识。最后列出了一些相关的参考资源。总的来说,这是一个循序渐进、实践驱动的Rust编程教程,目标是开发一款功能完整的NES游戏模拟器。

[
https://bugzmanov.github.io/nes_ebook/
](
https://bugzmanov.github.io/nes_ebook/
)
    


### TITLE

这篇文章讨论了在开发无法实际访问的远程硬件系统时(如火星探测器或海洋航运器),应该采用何种编程语言和设计理念。作者对比了两种观点:

1. Rust语言的理念是在开发初期就确保程序的正确性,这样一旦部署就可以永久运行而无需修补。

2. Elixir/BEAM VM语言的理念是允许出错,但提供了恢复错误的机制,即"让它崩溃"(Let it crash)。

一位评论者提到,虽然无法远程修补代码,但硬件本身也可能会发生老化等物理故障。他认为如果真的无法修补,Rust的理念会让人更加放心。但要注意除了类型安全外,还需要使运行时系统具备容错其他硬件降级错误的能力。

另一位评论者指出,即使是火星探测器这种极端情况,NASA也曾利用Lisp的REPL在1.5亿英里外成功远程调试过代码。因此即使无法修补,通过良好的设计也不是完全无法调试和恢复。

[
https://www.reddit.com/r/elixir/comments/1gp34om/language_philosophies_for_distant_hardware/
](
https://www.reddit.com/r/elixir/comments/1gp34om/language_philosophies_for_distant_hardware/
)
    


### TITLE

根据给出的内容,这是一个Google表单的链接,旨在征求对某个会议的赞助意向。如果您公司有兴趣赞助这个会议,可以通过这个表单联系他们,提供合适的联系人信息。总的来说,这是一个寻求赞助商的调查表单。

[
https://docs.google.com/forms/d/e/1FAIpQLSdamzdbUi3EIGBrmEw0-Na4myXP0088kvxVmVT4YU-1BEiyCg/viewform
](
https://docs.google.com/forms/d/e/1FAIpQLSdamzdbUi3EIGBrmEw0-Na4myXP0088kvxVmVT4YU-1BEiyCg/viewform
)
    


### TITLE

以下是对该网站内容的中文总结:

这是一个关于Rust编程语言的会议网站。会议名为"从巴黎与爱同行,用Rust编织代码"。会议非常重视多元化,旨在创建一个包容、听取不同声音的环境,庆祝多元化,共同推进Rust编程语言的未来发展。

会议将邀请该领域最杰出的演讲者,提供有见地的洞察和实用的Rust编码指导。这些专家将激励与会者追求更多灵感、创新和努力。

会议的核心使命是促进相互交流和建立人际关系。会议将提供充足的机会让与会的开发者互相交流经验、分享想法,在专门的交流环节和非正式聚会中结识新朋友。通过建立友谊,我们可以增进学习、发现新机遇,为活跃的Rust生态系统作出贡献。

网站上还有一个"申请演讲"的选项,可能是开放征集会议演讲人。

[
https://www.rustinparis.com/
](
https://www.rustinparis.com/
)
    


### TITLE

以下是对该内容的中文总结:

这是OpenMesh软件的发布历史记录,列出了最新的几个主要版本及其更新内容:

- OpenMesh 11.0版本于2024年5月14日发布,增加了一些改进和bug修复,包括添加了一个简单的孔洞填充算法、改进了OBJ读写器提高精度等。

- OpenMesh 10.0版本于2023年11月14日发布,添加了边-半边、边-顶点和边-面循环器,清理了构建系统,修复了读写器中的一些bug。

- OpenMesh Python绑定更新到1.2.1版本,支持Python 3.6及以上版本,适用于Linux、Mac和Windows系统。

- OpenMesh 9.0版本于2022年3月8日发布,添加了智能范围过滤器、智能句柄读取状态字段访问、CW/CCW范围等功能,更新了构建系统支持Qt6,但不再支持VS2015和32位架构测试。

- OpenMesh 8.1版本于2022年1月18日发布,引入了智能句柄简化网格导航方法访问,新增便利函数,改进了双精度和PLY读写器支持。

- OpenMesh 8.0版本于2020年4月23日发布,要求C++11及更高版本编译器,集成了Eigen3向量类型封装,改进多个读写器,添加SmartTagger标记类。

总的来说,这些版本更新主要包括功能增强、bug修复、性能优化和与新标准的兼容等方面。

[
https://www.graphics.rwth-aachen.de/software/openmesh/
](
https://www.graphics.rwth-aachen.de/software/openmesh/
)
    


### TITLE

这个仓库包含了一个名为Alum的半边数据结构(Halfedge)实现的多边形网格库。它的灵感来自于OpenMesh,因此API与OpenMesh非常相似。作者由于在Rust中找不到等效的库,因此编写了这个库。该库目前的功能是为了服务于作者的其他项目,与OpenMesh的Core模块功能相当,将来可能还会根据需要添加新功能。

该库可以通过crates.io作为依赖添加到Rust项目中。它默认使用glam提供几何类型(如点、法向量等),也可以通过实现适配器使用自定义几何类型。该库还包含了一个属性系统,类似于OpenMesh但做了一些改进,使用RefCell确保运行时借用检查。属性会随着网格元素的增删和垃圾回收而自动同步。文档中有更多关于使用属性系统的详细说明。

[
https://github.com/ranjeethmahankali/alum/
](
https://github.com/ranjeethmahankali/alum/
)
    


### TITLE

Flusso是一个用Rust语言编写的轻量级、高性能的Kubernetes Ingress控制器。它为现代云原生环境提供了安全性和灵活性。以下是Flusso的主要特点:

1. 轻量级和高速:使用Rust编写,占用资源少。
2. 高级负载均衡:支持自定义负载均衡算法。
3. 安全设计:实现了现代TLS协议。 
4. 动态后端:自动根据Kubernetes服务变化更新路由。
5. 灵活配置:可通过YAML文件或环境变量轻松配置。
6. 最小依赖关系:避免不必要的依赖项。

您可以使用Docker或Helm在Kubernetes集群中部署Flusso。它通过监视Ingress资源自动将传入流量路由到服务。Flusso还提供Web GUI用于监控后端和路由。Flusso欢迎贡献,并在MIT许可下发布。您可以通过GitHub或Docker Hub获取更多支持。

[
https://github.com/DioCrafts/flusso
](
https://github.com/DioCrafts/flusso
)
    


### TITLE

这是一个用Rust编写的简单高效的多线程端口扫描器。它可以扫描指定主机上的开放和关闭的服务器端口。主要特点包括:

1. 多线程扫描:利用所有可用的CPU内核,最大化扫描过程的性能和速度。

2. 并发性:利用tokio运行时进行异步网络操作。 

3. 用户友好的命令行界面:使用clap进行命令行参数解析。

该项目需要Rust、tokio异步运行库、clap命令行参数解析库和可用CPU内核数。可以通过git克隆仓库,使用cargo构建和运行:
cargo build --release
./target/release/port_scanner <host>

总的来说,这是一款功能强大的Rust编写的端口扫描工具。

[
https://github.com/ash2228/rust-portscanner
](
https://github.com/ash2228/rust-portscanner
)
    


### TITLE

这是一位开发者使用Rust编程语言创建了一个光线追踪器,用于基于广义相对论可视化虫洞图像。该项目基于O. James等人的工作,是开源的,托管在GitHub上。开发者使用自己的代码和EVE在线游戏中的360度壁纸图像生成了一段视频。

目前,开发者正在完善代码和扩展文档。未来的计划是实现多线程处理,并添加黑洞和中子星的支持。这个项目是开发者学习Rust语言的一个选择,过程虽然有些挑战,但也很有成就感。开发者欢迎任何改进代码的建议。

最后,开发者分享了一张由该项目生成的图像,因为视频无法上传。

[
https://old.reddit.com/r/rust/comments/1go844r/a_rust_raytracer_on_curved_spacetimes/
](
https://old.reddit.com/r/rust/comments/1go844r/a_rust_raytracer_on_curved_spacetimes/
)
    


### TITLE

这位学生正在上一门rust编程课程,课程要求制作一个完整的项目。他决定制作一个NES模拟器,并添加在线双人多人游戏功能。目前他已经完成了大部分工作,但是对于在线多人游戏这一部分还没有经验。他发现了一些可能有用的工具如Matchbox和GGRS,但作为初学者不太清楚如何使用。他希望能获得一些资源或建议,以便为项目添加一个基础的在线多人游戏功能。

[
https://old.reddit.com/r/rust/comments/1gp0fid/working_on_an_nes_emulator_what_are_some_ways_in/
](
https://old.reddit.com/r/rust/comments/1gp0fid/working_on_an_nes_emulator_what_are_some_ways_in/
)
    


### TITLE

这个帖子讨论了在开发无法物理访问的远程硬件系统时(比如火星探测器或海洋船舶)应该采用何种编程语言/理念更加可靠。

作者对比了两种不同的编程语言理念:

1. Rust的理念是在开发初期就确保程序的正确性,一旦部署就应该永久运行下去,不出错。

2. Elixir/BEAM虚拟机语言的理念是承认会发生错误,但提供崩溃和恢复的机制来应对。

作者想知道,对于这种无法重新物理访问的远程硬件系统,采用哪种语言理念会更加可靠。是像Rust那样追求开发初期就百分之百正确,还是像Elixir那样允许错误发生但有恢复机制?这是一个值得探讨的有趣话题。

[
https://old.reddit.com/r/rust/comments/1gp320c/language_philosophies_for_distant_hardware/
](
https://old.reddit.com/r/rust/comments/1gp320c/language_philosophies_for_distant_hardware/
)
    


### TITLE

总结:

这是一个关于Rust编程语言的会议通知。第一届"Rust in Paris"会议将于2025年3月14日在巴黎举行。会议官网是https://www.rustinparis.com/,在那里可以找到更多相关信息。如果有人想在会议上发表演讲,可以通过https://docs.google.com/forms/d/e/1FAIpQLSdamzdbUi3EIGBrmEw0-Na4myXP0088kvxVmVT4YU-1BEiyCg/viewform这个链接提交申请。这是一个面向Rust语言爱好者的技术会议活动。

[
https://old.reddit.com/r/rust/comments/1goqpaj/rust_in_paris_conference_on_the_14_of_march_2025/
](
https://old.reddit.com/r/rust/comments/1goqpaj/rust_in_paris_conference_on_the_14_of_march_2025/
)
    


### TITLE

这是一个关于在Rust 2024版本中稳定化"let链"特性的拉取请求说明。主要内容包括:

1. 介绍了"let链"特性,即在if和while语句中可以使用&&连接let语句和布尔表达式。

2. 解释了为什么这个特性需要在2024版本中才能稳定化,主要是由于之前版本中if let的临时值的释放顺序与let链存在不兼容,在2024版本中通过调整临时值生命周期来解决。

3. 提供了一些使用该特性的示例代码。

4. 讨论了在Rust编译器中引入该特性需要注意的一些事项,如编辑器/IDE集成、测试用例等。

5. 总的来说,这个拉取请求旨在为2024版本稳定化让链特性,并权衡了向后兼容性等因素。

[
https://github.com/rust-lang/rust/pull/132833
](
https://github.com/rust-lang/rust/pull/132833
)
    


### TITLE

这是一个关于Rust编程语言中新的半边数据结构(half-edge)多边形网格库alum的介绍。作者之前一直在C++中使用OpenMesh库,但在Rust中找不到类似的库,所以决定自己编写一个新的库alum。该库目前正在开发中,功能会根据作者其他相关项目的需求不断增加。不过该库目前已经与OpenMesh/Core模块有了近乎全面的功能兼容性,所以作者决定发布并在Rust社区中分享。该帖子提供了alum库的crates.io、GitHub仓库和文档链接。

[
https://old.reddit.com/r/rust/comments/1gp5ccx/alum_new_halfedge_polygon_mesh_library/
](
https://old.reddit.com/r/rust/comments/1gp5ccx/alum_new_halfedge_polygon_mesh_library/
)
    


### TITLE

该帖子询问Rust编程语言中最佳的创意编码包(crate)。作者表示很喜欢Nannou和Bevy这两个包,但前者多年没有更新,后者的实体组件系统(ECS)架构在一定程度上降低了创作流畅度。当学习数学和物理概念时,能够快速拼凑出带有动态绘图和可视化效果(类似于2Blue1Brown风格)的模拟是非常重要的。作者希望听取其他人对于创意编码的包的推荐。

[
https://old.reddit.com/r/rust/comments/1gp5fwd/best_creative_coding_crates/
](
https://old.reddit.com/r/rust/comments/1gp5fwd/best_creative_coding_crates/
)
    


### TITLE

这是一个介绍 Rust 语言中一个名为 Sled 的库的内容。Sled 库用于创建空间 LED 灯带照明效果。它提供了几个方法来设置 LED 灯带上特定位置的颜色。

`set_at_dist`方法可以设置距离中心或任意其他点指定距离处的 LED 灯为指定颜色。

`set_at_angle`和`set_at_dir`方法通过在线段和射线之间进行简单的相交测试来设置 LED 灯的颜色。

在内部实现中,Sled 库执行线段与以给定半径形成的圆之间的相交测试,并在相交点处设置 LED 灯的颜色。这些方法提供了一种方便的方式来创建各种空间照明效果。

[
https://davjcosby.github.io/all-published/miscellaneous-tech/Introducing%20Sled,%20a%20Rust%20Library%20for%20Creating%20Spatial%20LED%20Strip%20Lighting%20Effects.html
](
https://davjcosby.github.io/all-published/miscellaneous-tech/Introducing%20Sled,%20a%20Rust%20Library%20for%20Creating%20Spatial%20LED%20Strip%20Lighting%20Effects.html
)
    


### TITLE

这篇文章介绍了一个名为BuFFI的新工具,用于为Rust代码生成人性化和安全的C++API。BuFFI将所有类型替换为字节缓冲区,从而简化了Rust和C++两端的外部函数。作者在工作中使用该工具删除了数百行样板代码,并消除了许多可能的内存泄漏或错误指针处理。

文章提供了BuFFI的链接、幻灯片和一个基本示例。该工具目前支持Rust 1.82.0工具链,未来每次Rust发布新版本时,BuFFI也会发布新版本以保持兼容性。作者希望能与社区一起解决任何潜在的兼容性问题。

总的来说,BuFFI旨在简化Rust和C++之间的互操作性,提高安全性和工作效率。

[
https://old.reddit.com/r/rust/comments/1gouxoc/buffi_generate_ergonomic_c_apis_for_your_rust_code/
](
https://old.reddit.com/r/rust/comments/1gouxoc/buffi_generate_ergonomic_c_apis_for_your_rust_code/
)
    


### TITLE

该帖子是关于一位开发者发布了一个名为flusso的新开源Kubernetes Ingress控制器项目,用Rust编写。他希望社区能试用并提供反馈,以帮助改进该项目,使其更加可靠、安全和高效。

该项目的主要特点包括:

1. 提高安全性,利用Rust的内存安全特性避免常见漏洞。
2. 资源高效,即使在高负载下也能保持高吞吐量而不消耗过多CPU和内存。
3. 改进TLS管理,自动处理证书并简化多域名配置。
4. 简化配置,提供直观的GUI展示后端服务状态、TLS信息等。
5. 灵活的可观测性,内置对接Prometheus和Grafana的监控指标和结构化日志。

作者希望获得反馈,例如GUI的用户体验、性能表现、配置流程、期望的新功能等,以帮助项目发展。该项目目前处于0.0.1版本,刚刚起步,作者利用空余时间进行开发。他诚挚地邀请社区测试并提供反馈意见。

[
https://old.reddit.com/r/rust/comments/1gp10sh/i_have_launched_an_opensource_k8s_ingress/
](
https://old.reddit.com/r/rust/comments/1gp10sh/i_have_launched_an_opensource_k8s_ingress/
)
    


### TITLE

这个视频是Jon Gjengset在EuroRust 2024大会上的演讲。他指出在编写Rust代码时,我们经常会遇到一些令人困惑的编译器错误和限制,例如为什么某些类型需要实现Send trait、为什么某些生命周期注解看起来很复杂、异步代码为何只使用单核等等。这些问题源于对Rust底层原理和机制缺乏理解。演讲的目的是解释这些深层次概念,帮助开发者自行诊断和避免80%的常见挑战。

Jon自2015年开始使用Rust,曾在麻省理工学院构建一个快速SQL数据库作为博士论文。他现在是Helsing公司的首席工程师,也是Rust教学书籍"Rust for Rustaceans"的作者。他热衷于教学,自2018年以来一直制作Rust实时编码和教育视频,还是"Rustacean Station"播客的合伙人。EuroRust是一年一度为欧洲Rust社区举办的为期两天的大会。

[
https://youtu.be/8-KLX1PGg8Q?si=rqkSm2fHVafnF-cO
](
https://youtu.be/8-KLX1PGg8Q?si=rqkSm2fHVafnF-cO
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

