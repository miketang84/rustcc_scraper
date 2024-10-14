【Rust日报】2024-10-11


### TITLE

这是一个使用 Tauri 和 native_db 创建的简单应用程序示例。关键点包括:

1. 定义了一个 Person 模型,有两个版本。
2. 使用 native_db 作为 Tauri 的托管状态。
3. 在应用程序设置期间迁移数据库。

要运行该示例,需要先安装 Rust、Cargo 和 Tauri CLI,然后克隆仓库并使用 `cargo tauri dev` 命令运行项目。这是一个非常简单的演示如何在 Tauri 应用中使用本地数据库。

[https://github.com/vincent-herlemont/native_db_tauri_vanilla
](https://github.com/vincent-herlemont/native_db_tauri_vanilla
)
    


### TITLE

以下是对给定GitHub链接内容的中文总结:

这是一个基准测试项目,比较了Native DB、Redb和SQLite在插入、获取、选择范围、使用辅助键和删除数据等操作的性能表现。结果显示:

1. 插入操作中,当每个事务只有一个操作时,SQLite比Native DB快9倍左右,Redb也比Native DB快4倍。但当每个事务包含多个操作时,Redb比Native DB快4倍,而SQLite比Native DB快3倍左右。

2. 获取单个记录时,Redb最快,比Native DB快1.7倍。SQLite在查找带有多个辅助键的记录时,比Native DB慢1.4-1.6倍。

3. 选择范围查询时,SQLite在简单查询中比Native DB快2倍左右,但随着辅助键数量增加,SQLite逐渐变慢。

4. 删除操作时,SQLite总体最快,在每个事务只有一个操作时比Native DB快7.7倍,当事务包含多个操作时,SQLite比Native DB快16倍以上。Redb在只删除单个键值对时也比Native DB快近5倍。

总的来说,SQLite在大多数场景下性能最佳,Redb次之,而Native DB由于一些开销而性能相对较差。该基准测试旨在发现Native DB的性能瓶颈,将来会对其进行进一步优化。

[
https://github.com/vincent-herlemont/native_db/tree/main/benches
](
https://github.com/vincent-herlemont/native_db/tree/main/benches
)
    


### TITLE

这个项目描述了 Pumpkin,它是一个完全使用Rust语言构建的Minecraft服务器。Pumpkin的目标是提供高性能、高效率、高度可定制化的游戏体验,同时遵循原版游戏的核心机制。

主要特点包括:

1. 利用多线程实现最大化的速度和效率。
2. 支持最新的Minecraft服务版本,遵循原版游戏机制。
3. 重视安全性,防止已知的漏洞。 
4. 高度可配置,可禁用不需要的功能。
5. 为插件开发提供基础。

此外,该项目不打算提供与原版或Bukkit服务器的兼容性、配置和插件,也不会作为从头构建服务器的框架。

当前仍处于重度开发阶段,正在实现的功能包括配置、服务器状态、登录、玩家设置、注册表、服务器品牌等等。该项目有详细的文档,欢迎贡献,也可加入Discord社区进行交流。作者对wiki.vg提供的有价值信息表示感谢。

[
https://github.com/Snowiiii/Pumpkin
](
https://github.com/Snowiiii/Pumpkin
)
    


### TITLE

这篇文章讨论了一项关于使用GitHub的AI编程助手Copilot对开发人员工作效率的影响的研究。研究由编码管理软件公司Uplevel进行,跟踪了800名开发人员在使用Copilot前后3个月的表现。结果发现,使用Copilot并没有显著提高开发效率,反而会导致代码中不小心引入41%更多的错误。

文章指出,这一发现并不令人惊讶,因为Copilot是基于大型语言模型,而后者往往容易产生虚构信息和不正确的数据。另一项研究也发现,大型语言模型会生成大量"虚构包"的代码,即引用了不存在的文件或代码。

一些科技领导人开始担心,使用AI生成的代码可能会带来更多工作,因为调试这些代码变得更加困难,有时重写代码比修复更容易。总的来说,这项研究质疑了AI辅助编程能够提高生产力的说法。

