【Rust日报】2024-10-06


### TITLE

这是一个名为Eyra的Rust包,旨在支持完全用Rust编写的Rust程序。它使用Origin进行程序和线程的启动和关闭,使用c-gull实现与libc兼容的函数。目前在Linux的x86-64、x86、aarch64和riscv64架构上的Nightly Rust可用。

Eyra通过添加依赖项和构建脚本就可以在cargo中正常使用。它在底层使用Origin启动和停止程序、c-ward处理std中的libc调用、rustix进行打印输出,完全由Rust实现。

使用Eyra的一些原因包括:修复了Rust的环境变量实现中的内存泄漏问题、支持整个程序级别的LTO优化、支持使用备用调用约定编译程序、完全静态链接且支持平台的NSS/DNS配置等。不过它目前还不太成熟,功能还不够完整,只在Linux某些架构上可用,还依赖Nightly版本。

Eyra支持完全静态链接,不依赖任何动态库,但默认情况下仍依赖动态链接器。它还提供了可选的日志功能和与-Zbuild-std的兼容性。借助一些技术,Eyra还可以生成非常小的静态链接二进制文件。

Eyra遵循一些设计原则,如尽可能遵守Rust规则、将C兼容性作为Rust之上的一层等。它还可以编译为libc.a来编译C程序。

[https://github.com/sunfishcode/eyra
](https://github.com/sunfishcode/eyra
)
    


### TITLE

总结:

这个网站是一位拥有24年软件开发经验的人创建的个人网站。作者承认,他并不像新手那样天真,但同时也没有固步自封,仍然保持对新事物的开放态度。他希望通过这个网站分享自己的经验和见解,而不是要读者盲目关注。作者的语气显示出经验老到但又不失谦逊的特点。

[
https://thiagocafe.com
](
https://thiagocafe.com
)
    


### TITLE

以下是对该内容的中文总结:

rust_pixel是一个2D游戏引擎和快速原型工具,支持文本和图形渲染模式。它适合创建2D像素风格游戏和开发终端应用程序。它可以编译成FFI用于前端和后端,也可以编译成WASM用于web项目。

文本模式:使用crossterm在终端中运行,使用ASCII和Unicode Emoji进行绘制。
图形模式(SDL2和Web):使用glow和sdl2,使用PETSCII和自定义图形符号进行渲染。

rust_pixel提供了一些核心功能:
- 游戏循环和模型/渲染设计模式
- 事件/计时器消息机制
- 支持文本渲染模式
- 统一的OpenGL绘图模式,支持SDL和WASM
- 3种核心OpenGL着色器用于SDL2和Web图形模式
- 一些常见的游戏算法
- 音频和日志支持
- 演示游戏:俄罗斯方块、塔防等
- 演示终端UI应用程序:调色板等
- 将核心算法包装成FFI和WASM的示例

该项目还包括一些工具,如调色板、字符编辑器、图像到PETSCII转换器等。它提供了一些游戏示例,如贪吃蛇、俄罗斯方块、扑克等,并展示了如何将Rust算法包装成FFI和WASM以供其他语言调用。

总的来说,rust_pixel是一个功能丰富的2D游戏引擎,提供了文本和图形渲染模式,并附带了一些有用的工具和演示游戏。

[
https://github.com/zipxing/rust_pixel
](
https://github.com/zipxing/rust_pixel
)
    


### TITLE

这是一个关于在Windows任务栏缩略图预览中添加媒体控制按钮的问题。作者正在开发一个Tauri播客客户端应用程序,希望添加类似于Spotify缩略图中的播放/暂停、上一曲、下一曲等控制按钮。由于Tauri API似乎没有直接支持这一功能,作者想知道如何使用Rust语言实现这一需求。他查看了windows crate,但由于是Rust新手,发现理解这个库还有一些困难。所以他想请教是否有比较简单的实现方式。

[
https://old.reddit.com/r/rust/comments/1fxpuvl/add_windows_thumbbar_buttons_to_rust_app/
](
https://old.reddit.com/r/rust/comments/1fxpuvl/add_windows_thumbbar_buttons_to_rust_app/
)
    


### TITLE

该帖子讨论了在Rust语言中使用phi3视觉技术进行推理时遇到的一些挑战。作者认为编写推理代码时需要编写的代码量太多,这让人感到困扰。

帖子中包含了一个名为`merge_text_and_image_embeddings`的Rust函数代码,该函数将文本嵌入和图像特征进行合并,同时更新注意力掩码。这个函数的代码看起来相当复杂,需要大量的数组切片和赋值操作,这正反映了作者所说的"令人痛苦的代码"。

