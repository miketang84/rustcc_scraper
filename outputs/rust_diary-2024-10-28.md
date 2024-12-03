【Rust日报】2024-10-28


### TITLE

just是一个用于保存和运行特定于项目的命令的实用工具。它的语法受到make的启发,但比make简单很多,避免了很多复杂性和特殊情况。just支持Linux、macOS和Windows,无需额外依赖。它提供了详细的错误信息、命令行参数支持、环境变量加载、命令补全脚本等有用的功能。just还支持用任意语言编写命令,可以从任意子目录调用。just可通过各种包管理器安装,也提供了预编译的二进制文件供下载。如果需要帮助,可随时提出问题或在Discord上ping作者。该项目欢迎新功能请求和bug报告。

[https://github.com/casey/just
](https://github.com/casey/just
)
    


### TITLE

根据所给出的内容,主要阐述了为什么在TigerBeetle项目中选择使用Zig编程语言而不是Rust。虽然Rust在内存安全性和线程安全性方面有显著优势,但对于TigerBeetle这个项目来说,这些优势的重要性相对较低。

TigerBeetle项目有一个独特的对象模型,在启动后几乎不涉及内存分配,整个系统是单线程的。在这种特殊情况下,Rust的内存安全和线程安全特性就显得没那么重要了。相比之下,Zig语言在编译期计算和代码简洁性方面更有利于实现TigerBeetle的特殊对象模型。

另外,TigerBeetle团队采用了大量测试来确保代码正确性,这在一定程度上弥补了Zig在内存安全性方面的不足。因此,综合考虑项目的具体情况和需求,Zig语言更适合TigerBeetle的开发。

[
https://old.reddit.com/u/matklad
](
https://old.reddit.com/u/matklad
)
    


### TITLE

这是一个关于Rust Cargo工作空间中的链接器冲突问题的总结。作者有三个crate:

1. engine: 通过FFI调用C++应用程序的Rust crate
2. engine_rs: engine的纯Rust移植版本
3. ffi_test: 用于比较以上两个引擎输出的crate

engine_rs使用了最新版本的opencv-rs和openvino-rs。而engine使用了OpenCV和旧版本的OpenVINO,并在本地包含了这个旧版本的共享对象文件。

当单独运行ffi_test中的每个引擎测试时都可以正常工作。但当同时运行这两个引擎时,会出现OpenVINO版本太旧的错误。原因是Rust引擎engine_rs试图使用engine crate中的旧版OpenVINO库,尽管它们是独立的crate。

作者已经在engine_rs中启用了runtime-linking,但仍无法解决问题。他分享了engine和engine_rs的build.rs文件中的配置细节,并询问解决此问题的可选方案。

[
https://old.reddit.com/r/rust/comments/1ged5d3/linker_conflict_on_cargo_workspace/
](
https://old.reddit.com/r/rust/comments/1ged5d3/linker_conflict_on_cargo_workspace/
)
    


### TITLE

这个帖子讨论了一种巧妙的做数组边界检查的方法。原始的做法是通过将无符号索引转换为有符号进行边界检查,需要做6次类型转换和4次布尔检查。作者提出了一种利用Rust中usize类型的wrapping_add_signed方法的技巧,只需做2次布尔检查,而不需要任何类型转换。

这种技巧的核心思想是:对于负的索引增量,wrapping_add_signed会将其绕回到usize::MAX的高位,从而自动实现了负值边界检查。对于正的索引增量,则直接检查是否超出数组大小即可。这种技术在数组不太大且只检查相邻单元格的情况下很实用,可以减少运算开销。不过作者也说这看起来有点hack,所以可能存在一些隐患,需要进一步评估其合理性。

[
https://old.reddit.com/r/rust/comments/1gdp4tx/figured_out_a_clever_way_to_do_bounds_checking/
](
https://old.reddit.com/r/rust/comments/1gdp4tx/figured_out_a_clever_way_to_do_bounds_checking/
)
    


### TITLE

这个帖子讨论了一段看似简单的Rust代码无法编译的问题。代码如下:

```rust
fn main() {
    let mut buffet = Vec::new();
    drink(&mut buffet, eat(&buffet));
}

fn eat(_slice: &[u8]) -> i32 {
    42
}

fn drink(_slice: &mut [u8], _count: i32) {}
```

编译器报错"cannot borrow buffet as immutable because it is also borrowed as mutable"。

作者最初以为这段代码应该可以编译通过,因为`eat`函数只是返回一个简单的整数,与可变借用`buffet`无关。但事实证明,编译器在计算函数参数时遇到了借用检查的限制。

通过改变函数参数顺序或者先计算`eat`函数的返回值,这段代码就可以通过编译了。作者认为这可能不是编译器bug,而是借用检查器的一个意外限制。

