【Rust日报】2024-11-23


### TITLE

Embive是一个低级沙箱化库,专注于在受约束的环境中嵌入不受信任的代码。它通过解释RISC-V字节码来支持多种语言(Rust、C、C++、Zig、TinyGo等)。它默认不需要外部crate、动态内存分配或标准库(no_std和no_alloc)。Embive被设计为任何执行期间的错误都是可恢复的,允许主机根据需要处理它。

该库目前支持RV32I[M]非特权指令集(默认启用M扩展)。它提供了运行在Embive中的Rust、C/C++程序的模板示例。Embive的计划包括完全支持RV32G指令集、系统调用、资源限制器、CI/CD集成、字节码优化(AOT和JIT)、回调等功能。

Embive遵循Apache 2.0或MIT许可证。所有有意提交的贡献都将获得这两种许可,除非贡献者另有明确声明。

[https://github.com/embive/embive
](https://github.com/embive/embive
)
    


### TITLE

这是一个基于经典Quake III Arena虚拟机(Q3VM)的轻量级可嵌入式字节码解释器/虚拟机。它包含一个完整的C语言编译器(LCC)来生成.qvm字节码文件。该解释器只有一个C文件(vm.c),体积小巧,可嵌入到任何项目中。它支持在沙箱环境中运行不完全可信的代码,用于游戏引擎模组、IoT应用插件等。项目提供了独立的q3vm.exe演示程序,以及把解释器集成到自己项目的示例代码。通过实现4个回调函数,可以让字节码调用宿主程序的本地函数。该虚拟机经过20年Quake III引擎的实践检验,性能优于其他类似解释器。除了解释器,项目还包含了用于生成字节码的LCC编译器和q3asm链接器。

[
https://github.com/jnz/q3vm
](
https://github.com/jnz/q3vm
)
    


### TITLE

这是关于2024年圣诞编码挑战赛的介绍。这个由Shuttle发起的活动灵感来自于Advent of Code,邀请参与者使用Rust语言在一个轻松有趣的氛围中解决各种挑战。在每个挑战中,你需要实现HTTP端点来返回挑战的解决方案。这是一个不错的机会开始学习Rust和Shuttle!参与者将有机会提高Rust技能,结识志同道合的朋友,并享受CountDown圣诞的乐趣。第一个挑战将于2024年12月2日12:00 UTC发布,一共会在12月份发布7个挑战。注册和热身挑战将在11月底开放。如果你在2024年12月31日23:59 UTC之前完成所有7个挑战,就有资格获得奖品池,所以无需匆忙赶工。更多细节将陆续公布。现在就可以点击链接注册了。

[
https://shuttle.rs/cch
](
https://shuttle.rs/cch
)
    


### TITLE

总结如下:

作者在使用Deno FFI加载Rust库到JavaScript时遇到了一些问题,希望能找到一种改善开发体验的方法。目前的解决方案是在Rust端使用一些不太友好的C接口,在JavaScript端通过FFI调用这些C接口,整个流程非常笨拙且可能难以维护,尤其是由于生命周期等问题经常出现一些奇怪的问题。作者希望能找到一个好的库来简化这个过程,让Rust库能以更友好的方式集成到JavaScript中。之前有一个deno-bindgen库看起来不错,但似乎已被遗弃。因此作者在寻求改进建议。

[
https://old.reddit.com/r/rust/comments/1gxyrah/making_deno_ffi_easy/
](
https://old.reddit.com/r/rust/comments/1gxyrah/making_deno_ffi_easy/
)
    


### TITLE

这是一个关于 Embive 的介绍,Embive 是一个用于 RISC-V 架构的低级别沙箱库。它最初是作为 WebAssembly 的一个无标准库和无分配器的替代品,主要针对微控制器。该项目受到了 Q3VM 的启发,但不受语言/工具链的限制,因为它使用 RISC-V 字节码,可以利用大多数标准编译器对其的支持。你可以在 GitHub 仓库中查看其路线图,并找到用于开发 C/C++ 和 Rust 程序在 Embive 中运行的模板。该项目目前仍处于初期阶段,但作者已经有信心向公众发布。目前支持的所有指令都使用官方 RISC-V 测试套件进行测试。

