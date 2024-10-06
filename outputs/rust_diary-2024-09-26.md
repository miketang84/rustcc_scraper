【Rust日报】2024-09-26


### TITLE

本文描述了 gccrs 项目在编写备用 Rust 编译器 gccrs 时重用了 rustc 编译器中的一些组件和crate。主要包括以下几个方面:

1. 重用哪些组件?
   目前已经集成了 rustc_parse_format 来解析 Rust 格式化字符串。未来还计划集成 polonius 新一代借用检查器、新的 trait solver 等关键组件。

2. 为什么要与 rustc 保持行为一致?
   借用检查和 trait 解析是 Rust 语言的核心部分,确保与 rustc 行为一致至关重要。通过重用经过大量测试和改进的 rustc 组件,可以减少 gccrs 与 rustc 的行为差异。

3. 如何集成这些 Rust 组件?
   由于目前 gccrs 还不完整,无法自行编译这些 Rust 组件,因此目前依赖 cargo 和 rustc 来编译并链接它们。未来计划通过自举过程,先构建一个无借用检查的中间编译器,使用它编译 Rust 组件,最后链接到最终编译器中。

总的来说,重用 rustc 组件的目的是加快开发进度,并确保 gccrs 在关键的编译器过程中与 rustc 的行为保持一致,为 Rust 语言的生态系统做出贡献。

[https://rust-gcc.github.io/2024/09/20/reusing-rustc-components.html
](https://rust-gcc.github.io/2024/09/20/reusing-rustc-components.html
)
    


### TITLE

该Reddit帖子的作者对使用Rust进行一些科学计算优化感兴趣,特别是解决离散优化问题。他的使用场景是编写一个算法来解决任意的离散优化问题,然后加载具体的数值并求解。

作者想知道是否有一种简单/自动的方式来使用硬编码值重新编译算法,以提高性能。他还询问这种方式是否真的能显著加速计算。

总的来说,这个帖子旨在了解在Rust中是否可以方便地进行动态编译,将问题的具体数值硬编码到算法中,从而获得更好的性能表现,这对于科学计算和优化问题是很有价值的。

[
https://old.reddit.com/r/rust/comments/1fqfij3/dynamic_compilation_for_highperformance_computing/
](
https://old.reddit.com/r/rust/comments/1fqfij3/dynamic_compilation_for_highperformance_computing/
)
    


### TITLE

这位开发者正在寻找用Rust语言学习编程的最佳方式。他过去常用的方法是通过修改简单的现有项目,边实践边看到结果,以小部分的方式逐步学习新语言。他想知道对于Rust语言是否有类似的学习资源,可能是类似于rustlings这样的交互式编程挑战。总的来说,他希望能找到一些通过动手实践的方式来循序渐进地掌握Rust。

[
https://old.reddit.com/r/rust/comments/1fqd6i3/learning_by_doing/
](
https://old.reddit.com/r/rust/comments/1fqd6i3/learning_by_doing/
)
    


### TITLE

以下是egui 0.29.0版本的主要更新内容:

1. 新增了多通道布局(multi-pass layout)支持,可以更好地实现高级布局效果。引入了新的UiBuilder和Context::request_discard函数。

2. 新增了UiBuilder,允许更灵活地构建UI并响应点击和拖动事件。

3. 改进了对亮/暗模式自动切换的支持,可以为两种模式设置不同的界面样式。 

4. 改进了GUI的视觉效果,包括文本垂直居中、线条渲染、文本选择效果等。

5. 更新了一些API,如将id_source改为id_salt。

6. 添加和改进了多个小功能,如列布局、滑块设置等。

7. 进行了一些性能优化。

8. 修复了多个Bug。

9. eframe框架也作了相应更新,如升级winit、支持虚拟键盘、修复iOS相关问题等。

总的来说,这个版本针对布局、视觉效果、主题切换等方面做了很多改进,同时修复Bug、优化性能,并新增了一些实用功能。

[
https://github.com/emilk/egui/releases/tag/0.29.0
](
https://github.com/emilk/egui/releases/tag/0.29.0
)
    


### TITLE

以下是对该博文的中文总结:

作者认为Rust编程语言就像第一代iPhone一样,是一款非常先进但还没有完全成熟的产品。虽然Rust拥有令人兴奋的特性,如代数数据类型、内存安全性和现代化的包管理器,但经过4年的使用,作者感觉它似乎永远达不到理想状态。

Rust的发展进度放缓,新特性很少被加入稳定版本。有些重要的RFC提案,如协程特性,已经耗时7年但仍未能进入稳定版本。作者怀疑这是因为Rust的共识过程难以扩展。

为此,作者幻想着fork编译器,创建自己的"Seph"版本的Rust语言,添加一些突破性的新特性。他想为函数引入traits(效果),暴露函数是否可panic、是否有固定栈大小、是否为纯函数等性质。

他还想引入编译期能力系统,允许开发者明确授权代码访问敏感操作(如文件系统读写)的能力,从而降低供应链风险。

