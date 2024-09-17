【Rust日报】2024-09-16


### TITLE

tachyonfx是一个Rust库,用于在终端UI中创建像着色器一样的视觉效果。它提供了各种效果,如颜色转换、动画和复杂效果组合等,以增强终端应用程序的视觉吸引力。

该库包含多种类别的效果:

- 颜色效果:淡入淡出、色相饱和度光度变换等
- 文本/字符效果:溶解、滑入滑出、扫入扫出等
- 时间控制效果:重复、延长、无限循环等
- 几何效果:位移、缩放等
- 组合效果:并行、序列等
- 其他效果:自定义效果函数、离屏缓冲区等

该库使用EffectTimer控制效果的持续时间和插值。效果可应用于特定单元格或区域,支持复杂的单元格过滤器。该库还提供了多个示例程序,展示了不同效果的用法。

[https://github.com/junkdog/tachyonfx
](https://github.com/junkdog/tachyonfx
)
    


### TITLE

kube是一个用Rust语言编写的Kubernetes客户端,它旨在提供与client-go类似的通用客户端、受controller-runtime启发的运行时抽象,以及受kubebuilder启发的用于自定义资源定义(CRD)的派生宏。该库作为CNCF的一个Sandbox项目而托管。

kube库基于Kubernetes的API机制和API概念,为反射器、控制器和自定义资源接口提供了通用的抽象,使得编写应用程序变得更加简单。它提供了安装和升级指南,以及使用示例。

主要功能包括:

1. API:用于与Kubernetes资源交互的通用API。
2. 自定义资源定义:通过过程宏自动生成代码,支持处理CRD。
3. 运行时:提供更高级别的抽象,如Watcher(观察器)、Reflector(反射器)和Controller(控制器)。
4. TLS支持:默认使用rustls,也可切换为openssl。

kube使用Apache 2.0许可证。它旨在简化Kubernetes本地开发,并作为Rust编写的云原生应用程序的基础。

[
https://github.com/kube-rs/kube
](
https://github.com/kube-rs/kube
)
    


### TITLE

这是一个 Rust 实现的 SFTP (SSH文件传输协议) 客户端和服务器库,支持 SFTP 子系统。它实现了 SFTP 协议版本 3 的规范,旨在提供一种在任何级别与该协议交互的实现方式。该库支持基本数据包、扩展数据包和文件属性的简化表示。它包括客户端和服务器端的示例代码,支持多种扩展,进行了单元测试,并表达了对 Russh 作者的感谢。总的来说,这是一个功能完备的 SFTP 协议库。

[
https://github.com/AspectUnk/russh-sftp
](
https://github.com/AspectUnk/russh-sftp
)
    


### TITLE

以下是对该项目的中文总结:

Russh是一个基于Tokio的低级SSH2客户端和服务器Rust实现。它是对Thrussh项目的一个分支,增加了一些新功能和改进。

主要新增功能包括:

1. 提高了panic安全性
2. 支持async_trait
3. 支持远程端口转发(forward-tcpip)
4. 支持本地和远程UNIX套接字转发
5. 添加了多种新的加密算法,如aes256-gcm、aes256-ctr等

此外,它还提供了客户端和服务器端的示例,包括简单的客户端、交互式PTY客户端、echo服务器、SFTP客户端和服务器等。总的来说,Russh是一个功能丰富、现代化的Rust SSH库。

[
https://github.com/Eugeny/russh
](
https://github.com/Eugeny/russh
)
    


### TITLE

以下是对给定内容的中文总结:

kty是一个Kubernetes终端工具,让你可以无需kubectl就能轻松访问集群中的资源如pod等。只需在集群中安装kty,你就可以通过ssh获得一个仪表板,用于与集群交互。主要功能包括:

1. 使用Github或Google账号登录集群,无需麻烦的kubectl身份验证插件。
2. 在pod中运行shell,就像SSH进入主机一样。
3. 访问pod中运行和已退出容器的日志。
4. 在本地机器与集群之间,或集群与本地之间进行流量转发。
5. 从pod复制或传输文件。
6. 从任何有SSH客户端的设备访问集群,包括手机和嵌入式设备。

kty是用Rust编写的SSH服务器,提供基于TUI的仪表板,将Kubernetes概念映射到SSH。它依赖OpenID提供商如Github或Google来验证身份。与kubectl一样,Kubernetes的RBAC将验证访问权限,遵循组织政策。