[
https://old.reddit.com/r/rust/comments/1gxm414/embive_a_lowlevel_sandboxing_library_for_riscv/
](
https://old.reddit.com/r/rust/comments/1gxm414/embive_a_lowlevel_sandboxing_library_for_riscv/
)
    


### TITLE

这是一位大学三年级学生的求助。由于长时间盯着屏幕,眼睛开始疲劳,他希望能减轻这种症状。同时,他也想加深对Rust编程语言的了解。他已经读过在线的Rust书籍,现在寻求适合中高级程序员的实体书籍推荐,越难越好,他喜欢挑战。

此外,他目前正在学习系统开发(暂时使用C语言因为学校课程要求)。他知道有Rust操作系统教程,但貌似只有在线版本。如果有人能推荐一些低层次编程方面的书籍,那就再好不过了,不过其他主题的书他也乐意接受推荐。

[
https://old.reddit.com/r/rust/comments/1gxxqh1/advanced_physical_rust_book/
](
https://old.reddit.com/r/rust/comments/1gxxqh1/advanced_physical_rust_book/
)
    


### TITLE

该帖子讨论了如何在 Rust 中使用 mlua crate 实现 Lua 的 luaL_newmetatable 函数。luaL_newmetatable 函数用于创建一个新的元表(metatable)供 Lua 中的 userdata 使用。

作者提出了一个可能的等效实现方式:

```rust
let userdata_table = lua.create_table()?;
userdata_table.set("__name", "tname");
```

但他对以下几点表示疑虚:

1. 该实现是否能真正等效于 luaL_newmetatable 函数,特别是在将值推送到栈上这一部分。
2. 如何检查注册表(registry)中是否已经存在给定的键,因为 mlua 的 named_registry_key 函数需要提供类型,而 luaL_newmetatable 似乎可以不考虑类型就检查注册表中的键。

总的来说,该帖子在寻求 mlua 用户对于在 Rust 中模拟 Lua 的 luaL_newmetatable 函数的建议和反馈。

[
https://old.reddit.com/r/rust/comments/1gyapv0/how_can_i_implement_lual_newmetatable_in_rust/
](
https://old.reddit.com/r/rust/comments/1gyapv0/how_can_i_implement_lual_newmetatable_in_rust/
)
    


### TITLE

总结:

在这个Reddit帖子中,原帖者提到他尝试在线程之间通过通道发送Future,但目前无法正常工作,因为DST(动态大小类型)在线程之间是不安全的。如果async fn有一个固定大小的原始类型,那么就可以将其用作通道的泛型类型,从而实现在线程间传递Future。简而言之,当前Rust中缺乏对线程安全的async fn原始类型的支持,这使得在线程间传递Future变得困难。

[
https://old.reddit.com/r/rust/comments/1gyddc7/async_fn_primitive_type/
](
https://old.reddit.com/r/rust/comments/1gyddc7/async_fn_primitive_type/
)
    


### TITLE

该帖子询问了在Rust语言中使用何种技术栈来实现类似于Go语言框架PocketBase的功能,即将服务器和SQLite数据库打包到一个单一的可执行文件中,方便部署和托管。作者表示自己有两年没写过Rust代码了,看到有很多选择但不知道哪些是活跃的、生产就绪的。

简而言之,这是一个关于在Rust中开发便携式后端服务器程序的技术选型问题,需要一个能与SQLite集成、打包为单一可执行文件的Rust Web框架。

