【Rust日报】2024-11-15


### TITLE

该代码片段是一个复杂的数学表达式,包含大量的三角函数、算术运算、指数函数等组合。其中广泛使用了Mix函数、嵌套函数调用以及大量的数值常量。这种高度复杂的表达式可能是某种生成随机艺术图形或随机纹理的代码,通过将X、Y等变量代入不同的数学函数中获得具有一定规律但看似随机的值。由于缺乏上下文,无法确定其具体用途,不过这种极端复杂的编码通常出现在一些程序化艺术、可视化或仿真领域。

[https://github.com/samarthsushi/randomart
](https://github.com/samarthsushi/randomart
)
    


### TITLE

以下是对相关内容的总结:

这是一个编程humor社区的帖子,标题是"缓存失效和命名事物"。帖子包含一个代码片段,演示了如何记录网页图片加载的性能指标。下面的评论则出现了一些幽默内容。

一位用户推广了一个名为Adof的项目,宣称它能够提供管理、共享和同步配置文件的"最终解决方案"。这个项目名听起来类似"The Final Solution",在二战期间纳粹对犹太人所采取的种族屠杀行动,引起了一些争议性评论。

另一位用户分享了Adof项目的GitHub链接和之前在Rust版块的一个帖子链接。

总的来说,这个帖子包含了一些编程相关的幽默内容,也引发了一些有争议的评论,体现了程序员群体的幽默感和较为直白的表达方式。

[
https://www.reddit.com/r/ProgrammerHumor/comments/1grdmql/cacheinvalidationandnamingthings/
](
https://www.reddit.com/r/ProgrammerHumor/comments/1grdmql/cacheinvalidationandnamingthings/
)
    


### TITLE

这段内容是Reddit网站上有关搜索r/ProgrammingHumor子版块的说明和提示。它列出了一些缩小搜索范围的参数选项,如指定子版块、作者、网站、URL关键词、自我发帖内容、是否包含NSFW(不适合工作场合)内容等。此外,它还提供了高级搜索选项的链接,以通过作者、子版块等条件进行精确搜索。总的来说,这是一段为Reddit用户提供的子版块搜索使用说明。

[
https://old.reddit.com/r/ProgrammingHumor
](
https://old.reddit.com/r/ProgrammingHumor
)
    


### TITLE

作者非常高兴地分享了他新开发的工具 A dof,这个工具旨在简化管理、共享和同步配置文件跨系统的过程。A dof 具有以下主要功能:

1. 自动备份:可以快速初始化新的设置,自动查找并备份常用的配置文件。

2. 快速链接和部署:只需一个命令就可以将你的设置链接到 GitHub,让他人轻松获取你的配置,或者在几秒钟内部署其他人的设置。

3. 增强的提交信息:自动生成丰富、详细的提交信息,为每一个变更提供上下文说明。

4. 自动生成自述文件:为了方便在 GitHub 上分享,A dof 会创建一个现成的自述文件,使他人可以无缝地使用你的设置。

作者计划在将来的版本中推出以下新功能:自动更新、多配置文件管理、文件加密、云端友好的可移植模式等。

A dof 是作者的首个开源 Rust 项目,他渴望根据反馈来学习和改进。文中提供了 GitHub 仓库链接,欢迎大家提供反馈、建议或报告问题。最后,作者正考虑更改 A dof 这一名称。

[
https://www.reddit.com/r/rust/comments/1gr4pyx/adof_v100_release/
](
https://www.reddit.com/r/rust/comments/1gr4pyx/adof_v100_release/
)
    


### TITLE

以下是对该内容的中文总结:

ADOF是一个自动的dotfile组织工具,可帮助您无缝同步系统配置,使您的设置真正便携和可共享。它具有git集成、可定制的跟踪和部署功能,是维护和共享配置的终极助手。

ADOF主要功能包括:

1. 基于模式的文件跟踪,可使用fzf选择要跟踪的文件。
2. 自定义提交消息。
3. 从GitHub URL部署配置文件。 
4. 生成自述文件。
5. 集成加密功能(计划中)。
6. 基于时间限制的自动更新(计划中)。

它提供了诸如init、add、remove、link、unlink、push、update、deploy等命令,可方便地初始化、添加、删除、链接远程仓库、推送、更新和部署配置文件。