[
https://kty.dev
](
https://kty.dev
)
    


### TITLE

Symphonia是一个纯Rust音频解码和媒体解复用库,支持AAC、ADPCM、AIFF、ALAC、CAF、FLAC、MKV、MP1、MP2、MP3、MP4、OGG、Vorbis、WAV和WebM等多种格式。它具有以下特点:

1. 支持流行音频编码解码,支持无缝播放。
2. 支持常见媒体容器格式解复用。
3. 能读取多种元数据和标签格式。
4. 提供基本的音频操作原语,高效处理音频数据。
5. 100%安全的Rust代码,最小化依赖。
6. 高性能,速度不输主流开源解码器。

该库分为多个crate,默认只启用免版税的开放标准编解码器,其他的可通过feature flag启用。每种格式和编解码器都标注了开发状态。项目计划提供C API和WASM API以支持其他语言调用。此外,该库致力于正确解码媒体、防止拒绝服务攻击、进行模糊测试,并提供强大且易用的API。目前与FFmpeg相比,性能相当且有±15%的差异。该库支持SSE、AVX和Neon等SIMD优化,提供了基本示例和调试工具。

[
https://github.com/pdeljanov/Symphonia
](
https://github.com/pdeljanov/Symphonia
)
    


### TITLE

CPAL是一个用纯Rust编写的跨平台音频输入输出低级库。它支持以下功能:

- 枚举支持的音频主机
- 枚举所有可用的音频设备
- 获取当前默认的输入和输出设备
- 枚举设备已知支持的输入和输出流格式
- 获取设备的当前默认输入和输出流格式
- 在选定的设备上以给定的流格式构建和运行输入和输出PCM流

当前支持的主机包括Linux(通过ALSA或JACK)、Windows(默认通过WASAPI)、macOS(通过CoreAudio)、iOS(通过CoreAudio)、Android(通过Oboe)和Emscripten(用于Web汇编)。

该库提供了一些可选的音频后端功能标志,如JACK(在Linux上)、ASIO(在Windows上)和Oboe(在Android上)。在Windows上使用ASIO需要从Steinberg下载ASIO SDK,并通过设置环境变量CPAL_ASIO_DIR来指定路径。还需要安装LLVM并设置LIBCLANG_PATH环境变量。

该库还支持跨平台编译,在Windows上支持MSVC编译器支持的所有交叉编译目标,在Linux和macOS上可以使用MinGW-w64工具链进行编译。

[
https://github.com/RustAudio/cpal
](
https://github.com/RustAudio/cpal
)
    


### TITLE

总结如下:

1. 该问题提出了如何在Rust的Cargo包管理工具中为不同的binary目标指定不同的依赖项的需求。

2. 提供了三种可能的解决方案:

A. 将binary改为examples,这样binary只依赖于dev-dependencies中列出的包。

B. 使用可选依赖项和特性,为binary启用需要的可选依赖特性。

C. 将binary代码放在单独的包中,作为当前库包的依赖项,这样可以分别管理依赖。

3. 另一种解决方案是使用工作空间(workspace),将库和binary放在同一个工作空间的不同子目录下,共享一个Cargo.lock文件。

4. 当前Cargo还没有直接支持为不同的binary指定不同依赖的功能。

[
https://stackoverflow.com/questions/35711044/how-can-i-specify-binary-only-dependencies
](
https://stackoverflow.com/questions/35711044/how-can-i-specify-binary-only-dependencies
)
    


### TITLE

以下是对该内容的中文总结:

TensorZero是一个开源平台,旨在帮助大型语言模型(LLM)应用程序从API包装器发展为可防御的AI产品。它通过统一推理、可观测性、优化和实验来实现LLM的数据和学习飞轮。

TensorZero Gateway是一个高性能的模型网关,用Rust编写,为所有主要LLM提供商提供统一的API接口,支持跨平台集成和后备。它处理结构化基于模式的推理,延迟开销小于1毫秒,并内置可观测性和实验功能。它还收集与这些推理相关的下游指标和反馈,并将其存储在你控制的ClickHouse数据仓库中进行分析。

TensorZero Recipes利用这些结构化数据来优化提示词和模型,提供常见工作流的预构建配方,或使用任何语言和平台创建自己的配方。网关的实验功能和GitOps编排能够让你自信地迭代和部署,无论是单个LLM还是数千个LLM。

