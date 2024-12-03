【Rust日报】2024-11-07


### TITLE

这是发布说明for thiserror 2.0.0版本,主要变更包括:

1. 在格式字符串中引用字段时,不再允许使用原始标识符{r#type},改为直接使用无修饰名称{type}。

2. 对元组结构体和元组变体,当同时使用数字索引和额外位置参数时,会产生歧义,因此不再允许这种用法。

3. 不论错误数据结构内容如何,派生Error特性时都必须直接依赖thiserror crate。

4. 支持禁用默认对标准库的依赖。

5. 支持使用r#source作为字段名来避免与Error::source()冲突。

6. 如果Display派生实现存在无限递归,现在会产生unconditional_recursion警告。 

7. 新增#[error(fmt=path::to::myfmt)]属性,允许在其他地方定义格式化逻辑。

8. 对于有枚举级格式消息的枚举,现在可以使用#[error(transparent)]让某些变体直接透传底层错误。

总的来说,此版本引入了一些破坏性变更以增强一致性和可用性,同时添加了一些新功能。

[https://github.com/dtolnay/thiserror/releases/tag/2.0.0
](https://github.com/dtolnay/thiserror/releases/tag/2.0.0
)
    


### TITLE

这段内容讨论了结构化数组(Soa)和向量(Vec)在内存分配和扩容方面的差异。

结构化数组通过一次内存分配存储整个集合,这减少了分配开销。但作者提出,这种做法可能会使容器扩容变得更加昂贵,因为无法像向量那样简单地扩展分配区域。

对于大型向量,标准库使用了一种技巧:当当前分配区域不足以扩容时,操作系统会将后备页面移动到其他有空间的地址区域,然后扩展分配区域。这种方式避免了数据复制,只涉及内存映射(mmap)操作。

而对于结构化数组,如果数据按顺序存储在一个分配区域中,就无法使用这种技巧了。

最后,作者表示希望能够支持枚举类型,尽管这可能非常困难,但如果实现了,他会立即使用。

[
https://old.reddit.com/u/VorpalWay
](
https://old.reddit.com/u/VorpalWay
)
    


### TITLE

这段内容讨论了一种可能的Rust编译时优化方法。作者提出了一种替代方案,利用稳定且安全的语言特性来实现类似于特化的效果。

具体来说,作者建议可以使用一个隐藏的公共函数来预先实例化一个泛型函数的某个具体类型。例如,对于一个泛型函数`generic_fn<T>()`而言,可以生成一个`#[doc(hidden)]`的`____precompiled_generic_fn_u32()`函数,其函数体只是调用`generic_fn::<u32>()`。

作者猜测,即使`____precompiled_generic_fn_u32()`函数从未被调用,下游代码调用`generic_fn::<u32>()`时也会复用这个已经实例化的版本,从而减少编译时间。根据Rust内部讨论,这种方式看起来应该是可行的。

总的来说,这是一种利用现有语言特性来实现编译时优化的创新方法,避免了使用实验性的特化特性。

[
https://old.reddit.com/u/dtolnay
](
https://old.reddit.com/u/dtolnay
)
    


### TITLE

Plotlars是一个功能强大的Rust库,它作为Plotly库的包装器,弥补了Polars数据分析库和Plotly之间的差距。它简化了从数据框创建可视化的过程,使开发人员能够专注于数据洞见而不是绘图的细节。

Plotlars的创建是为了简化在Rust中创建复杂图表的过程,特别是在使用强大的Polars数据操作库时。生成可视化通常需要大量样板代码和对绘图库(Plotly)和数据结构的深入了解。这种复杂性可能是一个重大障碍,特别是对于需要专注于分析和解释数据而不是wrestle复杂的绘图逻辑的用户。

与不使用Plotlars创建散点图相比,使用Plotlars可以大大减少代码量。Plotlars抽象了处理单个绘图组件的复杂性,允许用户指定高级绘图特性。这种简化方法不仅节省时间,而且减少了出错的可能性,并使代码更易读和可维护。

Plotlars可以无缝集成到Jupyter笔记本中,允许您在笔记本环境中利用交互式数据可视化的强大功能。这种集成是通过使用evcxr项目实现的,evcxr项目为Rust编程语言提供了Jupyter内核。通过Polars、evcxr和Plotlars,Rust中的数据科学跃升到了新的高度,使强大的数据分析和可视化比以往任何时候都更加易于访问和高效。

[
https://github.com/alceal/plotlars
](
https://github.com/alceal/plotlars
)
    


### TITLE

以下是对该文章的中文总结:

私人飞机的使用正在飞速增长,而最大的排放贡献者就是超级富豪阶层。2019年至2023年期间,约一半的私人飞行航程都很短,有些甚至不到100英里,本可以开车代替。尽管只有约0.003%的世界人口使用私人航空,但它是高耗能的,每位乘客的碳排放量远高于商业航班。

一些私人飞机型号每小时的碳排放量可能超过普通人一年的排放量。经常乘坐私人飞机的人,一年的碳排放量可能是全球平均水平的500倍。该研究是首次针对私人航空业的全球碳排放量进行评估。

2023年,私人航空总计排放了约1560万吨二氧化碳,平均每个航班3.6吨,相当于2023年商业航空排放总量的1.8%。许多短途航班本可以改为陆路交通,但出于便利和节省时间的考虑,富人选择乘坐私人飞机。

一些重大活动如超级碗、戛纳电影节等会引发私人飞机排放的高峰。总的来说,私人航空的排放量仍在不断增长,主要集中在夏季周末和知名度假胜地。

[
https://www.nationalgeographic.com/environment/article/private-jet-flights-climate-change
](
https://www.nationalgeographic.com/environment/article/private-jet-flights-climate-change
)
    


### TITLE

这项研究发现,一些名人乘坐私人飞机产生的二氧化碳排放量是普通人年排放量的500多倍。该研究由瑞典林奈大学一个团队进行,首次量化了私人飞机对气候的全球影响,发现2019年至2022年期间,私人航空排放量年增长46%,达到1560万吨二氧化碳。研究人员利用2600万架私人飞机和1870万次飞行记录,结合72种喷气机型的平均燃油消耗数据进行分析。研究发现,大多数私人飞机飞行是为了休闲,而且在联合国气候大会期间也有一个峰值。研究强调了名人私人飞行对气候的巨大影响。

[
https://www.thetimes.com/uk/environment/article/celebrity-private-jets-co2-emissions-5gmvgncrl
](
https://www.thetimes.com/uk/environment/article/celebrity-private-jets-co2-emissions-5gmvgncrl
)
    


### TITLE

这篇报道警告,极富有的人正如同乘坐出租车一般频繁使用私人飞机。气候科学家通过追踪私人飞机航班计算出,2019年至2023年期间,它们排放的二氧化碳增加了46%。每小时乘坐私人飞机排放的二氧化碳相当于一个普通人一年的排放量。2023年,私人飞机共排放了1560万吨二氧化碳,相当于370万辆汽车全年排放量,虽然相对全球排放来说不算多,但对于这一小群体来说是巨大的排放。报告追踪了一些知名人士的私人飞机航班,有人2023年乘坐私人飞机169次,排放相当于571辆汽车全年排放量。科学家呼吁富人需要为气候变化做出表率,减少不必要的私人飞行,否则未来十年气候变暖将更加严重。

[
https://www.bbc.com/news/articles/cx2lvq4el5vo
](
https://www.bbc.com/news/articles/cx2lvq4el5vo
)
    


### TITLE

根据文章内容,近年来私人飞机的使用量急剧上升,导致相关温室气体排放增加了50%。研究人员对2019年至2023年近1900万次私人飞机飞行进行了分析,发现近一半航班飞行距离不足500公里,90万次航班甚至被当作"出租车"使用,飞行距离不足50公里。很多航班是为了度假,在夏季时节飞往阳光明媚的目的地。2022年卡塔尔世界杯吸引了超过1800架私人飞机前往。

私人飞行虽然只占全球人口的0.003%,但却是最污染的运输方式。研究发现,乘坐较大型私人飞机一小时的二氧化碳排放量,就超过了普通人一年的排放量。美国主导着全球私人飞机旅行,占比高达69%,加拿大、英国和澳大利亚也位列前十。英国每六分钟就有一架私人飞机起飞。2023年,私人飞机的总排放量超过1500万吨,高于坦桑尼亚6000万人口的排放总量。

行业预计到2033年还将有8500架新的私人飞机投入使用,远超效率提升水平,这意味着私人飞行排放将进一步增加。研究人员指出,这凸显了富人和穷人之间在排放上的巨大不平等,解决富裕少数群体的排放问题对于终止全球变暖至关重要。研究员表示,当前全球排放增长主要来自于富人阶层。研究建议为每吨二氧化碳排放征收200欧元的环境税,并大幅提高私人飞机的起降费用,以遏制私人飞行的激增。

[
https://www.theguardian.com/world/2024/nov/07/used-like-taxis-soaring-private-jet-flights-drive-up-climate-heating-emissions
](
https://www.theguardian.com/world/2024/nov/07/used-like-taxis-soaring-private-jet-flights-drive-up-climate-heating-emissions
)
    


### TITLE

这篇文章探讨了私人飞机对气候变化的影响。主要内容如下:

1. 虽然私人飞机只占全球航空排放量的小部分,但富人和名人使用私人飞机产生的碳足迹远高于普通人。

2. 一些富人开始意识到自己的碳足迹问题,并采取措施来抵消或减少排放,比如购买碳抵消信用或使用可持续航空燃料。

3. 但批评者认为,富人应该直接减少飞行次数,而不是依赖于碳抵消这种"绿色勾销"方式。

4. 一些新兴的"纯电力"飞机可能有助于减少私人飞行的碳排放,但可能需要数十年才能成熟并广泛使用。

5. 总的来说,文章呼吁富人采取切实行动减少奢侈的私人飞行,而不只是依赖于购买碳信用等间接手段,以真正应对气候变化挑战。

[
https://apnews.com/article/climate-change-private-jets-wealthy-carbon-pollution-0a2d1d2cd81906381953346bfdb879e8
](
https://apnews.com/article/climate-change-private-jets-wealthy-carbon-pollution-0a2d1d2cd81906381953346bfdb879e8
)
    


### TITLE

这个存储库包含了一个分析私人飞机航班数据的命令行应用程序。它使用 S3 存储来缓存数据,从而减少对 https://adsbexchange.com/ 的影响。这项工作产生了一篇发表在《地球与环境通讯》的科学论文,并受到了多家新闻媒体的报道。

该存储库提供了一个 SQL 分析的示例,以及详细的方法学说明和不同层次聚合数据的获取方式。它强调应谨慎使用该代码,避免对 adsbexchange.com 造成过大影响。所有缓存数据都存储在一个公开可读取的 S3 blob 存储端点上。

该项目提供了一些 Rust 二进制文件,用于从头开始构建数据库快照、计算飞机位置和航班航段数据。这些数据最终也会存储在 S3 存储上。二进制文件的使用示例和访问凭证的说明都包含在README中。

[
https://github.com/jorgecardleitao/private-jets
](
https://github.com/jorgecardleitao/private-jets
)
    


### TITLE

本文探讨了私人航空对气候变化的不断增长的影响。研究人员利用2019年至2023年ADS-B交换平台的飞行跟踪数据,计算了25,993架私人飞机和1865.5万个航班的二氧化碳排放量。结果显示,2023年私人航空直接排放至少1560万吨二氧化碳,每个航班平均3.6吨。近半数航班在500公里以内。私人航空高度集中在美国,68.7%的飞机在那里注册。飞行模式分析证实了大量的休闲、文化和政治活动相关旅行。2019年至2023年间,排放量增加了46%,行业预计将继续强劲增长。研究人员呼吁出台相关政策,解决这一不断增长的气候影响。文章指出,与商业航空相比,私人航空是最高耗能的运输方式,但其全球规模、分布和能耗强度尚未完全了解。富裕人群的消费模式是全球排放增长的主要驱动力,减排政策需要重点关注这一群体。

[
https://www.nature.com/articles/s43247-024-01775-z
](
https://www.nature.com/articles/s43247-024-01775-z
)
    


### TITLE

gccrs是一个正在开发中的Rust编程语言替代编译器,作为GCC项目的一部分。它的目标是与rustc编译器具有完全相同的行为。gccrs项目的主要目的包括:

1. 为Rust编程语言提供一个替代的编译选择。
2. 支持LLVM不支持的一些处理器架构,如Dreamcast游戏机的SuperH架构。
3. 利用GCC多年积累的安全特性和静态分析器,增强对不安全Rust代码的分析能力。
4. 通过复制rustc的功能,帮助精炼Rust语言的规范文档。
5. 作为一个有趣且有益的编译器项目,吸引更多人参与编译器开发。

该项目明确表示,gccrs不是为了创建一种改变了Rust语义或功能的"另一种Rust"。它的目标是与rustc保持一致,不会绕过RFC过程引入新特性。总之,gccrs旨在提供一种替代编译选择,同时为Rust语言规范化和编译器开发做出贡献。

[
https://blog.rust-lang.org/2024/11/07/gccrs-an-alternative-compiler-for-rust.html
](
https://blog.rust-lang.org/2024/11/07/gccrs-an-alternative-compiler-for-rust.html
)
    


### TITLE

这位程序员每天都编写C#代码,自认为在C#方面已经达到专家级水平。然而,尽管他已经写了5年的Rust代码,但感觉自己在Rust方面的水平还远远不如C#。他觉得自己在编写Rust代码时可能错过了一些优化机会或代码改进的方式。

他想寻求一些资源或指导,以提升自己在Rust编程方面的能力,特别是在生命周期(lifetimes)或常量泛型(const generics)等高级特性的使用上。他觉得大多数在线课程或YouTube视频都只涵盖了基础知识,而要真正发挥Rust的优势,需要编写非常高效的代码,这对他来说仍然很困难。

他希望从有经验的Rust开发人员那里获得一些建议,是否需要有指导性的实践或示例,以帮助他进一步提高Rust编程水平。

[
https://old.reddit.com/r/rust/comments/1gm3frf/next_leveling_my_rust_programming/
](
https://old.reddit.com/r/rust/comments/1gm3frf/next_leveling_my_rust_programming/
)
    


### TITLE

这篇帖子讨论了为什么 Rust 标准库中没有包含日期时间相关的功能。作者表示在开发一个个人的记账CLI应用时,由于标准库缺少日期时间处理的功能,给开发带来了极大的挑战。作者对此表示疑惑,认为日期时间功能应该是一个标准库应当包含的基本功能。

总的来说,该帖子引出了 Rust 标准库在一些常见场景下的不足,并对标准库缺少日期时间相关功能表示诧异和疑虑。这反映出标准库的设计在满足一些基本需求方面可能还有改进空间。

[
https://old.reddit.com/r/rust/comments/1glf5wl/why_rust_doesnt_have_a_std_lib_for_date_time/
](
https://old.reddit.com/r/rust/comments/1glf5wl/why_rust_doesnt_have_a_std_lib_for_date_time/
)
    


### TITLE

这篇帖子宣布了Rodio 0.20版本的发布,Rodio是一个用于音频播放的Rust库。新版本增加了许多新功能,如高效的音频流定位、精确跟踪播放位置、生成噪音和不同波形、自动调整增益等。

帖子还呼吁社区提供反馈意见,以帮助Rodio在未来9年进行重大改进和重构。他们特别希望听到以下用户的反馈:感觉API使用困难的用户、遇到过缺陷或痛点的用户、原本想使用但因用例限制而无法使用的用户(除了复杂的游戏音频和高级DSP)。用户可以在issue tracker上分享反馈。

此外,Rodio团队也欢迎大家就他们计划的架构和API变更提出建议和意见。

[
https://old.reddit.com/r/rust/comments/1gm46rh/announcing_rodio_020_and_call_for_help/
](
https://old.reddit.com/r/rust/comments/1gm46rh/announcing_rodio_020_and_call_for_help/
)
    


### TITLE

这篇帖子介绍了Rust的`thiserror`错误管理库的2.0.0版本发布。该版本现在支持`#[no_std]`环境,可通过设置`default-features = false`在Rust 1.81.0+版本中使用,这对于嵌入式、裸机或Wasm开发很有用。`thiserror`库旨在简化定制错误层次结构的创建和维护,最小化样板代码而不会暴露在包的公共API中。帖子对该库的作者和维护者表示感谢,并补充说明了一些其他新特性。

[
https://old.reddit.com/r/rust/comments/1glb3ya/psa_thiserror_200_released_officially_gains_no/
](
https://old.reddit.com/r/rust/comments/1glb3ya/psa_thiserror_200_released_officially_gains_no/
)
    


### TITLE

总结如下:

这篇文章解释了为什么Rust中的std::pin::Pin包装器看起来非常奇怪。某些Rust类型的值需要被固定(pinned),以防止它们在内存中移动。这是通过std::pin::Pin包装器类型来表示的,通常体现为函数接受Pin<&mut T>而不是&mut T。

固定值会使得使用许多"正常"的编程技术变得困难,并产生一些奇怪的副作用,这些副作用与固定值的目的没有明显的联系。

文章首先简要介绍了什么是固定值,探讨了一些常见的令人困惑的情况,并试图通过确定产生这些奇怪副作用的原因来解释它们。这是一个关于学习掌握固定值的经验之谈的集合。

文中举例说明,如果一个类型包含指向自身的指针或引用,在移动该类型到不同的内存位置时,这些指针或引用就会变为无效,导致未定义行为。std::pin::Pin的存在就是为了防止这种情况发生。

总的来说,这篇文章旨在帮助读者更好地理解Rust中固定值的概念及其一些看似奇怪的表现。

[
https://sander.saares.eu/2024/11/06/why-is-stdpinpin-so-weird/
](
https://sander.saares.eu/2024/11/06/why-is-stdpinpin-so-weird/
)
    


### TITLE

这个 issue 讨论了 .io 顶级域名将在未来几年内被"逐步淘汰"的潜在风险,以及这可能对 crates.io 和整个 Rust 生态系统产生的影响。

作者首先列举了几个因地缘政治原因而被废止的国家顶级域名的例子,例如东德的 .dd、南斯拉夫的 .yu 等。他认为 .io 域名的情况可能类似于前苏联的 .su,IANA 可能会给予特殊对待,但这并不是完全可控的。

接下来,作者分析了这种风险对 crates.io 的影响。虽然短期内可能不会发生严重问题,但一些实际问题可能会逐渐出现,比如像英国公民突然无法注册或续费 .eu 域名这种情况。

总的来说,作者认为这是一个值得讨论和提前规划的重大风险,以免给 Rust 生态系统带来灾难性影响。

[
https://github.com/rust-lang/crates.io/issues/9864
](
https://github.com/rust-lang/crates.io/issues/9864
)
    


### TITLE

该内容是一个Rust编程语言的测试运行结果。总共运行了68个测试用例,全部通过。涉及到的测试主题包括词法分析器(lexer)对字符串(string)的处理,以及对空输入(empty input)的处理。测试结果显示全部68个测试用例都成功通过,没有失败的case,耗时0.00秒完成所有测试。这表明该代码在相关功能点上的实现是正确的,符合预期。

[
https://xnacly.me/posts/2024/rust-pldev/
](
https://xnacly.me/posts/2024/rust-pldev/
)
    


### TITLE

经过长时间的中断,Plotlars 0.7.1版本终于发布了!该项目通过深度重构,旨在降低Rust中制作数据可视化图表的复杂性。这个版本增加了热力图和三维散点图的支持,使得更容易发现数据中的模式和异常。开发者感谢大家的支持和反馈,并鼓励大家探索新的文档、试用新功能,如果喜欢这个项目,可以在GitHub给予星标支持。开发者相信Rust能够成为数据科学的优秀候选语言,尽管前路漫漫,但这是一个突破性的项目,将改变这一领域。最后,开发者期待大家的支持,并祝大家绘图愉快!

[
https://old.reddit.com/r/rust/comments/1glpzdl/announcing_plotlars_071_were_back_with_deep/
](
https://old.reddit.com/r/rust/comments/1glpzdl/announcing_plotlars_071_were_back_with_deep/
)
    


### TITLE

这是一篇关于使用Rust语言完成私人飞机对气候变化影响的科研论文的帖子。作者花了一年时间进行开发和科学研究,最终完成了题为"私人航空对气候变化的日益增长的贡献"的论文。论文的核心分析完全使用Rust语言完成,源代码托管在GitHub上。该论文获得了包括美联社、卫报、BBC、泰晤士报和国家地理等多家媒体的报道。作者对选择使用Rust语言感到非常高兴,因为"如果代码能编译通过,就意味着它是正确的"。

[
https://old.reddit.com/r/rust/comments/1glwewo/paper_on_use_of_private_jets_done_in_rust/
](
https://old.reddit.com/r/rust/comments/1glwewo/paper_on_use_of_private_jets_done_in_rust/
)
    


### TITLE

总结如下:

2024年,Rust项目首次参与了谷歌编码之夏(GSoC)计划。9名贡献者在过去几个月中努力工作在他们令人兴奋的项目上。这些项目有不同的持续时间,有些在8月结束,最后一个在10月中旬结束。现在所有项目的最终报告都已提交,9名贡献者全部通过了最终评审,他们的项目被认为是成功的,尽管可能没有完全达到最初目标。

Rust项目与GSoC贡献者有了很好的互动,贡献者反馈显示他们对计划感到满意并学到了很多。Rust项目对他们的贡献表示由衷的感谢,一些人甚至在项目结束后继续做出贡献。

总的来说,2024年的GSoC对Rust项目是一次成功,我们期待在不久的将来再次参与GSoC或类似计划。概述了9个GSoC项目,包括:cargo-semver-checks添加lint级别配置、为Cranelift实现更快的寄存器分配器、改进rustc编译时间等。

[
https://blog.rust-lang.org/2024/11/07/gsoc-2024-results.html
](
https://blog.rust-lang.org/2024/11/07/gsoc-2024-results.html
)
    


### TITLE

这篇文章欢迎两位新的Rust GPU项目维护者Schell Scivally和Firestar99。Schell是一位来自新西兰的游戏和图形程序员,目前在一家公司工作,利用AI简化摄影师的工作流程。他是Renderling项目的开发者,这是一个跨平台的基于WebGPU的GPU渲染器。他使用rust-gpu来在CPU上调试着色器,同时享受Rust开发体验的好处。Firestar99是一位来自德国的图形程序员和硕士研究生,他的论文是在自定义的Rust引擎中复制UE5的Nanite技术。他一直在为rust-gpu项目做出贡献,包括实现Mesh Shader、子组内置函数等功能。他希望能为rust-gpu贡献无绑定API,并在业界找到一份与该项目相关的工作。文章对两位新维护者表示热烈欢迎,并期待吸引更多用户和贡献者加入rust-gpu项目。

[
https://rust-gpu.github.io/blog/2024/11/06/new-maintainers
](
https://rust-gpu.github.io/blog/2024/11/06/new-maintainers
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

