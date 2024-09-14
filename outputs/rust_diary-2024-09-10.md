【Rust日报】2024-09-10


### TITLE

以下是对该新闻的中文总结:

Alice I. Cecile是Bevy游戏引擎的一名全职开源开发者。从业余爱好到正式工作,她分享了自己在Bevy基金会工作的几个月经历。起初她尝试遵循正常的9-5工作制,但发现这让她感到沮丧。于是她采用了专注于单一任务的灵活工作方式,同时平衡工作与生活。

尽管这是理想的工作,但也存在一些缺陷,比如公开的工作环境、工作生活界限模糊、依赖捐助者资助等。不过,Alice强调持续学习对于她的工作至关重要,需要熟悉各个领域的知识。

她介绍了"工作组"的概念,让贡献者能够自我组织、解决复杂问题。在技术层面,她计划在未来一年改进Bevy的UI解决方案,使其更易用。总的来说,尽管存在挑战,但能为开源社区和游戏开发做出贡献是非常有意义的。

[https://bevyengine.org/news/dream-job/
](https://bevyengine.org/news/dream-job/
)
    


### TITLE

这是一个名为Pax的工具的Beta版本发布公告。Pax是一个用于构建原生应用程序和网站的工具,类似于SwiftUI或Flutter。它使用一种声明式的用户界面描述语言,可以与Rust应用程序逻辑相结合。

Pax内置了一个矢量设计工具,可以双向查看和编辑Pax代码库:在设计师中打开代码库,进行可视化更改,或使用任何代码编辑器手动编辑pax-lang或Rust代码,并在可视化模式和书面模式之间来回切换。

与此Beta版本的发布一起,Pax设计师(Pax的集成矢量设计工具)被开源,它本身就是用Pax构建的,是Pax在生产中的一个很好的参考示例。

接下来,他们正在开发Pax设计师的一个功能更加完善的托管版本,名为Pax Pro,这将成为一个团队协作工具,使非开发人员能够与开发人员并肩为GitHub存储库做出可视化贡献。他们还在开发Pax JavaScript,这将允许pax-lang与JavaScript/TypeScript应用程序逻辑相结合,作为Rust的替代方案。

该公告邀请用户尝试Pax,构建一些东西,并提供反馈,因为未来的功能和修复将取决于用户反馈。尽管Pax今天在Beta版本中还远远不完美,但他们为Pax走过的路感到自豪,并对其未来发展方向感到兴奋。

[
https://old.reddit.com/r/rust/comments/1fdmjzl/pax_enters_beta_rust_guis_with_an_integrated/
](
https://old.reddit.com/r/rust/comments/1fdmjzl/pax_enters_beta_rust_guis_with_an_integrated/
)
    


### TITLE

这篇博文主要介绍了 Cloudflare 如何通过优化一个简单的函数来节省 1% 的 CPU 利用率。该函数的职责是在请求离开 Cloudflare 基础设施之前,移除一些内部使用的请求头。由于这个函数处于非常热门的执行路径,占用了 1.7% 的总 CPU 时间,因此对它进行优化可以带来明显的性能提升。

优化过程包括:

1. 通过反转查找方向来减少读取次数,将函数执行时间从 3.65 微秒降低到 1.53 微秒。

2. 使用不同的数据结构存储内部请求头名称,例如哈希表、有序集、正则表达式等,并对它们进行基准测试。

3. 最终自行实现了一种基于 Trie 树的高效数据结构 trie-hard,将函数执行时间降低到 0.93 微秒。

4. 在生产环境中部署 trie-hard,实现将 CPU 利用率降低 1.28% 的目标。

该项目展示了通过持续优化和基准测试,即使是一个简单的函数,也可以为大规模分布式系统带来明显的性能提升。Cloudflare 还将 trie-hard 开源,方便其他人使用和改进。

[
https://blog.cloudflare.com/pingora-saving-compute-1-percent-at-a-time/
](
https://blog.cloudflare.com/pingora-saving-compute-1-percent-at-a-time/
)
    


### TITLE

这篇文章介绍了使用Rust语言开发一个Game Boy模拟器的第一步 - 内存管理。主要内容包括:

1. Rust中的包(Package)、箱(Crate)和模块(Module)的概念,以及它们的层级关系。

2. Game Boy的内存映射情况,包括不同区域内存的用途。 

3. 在Rust中使用结构体(struct)和实现(impl)关键字创建RAM模块,定义了一个65536字节的u8类型数组来模拟Game Boy的内存。

4. 实现了RAM模块的读取(read)功能,通过地址(u16)返回该位置的字节值(u8)。

5. 解释了Rust中实例方法需要使用&self作为第一个参数的原因和用途。

6. 介绍了Rust中无符号整数u8、u16等的概念。

总的来说,这是作者探索使用Rust语言开发Game Boy模拟器的第一篇文章,着重介绍了Rust的模块化编程和内存管理的实现。