TensorZero旨在帮助工程师构建、管理和优化下一代LLM应用程序:从真实世界经验中学习的系统。他们提供了快速入门、教程、示例等资源来开始使用。

[
https://github.com/tensorzero/tensorzero/
](
https://github.com/tensorzero/tensorzero/
)
    


### TITLE

in-vite是一个受到Laravel的Vite插件的启发而开发的小型Rust库。它允许您将Vite的打包功能集成到Rust后端。该库围绕一个名为Vite的结构体, 用于处理大多数方面并进行集成。

该库需要您先自行设置Vite实例。设置Vite后,您需要扩展Vite的vite.config.js文件,启用构建清单和指定入口点。

该库默认假设您处于开发模式,除非某些环境变量被设置为生产模式。您也可以使用ViteOptions显式指定模式。

in-vite提供了与模板引擎tera和minijinja的集成,可以通过相应的feature flag来激活。

该库欢迎错误报告、功能请求和新集成的拉取请求。作为该作者的第一个Rust库,也欢迎任何代码审查和改进建议。该项目接受小额捐赠,用于为学生作者提供所需的咖啡因饮料。该项目基于MIT许可证。

[
https://github.com/HiImJulien/in-vite
](
https://github.com/HiImJulien/in-vite
)
    


### TITLE

这篇内容主要介绍了加拿大BMO银行为投资者提供的免费在线期权交易资源。BMO提供了一个免费的互动式期权交易课程,可以帮助投资者深入了解期权交易的策略。此外,BMO还举办免费的网络研讨会,由行业专家分享高级期权交易策略。BMO还拥有一个综合的免费视频库,涵盖了期权基础知识和高级策略。通过利用这些免费资源,投资者可以提高期权交易水平,实现更好的投资回报。最后,内容鼓励投资者注册BMO的在线交易平台BMO InvestorLine,开启新的投资之旅。

[
https://old.reddit.com/user/BMO/comments/1feln6j/megathread_3_ways_bmo_investorline_can_help_you/
](
https://old.reddit.com/user/BMO/comments/1feln6j/megathread_3_ways_bmo_investorline_can_help_you/
)
    


### TITLE

kty是一个使用Rust语言编写的SSH服务器,它提供了一个基于TUI的仪表板,将Kubernetes的概念映射到SSH上。它依赖于OpenID提供商进行身份验证,因此您无需在堆栈中引入任何特殊内容。通过SSH,您可以做一些有趣的事情,否则很难实现:

1. 使用`ssh -R`将集群上的服务流量转发到本地笔记本电脑。
2. 使用`ssh -L`将本地端口的流量转发到集群。
3. 使用`scp`或任何SFTP客户端在容器内部本地复制文件。
4. 从任何支持SSH的地方访问容器的Shell。

到目前为止,构建kty是一个非常有趣的过程,它突出了Rust生态系统的丰富性:

- ratatui用于TUI。
- russh用于SSH服务器。
- russh-sftp用于SFTP功能。
- kube-rs用于Kubernetes交互。
- tachyonfx用于动画。

[
https://old.reddit.com/r/rust/comments/1fi9ta4/kty_the_terminal_for_kubernetes/
](
https://old.reddit.com/r/rust/comments/1fi9ta4/kty_the_terminal_for_kubernetes/
)
    


### TITLE

这位开发者正在用Rust语言开发一个音乐播放器。最初他使用了基于CPAL的Rodio库,后来也尝试过将CPAL与Symphonia和SDL2结合使用。

他发现Rodio运行正常,但CPAL目前无法获取当前播放时间,这意味着虽然可以实现定位功能,但无法显示当前播放时间,只能显示总时长。另一方面,SDL2允许访问当前播放时间。

因此,他的问题是:是否值得切换到SDL2以获得使用可拖动进度条在歌曲中定位的能力,抑或这一功能对用户不够重要,不值得为此换库,而等待CPAL自身实现该功能。

该程序目标是在Windows和Linux操作系统上运行,所有提到的库都支持这两个系统。具体的"定位"功能是指用户可以拖动进度条到歌曲的特定时间点,松开后播放将跳到该时间,进度条也会向右移动。

[
https://old.reddit.com/r/rust/comments/1fiqsoj/seeking_input_is_draggable_playback_worth_the/
](
https://old.reddit.com/r/rust/comments/1fiqsoj/seeking_input_is_draggable_playback_worth_the/
)
    


