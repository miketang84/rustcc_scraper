【Rust日报】2024-09-21


### TITLE

这是一个名为waylock的Rust项目的GitHub仓库。该仓库目前处于开发初期阶段,README文件中只写有"WIP"(Work In Progress,进行中的工作)三个字母。仓库中包含了一些Rust相关的文件,如Cargo.toml、Cargo.lock等,还有一个assets文件夹和src源代码文件夹。最新的提交记录显示做了一些清理工作,如删除下一步按钮。总的来说,这是一个刚启动的Rust项目,当前进展未知,处于较早的开发阶段。

[
https://github.com/waycrate/waylock
](
https://github.com/waycrate/waylock
)
    


### TITLE

这是一个名为Twenty的开源项目,旨在帮助用户每20分钟远离屏幕20秒钟,放松眼睛。它利用waycrate/exwlshelleventloop/iced_sessionlock来锁定屏幕。

用户可以通过Cargo或baker编译并安装该程序。使用twenty --init [light/dark]初始化,默认暗色模式。twenty --kill杀死程序进程。sudo bake uninstall卸载程序。

该项目由rv178和shivkr6共同创作,旨在通过强制实施20-20-20规则来保护用户的眼睛健康。它主要针对实现了ext-session-lock的合成器。

[
https://github.com/waycrate/twenty
](
https://github.com/waycrate/twenty
)
    


### TITLE

这是一个用Rust编程语言编写的类似KDE的图形桌面环境程序,名为Lala bar。它是一个图层shell程序,包含了启动器、侧边栏和通知守护进程。

通知守护进程支持内嵌回复和默认操作。未来计划添加通知超时功能。

该程序使用了iced_layershell库,是作者waycrate的另一个项目。附有一张显示程序侧边栏界面的图片示例。

[
https://github.com/waycrate/lala-bar
](
https://github.com/waycrate/lala-bar
)
    


### TITLE

这个仓库包含了5个子项目,旨在为Wayland下的layer shell和session lock提供更易于使用的事件循环和iced绑定。

1. waycrate_xkbkeycode: 处理xkbcommon事件,参考了winit。
2. layershellev: 类似winit的layer shell事件处理库,用于构建虚拟键盘等程序。
3. sessionlockev: 类似winit的session lock事件处理库,用于锁屏解锁功能。
4. iced-layershell: iced的layer shell绑定,支持打开新层和弹出窗口,可用于构建KDE shell、通知等应用。
5. iced-sessionlock: iced的session lock绑定,可用于构建锁屏程序,并将与pam集成。

该项目需要熟悉iced的开发者协助,因为iced 0.13版本将有重大更改,且目前还未支持文本输入v3和输入法功能。

[
https://github.com/waycrate/exwlshelleventloop
](
https://github.com/waycrate/exwlshelleventloop
)
    


### TITLE

mocks是一个用于快速搭建模拟REST API服务的工具,主要有以下特点:

1. 可以通过命令行快速启动一个基于JSON文件的模拟REST API服务器。
2. 支持自动生成常见的CRUD接口,包括GET、POST、PUT、PATCH、DELETE等。
3. 支持基于JSON文件热更新API响应数据。
4. 支持调试模式,可将服务器接收到的请求数据持久化存储到文件中,方便调试。
5. 提供健康检查端点,方便监控服务运行状态。
6. 可通过Homebrew或Cargo进行安装。
7. 开源项目,采用MIT许可协议。

总的来说,mocks为开发和测试场景下快速创建模拟数据服务提供了极大的便利,可以有效提升开发效率。

[
https://github.com/mocks-rs/mocks
](
https://github.com/mocks-rs/mocks
)
    


### TITLE

这是一个名为json-server的GitHub仓库的自述文件总结。json-server是一个Node.js库,允许你快速搭建一个模拟REST API。主要功能包括:

1. 通过创建一个db.json或db.json5文件,为API提供数据。

2. 启动一个本地服务器,可以对这些数据进行GET、POST、PUT、PATCH和DELETE等REST操作。

3. 支持过滤、分页、排序、嵌套资源等高级查询。

4. 可以直接托管静态文件。

5. 提供命令行选项对服务器进行配置。

该项目使用Fair Source许可证,要求拥有3个以上用户的组织需赞助才能使用。总的来说,json-server是一个方便的工具,可以快速模拟一个REST API服务器,适用于前端开发、学习和原型制作等场景。

[
https://github.com/typicode/json-server
](
https://github.com/typicode/json-server
)
    


### TITLE