总的来说,这个帖子反映了Rust语言在进行某些复杂的机器学习和计算机视觉任务时,代码可能会变得冗长和繁琐,这给开发人员带来了一定的挑战。

[
https://old.reddit.com/r/rust/comments/1fxyjzs/writing_inference_code_in_rust_often_involves/
](
https://old.reddit.com/r/rust/comments/1fxyjzs/writing_inference_code_in_rust_often_involves/
)
    


### TITLE

这是一位开发者在询问是否有库支持在 Dioxus 框架中使用类似 Leptos 的 HTML 风格宏语法。他比较喜欢 Leptos 的 HTML 语法,但认为 Dioxus 相对来说文档更好、功能也更全面,因此希望能在 Dioxus 中使用类似的 HTML 语法。他在权衡 Leptos 和 Dioxus 的优缺点,寻求是否有现成的解决方案来满足自己的需求。

[
https://old.reddit.com/r/rust/comments/1fxz7ix/leptoslike_syntax_for_dioxus/
](
https://old.reddit.com/r/rust/comments/1fxz7ix/leptoslike_syntax_for_dioxus/
)
    


### TITLE

这是一个关于Rust编译器如何优化二进制文件大小的讨论。主要内容包括:

1. 作者尝试了不同的编译选项来观察对二进制文件大小的影响,包括使用musl/libc、添加链接参数、切换编译器版本等。

2. 作者发现在某些情况下,即使添加了"Hello, world!"字符串输出,二进制文件大小也没有增加,这看起来很奇怪。

3. 作者还尝试了使用Zig编译器,结果显示Zig可以更好地去除未使用的代码,产生更小的二进制文件。

4. 作者推测Rust编译器可能在某些情况下无法完全移除未使用的代码,导致生成的二进制文件相对较大。

5. 后来作者发现,当切换到LLD链接器时,musl构建的二进制文件大小减小到3770-3952字节,表明之前相同大小可能只是巧合。

6. 对于静态链接glibc,Rust也无法像Zig那样很好地优化,导致生成的可执行文件较大。

总的来说,这个讨论探索了Rust编译器在某些特殊情况下优化二进制大小的能力,并与Zig进行了比较。

[
https://old.reddit.com/r/rust/comments/1fx9ock/why_wouldnt_the_binary_size_increase_when_a/
](
https://old.reddit.com/r/rust/comments/1fx9ock/why_wouldnt_the_binary_size_increase_when_a/
)
    


### TITLE

以下是对给定内容的中文总结:

Texted是一个无需数据库的博客平台,你可以使用Markdown或HTML来撰写文章。作者目前正在使用Texted运行他的两个博客 https://thiagocafe.com 和 https://blogaro.com.br

作者选择使用Texted的原因是,他经常需要处理不同格式和平台的内容,而要恢复或渲染/阅读这些内容并不简单。从依赖付费专有软件阅读Word文档,到难以导出Google文档和Wordpress博文,作者对这种混乱感到厌烦。更糟糕的是,有时候对这些文档的一个小小修改(有时是无心之失)就会导致它们无法恢复。

因此,Texted这种基于纯文本的博客系统可以很好地解决上述问题,作者无需担心内容的丢失或格式兼容性问题。

[
https://old.reddit.com/r/rust/comments/1fxnohe/texted_databaseless_blogging_platform/
](
https://old.reddit.com/r/rust/comments/1fxnohe/texted_databaseless_blogging_platform/
)
    


### TITLE

总结:

原帖子作者由于家中紧急情况,无法前往维也纳参加今年的EuroRust大会。他手上有一张额外的个人门票,原价520欧元,现在希望以原价出售给有需要的人。这是一张最后一刻购买的个人门票,不是公司报销的,也无法使用增值税识别号。如有人感兴趣,可以与帖子作者联系购买该门票。

[
https://old.reddit.com/r/rust/comments/1fxkrnr/unable_to_attend_eurorust_selling_a_ticket/
](
https://old.reddit.com/r/rust/comments/1fxkrnr/unable_to_attend_eurorust_selling_a_ticket/
)
    


### TITLE

总结:

RustPixel是一个2D游戏引擎和快速原型制作工具,支持文本和图形两种渲染模式。它适用于创建2D像素风格游戏和开发终端应用程序。

它可以编译成FFI用于前端和后端,也可以编译成WASM用于网络项目。

文本模式使用crossterm库在终端中运行,使用ASCII和Unicode Emoji进行绘制。