总的来说,ADOF致力于简化配置管理流程,提高效率,并促进配置共享。作者计划在未来进一步扩展其企业和专业用例,并基于赞助的方式进行持续开发。

[
https://github.com/fnabinash/adof
](
https://github.com/fnabinash/adof
)
    


### TITLE

Floem 是一个使用 Rust 语言编写的原生 UI 库,具有细粒度反应性。它旨在成为高性能的声明式 UI 库,并提供人性化的 API。主要特点包括:

1. 跨平台支持 Windows、macOS 和 Linux,支持 GPU 渲染和 CPU 渲染。
2. 基于响应式原语,实现细粒度反应性,使 UI 保持最新状态且高效。
3. 通过优化视图树构建,提高性能并避免瓶颈。
4. 提供 Flexbox 和网格布局系统。
5. 可定制化的小部件外观和行为。
6. 支持过渡和动画,包括关键帧动画和弹性动画。
7. 提供元素检查器用于调试布局。
8. 提供文档和示例代码以帮助学习。

该项目仍在发展阶段,路线图指向 v1 版本。欢迎贡献,包括提交问题、拉取请求或加入 Discord 频道与开发者讨论。

[
https://github.com/lapce/floem
](
https://github.com/lapce/floem
)
    


### TITLE

这是一个使用Rust语言实现的"随机艺术"生成器项目。该项目通过以下步骤生成基于输入字符串的随机图像:

1. 将输入字符串转换为u64哈希值。
2. 使用哈希值作为伪随机数生成器(PRNG)的种子。
3. 构建一个概率上下文无关文法(PCFG),用于扩展成各种数学公式和常数。
4. 使用PCFG和PRNG创建一棵解析树。
5. 生成一个将像素坐标映射到RGB颜色通道值的函数。
6. 使用该函数根据像素值生成彩色图像。

作者是Rust的新手,欢迎对这个项目提供反馈和改进建议。相关资源和代码库链接在GitHub上的README.md文件中。

[
https://old.reddit.com/r/rust/comments/1gro7r5/randomart_a_rust_implementation_of_generating/
](
https://old.reddit.com/r/rust/comments/1gro7r5/randomart_a_rust_implementation_of_generating/
)
    


### TITLE

这是一个关于Rust编译器rustc中的一个bug引发的故障排查过程的总结。主要内容如下:

1. 作者在开发一个名为rooc的优化建模语言时,想在浏览器中运行解算器,于是尝试使用一个纯Rust编写的线性规划库minilp。

2. 在测试阶段,作者发现minilp在Web Assembly(wasm)环境下会panic(运行时崩溃)。

3. 作者开始调试,首先尝试wasm2map添加调试映射,发现panic是由于unwrap一个None值引起的。

4. 令人费解的是,作者专门编写的测试用例却可以通过,没有重现panic。

5. 作者深入研究minilp的pop_min函数,发现在某些条件下会返回None,从而引发unwrap panic。

6. 由于minilp已被归档多年未更新,作者打算自行fork并修复这个bug。

总的来说,这是一个追踪和调试rustc编译器中一个潜在bug的过程记录,展示了作者在追究编程语言底层运行细节时所付出的努力。

[
https://specy.app/blog/posts/a-rustc-soundness-bug-in-the-wild
](
https://specy.app/blog/posts/a-rustc-soundness-bug-in-the-wild
)
    


### TITLE

这段内容讨论了在 Rust 中设计一个临时结构体的最佳方式。作者有一部分应用程序仅在定时器存活期间存在。他目前的设计是在一个可选元组中包含定时器和一个结构体,在 update_recording 函数中检查定时器是否已经结束,如果结束则清除相关数据。

但作者不太满意这种设计,希望能够以一种更加类型安全的方式,在定时器结束时自动清理相关数据,而不需要手动检查。他征求社区的建议,看是否有更好的代码结构可以实现这一目标。

[
https://old.reddit.com/r/rust/comments/1gs81ed/best_design_for_a_temporary_struct/
](
https://old.reddit.com/r/rust/comments/1gs81ed/best_design_for_a_temporary_struct/
)
    


### TITLE

这位开发者需要在项目中使用Rust的cargo作为库来动态编译一些运行时dll并注入到进程中。他遇到了一个问题,当包含dashmap crate时,编译时需要传递-Z unstable-options标志以启用check-cfg选项。