总的来说,作者希望Rust语言能有一个"更好的自己",并为之提出了一些设想和改进方向。

[
https://josephg.com/blog/rewriting-rust/
](
https://josephg.com/blog/rewriting-rust/
)
    


### TITLE

这篇文章主要讨论了Rust编程语言在Linux内核中的进展、挑战,以及开发人员和维护者之间的协作需求。主要内容包括:

1. Linux内核开发者就Rust在内核中的集成进行了圆桌讨论,分享了进展和挑战。

2. Rust集成进展虽然缓慢但在稳步推进,因为内核维护者对新语言持谨慎态度,需要开发人员的协助来审查和调试Rust代码。

3. Rust帮助改善了部分现有C代码的文档和API设计,但Linux内核缺乏专门的文档工作资金。

4. Rust和C在编程理念上存在差异,给维护者带来一些阻力,但总体来说维护者正在适应这种变化。

5.一些Linux发行版如Debian和Ubuntu已开始采用Rust。

6.虽然进程缓慢,但Rust在Linux内核中的地位正在稳固,开发人员和维护者需要相互协作推动这一进程。

[
https://www.zdnet.com/article/rust-in-linux-now-progress-pitfalls-and-why-devs-and-maintainers-need-each-other/
](
https://www.zdnet.com/article/rust-in-linux-now-progress-pitfalls-and-why-devs-and-maintainers-need-each-other/
)
    


### TITLE

以下是对该文章的中文总结:

这篇博文讨论了一个新想法,通过引入一个名为Overwrite的新trait,可以opt-in控制哪些类型可以被完全覆盖赋值,从而减少很多借用检查错误,并简化pin的使用。

目前Rust允许对任何&mut T类型的值进行完全覆盖赋值,如*x = new_value。但这引发了一些问题,比如无法区分结构体的可变和不可变字段,从而导致借用检查产生多余的错误。

作者提出在结构体中显式声明字段是否可变,不可变字段就无法被间接修改。但单靠这一点还不够,因为通过like *x = new_struct仍可以绕过。

于是作者想到引入一个Overwrite trait,只有实现了该trait的类型,才能对&mut T完全覆盖赋值。大多数类型默认实现该trait,但在未来版本中,默认行为将改为不实现Overwrite,需要显式声明。

这一改变能减少很多借用检查错误,让不可变数据的不可变性更明确,同时也简化了Pin的实现等。该想法值得进一步探讨和实验。

[
https://smallcultfollowing.com/babysteps/blog/2024/09/26/overwrite-trait/?utm_source=atom_feed#fnref:2
](
https://smallcultfollowing.com/babysteps/blog/2024/09/26/overwrite-trait/?utm_source=atom_feed#fnref:2
)
    


### TITLE

这篇文章介绍了Rust语言中 zerocopy 0.8 版本的一个新特性 #[diagnostic::on_unimplemented]。该特性可以改善派生trait时产生的一些令人困惑的错误信息。

文章举例说明了在使用 #[derive(IntoBytes)] 时遇到的一个错误信息相当晦涩的情况。但是通过使用 #[diagnostic::on_unimplemented]，可以生成更加人性化、易于理解的错误信息,指出了导致错误的原因及一些可能的解决方案。

最后,作者强烈推荐在 Rust 项目中使用 #[diagnostic::on_unimplemented] 这一特性,因为它可以极大地改善编译器产生的错误信息的可读性,提高开发效率。

[
https://old.reddit.com/r/rust/comments/1fpwvcj/psa_use_diagnosticon_unimplemented_its_amazing/
](
https://old.reddit.com/r/rust/comments/1fpwvcj/psa_use_diagnosticon_unimplemented_its_amazing/
)
    


### TITLE

根据该文章,谷歌转向使用内存安全语言Rust作为其"安全性设计"方法的一部分,使Android中发现的内存安全漏洞的百分比从6年前的76%降至24%。谷歌表示,对于新功能采用安全编码不仅降低了代码库的整体安全风险,而且也使转换过程更加"可扩展和经济高效"。随着时间推移,新的内存不安全代码开发放缓,内存安全开发取而代之,内存安全漏洞就会逐渐减少。

该文章还指出,尽管新的内存不安全代码量增加,内存安全漏洞的数量往往会下降,原因是漏洞会呈指数级衰减,新代码或最近修改过的代码中往往存在很多漏洞。谷歌还强调应进一步推进内存安全策略,从"高度确保性预防"的角度,将安全性纳入基础设施。

谷歌表示,它正专注于提供Rust、C++和Kotlin之间的互操作性,而不是代码重写,作为采用内存安全语言的"实用且渐进"方式,从而最终消除整个漏洞类别。该公司还与Arm合作,提高GPU软件/固件栈的整体安全性。

[
https://thehackernews.com/2024/09/googles-shift-to-rust-programming-cuts.html?m=1
](
https://thehackernews.com/2024/09/googles-shift-to-rust-programming-cuts.html?m=1
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