[
https://old.reddit.com/r/rust/comments/1gy35hz/recommended_frameworks_for_a_server_sqlite/
](
https://old.reddit.com/r/rust/comments/1gy35hz/recommended_frameworks_for_a_server_sqlite/
)
    


### TITLE

这个存储库是FastEmbed的Rust实现,提供了快速的文本嵌入、图像嵌入和候选项重新排序功能。它具有以下主要特性:

1. 支持同步使用,无需依赖Tokio。
2. 使用@pykeio/ort进行高性能的ONNX推理。
3. 使用@huggingface/tokenizers进行快速编码。
4. 支持使用@rayon-rs/rayon进行批量嵌入生成和并行计算。
5. 默认模型是Flag Embedding,在MTEB排行榜上排名靠前。
6. 提供多种文本嵌入模型、图像嵌入模型和重新排序模型可供选择。
7. 易于安装和使用,提供了使用示例。
8. 采用Apache 2.0许可证。

该库的快速特性来自于使用量化模型权重、ONNX运行时推理引擎以及避免了对Hugging Face Transformers的隐藏依赖。总的来说,这是一个高性能的Rust库,提供了文本、图像嵌入和重新排序等功能。

[
https://github.com/Anush008/fastembed-rs
](
https://github.com/Anush008/fastembed-rs
)
    


### TITLE

Shuttle公司将在2024年再次举办圣诞代码挑战赛。这个活动灵感来自Advent of Code,参与者将使用Rust语言在一个轻松的环境中解决编程挑战。每个挑战都需要实现一个HTTP端点,返回挑战的解决方案。完成所有挑战的用户还将获得奖金。点击https://shuttle.dev/cch可以获得更多信息和申请方式。Shuttle公司期待广大开发者和朋友们的积极参与。

[
https://old.reddit.com/r/rust/comments/1gy17jx/shuttle_christmas_code_hunt_2024_aocstyle_rust/
](
https://old.reddit.com/r/rust/comments/1gy17jx/shuttle_christmas_code_hunt_2024_aocstyle_rust/
)
    


### TITLE

这个帖子讨论了在Rust编程语言中使用Arc和clone来快速迭代和原型开发的可行性。作者提到,当人们抱怨"在Rust中无法快速迭代或原型开发"时,常常会收到"只需使用Arc和clone"这样的建议。

然而,作者在学习Rust的生命周期章节时感到有些困难,但由于是出于兴趣爱好在学习,因此可以耐心地学习。作者的直觉是,这种在学习初期"绕过生命周期"的做法只是延迟了对语言的深入理解,但作者不确定这是否是Rust开发者通常的做法。

总的来说,这个帖子在探讨在学习Rust语言初期是否可以先采用一些"捷径"来加快原型开发的步伐,还是应该坚持从一开始就深入学习语言的特性,如生命周期等概念。

[
https://old.reddit.com/r/rust/comments/1gy4n8i/how_feasible_is_the_just_use_arc_and_clone/
](
https://old.reddit.com/r/rust/comments/1gy4n8i/how_feasible_is_the_just_use_arc_and_clone/
)
    


### TITLE

这是一个来自Rust语言GitHub仓库的Pull Request,目的是为Rust 1.85版本(预计于2025年2月20日发布)稳定2024年版本。该PR由ehuss提交,并获得了rust-lang项目的审查和批准。一些主要内容包括:

1. 稳定2024版本,这是Rust的一个重要里程碑。

2. 获得了rust项目团队的大力支持和欢呼,认为这是成果令人振奋。

3. 经过团队的审查,该PR被批准合并。

4. 合并后,会继续后续工作,如更新文档、Cargo、rustfmt等。

5. 该PR也修改了一些测试用例,需要相关人员注意。

总的来说,这是Rust新年版本发布的一个关键步骤,标志着2024版本正式进入发布流程,为下一个发布版本的到来做准备。

[
https://github.com/rust-lang/rust/pull/133349
](
https://github.com/rust-lang/rust/pull/133349
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