他提供了一段Rust代码,用于生成编译选项并调用cargo的编译操作。他想知道如何正确地为dashmap crate传递所需的编译标志。代码示例展示了如何设置编译配置、特性列表等选项,但似乎缺少了处理不稳定标志的部分。他寻求社区的建议来解决这个问题。

[
https://old.reddit.com/r/rust/comments/1gs875p/cargo_as_lib_for_dynamic_project_build/
](
https://old.reddit.com/r/rust/comments/1gs875p/cargo_as_lib_for_dynamic_project_build/
)
    


### TITLE

这是一篇关于在使用Rust编写嵌入式设备驱动程序时，如何生成Python绑定以便在Python应用程序中使用这些驱动程序的讨论。作者目前在使用Rust编写嵌入式固件,并希望能够在Python中方便地与这些固件进行交互。他描述了自己的项目结构,以及Rust代码示例。

他想要实现一个理想的Python API,可以直接调用Rust中定义的枚举类型和方法。他考虑过几种方案,包括在同一模块中定义PyO3类和方法,或者在单独的bindings包中包装原有类型。但作者担心这些方案可能会破坏绑定或引入冗余代码。

总的来说,这是一个关于如何在Python中优雅地使用Rust编写的嵌入式代码的讨论和探索。作者希望获得相关的建议和最佳实践。

[
https://old.reddit.com/r/rust/comments/1gs8935/rust_to_python_bindings/
](
https://old.reddit.com/r/rust/comments/1gs8935/rust_to_python_bindings/
)
    


### TITLE

这是一个关于Rust编程语言借用检查器的问题。作者在学习Rust时遇到了一段代码,代码中创建了两个向量,一个存储整数,另一个存储整数的可变引用。通过循环,作者将第一个向量中的元素引用存储到第二个向量中。接着,作者修改了第二个向量中的第一个元素的值为5。

代码的最后一部分,作者试图同时获取修改后的值和原始向量中的值,打印出来时两个值都是5。作者对此感到疑惑,因为他以为在Rust中,别名和可变性是互斥的,即一个值如果已经被不可变地借用,就不应该能再被可变借用。但是这段代码似乎违反了这一规则,并且也被借用检查器所允许。

因此,作者提出了这个问题,想要了解为什么借用检查器没有拒绝这段代码,因为它似乎违反了Rust中关于别名和可变性的规则。

[
https://old.reddit.com/r/rust/comments/1gs9y05/borrow_checker_question/
](
https://old.reddit.com/r/rust/comments/1gs9y05/borrow_checker_question/
)
    


### TITLE

这是一位Rust开发者分享的两个故事。首先,他发布了自己的第一个开源软件项目ADOF(Automatic DotFile ORganizer Friend),意为一个自动管理配置文件的工具。但由于这个名字的缩写与"阿道夫•希特勒"(Adolf Hitler)太相似,引起了一些人的反感和嘲笑,也有人在程序员幽默社区分享了这件事。尽管如此,该项目还是得到了一些人的支持和建议。

开发者解释说,他的主要动机是想通过开源项目赚钱维持生活,因为他的经济状况不太好。接下来,他准备开发一个新的Rust项目脚手架工具,可以根据不同的项目类型生成模板代码、集成测试、CI/CD设置等,并介绍了一些其他类似工具的对比。最后,他征求大家的意见和建议。

[
https://old.reddit.com/r/rust/comments/1grt8ld/my_experience_of_being_trolled_on_my_first_oss_in/
](
https://old.reddit.com/r/rust/comments/1grt8ld/my_experience_of_being_trolled_on_my_first_oss_in/
)
    


### TITLE

这篇博客主要介绍了Rerun最新版本0.20的两个新功能:

1. 支持地理空间数据 
- 新增了地理空间数据原语(如GeoPoints和GeoLineStrings),可以将它们发送到Rerun
- 新的地图视图可以在OpenStreetMap或Mapbox地图上绘制这些原语
- 未来还将增加GeoPolygons原语,以完全兼容GeoArrow格式
- 地理空间数据支持是根据社区贡献开发的

2. 视频解码支持
- 原生查看器现在支持解码H.264视频文件
- 未来将支持逐帧发送编码视频数据到Rerun,为实时视频流铺平道路  
- 视频解码通过调用系统上的FFmpeg可执行文件实现,使用开源crate ffmpeg-sidecar
- 这种架构设计考虑到了在不同环境下的灵活性