[
https://medium.com/@wolferxy/rust-adventure-to-develop-a-game-boy-emulator-part-1-memory-3ea6e29c254c
](
https://medium.com/@wolferxy/rust-adventure-to-develop-a-game-boy-emulator-part-1-memory-3ea6e29c254c
)
    


### TITLE

这篇文章介绍了 Wasmtime 和 Cranelift 对栈映射(stack map)基础设施的改进。栈映射用于垃圾收集,它描述了在安全点(safepoint)上活跃对象在栈帧中的位置,以避免使用后错误。

文章解释了旧的栈映射实现方式存在一些缺陷和复杂性,例如需要在编译器的大部分流程中追踪垃圾收集引用、可能导致未优化或错误优化等。

新的栈映射实现将栈映射生成从寄存器分配移动到了早期的中间代码生成阶段。这简化了编译器,使中期能够正确地优化代码,并避免对垃圾收集引用做不必要的区分。该改进还允许在 64 位系统上使用 32 位的垃圾收集引用,提高了密集性。

总的来说,这个改进降低了 Wasmtime 和 Cranelift 的复杂性,改善了优化效果,并为进一步的垃圾收集支持做好了准备。

[
https://bytecodealliance.org/articles/new-stack-maps-for-wasmtime
](
https://bytecodealliance.org/articles/new-stack-maps-for-wasmtime
)
    


### TITLE

总结如下:

这位开发者在长时间没有专注于个人项目后,最近终于抽出时间更新了他用Rust编写的贪吃蛇游戏。最新的更新虽然没有完全改变游戏的核心,但需要对源代码进行大量修改。主要变化是实现了游戏元素的缩放以及窗口大小调整的支持,同时还添加了一些新的难度等级,使游戏更具可玩性和趣味性。

他希望能够获得大家的反馈和建议,以便进一步改进游戏的设计和Rust代码的质量,特别是在最佳实践和代码优化方面。文中提供了该游戏的GitHub仓库链接和一张游戏截图。

[
https://old.reddit.com/r/rust/comments/1fdjurp/updated_my_rust_snake_game/
](
https://old.reddit.com/r/rust/comments/1fdjurp/updated_my_rust_snake_game/
)
    


### TITLE

这是一位开发者和他的朋友用Rust语言从头开发了一个高性能的Minecraft服务器实现,名为Ferrumc。该服务器完全支持多线程,当前可以加入世界并四处游走。它针对1.20.1版本的Minecraft,加载16个方向上的区块,内存占用只有10-14MB,比原版Minecraft服务器节省了大量内存。虽然目前功能还不完整,但它是一个有前景的项目,开发者希望能够获得反馈。他们提供了GitHub仓库链接和Discord社区链接,以便大家了解和参与。

[
https://old.reddit.com/r/rust/comments/1fcvyjb/ferrumc_an_actually_fast_minecraft_server/
](
https://old.reddit.com/r/rust/comments/1fcvyjb/ferrumc_an_actually_fast_minecraft_server/
)
    


### TITLE

这个链接指向 GitHub 上一个名为 posixutils-rs 的存储库,它是 Rust Core Utils 项目的一部分。具体来说,这个链接展示了该项目中与 AWK 相关的部分。

AWK 是一种用于文本处理的编程语言,在 POSIX 命令行工具中很常见。这个 Rust 项目旨在用 Rust 语言重新实现 POSIX 命令行工具,包括 AWK。

该链接页面展示了该项目中 AWK 部分的源代码文件。通过查看这些源代码,可以了解该 AWK 实现的细节和功能实现方式。作为命令行工具的重新实现,该项目力求与标准 AWK 行为保持兼容。

总的来说,这是一个使用 Rust 语言重新实现命令行工具 AWK 的项目的源代码页面。

[
https://github.com/rustcoreutils/posixutils-rs/tree/main/awk
](
https://github.com/rustcoreutils/posixutils-rs/tree/main/awk
)
    


### TITLE

总结:

在学习Rust语言的过程中,作者尽量避免使用宏,目的是更深入地了解Rust语言的内部工作原理。他质疑这种学习方法是否合理,或者是否给自己增加了不必要的困难。

作者的这种学习方式可以说是"刻意练习"的一种形式。通过暂时避免使用宏这样的高级语言特性,可以迫使自己深入研究Rust的基本语法和编程范式,从而加深对语言本身的理解。但同时,这可能会增加一些编码工作量,让学习过程变得更加艰难。

总的来说,这种学习方法是否合理需要根据个人情况而定。对于一些初学者来说,先掌握基础语法可能更为重要。而对于有经验的程序员,则可以尝试这种"刻意练习"的方式,加深对新语言的理解。

[
https://old.reddit.com/r/rust/comments/1fdljdj/i_am_avoiding_macros_while_learning_rust/
](
https://old.reddit.com/r/rust/comments/1fdljdj/i_am_avoiding_macros_while_learning_rust/
)
    


### TITLE

这是一位来自费城的经验丰富的工程师,他对 Rust 语言感兴趣,希望在当地建立一个 Rust 语言学习小组。他希望通过这个小组来分享 Rust 的使用案例,并在当地推广这门语言。他正在寻求一些建议:

1. 是否先在线上建立一个小群组,然后再尝试线下见面?

2. 适合多少人领导这样一个小组?他希望能找到一个合作伙伴共同创办(如果你在费城的话,欢迎与他联系)。

3. 如何权衡针对新手和老手的活动主题?如何在入门学习和高级话题之间取得平衡?

总的来说,他希望能从已经创办过类似小组的人那里获得一些有用的建议和经验。

[
https://old.reddit.com/r/rust/comments/1fdquhy/any_tips_for_starting_a_local_usermeetup_group/
](
https://old.reddit.com/r/rust/comments/1fdquhy/any_tips_for_starting_a_local_usermeetup_group/
)
    


### TITLE

这位新手Rust程序员希望找到一种集中和自动加载单独的.env文件来覆盖基础.env文件中定义的值的方法,以在命令行运行单元测试和集成测试时使用。他列出了三种可能的方法:

1. 在项目根目录下有.env和.env.test文件,后者在cargo test时自动覆盖前者的值。

2. 在cargo test时使用 --config-override参数指定一个.env.test文件路径。

3. 在单个.env文件中使用子标题声明不同环境的覆盖值,并在cargo test时传入 --config-override标识符。

他希望找到Rust中类似的解决方案,还想知道如何集中覆盖不同环境(如staging、production等)的环境变量。

[
https://old.reddit.com/r/rust/comments/1fdnm2f/in_rust_how_to_centrally_load_and_override/
](
https://old.reddit.com/r/rust/comments/1fdnm2f/in_rust_how_to_centrally_load_and_override/
)
    


### TITLE

这是关于Pax项目的总结。Pax是一个由两部分组成的工具:1)一个矢量设计工具,称为Pax Designer;2)一个跨平台的用户界面引擎,称为Pax Engine。

Pax Designer是一个矢量设计工具和可视化构建器,可以读取和编写用户界面定义代码。Pax Engine是一个跨平台的本地应用程序和网站构建引擎,使用Rust语言开发。

Pax的主要特点包括:集成的可视化构建器、跨平台的本地构建支持、热模块重载、响应式布局引擎、可复用组件库、健壮的文本渲染、动画引擎、表达式语言和轻量级footprint。

该项目目前处于Beta阶段,可以构建实际应用,但可能存在一些粗糙边缘和漏洞。当前的发展重点包括托管版Pax Designer、JavaScript绑定、反馈响应和功能改进。

Pax旨在使软件创作更具创意性,也更容易被人类所掌握。它被设计为"可设计性",在视觉矢量设计和代码之间架起了一座桥梁。Pax还可以与AI紧密结合,让人类和AI一起设计和编码用户界面。该项目遵循MIT和Apache 2.0开源许可证。

[
https://github.com/paxdotdev/pax
](
https://github.com/paxdotdev/pax
)
    


### TITLE

这个仓库包含一个用Rust编写的简单贪吃蛇游戏。该游戏支持墙壁碰撞、自我碰撞和吃食物长蛇身。游戏顶部会记录分数,并且会跟踪每局的最高分。游戏有一个主菜单用于开始游戏,在发生碰撞后也会回到主菜单。游戏共有4个不同难度级别,前3个难度通过改变蛇的速度实现,最后一个特殊难度会根据当前分数动态改变蛇的速度。

该项目最初发布了v0.1.1版本,包括基本的游戏功能。v1.0.0版本增加了主菜单、高分记录等功能,并为macOS和Windows提供了发布包。v1.2.0版本支持调整窗口大小,新增4个游戏难度模式,但高分数据的持久化保存仍在实现中。该游戏使用了GGEZ游戏开发框架。

[
https://github.com/Feromond/rust_snake_game
](
https://github.com/Feromond/rust_snake_game
)
    


### TITLE

该内容介绍了 FerrumC 这个由 Rust 语言从头编写的 Minecraft 服务器实现。FerrumC 充分利用了 Rust 语言的多线程能力,提供高性能和出色的内存效率。它的主要特性包括:

- 可定制的服务器列表
- 极快的更新速度和适应能力
- 高效的内存使用
- 可定制的配置
- 可导入现有的 Minecraft 世界
- 与原版 Minecraft 客户端兼容(目前只支持1.20.1版本)
- 强大的实体组件系统(ECS)来处理大量实体
- 完全多线程化,利用所有可用CPU内核
- 自定义网络和NBT编码系统,最小化I/O延迟
- 高速数据库确保极快的世界加载速度(目前使用RocksDB)
- 支持32个渲染距离

未来还将添加更多功能,如查看其他玩家、修改世界、聊天和命令系统、优化、插件支持等。

该项目目前还在开发中,不适合用于生产环境。用户可以从源码编译来试用。文档提供了从源码编译和运行的步骤说明。该项目欢迎新的贡献,提供了贡献流程的指引。最后,介绍了该项目与其他 Minecraft 服务器实现项目的区别和一些常见问题的解答。

[
https://github.com/sweattypalms/ferrumc
](
https://github.com/sweattypalms/ferrumc
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

