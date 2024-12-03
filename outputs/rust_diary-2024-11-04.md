【Rust日报】2024-11-04


### TITLE

这个 Rust 库 refined_type 提供了一种增强类型的方式,让类型更健壮,并扩展应用程序静态保证的范围。您可以为某种类型创建各种规则,例如电话号码、地址、时间等。

一旦建立了规则,您就可以轻松地将它们组合起来。具体来说,如果你创建了"非空字符串"和"只由字母组成的字符串"的规则,你就不需要重新定义"非空且只由字母组成的字符串"的新规则。

只要目标类型匹配,所有规则都可以任意组合和扩展。享受美好的类型生活吧!

该库提供了一些规则组合器(And、Or、Not)来方便组合规则。它还有一些针对数字的特殊规则,如 MinMax 可以指定数字在某个范围内。

总的来说,这个库为 Rust 类型增加了更多的约束和组合能力,提高了类型的健壮性和安全性,同时保持了很好的可用性。

[https://github.com/tomoikey/refined_type
](https://github.com/tomoikey/refined_type
)
    


### TITLE

这是Rust-GB项目的第一个Alpha版本发布说明。主要包括以下几个特性:

1. `io::GbStream`允许在Game Boy屏幕上打印字符,提供了`print!`和`println!`宏。
2. `io::Joypad`结构体可以检查Game Boy的键盘输入。
3. `mmio`模块方便地使用`VolAddress`进行内存映射IO操作,但安全性还不够准确。
4. `drawing`是使用GBDK的All Points Addressable (APA)模式的绘图库,适合简单测试但长期使用会有较大的副作用和性能影响。
5. `gbdk_c`提供了对GBDK-2020的不完整绑定。

该版本的编译过程仍然非常不稳定,只能在Linux x64平台上运行编译器,并且依赖libc。版本号采用X.Y.Z-alpha.W的形式,以适应当前频繁的新特性和文档修改。项目仍处于alpha阶段,不太适合用于生产或大型项目。

[
https://github.com/zlfn/rust-gb/releases
](
https://github.com/zlfn/rust-gb/releases
)
    


### TITLE

这是一个用于GameBoy(Color)游戏开发的Rust crate。它是一个工具链和库,可以将Rust代码编译为任天堂GameBoy的ROM映像。它通过LLVM-CBE和GBDK-2020将Rust代码编译为有效的GameBoy ROM。

文档介绍了如何安装Rust-GB编译器、设置项目、编译项目以及执行ROM等步骤。它还列出了可用的库特性(如Color特性、Prototype特性)和二进制特性(如编译器特性)。

该crate提供了几个模块,包括绘图(drawing)、直接访问GBDK外部函数(gbdk_c)、GameBoy IO辅助(io)、GameBoy组件的内存映射IO地址(mmio)等。它还提供了print和println宏来在GameBoy屏幕上打印文本。

总的来说,这个crate旨在简化GameBoy(Color)游戏的开发,让开发者可以使用Rust语言编写GameBoy游戏。

[
https://docs.rs/rust-gb/latest/gb/
](
https://docs.rs/rust-gb/latest/gb/
)
    


### TITLE

这个帖子描述了将Rust编程语言代码编译为任天堂Game Boy游戏机的过程。由于Rust编译器没有直接生成Z80/GB系列处理器的机器码,作者采取了以下步骤:

1. 使用Rust编译器生成LLVM中间代码(IR)。

2. 使用LLVM-CBE后端将LLVM IR翻译为C代码。

3. 使用SDCC(Small Device C Compiler)编译C代码为Game Boy的Z80目标代码。

4. 使用GBDK-2020工具链链接SDCC生成的目标文件、GBDK提供的库文件,最终生成可在Game Boy上运行的ROM文件。

整个过程涉及了编程语言前/后端编译、中间代码生成、目标代码生成、链接等编译原理概念。帖子作者还解释了链接的作用是将多个目标文件合并为一个可执行文件或库文件。GBDK作为Game Boy开发工具包,提供了必要的系统库和特定硬件支持。通过这种交叉编译方式,可以在Game Boy这样的老旧系统上运行用Rust编写的程序。

[
https://www.reddit.com/r/rust/comments/1fhdi28/i_compiled_rust_code_to_nintendo_gameboy/
](
https://www.reddit.com/r/rust/comments/1fhdi28/i_compiled_rust_code_to_nintendo_gameboy/
)
    


### TITLE

这是一个名为 hoard_chunker 的开源项目,旨在高效地将大文件分割成较小的可管理的块,并在需要时重新组装它们。该功能对于处理大型数据集、备份、存储或传输非常有用。

该项目具有以下主要功能:

1. 备份:从输入目录备份文件到输出目录,并使用 FastCDC 将其分块。
2. 恢复:从输入目录中的块恢复到输出目录中的备份文件。

该项目使用 Rust 编程语言编写,提供了命令行界面进行操作。用户可以克隆项目仓库、构建项目,然后使用 backup 和 restore 子命令进行备份和恢复操作。

该项目欢迎贡献,用户可以提交拉取请求或提出改进建议。项目使用 MIT 许可证,用户可以在 LICENSE 文件中查看详细信息。如有任何问题或功能需求,可以在 GitHub 仓库中提出。

[
https://github.com/bykof/hoard_chunker
](
https://github.com/bykof/hoard_chunker
)
    


### TITLE

这位Rustacean开发了一个名为refined_type的crate,旨在通过类型验证来提高程序的运行时安全性。这个crate支持数值类型的范围约束、满足特定条件的数组、非空字符串等特性。它还包含了一些功能,可以轻松地组合多个验证规则。

通过将refined_type与serde相结合,可以基于类型对JSON或YAML数据进行验证。作者提供了几个示例,展示了如何使用MinMax类型对数值范围进行约束,使用ForAll类型对集合中的每个元素应用特定规则,以及如何使用And、Or和Not组合多个规则来定义复杂类型。

该crate的GitHub仓库、文档和crates.io链接也一并提供。作者希望通过这个crate提高Rust程序的安全性,并邀请社区审视并提出改进建议。

[
https://old.reddit.com/r/rust/comments/1gil59d/tell_me_why_my_selfmade_crate_is_terrible_and/
](
https://old.reddit.com/r/rust/comments/1gil59d/tell_me_why_my_selfmade_crate_is_terrible_and/
)
    


### TITLE

根据帖子内容,该用户在尝试使用Rust交叉编译aarch64-unknown-linux-gnu和aarch64-unknown-linux-musl目标时遇到了编译错误。主要问题包括:

1. 编译过程中使用了Clang编译器而非预期的GCC,可能需要手动指定cc编译器。

2. 尽管musl-dev包含stddef.h头文件,但编译器仍然找不到该头文件,造成了致命错误。 

3. 目标aarch64-unknown-linux-gnu和aarch64-unknown-linux-musl已正确安装,但编译仍失败。

用户已经尝试了一些基本的排查方法,但最终原因仍不明确。可能需要进一步调查编译器配置、依赖库链接等问题才能解决该交叉编译错误。

[
https://old.reddit.com/r/rust/comments/1gj9the/crosscompile_aarch64unknownlinuxgnu_to/
](
https://old.reddit.com/r/rust/comments/1gj9the/crosscompile_aarch64unknownlinuxgnu_to/
)
    


### TITLE

总结如下:

Rust是一种相对较新的编程语言,旨在构建高效可靠的应用程序。对于编程新手来说,学习Rust是一个非常好的选择,原因如下:

1. Rust是一种直白的语言,没有复杂的语法,初学者可以很容易上手并自学。

2. Rust从一开始就培养良好的编码习惯,帮助初学者建立行业标准的编码行为。

3. Rust有深入的错误日志消息,可以清晰地解释错误并提供建议,非常友好。

4. Rust的编译器严格但富有支持性,像一位老师般指导你修正代码。

5. Rust没有手动内存管理,使用所有权系统自动分配内存,避免了复杂的内存管理。

6. Rust拥有一个支持友好的社区,初学者可以得到热心的帮助。

7. Rust结合了现有语言的优点,同时轻巧高效,是资深开发人员的利器。

因此,无论是编程新手还是有经验的开发者,学习Rust都是一个极佳的选择。

[
https://www.howtogeek.com/why-you-should-learn-rust/
](
https://www.howtogeek.com/why-you-should-learn-rust/
)
    


### TITLE

这是关于一个名为 RustGB 的 Rust crate(包)的介绍,它旨在为 GameBoy 游戏开发提供支持。作者两个月前提出了一个将 Rust 代码编译到 GameBoy ROM 的简单想法,但当时构建过程太过复杂,无法实际测试。在过去两个月的工作后,作者终于发布了 RustGB 的首个 alpha 版本。

该 crate 现已上传到 crates.io,文档可在 docs.rs 上查看,源代码发布在 GitHub 上。按照 docs.rs 上的简要说明,开发者就可以使用 Rust 创建简单的 GameBoy ROM 了!目前功能还很简单,但将来会逐步添加更多功能。作者希望大家能尽兴地使用该库,如有任何反馈也欢迎提出。该项目还包含了一个可爱的 GameBoy Ferris logo。

[
https://old.reddit.com/r/rust/comments/1giqx43/rustgb_a_crate_for_gameboy_development_with_rust/
](
https://old.reddit.com/r/rust/comments/1giqx43/rustgb_a_crate_for_gameboy_development_with_rust/
)
    