此外,该版本还包括一些稳定性和性能改进,如优化MP4文件解析效率。最后,博文邀请用户尝试新功能并提供反馈。

[
https://rerun.io/blog/maps
](
https://rerun.io/blog/maps
)
    


### TITLE

最近,我决定从Neovim切换到Helix编辑器,主要原因是我厌倦了不断膨胀的配置,只想要一个开箱即用的编辑器。

Helix是用Rust编写的,但它缺少了一些我非常想要的HTML标签相关功能,比如:为选中内容添加HTML标签、删除最近一对HTML标签、重命名最近一对HTML标签等。由于我主要是为了兴趣而使用React制作静态网站,而TypeScript是我唯一了解的语言,所以这些功能对我来说非常重要。

为了能够为Helix添加这个功能,我决定学习Rust。我完整地学习了Rust书,花费了至少40个小时,因为像所有权这样的概念对我来说完全陌生,让我觉得很难以理解。我之前从未接触过堆栈或堆这样的概念,但由于Rust本身很有趣,我设法完成了学习。在此期间,我还提交了10个较小规模的Pull Request,从中获得了一些自信。

终于有一天,我开始着手开发这个功能。对于像我这样的初学者来说,Helix是一个庞大的项目,有83,000行Rust代码。由于缺乏足够的文档,我最大的困难是不知从何入手,如何理解项目的工作原理。

幸运的是,凭借rust-analyzer、grep和find的力量,我终于弄清楚了各个函数的调用关系,知道了应该在哪里进行修改。

一旦理解了他们的内部API是如何修改文档的,我就需要创建一个算法来查找结束标记(向前搜索)和开始标记(向后搜索),过滤掉不成对的标记,并提取标记名称的确切位置。对于像我这样从未接触过算法的人来说,这并非一件小事。

经过将近2天的努力,我终于完成了这个功能:https://github.com/helix-editor/helix/pull/12055!

我为自己感到高兴,因为从来没想过仅仅一周时间就能学会足够的Rust知识,为一个非平凡的项目做出实际贡献,并开始对这门语言有更多的掌握感。作为一个只有11个月编程经验的业余爱好者,这是一个了不起的成就。

现在,我计划学习后端Web开发,因为在从事前端工作后,我发现自己更喜欢算法相关的工作。而且我会使用Rust来学习。

我意识到我的Pull Request可能要几个月才能被审核或合并,但我不介意,因为我可以在自己的分支中使用这个功能。

[
https://old.reddit.com/r/rust/comments/1gs0yhe/making_my_first_oss_contribution_in_rust_after/
](
https://old.reddit.com/r/rust/comments/1gs0yhe/making_my_first_oss_contribution_in_rust_after/
)
    


### TITLE

这是Godot-Rust项目2024年11月的开发更新。主要内容包括:

1. Godot-Rust版本0.2的发布,带来了更人性化和高效的参数传递方式。

2. 引入了几种新的参数传递方式:
   - 按引用传递容器类型,避免不必要的克隆
   - 自动执行向上转型,无需手动upcast
   - 可直接传递字符串字面值,无需显式转换

3. 通过大量代码示例展示了新老方式的区别,新方式大大简化和优化了代码。

4. 添加了基于路径的节点初始化新特性,可以使用#[init(node)]属性直接初始化字段与找到的节点。

5. 这些改进虽然实现过程艰辛,但值得为了获得更好的人机工程学体验。

总的来说,这个版本着重提升了Godot-Rust的开发体验,使Rust代码在Godot中更加自然、高效。

[
https://godot-rust.github.io/dev/november-2024-update/
](
https://godot-rust.github.io/dev/november-2024-update/
)
    


### TITLE

以下是总结:

Floem是一个跨平台的GUI库,旨在提供极高的性能和优秀的开发者体验。最新的0.2.0版本经过近一年的工作,修复了许多bug,添加了许多新功能,并大幅提升了API的人体工程学设计。开发者可以从新的网站获取最新版本的代码库链接以及入门教程。这个版本让Floem在性能和易用性方面都有了长足进步。

[
https://old.reddit.com/r/rust/comments/1gryrh3/floem_020_native_gui_now_with_wasm_vello_dynamic/
](
https://old.reddit.com/r/rust/comments/1gryrh3/floem_020_native_gui_now_with_wasm_vello_dynamic/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