这个存储库是一个练习集合,旨在学习Rust语言并构建生产级别的命令行工具。它包含240多个练习,涵盖了从基础知识(如变量、控制流)到高级主题(如异步编程和不安全的Rust)的各种Rust概念。练习分为四个主要部分:基础Rust、中级Rust、高级Rust和项目。

每个部分都包含多个主题,每个主题都是一个Cargo工作空间。每个主题又包含10个单独的练习。练习的目的是教会如何构建高效可扩展的生产就绪命令行应用程序。

该存储库的组织结构使得用户可以方便地找到和导航到想要练习的特定主题和练习。它还包含一个快速链接文件,其中包含指向所有练习解决方案的直接链接。

此外,该存储库鼓励贡献和反馈,并提供了一些指导方针。总的来说,这是一个全面的Rust实践项目,适合初学者和有经验的开发人员学习Rust并获取构建生产级CLI工具的实践经验。

[
https://github.com/fnabinash/rust-practice
](
https://github.com/fnabinash/rust-practice
)
    


### TITLE

这是一个用于合并不同来源的BibTeX文件的Python工具,名为bibpy。它的工作原理是将不同格式的BibTeX文件放在data/input/文件夹下的不同子文件夹中。程序会解析这些文件,并将合并后的BibTeX文件输出到data/output/文件夹中,每个来源对应一个输出文件。

使用方法很简单:
1. 将BibTeX文件按来源分别放入data/input/文件夹下不同的子文件夹中。
2. 克隆该仓库并进入bibpy目录。
3. 运行`make install-py`安装依赖库。  
4. 运行`make merge`来合并BibTeX文件。

该工具已在Ubuntu 22.04.1 LTS上使用Python 3.12.3进行了测试,可以方便地合并多个来源的参考文献数据。

[
https://github.com/arthurazs/bibpy
](
https://github.com/arthurazs/bibpy
)
    


### TITLE

以下是对BibTeX的总结:

BibTeX是一种文献引用管理格式和处理程序,广泛用于LaTeX文档编写系统中引用文献资料。它将参考文献信息与格式呈现分离,使用户能够以一致的方式引用来源。

BibTeX工作原理是从.aux、.bst样式文件和.bib文献数据库文件中读取数据,生成格式化的.bbl文件,LaTeX再从中插入引用和参考文献列表。

BibTeX由Oren Patashnik和Leslie Lamport于1985年创建,编写于WEB/Pascal语言。它经历了多次小版本更新,2010年发布0.99d版本后继续更新。

此外还存在一些BibTeX的重新实现版本,如支持UTF-8的BibTeXu、多语种的MLBibTeX,以及完整重写的BibLaTeX等,提供更多功能特性。

BibTeX使用.bib格式的纯文本文件存储参考文献条目信息,每条目对应一种文献类型,通过字段描述标题、作者、出版信息等元数据。运行BibTeX可将引用的条目按指定格式风格输出格式化参考文献列表。

[
https://en.wikipedia.org/wiki/BibTeX
](
https://en.wikipedia.org/wiki/BibTeX
)
    


### TITLE

这位开发者正在使用Rust语言将JSON数据批量插入PostgreSQL数据库。他创建了一个包含多个JSON对象的向量,然后使用循环逐个将JSON对象插入到数据库的markets表中。不过,这种逐个插入的方式效率较低。他希望能够通过单个SQL语句一次性将整个向量插入数据库,以提高插入效率。他目前遇到的问题是如何将向量值作为参数传递给单个查询语句。他在代码中展示了创建JSON向量、定义数据表以及使用循环插入数据的示例代码。他希望有人能提供一种更高效的批量插入JSON数据到PostgreSQL的方法。

[
https://old.reddit.com/r/rust/comments/1fktop2/bulk_inserting_json_into_postgresql_using/
](
https://old.reddit.com/r/rust/comments/1fktop2/bulk_inserting_json_into_postgresql_using/
)
    


### TITLE

这是一个名为AirNope的Telegram机器人,旨在识别和删除加密空投垃圾信息,并移除发送该垃圾信息的用户。它不会在群组中发送任何消息,也不会保存任何消息或用户历史记录。用户可以通过网页、Telegram群组或终端来测试哪些消息会被识别为垃圾信息。AirNope还提供了隐私政策,不会处理或存储任何个人身份信息。用户可以通过Docker轻松运行自己的AirNope实例。该项目遵循GPL-3.0许可证。

[
https://github.com/cuducos/airnope
](
https://github.com/cuducos/airnope
)
    