[
https://futurism.com/the-byte/ai-programming-assistants-code-error
](
https://futurism.com/the-byte/ai-programming-assistants-code-error
)
    


### TITLE

以下是TentHash规范v0.4的中文总结:

TentHash是一种新的哈希函数。它的哈希过程包括以下几个步骤:

1. 初始化4个64位无符号整数A、B、C、D作为内部哈希状态。

2. 将输入数据分成256位块,对每个块执行以下操作:
    - 将块的4个64位小端整数异或到A、B、C、D中
    - 对A、B、C、D进行混合(mix)操作

3. 哈希全部输入数据后,将输入长度(比特数)异或到A中。 

4. 再对A、B、C、D执行两次mix操作。

5. 取A、B、C的前160位作为最终的160位哈希值输出。

其中mix操作包括几轮位循环、异或和交换操作,用于混合哈希状态。

该文档还给出了几个测试向量,以验证实现的正确性。TentHash规范旨在简洁易读,以方便不同语言实现该哈希函数。该版本有可能在最终版本前作出修改。

[
https://github.com/cessen/tenthash/blob/main/docs/specification.md
](
https://github.com/cessen/tenthash/blob/main/docs/specification.md
)
    


### TITLE

这个存储库包含了一个名为TentHash的160位非加密哈希函数的规范和实现。TentHash旨在作为一种高质量、合理快速且输出较大的哈希函数,主要应用于数据指纹、内容可寻址系统等无法容忍哈希冲突的场景。它的设计理念是为了在不需要加密安全性的情况下,提供一个与加密哈希相当且更简单高效的实现。该存储库提供了TentHash的设计理念文档、与其他哈希函数的性能比较,以及Rust语言的实现。作者欢迎外界对设计的审计、在其他语言的实现等贡献。该项目采用MIT和Apache 2.0双重许可。

[
https://github.com/cessen/tenthash
](
https://github.com/cessen/tenthash
)
    


### TITLE

这个帖子讨论了Rust语言中引入某种形式的有限继承或特征实现委托的可能性。作者最初提到使用"继承"这个词可能不太恰当,并提出了"特征实现委托"这个更准确的说法。

作者描述了一个场景:假设有一个非常模块化的库,使用了数百个特征,作者想在现有实现的基础上,只对其中几个特征实现进行小幅修改。为了实现这一目标,作者不得不为新类型实现所有特征,即使大多数特征实现与原有类型完全相同,这导致了大量重复代码。

为了解决这个问题,作者建议引入一种新的语法,允许类型自动将除了显式重写的部分之外的所有特征实现委托给内部类型。他提供了一些伪代码示例,展示了这种语法可能的样子。

总的来说,这个建议旨在减少重复代码,提高代码的可维护性,同时保留Rust语言的安全性和高性能特征。

[
https://old.reddit.com/r/rust/comments/1g1oczx/i_think_rust_needs_some_sort_of_limited/
](
https://old.reddit.com/r/rust/comments/1g1oczx/i_think_rust_needs_some_sort_of_limited/
)
    


### TITLE

以下是总结:

作者宣布了Native DB版本0.8.0的发布,带来了重大的bug修复、新功能以及与SQLite的初步基准测试对比。虽然基准测试是以基本方式进行的,但作者计划进一步完善测试,并欢迎大家在issues中提供反馈。此外,一些issues被标记为"需要帮助",供有兴趣贡献的人参与。

另外,作者更新了使用Tauri V2的示例native_db_tauri_vanilla。

最后,作者祝大家周末愉快!

[
https://old.reddit.com/r/rust/comments/1g1x2a1/native_db_release_080_benchmarks_vs_sqlite_redb/
](
https://old.reddit.com/r/rust/comments/1g1x2a1/native_db_release_080_benchmarks_vs_sqlite_redb/
)
    


### TITLE

这是一个用于查询和操作HTML文档的Rust crate库DOM_QUERY的介绍。主要功能包括:

1. 解析HTML文档或片段到Document对象。

2. 使用类似jQuery语法通过CSS选择器查询和选择元素。 

3. 选择单个或多个匹配元素。

4. 访问元素的属性、文本内容、HTML等。

5. 修改元素的属性。

6. 序列化元素到HTML字符串。