### TITLE

这段代码试图将一个枚举值 `Direction` 转换为 `f32` 类型的浮点数值。具体来说:

1. 定义了一个 `Direction` 枚举类型，包含 `Left` 和 `Right` 两个变量。

2. 为 `Direction` 实现了 `Into<f32>` trait，以便将其转换为浮点数。如果枚举值为 `Left`，返回 `1.0`；如果为 `Right`，返回 `-1.0`。

3. 在 `main` 函数中，创建一个 `Direction::Left` 的实例 `dir`。

4. 尝试将 `dir` 与 `10` 相乘得到 `res`，但编译器报错。

该代码无法通过编译的原因是 Rust 无法自动解析 `dir` 需要实现 `Into<f32>` trait。作者需要显式调用 `dir.into()` 将 `dir` 转换为 `f32` 类型，然后再与 `10` 相乘得到 `res`。

[
https://old.reddit.com/r/rust/comments/1gj22t4/why_cant_i_use_intof32_to_allow_multiplication_of/
](
https://old.reddit.com/r/rust/comments/1gj22t4/why_cant_i_use_intof32_to_allow_multiplication_of/
)
    


### TITLE

这位开发者编写了一个纯Rust命令行工具,它可以接受几个文件作为输入,并生成一个输出文件。由于同事不愿使用命令行,他希望将该工具打包成WebAssembly(wasm),以便通过一个简单的网页界面上传输入文件、执行计算并下载结果文件。

他希望找到一种最简单、最无痛的方式实现这一目标,具体要求包括:

1. 避免使用大型库和npm等JavaScript相关技术,因为他对JavaScript经验有限。

2. 计算需要在用户设备上执行,而非客户端-服务器架构。

3. 不需要精美的用户界面,只需一个可以勉强使用的简单网页。

4. 寻求最新的解决方案,因为一些六年前的信息可能已过时。

他希望获得一些指导,以确定实现该目标的最佳方向。

[
https://old.reddit.com/r/rust/comments/1gisk02/what_is_the_least_painful_way_to_ship_a_rust_tool/
](
https://old.reddit.com/r/rust/comments/1gisk02/what_is_the_least_painful_way_to_ship_a_rust_tool/
)
    


### TITLE

这篇文章介绍了作者最近为有C语言编程经验的人撰写的一本新的Rust入门书籍《Rust for C-Programmers》。该书目前仍在起步阶段,但已经成为一个简洁实用的Rust入门教程。作者利用AI工具对内容进行了多次优化,使解释更加精准、简洁,用词也更加清晰准确,方便非母语读者阅读理解。

书籍的内容和示例与其他Rust资源保持一致,包括官方的Rust书籍。作者正在人工校对和测试所有示例,以确保核心概念的准确性。该书面向有基本系统编程概念和C语言经验的读者,对于完全初学者,建议从官方的Rust书籍或其他入门资源开始学习。

该书的在线版本目前可在https://rust-for-c-programmers.salewskis.de/访问,未来网址可能会有变动。作者考虑过使用与书名对应的域名,但为避免与官方资源混淆和法律纠纷,最终选择了当前网址。

在未来几个月内,作者计划继续增加在线版本的内容,具体进度取决于公众的兴趣和其他因素。由于之前开源经历存在一些问题,源代码暂时不会公开,反馈可通过电子邮件的传统方式提交。最后,作者暂未确定是否会推出该书的印刷版,因为在线免费版本的需求应该不高。

[
https://old.reddit.com/r/rust/comments/1gjbdsi/new_online_rust_introduction_for_c_programmers/
](
https://old.reddit.com/r/rust/comments/1gjbdsi/new_online_rust_introduction_for_c_programmers/
)
    


### TITLE

这是一个总结:

这位开发者刚开始学习Rust语言2个月,之前有使用C++、Go、Python和Java的编程经验。他最近开发了一个名为hoard_chunker的项目,这是一个备份/恢复工具,可以遍历给定输入目录中的路径,并使用fastcdc算法将所有文件分块存储到指定的输出目录。它也可以从之前分块的文件中收集所有块,并将其恢复到指定的输出目录。

他希望来自有经验的Rust程序员的建设性反馈,特别是对于Rust语言中的所有权概念给他带来了一些困惑。他打算通过这个项目来提高自己的Rust编程能力。

[
https://old.reddit.com/r/rust/comments/1gj8msm/roast_me_i_build_a_backuprestore_tool_to_chunk/
](
https://old.reddit.com/r/rust/comments/1gj8msm/roast_me_i_build_a_backuprestore_tool_to_chunk/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