[
https://old.reddit.com/r/rust/comments/1ge6y8d/quiz_do_you_think_this_should_compile/
](
https://old.reddit.com/r/rust/comments/1ge6y8d/quiz_do_you_think_this_should_compile/
)
    


### TITLE

这段内容描述了在Rust编程语言中使用闭包作为回调函数时遇到的一个问题。当在一个外部函数 `outer_fn` 中多次调用内部函数 `inner_fn` 时,如果内部函数需要捕获该闭包,就会导致所有权移动和使用后移动的错误。

作者提出了一个解决方案,即在 `outer_fn` 中传递闭包的引用给 `inner_fn`,但认为这种间接的方式看起来有些多余。作者希望找到一个更好的解决方案,能够在不要求闭包实现 `Copy` trait的情况下,任意多次调用该闭包,同时也能支持 `FnMut` 闭包作为回调。

总的来说,这是一个关于如何在Rust中正确使用闭包回调的实践问题,作者希望获得更恰当的解决方式。

[
https://old.reddit.com/r/rust/comments/1ge4u9g/how_to_avoid_seemingly_unnecessary_indirection/
](
https://old.reddit.com/r/rust/comments/1ge4u9g/how_to_avoid_seemingly_unnecessary_indirection/
)
    


### TITLE

这篇博文主要围绕着Iggy.rs消息传递平台的最新进展和未来目标进行了阐述。主要内容包括:

1. Iggy.rs被著名技术咨询公司Thoughtworks列入了值得探索和评估的项目技术雷达,这对Iggy.rs来说是个重大成就,意味着获得了更多认可和信任。

2. Iggy.rs的当前主要目标包括:

- 复制(Replication):实现基于Viewstamped Replication的分布式共识算法,支持高吞吐的环形拓扑和链式复制模型。

- S3存储:已支持将服务器状态日志和流数据归档到S3兼容的云存储。未来计划实现直接从S3读写实时数据,形成三级缓存层次结构(内存、磁盘、S3)。

- OpenTelemetry:集成开放遥测框架,提高可观测性。

- 优化:包括使用io_uring技术提升I/O性能等。

总的来说,这反映了Iggy.rs不断增强分布式、存储、可观测性等关键能力,以满足高性能、高可用消息传递的需求。

[
https://blog.iggy.rs/posts/technology-radar-and-currrent-goals/
](
https://blog.iggy.rs/posts/technology-radar-and-currrent-goals/
)
    


### TITLE

这个更新日志总结了Rust分析器在2024年10月28日发布的版本0.3.2162的主要变化。新功能包括支持Option的包装/解包返回类型、范围运算符和模式上的"Go to definition"、不在初始化时启动服务器的选项、实现了混合站点卫生、诊断的拉模型、改进了文档渲染、错误诊断分割等。还修复了一些格式化、补全、解析等问题。内部改进包括切换到合并队列CI、合并重叠的行内提示、改进宏错误消息等。此外,Windows构建将不再包括.gz工件,改为使用.zip文件。

[
https://rust-analyzer.github.io/thisweek/2024/10/28/changelog-257.html
](
https://rust-analyzer.github.io/thisweek/2024/10/28/changelog-257.html
)
    


### TITLE

这位Rust的crate开发者想要在诊断信息中添加自定义错误消息的功能。目前,这个功能只能用于"未实现的trait"错误,但他希望将其扩展到未解析的导入(告知用户模块或结构体已移动)或不应该调用的函数(这与负trait实现的目的类似,但后者已被跟踪近5年)。他想知道除了"未实现的trait"错误之外,自定义诊断信息还能应用于哪些其他场景。但在GitHub上很难找到相关的跟踪issue。总的来说,他希望能扩展自定义诊断信息的使用范围,为crate开发提供更好的错误说明支持。

[
https://old.reddit.com/r/rust/comments/1geb3zw/wheres_the_tracking_for_the_other_diagnostic/
](
https://old.reddit.com/r/rust/comments/1geb3zw/wheres_the_tracking_for_the_other_diagnostic/
)
    


### TITLE

根据博文内容总结如下:

该博文讨论了在Web客户端是使用Rust还是TypeScript编写更合适。作者认为UI层面应该用TypeScript和流行的Web框架编写,方便前端开发者贡献代码。但对于处理媒体体验的底层部分存在两种选择:

1. 使用现有的TypeScript库moq-js,它涵盖了网络、媒体容器、编解码和捕获/渲染等功能。

2. 使用WebAssembly技术,将Rust库moq-wasm编译为WASM,用于解码和渲染媒体。

作者列出了在Web Worker线程中使用WASM的一些优势和担忧。优势是代码可重用于原生应用,但担心性能开销,比如需要在JS和WASM间拷贝数据。