7. 使用伪类选择器如:has、:has-text、:contains等高级查询。

8. 支持预编译CSS选择器以提高重用性能。

总的来说,DOM_QUERY是一个强大的工具,可以方便地用Rust操作HTML文档,类似于JavaScript中的jQuery库。它建立在html5ever和selectors等crate之上,提供了易用的API。

[
https://github.com/niklak/dom_query
](
https://github.com/niklak/dom_query
)
    


### TITLE

该帖子是一位开发者对C++管理外部库的困难感到非常沮丧。他原本打算使用C++进行机器学习和REST API开发,但发现管理依赖库(如使用CMake)的过程非常痛苦,耗费了大量时间去学习如何使用这些工具,而不是真正学习C++本身和进行项目开发。

由于C++和Rust在性能方面相近,而Rust安装外部库的过程更简单,因此该开发者考虑转而学习Rust。不过,他也表示有些可惜,因为他原本期待学习C++来进行一些其他领域的开发,如Godot游戏引擎。

总的来说,这位开发者由于对C++管理依赖库的痛苦经历,正在考虑放弃C++转而学习Rust,因为后者在处理库方面更加简单直观。

[
https://old.reddit.com/r/rust/comments/1g1pguq/managing_libs_for_c_is_just_painful_should_i/
](
https://old.reddit.com/r/rust/comments/1g1pguq/managing_libs_for_c_is_just_painful_should_i/
)
    


### TITLE

AMD正在将开源Root of Trust(RoT)计划"Project Caliptra"整合到其2026年及以后的产品线中,以加强产品的安全性和可靠性。Project Caliptra是一个行业内开放计算项目(OCP),旨在建立标准化、安全的硬件安全基础,促进整个行业的协作与创新。AMD作为技术咨询委员会成员和协创者之一,与微软、谷歌、英伟达等公司合作推进这一计划。

整合Project Caliptra将为AMD带来以下关键好处:

1. 透明度与协作 - 开源基础增强透明度,促进信任与持续改进;社区驱动有助保持领先技术。

2. 一致性与可靠性 - 标准化安全框架使产品整合和维护更简单;统一的RoT为整合了该技术的解决方案提供更佳安全选择。

3. 增强安全性 - 强大的保护机制有助保护敏感数据和维护系统完整性;开源性质允许快速更新创新,应对新出现的威胁。

AMD坚持交付确保最终用户获得安全性、透明度和一致性的解决方案。从2026年开始将Project Caliptra整合到产品中,标志着AMD为满足客户关于产品安全性和可信度的需求迈出了重要一步。这一开放式RoT由业界领导者共同打造,确保未来产品将处于安全技术的最前沿。

[
https://community.amd.com/t5/corporate/addressing-security-integrating-project-caliptra-into-amd-s/ba-p/716837
](
https://community.amd.com/t5/corporate/addressing-security-integrating-project-caliptra-into-amd-s/ba-p/716837
)
    


### TITLE

根据Reddit帖子,该用户正在使用Rust语言开发一个游戏服务器(Pumpkin)。他非常重视安全性,希望防止任何崩溃或漏洞的发生。因此,他在询问是否有任何良好的概念或技术可以用来加固和确保游戏服务器的安全性。

总的来说,这位开发者在开发Rust编写的游戏服务器时,非常关注安全问题,希望获得专业建议来提高服务器的安全性和可靠性。

[
https://old.reddit.com/r/rust/comments/1g28siy/how_i_can_hardening_my_server/
](
https://old.reddit.com/r/rust/comments/1g28siy/how_i_can_hardening_my_server/
)
    


### TITLE

这是一个用Rust语言实现的 AT 协议的开源项目。AT协议是一种分布式社交网络协议。该项目的主要特点包括:

1. 遵循 AT 协议规范,提供加密、身份验证、数据存储等功能。

2. 完全开源,使用 Apache 2.0 许可证。 

3. 支持巴勒斯坦人权利,附有相关徽标。

4. 提供依赖管理状态徽章。

5. 包含项目的 README 文件、许可证文件、Git 忽略文件等。

6. 分为多个子模块,如crypto、feedgen、firehose、identity等。