图形模式使用glow和sdl2库,使用PETSCII和自定义图形符号进行渲染。

该引擎提供了多种开发选择,满足不同场景和需求。通过简单而强大的接口,开发者可以高效地构建2D游戏和应用程序。

[
https://old.reddit.com/r/rust/comments/1fxc651/rust_pixel_updated_to_v051/
](
https://old.reddit.com/r/rust/comments/1fxc651/rust_pixel_updated_to_v051/
)
    


### TITLE

这位用户想要使用Rust语言中的ffmpeg_next库进行屏幕录制,但遇到了一些问题。他找不到ffmpeg_next的绑定,虽然发现了format::format::list函数,但它被锁定在一个不存在的功能特性后面,而且源代码对它是空白的。他想知道有没有办法通过ffmpeg_next或在Rust中找到gdigrab设备来实现屏幕捕捉。

[
https://old.reddit.com/r/rust/comments/1fxxvzz/screen_recording_with_ffmpeg/
](
https://old.reddit.com/r/rust/comments/1fxxvzz/screen_recording_with_ffmpeg/
)
    


### TITLE

这是一个关于Rust编程语言的库的介绍。该库名为HypoRS,它为Polars系列对象提供了假设检验的功能。

该库包括多种参数检验方法,如Z检验、学生t检验、比率检验、方差分析和卡方检验,还有曼-惠特尼U检验及其变体。用户可以配置显著性水平α值、检验类型(单侧或双侧)以及功效性(用于计算样本量)。

该库旨在全面覆盖假设检验的需求,但开发者仍欢迎任何问题反馈和贡献。该库可在crates.io上获取。

[
https://old.reddit.com/r/rust/comments/1fxguu0/hypors_statistical_hypothesis_testing_for_polars/
](
https://old.reddit.com/r/rust/comments/1fxguu0/hypors_statistical_hypothesis_testing_for_polars/
)
    


### TITLE

这是一位开发者成功地在Android WebView实例中加载WASM文件并构建了APK应用程序,还集成了OpenAI REST API。如果有人感兴趣了解他是如何做到的,他可以公开源代码。他还附上了一张截图,显示了该应用程序的界面。

[
https://old.reddit.com/r/rust/comments/1fxz8aq/successfully_loaded_wasm_in_android_webview/
](
https://old.reddit.com/r/rust/comments/1fxz8aq/successfully_loaded_wasm_in_android_webview/
)
    


### TITLE

该帖子的作者想要一种非常简单的方式来将CPU和所有核心100%占用,目的是模拟在运行其他程序时的极端CPU负载情况。作者提供了一段Rust代码,通过在每个CPU核心上启动一个线程,在线程中进行一个无限循环计算sin(x+1)的方式来实现CPU 100%占用。

作者还询问是否有更好的方式,比如测试寄存器、缓存等,使CPU完全饱和,以便测试在同时运行其他程序时的各种延迟。总的来说,作者希望找到一种可靠的方式将系统CPU完全占满,以进行相关测试。

[
https://old.reddit.com/r/rust/comments/1fxpoq0/best_way_to_peg_the_cpu_at_100/
](
https://old.reddit.com/r/rust/comments/1fxpoq0/best_way_to_peg_the_cpu_at_100/
)
    


### TITLE

本文讨论了编程语言在可用性和性能之间的二元性,以及如何解决这种二元性。文章分析了尝试在单一语言中集成高级和低级功能的方法,比如C#、D语言和Python等。但作者认为,这种方法无法真正解决性能问题,因为高级语言默认的垃圾回收和指针密集的对象模型会限制性能上限。

作者认为Rust语言可能无意中解决了这个二元性。Rust没有需要选择退出的重运行时特性,在整个语言中使用统一的对象模型。Rust提供了内存安全性,同时也具备诸如代数数据类型、泛型等现代编程语言的基本便利特性。Rust社区还致力于为系统程序员提供良好的开发体验。

因此,Rust在性能上存在一个连续的光谱,从高级的Sloppy Rust到低级的DoD Rust和Crazy Rust,涵盖了从开发便利性到极致性能优化的各个层次。这种统一的对象模型确保了在性能光谱的不同层次之间流动无阻,不会产生性能税。Rust还允许专家用不安全的抽象包装优化代码,供非专家使用。最重要的是,Rust在开发之初就为未来的性能需求买了一份保险,可以在不进行全面重写的情况下渐进式地优化性能。

[
https://matklad.github.io/2024/10/06/ousterhouts-dichotomy.html
](
https://matklad.github.io/2024/10/06/ousterhouts-dichotomy.html
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