### TITLE

这篇内容讨论了Rust编程语言中关于二进制文件和库依赖的问题。作者提出了一个简单的crate和一个使用clap库的CLI工具,如果其他人只将crate用作库,就不需要安装clap依赖。作者发现了一些解决方案,但都不完美。

早在2015年,就有人提出了这个问题。文中列出了三种解决方案:

1. 将二进制文件作为示例(example)运行,适合测试但不适合发布到crate.io。

2. 将二进制文件作为单独的包发布,这是最简单的方法,但作者希望crate和CLI名称相同。

3. 使用可选依赖和必需特性,需要在运行时添加特性。

作者表示,阻碍bin-dependencies实现的原因不太清楚,可能是因为存在多个bin文件的情况。

总的来说,这是关于Rust中处理二进制文件和库依赖问题的讨论和探索。

[
https://old.reddit.com/r/rust/comments/1fi32rm/why_doesnt_rust_have_bindependencies/
](
https://old.reddit.com/r/rust/comments/1fi32rm/why_doesnt_rust_have_bindependencies/
)
    


### TITLE

这是一封来自Gabriel和Viraj的信息,他们兴奋地向Rust社区开源了他们的项目TensorZero。TensorZero是一个开源平台,旨在帮助大型语言模型(LLM)应用程序从API包装器发展成为可防御的人工智能产品。

TensorZero通过统一推理、可观测性、优化和实验来实现LLM的数据和学习飞轮。它提供了一个用于所有LLM的API,具有极低的延迟overhead。它还支持将推理和反馈发送到数据库、提示/模型/策略优化以及A/B测试等功能。

Gabriel和Viraj分享了一些运行示例,展示了TensorZero的数据和学习飞轮,包括写俳句满足隐藏偏好的法官、实体识别的JSON函数微调,以及使用自定义配方进行数学推理的自动提示工程等。

他们使用Rust编写了约45,000行代码,认为Rust是构建TensorZero这种MLOps工具的理想选择,能提供出色的性能。他们欢迎反馈和疑问,希望TensorZero对Rust社区有用。

[
https://old.reddit.com/r/rust/comments/1fibmoh/our_first_serious_rust_project_tensorzero/
](
https://old.reddit.com/r/rust/comments/1fibmoh/our_first_serious_rust_project_tensorzero/
)
    


### TITLE

这位作者是一位从C++转向Rust的程序员,他在处理一个复杂的数据结构时遇到了困难。他需要从一个大型结构体BigStruct中提取出几个字段,并将它们传递给另一个结构体BigInterface的构造函数。

最初的实现方式是在main函数中手动提取所需字段,然后构造BigInterface的实例,最后再构造目标结构体MyStruct。但这种方式代码显得很冗长和笨拙。

为了简化代码,作者使用了一个宏来封装字段提取和实例构造的过程。这使得main函数看起来更加简洁,但同时也隐藏了具体的实现细节,可能会给其他程序员带来理解上的困难。

作者在权衡简洁性和可读性之间存在困扰,希望从Rust社区获得一些建议和更优雅的实现方式。他对Rust的习惯用法和权衡思路很感兴趣。

[
https://old.reddit.com/r/rust/comments/1fii722/how_to_idiomatically_abstract_away_complex/
](
https://old.reddit.com/r/rust/comments/1fii722/how_to_idiomatically_abstract_away_complex/
)
    


### TITLE

该帖子的作者在探索使用Rust语言查询MySQL数据库的性能时,发现与Golang相比,Rust的查询性能似乎较慢。作者使用了Rust MySQL异步crate来执行查询,返回约30k行,每行大小为100-200字节。尽管将Rust可执行文件构建为发布模式并启用最大优化后,查询执行时间从40毫秒降低到9毫秒,但Golang仍然更快,只需5-6毫秒。

作者已尝试分析性能,但未发现明显问题。因此,他想请教其他人是否有任何想法或建议来优化Rust的查询性能,以缩小与Golang之间的性能差距。

另外,作者还注意到Rust MySQL crate支持的选项和功能较少,例如不支持charset url参数。因此,他询问是否有其他可供替代的crate。

[
https://old.reddit.com/r/rust/comments/1fio1p5/rust_mysql_async_query_performance/
](
https://old.reddit.com/r/rust/comments/1fio1p5/rust_mysql_async_query_performance/
)
    


### TITLE