### TITLE

这是一个关于Rust编程语言的Wayland平台绑定库exwlshelleventloop的更新。该库为Iced GUI库提供了Wayland平台的支持,特别是针对层壳(layershell)和会话锁定(sessionlock)功能。

更新内容包括:

1. exwlshelleventloop现在跟上了Iced最新版本。

2. iced_layershell现在支持多个层壳,可用于创建类似通知守护进程的应用程序,比如lala-bar项目。

3. iced_sessionlock可用于在Wayland平台上创建锁屏程序,如twenty项目,以及正在开发中的waylock版本。

该更新发布在v0.7.0-beta1版本,相关代码可在GitHub上获取。

[
https://old.reddit.com/r/rust/comments/1fklutx/iced_layershell_iced_sessionshell_has_publish/
](
https://old.reddit.com/r/rust/comments/1fklutx/iced_layershell_iced_sessionshell_has_publish/
)
    


### TITLE

这位开发者在Rust中使用SQLite时遇到了一些问题。他编写了一些涉及SQL操作的方法,并希望为这些代码编写一些测试用例。但是,他不希望测试用例影响正常的逻辑,因此封装了一个`with_connection`方法。

该方法的作用是在非测试环境下,从连接池中获取一个数据库连接,并在获取连接成功后执行传入的操作。而在测试环境下,该方法目前是空的,开发者希望能够返回一个内存中的SQLite连接,以避免影响正常数据。

开发者最初的想法是使用`sqlite::Connection::open_in_memory()`来创建一个内存中的数据库,用于后续的操作。但这需要在每次调用时获取相同的SQLite实例。作为初学者,他不确定是否有更好的解决方案,并希望能得到帮助。

[
https://old.reddit.com/r/rust/comments/1fl2qjb/some_questions_about_sqlite_and_test/
](
https://old.reddit.com/r/rust/comments/1fl2qjb/some_questions_about_sqlite_and_test/
)
    


### TITLE

这篇文章讨论了在 Rust 和 C++ 之间进行互操作的一种技巧。主要内容如下:

1. alice.write_comment(..) 和 RUST_write_comment(alice, ..) 这两种调用方式在底层实际上是等价的,C++ 编译器会将前者转换为后者。所以 Rust 函数只是在模拟 C++ 编译器的行为。

2. 我们依赖于 API 的兼容性,而不是 ABI 的兼容性。Rust 实现可以自由读写 C++ 类的私有成员,因为公有/私有只是 C++ 编译器实施的规则,而 CPU 不关心这些规则,只看内存字节。

3. 我们必须使用繁琐的类型转换,因为我们确实在将一种类型 (User) 的内存重新解释为另一种类型 (UserC)。这在 C++ 标准布局类中是允许的,否则会导致不确定行为。

总的来说,这种技巧利用了 Rust 和 C++ 之间的 API 兼容性,绕过了 ABI 的限制,从而实现了在两种语言之间自由访问对方的数据结构。但同时也带来了一些低级别的不便,比如需要手动进行类型转换。

[
https://gaultier.github.io/blog/rust_c++_interop_trick.html
](
https://gaultier.github.io/blog/rust_c++_interop_trick.html
)
    


### TITLE

这是一个介绍Rust编写的命令行工具mocks的内容。该工具的灵感来自于npm的json-server包,用于启动模拟的REST API服务器,旨在为测试和开发提供方便。作者刚刚发布了mocks的代码仓库,希望能获得反馈。如果有任何疑问或建议,可以在GitHub或通过仓库中提供的电子邮件与作者联系。仓库链接为https://github.com/mocks-rs/mocks。

[
https://old.reddit.com/r/rust/comments/1fkout1/mocks_get_a_mock_rest_apis_with_zero_coding/
](
https://old.reddit.com/r/rust/comments/1fkout1/mocks_get_a_mock_rest_apis_with_zero_coding/
)
    


### TITLE

这是一个简单的Bash脚本,旨在在ARM系统上运行Rust编译的可执行文件进行交叉测试。它首先要求用户在运行前安装qemu-user、qemu-user-static和jq包,这些包用于模拟ARM环境和解析JSON输出。

脚本利用cargo test命令生成针对armv7-unknown-linux-gnueabihf目标的测试可执行文件列表。它使用jq工具从JSON输出中提取每个可执行文件的路径。

然后,对于每个可执行文件,脚本使用qemu-arm-static命令在模拟的ARM环境中运行该可执行文件,从而在主机系统上执行交叉编译的Rust测试。qemu-arm-static使用-L参数指定ARM系统库的位置。