最后,作者没有明确表态,而是征求读者的意见,看是继续使用moq-js还是转向moq-wasm。这是一个需要权衡的决策,涉及到开发效率、性能和可重用性等多方面的考量。

[
https://quic.video/blog/to-wasm
](
https://quic.video/blog/to-wasm
)
    


### TITLE

这段代码演示了Rust中变量的作用域和值移动的概念。主要内容如下:

1. 在main函数中,首先定义了一个整数x=5。

2. 在内部作用域中,用let x = x * 2创建了一个新的x变量,其值为10,并打印出来。

3. 离开内部作用域后,外部的x值仍为5,因为整数实现了Copy trait,在内部作用域只是创建了一个副本。

4. 对于像String这种没有实现Copy trait的类型,如果在内部作用域重新绑定同名变量,则会导致编译错误,因为所有权已经移动到内部作用域。

5. 可以使用clone()方法显式克隆所有权,避免所有权移动导致的编译错误。

总的来说,这个例子说明了Rust ownership和作用域对变量赋值和使用的影响,尤其是值语义类型和引用语义类型的区别。

[
https://old.reddit.com/r/rust/comments/1ge1l0o/why_doesnt_x_get_moved_here_should_the_last/
](
https://old.reddit.com/r/rust/comments/1ge1l0o/why_doesnt_x_get_moved_here_should_the_last/
)
    


### TITLE

这篇文章比较了在 Windows 11 和 WSL2 Ubuntu 环境下运行各种 Cargo 任务的性能表现。作者在 Windows 11 系统上使用新的 Dev Drive 特性,在 WSL2 Ubuntu 环境中,分别克隆了 https://github.com/casey/just 项目,并使用 hyperfine 工具进行基准测试。

测试结果显示,在所有测试任务中,包括 fd、rg 和各种 cargo 命令,WSL2 Ubuntu 环境的性能都明显优于 Windows 11,WSL2 Ubuntu 的运行速度是 Windows 11 的近两倍或更快。即使使用了专为开发工作负载优化性能的 Dev Drive,Windows 11 的表现仍然落后于 WSL2 Ubuntu。

作者总结说,尽管 Windows 11 使用了 Dev Drive 等优化,但在运行 Rust 相关任务时,WSL2 Ubuntu 环境的性能更加出色,如果需要在 Windows 系统上进行 Rust 开发,使用 WSL2 可能会带来明显的性能提升。

[
https://old.reddit.com/r/rust/comments/1gef1of/comparing_performance_of_native_windows_vs_wsl2/
](
https://old.reddit.com/r/rust/comments/1gef1of/comparing_performance_of_native_windows_vs_wsl2/
)
    


### TITLE

CubeCL 0.3版本发布了新的运行时和增强的编译器,扩展了对AMD GPU的支持。通过`rocm`运行时和`HIP` C++接口,可以利用针对CUDA优化的编译器,为AMD GPU带来性能提升。下一步将实现矩阵乘法累加(MMA),大幅提高内核性能。

之前AMD支持仅通过`wgpu`运行时,受WebGPU限制无法使用半精度和MMA。新版本可直接从CubeCL IR生成`SPIR-V`,通过`wgpu`运行时支持更多GPU上的低精度和MMA。

新版本还改进了宏系统,扩展了Rust语法支持,引入了更多编译期优化。只需设置环境变量即可简化内核分析。

此版本大幅增强了矩阵乘法内核性能,达到了cuBLAS水平,确保了CubeCL在任何GPU上都可以匹配手工优化的cuBLAS内核性能。开发者还将继续优化这些内核,适配各种GPU架构。

作者对社区的宝贵贡献表示特别感谢。CubeCL旨在结合卓越性能、灵活性和可移植性,提供统一实用的API,Rust持续显示出在高性能计算领域的潜力。

[
https://old.reddit.com/r/rust/comments/1geb3m2/cubecl_03_released_rocmhip_spirv_support_for/
](
https://old.reddit.com/r/rust/comments/1geb3m2/cubecl_03_released_rocmhip_spirv_support_for/
)
    


### TITLE

这个帖子讨论了Rust在跟踪数据对齐方面的不足之处。几个月前,rust-analyzer项目的维护者u/matklad表示,就空间安全性而言,Zig可能比Rust更安全,因为Zig对数据对齐的跟踪做得更好。

该帖子的发帖人询问是否有人能给出一个例子,说明Rust在跟踪数据对齐方面存在缺陷。总的来说,这个帖子旨在讨论和理解Rust在这一特定方面可能存在的局限性。

[
https://old.reddit.com/r/rust/comments/1ge959g/when_does_rust_fail_to_track_alignment/
](
https://old.reddit.com/r/rust/comments/1ge959g/when_does_rust_fail_to_track_alignment/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