这篇文章讨论了在illumos操作系统上使用Rust语言的现状和挑战。主要内容包括:

1. 虽然Linux内核社区正在推动将Rust引入内核,但illumos作为另一个操作系统,其开发模式不同,目前还没有在上游版本中引入Rust驱动程序。不过,illumos的一个分支Oxide已经包含了一些用Rust编写的网络驱动。

2. 与C语言相比,使用Rust开发需要更多时间成本,需要学习绑定生成器bindgen和Rust语言本身。目前Rust在用户态工具上的应用相对较小,但在内核驱动方面还需要更多工作。

3. Rust的cargo包管理器更多面向软件开发人员,与操作系统发行版的包管理器存在一些差异,如无法直接构建共享库等,这给操作系统打包带来了一些挑战。

4. 文章最后呼吁Rust社区的开发者加入illumos社区,为用户态工具、安装程序、驱动程序等作出贡献,共同推进Rust在illumos上的应用。

总的来说,这是一篇介绍illumos对于采用Rust语言所面临的技术和社区挑战的文章。

[
https://wegmueller.it/blog/posts/2024-09-02-rust-on-illumos
](
https://wegmueller.it/blog/posts/2024-09-02-rust-on-illumos
)
    


### TITLE

以下是对给定内容的中文总结:

这是Rust编程语言在运行时遇到panic(恐慌)情况时的输出。panic是Rust中用于错误处理的概念,类似于其他语言的异常。

输出显示有三种不同的panic场景:

1. 正常的panic,发生在src/main.rs文件的第12行第5列。这可能是由于程序中的某些不可恢复的错误或断言失败而引发的。

2. 带有"help me"信息的panic,发生在src/main.rs的第5行第9列。这可能是开发者为了调试而有意引发的panic。

3. 最后一个panic发生在程序清理过程中,是由于某个结构体的析构函数引发的。这种情况通常意味着程序将直接终止,而不会进行正常的清理工作。

输出中还提供了运行时环境变量RUST_BACKTRACE=1的建议,以显示完整的调用堆栈跟踪信息,有助于调试。

总的来说,这些输出反映了Rust在运行时处理不同错误场景的能力,panic机制为开发者提供了强大的错误处理和调试工具。

[
https://konnorandrews.gitlab.io/descent-into-madness/post/thought-2-panic-bomb/
](
https://konnorandrews.gitlab.io/descent-into-madness/post/thought-2-panic-bomb/
)
    


### TITLE

该帖子宣布作者推出了第一个公开的 Rust 库 `in-vite`。`in-vite` 是一个小型库,允许您将 Vite 捆绑器集成到 Rust 后端中,从而在开发前端时受益于 Vite 的诸多特性,如树状树摇树、延迟加载、公共块拆分、监视模式和热模块替换等。Vite 还原生支持许多功能,无需像 Webpack 那样痛苦地配置规则来编译 TypeScript。

此外,`in-vite` 还为模板引擎 `tera` 和 `minijinja` 提供了集成支持,使得在模板中使用 Vite 变得非常简单。作者希望这个库对基于 HTMX 或类似项目有所帮助,并欢迎反馈和提问。作为作者的第一个公开 Rust 库,他表示可能会有一些粗糙的边缘,并感谢任何改进建议。

[
https://old.reddit.com/r/rust/comments/1fig61d/invite_vite_integration_for_rust_backends/
](
https://old.reddit.com/r/rust/comments/1fig61d/invite_vite_integration_for_rust_backends/
)
    


### TITLE

经过两年的内存安全压力,C++社区发布了一项提案旨在帮助开发人员编写更安全的代码。这个名为"安全C++扩展"的提案旨在解决C++这种容易出现内存安全漏洞的编程语言的痛点。该提案将为C++添加内存安全功能,如借用检查来防止使用后释放错误,以及初始化分析来保证类型安全性。尽管也有人对C++是否能达到像Rust那样的内存安全水平表示怀疑,但该提案的目标是为C++开发人员提供与Rust一样的内存安全保证,同时降低重写代码到其他语言的成本。接下来的步骤是让整个行业参与进来,完善该提案,为C++的所有功能指定安全版本。减少C++安全漏洞的重要性促使该努力得以付诸实施。

[
https://www.theregister.com/2024/09/16/safe_c_plusplus/
](
https://www.theregister.com/2024/09/16/safe_c_plusplus/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

