【Rust日报】2024-11-25


### TITLE

这是一个命令行界面 (CLI) 工具,用于从官方笑话 API 获取随机笑话。它的主要功能包括:

1. 通过运行 `joke` 命令获取一个随机笑话。
2. 首次运行时,它会从官方笑话 API 下载所有笑话。
3. 定期运行 `joke --update` 命令可以更新笑话列表。
4. 运行 `joke --help` 可以查看其他选项。
5. 它可以与 `cowsay` 工具结合使用,为终端增添乐趣。例如 `joke -c programming | cowsay -f tux` 会以企鹅形象输出一个关于编程的笑话。

该仓库提供了从源代码构建和安装该工具的说明,并使用 GPL-3.0 许可证。总的来说,它是一个有趣的命令行工具,可以在终端获取笑话,为日常工作增添一些乐趣。

[https://github.com/cool-mist/joke-cli
](https://github.com/cool-mist/joke-cli
)
    


### TITLE

RustedSciThe是一个Rust库,用于符号和数值计算。它可以解析字符串表达式为符号表达式/符号函数,计算符号导数或将符号表达式转换为常规Rust函数。它还可以计算符号Jacobian矩阵,并使用BDF和Backward Euler方法求解刚性常微分方程组的初值问题,使用Newton迭代法求解非刚性常微分方程和边值问题。

该库的主要特性包括:

1. 解析字符串表达式为符号表达式/函数
2. 对符号表达式/函数进行符号/解析微分
3. 比较解析导数和数值导数
4. 计算偏导数向量
5. 将符号表达式/函数(包括导数)转化为常规Rust函数
6. 计算符号/解析Jacobian矩阵并转换为函数形式
7. 使用解析Jacobian矩阵的Newton-Raphson方法、Backward Euler方法和BDF方法求解刚性常微分方程组
8. 使用RK45和DP等经典方法求解非刚性常微分方程
9. 使用Newton-Raphson方法求解常微分方程的边值问题

该库可用于解析多变量字符串表达式、微分、"lambdify"(转换为Rust函数)、比较解析和数值导数、求解常微分方程初值问题和边值问题等。

[
https://github.com/Gleb-Zaslavsky/RustedSciThe
](
https://github.com/Gleb-Zaslavsky/RustedSciThe
)
    


### TITLE

这是一个用Rust编写的化学热力学、化学动力学、反应器模拟、燃烧模拟、冲击波管和火箭发动机模拟等领域的软件包。它包含了以下主要功能:

1. 解析反应方程式并将其转换为物质列表。
2. 解析反应方程式并生成化学计量矩阵、正反应系数矩阵、反应级数矩阵等。
3. 内置了从公开数据库解析获得的大量化学反应动力学参数库。可以查看所有反应库、按物质搜索反应等。
4. 自动生成化学反应机理,根据输入的物质和反应库,找出所有该物质及其产物之间可能的反应。
5. 计算分子式对应的原子组成和摩尔质量。
6. 提供了单元测试,可以运行cargo test进行测试。
7. 欢迎对该项目提出问题、评论或贡献。
8. 未来计划添加更多反应库、物质库、数值计算方法等。

总的来说,这是一个综合性的化学过程模拟与分析工具包。

[
https://github.com/Gleb-Zaslavsky/KiThe
](
https://github.com/Gleb-Zaslavsky/KiThe
)
    


### TITLE

这个版本发布说明总结了sled 0.2.0版本的主要变更内容:

1. 为支持no_std系统的错误处理,最低支持Rust版本提升到1.81。

2. 移除了之前已废弃的Scheduler.change_hz()方法,改用Scheduler.set_hz()。

3. 移除了Sled::new_from_string(String)和Config::from_string(String),改用Sled::new_from_str(&str)和Config::from_str(&str)。

4. 增加了对大部分no_std环境的支持,引入了CustomDriver和CustomScheduler等特性。

5. 增加了libm特性标记以支持需要libm的no_std环境。