总的来说,这个脚本简化了在非ARM主机上测试针对ARM目标交叉编译的Rust代码的过程。

[
https://old.reddit.com/r/rust/comments/1fl1gk2/a_simple_script_for_running_crosscompiled_rust/
](
https://old.reddit.com/r/rust/comments/1fl1gk2/a_simple_script_for_running_crosscompiled_rust/
)
    


### TITLE

这是一份关于 Rust 练习分支的发布公告。该分支旨在为各个级别的开发者提供 Rust 练习题目,让他们可以自主练习和掌握 Rust。

主要特点包括:

1. 可以轻松 Fork 仓库并切换到 practice 分支开始做练习题。

2. 可以直接在仓库中解决练习题,并随时追踪学习进度。 

3. 无论你是 Rust 新手还是有经验的开发者,练习题都可以帮助提高 Rust 技能。

4. 不设截止日期,你可以自主安排学习进程。

开始使用的步骤是:

1. Fork 该仓库
2. 切换到 practice 分支
3. 开始解决提供的练习题目
4. 提交进度来追踪学习过程

该分支旨在帮助开发者掌握 Rust,祝大家 Rust 学习之旅愉快!

[
https://old.reddit.com/r/rust/comments/1fkgrnu/practice_v010_release/
](
https://old.reddit.com/r/rust/comments/1fkgrnu/practice_v010_release/
)
    


### TITLE

总结如下:

1. chili是一个Rust语言库,用于高效并行计算。它受到Spice库的启发,提供了类似于rayon::join的低开销并行原语。在计算过程中的任何分支点,它都可以并行执行两个给定的闭包。

2. chili在计算量小、难以估计当前分支剩余计算量的情况下表现最佳。作者举了一个并行求二叉树节点值之和的例子说明了chili的用法。

3. 文中提供了在AMD Ryzen 7 4800HS和Apple M1处理器上对不同规模二叉树进行求和的基准测试结果。结果显示,随着节点数量的增加,chili相比于单线程基准和Rayon库都有明显的加速效果。

4. 作者还测试了在AMD Ryzen 7上chili在小规模(1K节点)时的开销,发现开销随线程数的增加保持大致不变。

5. 另一个Reddit帖子里,有人想知道如何在Rust中高效读取100GB以上的大文件。作者表示尽管知道可以分块读取,但仍想尝试一次性读取这么大的文件,以此作为一个挑战。

[
https://github.com/dragostis/chili
](
https://github.com/dragostis/chili
)
    


### TITLE

这是一个用Rust语言编写的纯文本界面扑克游戏应用程序。作者使用有限状态机(FSM)的设计模式来实现扑克游戏的逻辑,游戏数据结构是一个泛型,被包裹在一个枚举中以控制状态转换。

作者选择使用mio库来进行低级别的网络编程,实现了基于非阻塞套接字的轮询,避免了使用Web和异步框架。文本用户界面使用Ratatui库实现,游戏命令解析使用clap库。

总的来说,虽然作者最初使用Rust语言有些不习惯,但后来对其内置工具和严格性产生了赞赏,这使得开发过程更加自信。作者希望获得一些建议,如项目组织结构的优化以及是否有被忽视的Rust语言特性可以改进库和应用程序的设计。

[
https://old.reddit.com/r/rust/comments/1fkzjkf/poker_over_ssh/
](
https://old.reddit.com/r/rust/comments/1fkzjkf/poker_over_ssh/
)
    


### TITLE

总结如下:

作者是一名有十多年Python编程经验的程序员,最近尝试使用Rust重写了一个用于读取BibTeX文件并创建内存对象的Python代码。但是,当比较两个项目的运行时间时,Rust代码所需时间是Python代码的两倍。

作者希望通过这个帖子获得改进Rust代码的建议,同时也希望获得更好编写"解析"工具的技巧。他提供了Python和Rust两个版本的代码链接,并指出了代码中的一些关键函数,如next_entry、get_category、get_key等。

最后,作者给出了一个BibTeX文件示例,供读者参考。通过使用BufReader,作者已经将Rust代码的平均运行时间从0.938毫秒降低到0.073毫秒,快于Python版本,但他仍在寻求进一步改进建议。

[
https://old.reddit.com/r/rust/comments/1fkl97n/my_python_code_is_faster_than_my_rust_code_what/
](
https://old.reddit.com/r/rust/comments/1fkl97n/my_python_code_is_faster_than_my_rust_code_what/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