7. 项目处于活跃开发中,提交记录超过300次。

8. 鼓励关注官方的 Bluesky 社交媒体账号以了解更多信息。

总的来说,这是一个值得关注的开源 AT 协议实现项目,具有活跃的开发、社会意识以及遵循最新规范等特点。

[
https://github.com/blacksky-algorithms/rsky
](
https://github.com/blacksky-algorithms/rsky
)
    


### TITLE

该帖子讨论了一些开发人员对于像 Copilot 这样的人工智能工具的过度信任。一项最新报告发现,使用此类工具会导致代码中的 bug 数量增加 41%。作者并不反对使用人工智能工具,认为它们在正确使用的情况下可以节省开发人员的时间,但我们不能完全信赖它们生成正确的代码。这就是为什么像 Rust 这样的强类型系统仍然很重要,它可以结合单元测试等手段来检查程序的正确性。最后,作者征求读者对此问题的看法。

[
https://old.reddit.com/r/rust/comments/1g1vwxj/the_curse_of_ai_and_how_rust_helps/
](
https://old.reddit.com/r/rust/comments/1g1vwxj/the_curse_of_ai_and_how_rust_helps/
)
    


### TITLE

这位用户提出了一些对Rust语言的期望和建议。主要包括以下几点:

1. 希望能够在类型上实现宏作为函数或方法,并能访问Self/self。在实现宏时,可以指定它是接收self、&self、&mut self还是作为普通函数。

2. 期望宏能够更加强大,并解决目前存在的一些限制。比如在宏中嵌套使用其他宏时,定义分支(branches)存在一些障碍(原文中提到但没有详细说明)。

3. 对Reddit的评分机制表示不解,认为明明有人希望就此展开讨论,但帖子却没有获得足够的关注度(upvotes)。

总的来说,这位用户希望Rust的宏系统能够更加灵活和强大,并提出了自己的一些构想,同时也对社区的一些运作模式表示疑惑。

[
https://old.reddit.com/r/rust/comments/1g21n2o/what_feature_would_you_like_added_to_rust/
](
https://old.reddit.com/r/rust/comments/1g21n2o/what_feature_would_you_like_added_to_rust/
)
    


### TITLE

这是一篇关于一种新开发的非加密散列函数TentHash的介绍。与用于哈希映射的散列函数不同,TentHash主要针对数据指纹和内容可寻址系统等场景,不能容忍冲突。作者解释了开发新散列函数的原因,即现有的大尺寸散列函数缺乏足够的可靠性证明。TentHash旨在实现简单、适用于数据指纹的大尺寸散列。

该散列函数主要由Rust语言实现,但也有相应的规范,可在任何语言中实现。作者计划在数月后,如果没有发现问题,就将该规范确定为最终版本。总的来说,这是一个新开发的专注于数据指纹等场景的非加密大尺寸散列函数。

[
https://old.reddit.com/r/rust/comments/1g27l73/tenthash_a_simple_portable_largesize_hash_for/
](
https://old.reddit.com/r/rust/comments/1g27l73/tenthash_a_simple_portable_largesize_hash_for/
)
    


### TITLE

以下是对该文章的中文总结:

Deno 2.0版本正式发布,这是一个现代化、全能的JavaScript和TypeScript开发工具链。它与Node.js和npm向后兼容,允许您无缝运行现有Node应用程序。同时也支持直接导入npm包,无需package.json和node_modules文件夹。

Deno 2.0新增了几个重要命令:deno install可以超快速度安装依赖,deno add和deno remove可以添加/移除依赖。它支持私有npm注册表、工作区和monorepor、以及长期支持(LTS)版本。

Deno标准库现已稳定,涵盖了数据操作、网络等多个领域的模块。Deno还推出了一个新的JavaScript注册库JSR,支持发布TypeScript源码模块、自动生成文档等。

Deno 2.0为企业级团队提供了可靠的生产环境支持,包括LTS版本和Deno企业版服务。总的来说,它旨在简化Web编程、提高开发效率,并与现有JavaScript基础设施实现无缝互操作。

[
https://deno.com/blog/v2.0
](
https://deno.com/blog/v2.0
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