6. 示例代码从主代码库中剥离到spatial_led_examples库中。

7. 改进了文档说明。

8. 新贡献者@claudiomattera的第一次贡献。

总的来说,这个版本主要增强了对no_std环境的支持,移除了一些废弃接口,同时对代码库进行了重构优化。

[
https://github.com/DavJCosby/sled/releases/tag/0.2.0
](
https://github.com/DavJCosby/sled/releases/tag/0.2.0
)
    


### TITLE

这是一个关于在 ESP32-C3 上使用 Embassy 的示例。作者在 GitHub 上提供了一个通用示例,演示了如何使用异步编程来与 I²C 传感器、SPI E-Ink 显示器和 WiFi 时间服务器进行通信。该示例采用 no_std 和无 ESP-IDF 的方式,使用了最新的箱库版本(除了 reqwless)。作者对现有示例感到失望,尤其是因为 esp-hal 及相关库interface在每个版本中都会发生变化,导致示例代码很快就过时了。该作品旨在提供一个更加稳定和现代化的 ESP32-C3 编程示例。

[
https://old.reddit.com/u/claudiomattera
](
https://old.reddit.com/u/claudiomattera
)
    


### TITLE

这是一个介绍 Rust 编程语言库 Sled 的文章摘录。Sled 是一个用于创建空间 LED 灯条灯光效果的库。

文中介绍了几个 Sled 库的主要方法:

1. `set_at_dist()`方法可以设置距离中心某个距离处的所有 LED 为指定颜色。

2. `set_at_dist_from()`方法可以相对于任何其他点设置某个距离处 LED 的颜色。

3. `set_at_angle()`和`set_at_dir()`方法通过与射线的线段相交测试来设置 LED 颜色。

该库的工作原理是对每个线段与圆形半径进行相交测试,然后在相交点处着色 LED。这种简单的线段和几何形状相交检测方法使得创建各种空间灯光效果成为可能。

[
https://davjcosby.github.io/all-published/miscellaneous-tech/Introducing%20Sled,%20a%20Rust%20Library%20for%20Creating%20Spatial%20LED%20Strip%20Lighting%20Effects.html
](
https://davjcosby.github.io/all-published/miscellaneous-tech/Introducing%20Sled,%20a%20Rust%20Library%20for%20Creating%20Spatial%20LED%20Strip%20Lighting%20Effects.html
)
    


### TITLE

这段内容展示了如何使用Typer工具在命令行中运行Python应用程序。主要步骤包括:

1. 首先尝试运行 `typer main.py run` 时出现缺少必需参数NAME的错误提示。

2. 使用 `--help` 选项查看命令的使用帮助,发现需要传入NAME参数。

3. 接着运行 `typer main.py run Camila`,正确传入NAME参数,应用程序输出"Hello Camila"。

4. 最后一行提示成功运行该应用程序。

总的来说,这个例子向用户演示了如何使用Typer这个命令行工具运行一个简单的Python应用程序,并介绍了基本的命令参数使用方式。

[
https://typer.tiangolo.com/
](
https://typer.tiangolo.com/
)
    


### TITLE

总结:

该Reddit帖子的作者开始学习Rust语言已有一年时间。最近,他终于创建了一个"有用"的Rust工具,每天都会在他的工作流程中运行。这个工具名为joke-cli(https://github.com/cool-mist/joke-cli),它会在shell启动时运行,为用户提供一个笑话。作者已经将该工具添加到了自己的shell启动脚本中,每次启动shell时它都会执行并显示一个新的笑话。这是作者在学习Rust过程中创建的第一个真正"有用"的工具,对他的日常工作流程有一定帮助。

[
https://old.reddit.com/r/rust/comments/1gzooib/i_made_a_tool_in_rust_that_i_now_use_everyday/
](
https://old.reddit.com/r/rust/comments/1gzooib/i_made_a_tool_in_rust_that_i_now_use_everyday/
)
    


### TITLE

这篇文章介绍了如何实现一个简单但高效的异步 "oneshot" 通道。oneshot 通道是一种只能发送和接收一个值的通道,常见于异步请求-响应系统中。文章解释了为什么需要 oneshot 通道,描述了其应满足的要求,并逐步实现了一个真实的、无依赖的 async oneshot 通道库。

文章阐述了 oneshot 通道需要满足的几个主要要求:
1. 可靠地发送至多一个值
2. 接收至多一个值
3. 如果所有发送端句柄被丢弃,告知接收端值永远不会到达
4. 接收值的操作应该是异步非阻塞的

接着文章提供了一个初步的实现思路,并解释了为什么需要使用原子操作和条件变量等同步原语。最后,作者逐步优化了实现,提高了性能,并发布到了 crates.io 上供大家使用。总的来说,这篇文章全面地介绍了 oneshot 通道的概念、需求和实现细节,有助于理解更复杂的异步原语。

[
https://amit.prasad.me/blog/async-oneshot
](
https://amit.prasad.me/blog/async-oneshot
)
    


### TITLE

这是一篇介绍两个用Rust语言开发的科学计算相关crate的帖子。

第一个crate是KiThe,主要涉及化学反应动力学和热力学计算,包括解析化学反应方程、组建反应机理、计算物质组成和摩尔质量等功能。作者计划在未来添加纯化学动力学、平衡组分以及一维常微分和偏微分方程组求解等功能。

第二个crate是RustedSciThe,针对求解包含反应、传热传质的刚性(stiff)系统方程组。它包含计算机代数系统,可以对符号函数进行操作如求解析雅可比矩阵;实现了多种刚性常微分方程组算法;借鉴Cantera软件思路实现了一维边值问题求解,并作了多方面改进;还有一些其他实用功能。

作者认为Rust在科学计算领域有广阔的发展前景,欢迎感兴趣的人加入或提出对该项目的意见和建议。

[
https://old.reddit.com/r/rust/comments/1gzqzas/chemical_reactors_combustion_kinetics/
](
https://old.reddit.com/r/rust/comments/1gzqzas/chemical_reactors_combustion_kinetics/
)
    


### TITLE

这是一个关于在Rust语言中重建《我的世界》(Minecraft)游戏的想法。作者提出了一个想法,即只是将原版的Java版《我的世界》移植到Rust语言,以获得更好的速度和着色器支持。他正在研究Bevy游戏引擎的文档,虽然这并不是一件简单的事情,但感觉通过一些外部帮助是可以实现的。作者邀请有兴趣的人加入这个项目,共同合作开发一个Rust版的《我的世界》。即使不想合作,作者也希望能获得一些对这个想法的反馈意见。

[
https://old.reddit.com/r/rust/comments/1gzg6gv/how_about_minecraft_rebuilt_in_rust/
](
https://old.reddit.com/r/rust/comments/1gzg6gv/how_about_minecraft_rebuilt_in_rust/
)
    


### TITLE

本文总结了Ralf的研究小组刚刚在OOPSLA会议上发表的一篇论文。该论文介绍了一种名为Rustlantis的差分随机测试工具,用于模糊测试Rust编译器的优化和代码生成。

Rustlantis通过随机生成MIR程序,并确保它们在不同的后端、优化级别和Miri模拟器下表现一致来发现编译器中的bugs。这项工作的核心部分是由Andy(Qian Wang)在他的硕士论文中完成的。尽管Andy后来已经有了正式工作,但他仍然在继续改进这项工作,最终成为了一篇很棒的论文。

总共发现了22个新的Rust编译器bugs,其中12个是在经过大量模糊测试的LLVM后端中发现的。文中提供了论文链接和Andy的演讲视频链接,供读者了解更多细节。

[
https://www.ralfj.de/blog/2024/11/25/rustlantis.html
](
https://www.ralfj.de/blog/2024/11/25/rustlantis.html
)
    


### TITLE

这个Rust crate imply_hack旨在通过添加Imply作为一个父trait来为你的trait添加隐含的边界条件。它解决了这个问题:假设你有一个trait MyTrait<T>需要T实现某个trait Bound,而另一个trait FooUser<T>又依赖于MyTrait<T>,那么在使用FooUser<T>的时候,就需要重复写T: Bound这个边界条件。imply_hack利用Rust 1.79新增的对关联类型的隐含边界条件支持,通过一些trait的巧妙设计,让你只需要声明trait MyTrait<T>: Imply<T, Is: Bound>,就自动获得了T: Bound的边界条件约束,避免了重复编写。它简化了这种场景下的代码编写,提高了代码的可读性和维护性。

[
https://docs.rs/imply-hack
](
https://docs.rs/imply-hack
)
    


### TITLE

这是一篇介绍 Rust 新数据库查询库 rust-query 的博文。主要内容包括:

1. 作者认为现有的 Rust 数据库交互方式存在缺陷,因此开发了 rust-query 库。

2. rust-query 深度集成了 Rust 的类型系统,提供了类型安全的关系数据库查询。

3. 该库的主要特性包括:显式表别名、空安全性、直观聚合函数、类型安全外键导航、类型安全唯一查找、多版本模式、类型安全迁移、类型安全唯一冲突处理、行引用与事务生命周期绑定、封装的类型化行 ID 等。

4. 博文展示了如何使用 rust-query 定义模式、插入和查询数据的示例代码。

5. rust-query 旨在提供一种安全、类型友好的方式来与关系数据库交互,无需手写 SQL 查询。

总的来说,这是对一个新的 Rust 数据库查询库 rust-query 及其主要特性和设计理念的介绍。

[
https://blog.lucasholten.com/rust-query-announcement/
](
https://blog.lucasholten.com/rust-query-announcement/
)
    


### TITLE

这是一个关于Rust编程语言的帖子,介绍了一个名为spatial_led的crate(库)。该crate允许以一种全新的方式管理LED灯条,不再是通过索引设置LED颜色,而是将房间的形状映射到2D空间中,然后可以通过方向、距离等查询LED。

两周前,作者分享了这个新发布的crate,并收到了很多积极反馈。最显著的是,有人提交了一个Pull Request,为crate添加了no_std支持,这将使spatial_led能够为更多用户所用。该版本引入了这一支持,以及一些小的改进。

作者希望将这个库推广给那些会从中获得乐趣的人,因为他们会提供最好的反馈。如果有人知道一些在线社区会对这个项目感兴趣,作者很乐意了解。最后,作者提供了crate的链接和0.2.0版本的更新日志。

[
https://old.reddit.com/r/rust/comments/1gzx6kf/spatial_led_020_a_crate_for_creating_super_cool/
](
https://old.reddit.com/r/rust/comments/1gzx6kf/spatial_led_020_a_crate_for_creating_super_cool/
)
    


### TITLE

这篇文章介绍了如何使用Rust GPU优化矩阵乘法kernel。主要内容包括:

1. Rust GPU项目概览及工作原理介绍。它允许使用Rust编程语言编写GPU程序(kernel),并将其编译为SPIR-V格式,可与Vulkan等GPU API集成。

2. 使用Rust GPU和wgpu库来实现矩阵乘法kernel,wgpu可跨平台运行。

3. 介绍了GPU程序的基本概念,如线程、工作组等。

4. 实现了三个版本的矩阵乘法kernel,从最初的朴素版本,到利用工作组共享内存的优化版本,再到充分利用GPU向量指令的高度优化版本。

5. 展示了使用Rust GPU的一些独特优势,如与CPU代码无缝集成、跨平台支持、丰富的Rust生态等。

6. 总的来说,该文全面地演示了如何使用Rust GPU编写高性能的GPU kernel,并探讨了与其他GPU编程方式的区别。

[
https://rust-gpu.github.io/blog/optimizing-matmul
](
https://rust-gpu.github.io/blog/optimizing-matmul
)
    


### TITLE

这篇文章讨论了在大型互动直播活动期间,用户认证和登录服务可能面临的挑战。文中介绍了芬兰广播公司 Yle 的 ID 团队如何利用 AWS Lambda 和 Rust 编程语言来提高登录性能。

主要内容包括:

1. 密码不能明文存储,需要通过密码哈希算法存储。这种算法设计复杂,防止暴力破解,但也意味着每次登录时都需要重新计算密码哈希,对 CPU 有较高要求。

2. 在正常情况下计算密码哈希不成问题,但在大型活动期间会出现大量用户同时登录的高峰,导致后端服务器 CPU 资源耗尽,无法响应请求。

3. 简单的解决方案是增加更多服务器资源,但往往无法预测峰值流量,会造成资源浪费。

4. Yle ID 团队使用 AWS Lambda 无服务器架构和 Rust 编程语言来编写高性能的密码哈希计算函数,提高了登录的吞吐量和可伸缩性,有效解决了这一问题。

总的来说,文章分享了在高并发场景下保证用户认证服务高性能的技术细节和实践经验。

[
https://yle.fi/aihe/a/20-10008009
](
https://yle.fi/aihe/a/20-10008009
)
    


### TITLE

这位程序员开始使用Rust语言时,曾抱怨Rust在处理C语言字符串字面量方面不太友好,需要借助第三方crate来解决。但最近他发现,在Rust 1.77版本中,C字符串字面量得到了支持,而且通过引入&CStr类型,使用起来也变得更加实用方便。这使得他能够移除曾经使用的const-cstr crate,该crate已停止维护且有安全风险警告。总的来说,这一改进大大简化和增强了Rust与C语言交互的能力,使FFI代码的开发变得更加简单和安全。这位程序员由衷感谢Rust团队引入了这个极佳的新功能。

[
https://old.reddit.com/r/rust/comments/1gzol4f/c_string_literals_are_awesome/
](
https://old.reddit.com/r/rust/comments/1gzol4f/c_string_literals_are_awesome/
)
    


### TITLE

这篇博文介绍了在Rust语言中创建一种特殊类型 Undroppable<T>，它无法被drop掉。通过在它的Drop trait实现中使用 assert(!mem::needs_drop::<T>(), "This cannot be dropped.")，如果试图drop这种类型的值，就会触发编译期错误。

作者解释说，虽然当前的编译错误信息不太理想,但在某些场景下,编译期错误比运行时panic更可取。他还提到这种方法可能会在 zerocopy 库中使用,用于消除 Unalign<T> 布局工具中存在的 T: Sized bound 限制,同时确保它不会无声无息地忘记具有非平凡 Drop 实现的类型。

最后,作者提供了自己的电子邮件和Twitter账号,以便读者留言和纠正。

[
https://jack.wrenn.fyi/blog/undroppable/
](
https://jack.wrenn.fyi/blog/undroppable/
)
    


### TITLE

这是一个Rust程序员分享了他最近创建的一个名为terse_cli的Rust宏,旨在简化命令行界面(CLI)程序的开发。在Python世界中,有一个名为typer的流行库,可以使用装饰器将任何带类型的函数转换为CLI命令。但是,作者认为Rust中流行的Clap库的派生语法需要编写过多的struct/enum结构,因此他创建了一个宏,可以根据函数参数和类型自动生成Clap派生语法。

他提供了一个示例程序,演示了如何使用该宏来定义两个命令add和greet,以及如何运行结果CLI程序。该程序生成的CLI界面显示了命令的用法、可用的子命令和选项。最后,作者邀请大家尝试使用该宏,提供反馈意见,并欢迎贡献。

[
https://old.reddit.com/r/rust/comments/1gzxvur/i_built_a_macro_that_lets_you_write_cli_apps_with/
](
https://old.reddit.com/r/rust/comments/1gzxvur/i_built_a_macro_that_lets_you_write_cli_apps_with/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

