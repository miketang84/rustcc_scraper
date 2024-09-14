【Rust日报】2024-09-13


### TITLE

本文介绍了Rust语言中处理错误和缺少数据情况的两种类型Option和Result。Option类型表示一个值要么存在(Some(x))要么不存在(None)。Result类型表示一个操作要么成功返回值(Ok(x))要么失败返回错误原因(Err(e))。这两种类型让Rust能在编译期就捕获开发者忘记处理这些情况的错误,提高代码健壮性。文章详细解释了如何使用match表达式或?操作符来处理Option和Result,避免程序崩溃。总的来说,Option和Result是Rust优雅处理错误和缺少数据的解决方案,提高了代码的安全性和开发体验。

[https://bitfieldconsulting.com/posts/rust-errors-option-result
](https://bitfieldconsulting.com/posts/rust-errors-option-result
)
    


### TITLE

这是一个使用Rust宏实现的简单Lisp解释器。该解释器在Rust的编译期间通过宏展开来计算Lisp表达式的值,并将结果字符串化。它支持一些常见的Lisp形式,如QUOTE、LAMBDA、LET、PROGN、CAR、CDR等,但目前不支持显式递归和点对列表。

该解释器非常简洁,只有250行左右的代码。作者认为它很酷,因为它完全使用Rust的宏来实现。代码中给出了一些使用示例,包括一个求解自身的"quine"程序。

作者还提供了一个使用自身Lisp实现的元循环解释器,但由于使用了Y组合子进行递归,效率较低。作者计划添加显式递归原语来改进效率。

最后,作者提供了一些关于函数式编程和Lisp实现的绝佳资源,以及一些待办事项,如添加letrec和递归define。

[
https://github.com/RyanWelly/lisp-in-rs-macros
](
https://github.com/RyanWelly/lisp-in-rs-macros
)
    


### TITLE

这是对Rust编程语言中数据可视化库Plotlars 0.5.0版本的新功能发布公告。主要新增功能包括:

1. 统一了柱状图和箱线图的绘制,简化了API并提高了灵活性。

2. 增加了统一颜色参数,方便设置单一颜色样式。

3. 可自定义标记形状,更好地表示数据点。

4. 可调整线条宽度和样式,为折线图和时间序列图提供更多控制。

此外,该项目在GitHub上已获得200颗星,作者对社区的支持和反馈表示感谢。最后呼吁大家继续使用Plotlars库,给予星标支持,帮助项目持续发展。

[
https://old.reddit.com/r/rust/comments/1ffy6fb/announcing_plotlars_050_unified_bar_and_box_plots/
](
https://old.reddit.com/r/rust/comments/1ffy6fb/announcing_plotlars_050_unified_bar_and_box_plots/
)
    


### TITLE

这位同学在学习操作系统课程时,需要与UNIX API进行交互,使用pthread.h、unistd.h、signal.h等头文件。课程原本是使用C语言设计的,但他希望能够使用Rust语言完成相关项目。不过,老师提出条件是,使用的任何工具都不能掩盖C版本中系统调用的特性。

为此,他考虑使用nix和/或rustix这两个Rust crate来实现目标。他目前比较倾向于使用nix,但希望能获得建议,哪一个更适合他的需求。同时,由于不能使用Rust原生的线程,他也需要一些pthread库的建议。

总的来说,这位同学希望在操作系统课程项目中使用Rust,但需要遵循与C版本一致的系统调用特性,并寻求合适的Rust crate和线程库来满足需求。

[
https://old.reddit.com/r/rust/comments/1ffuvak/unix_functions_in_rust/
](
https://old.reddit.com/r/rust/comments/1ffuvak/unix_functions_in_rust/
)
    


### TITLE

这个项目名为pyrtls,是一个为Python提供现代TLS支持的库。它基于Rust语言实现的rustls库,旨在取代Python标准库ssl模块。与ssl相比,pyrtls默认情况下更加安全,不支持已知存在安全问题的旧协议版本、密码套件和部分TLS特性。

pyrtls目前是一个新兴项目,正在逐步完善功能。它目前支持TLS 1.2和1.3版本、常用的安全密码套件、ALPN协议协商、服务器名称指示(SNI)、会话恢复等特性。性能表现与ssl模块相当。但它目前还不支持TLS 1.1及更早版本、旧的不安全密码套件,也不支持直接使用CA证书认证服务器/客户端(自签名证书)。

该项目欢迎各种形式的贡献,包括资金赞助、Pull请求和反馈建议等。对于商业支持需求,可以直接联系作者。总的来说,pyrtls旨在为Python带来rustls的安全性和性能优势。

[
https://github.com/djc/pyrtls
](
https://github.com/djc/pyrtls
)
    


### TITLE

这段内容描述了一个编程问题。有两个结构体Foo和FooPart,FooPart是Foo的一个子集。有一个foo()方法是实现在FooPart上的,但有时需要在拥有完整的Foo结构体时也能调用foo()方法。作者提出了两种解决方案:

A. 显式构造FooPart,然后调用foo(),如foo.into_part().foo()

B. 为Foo实现Deref trait,从而可以直接在Foo上调用foo(),如foo.foo()

作者在询问这两种方案哪种更合适。这是一个关于如何方便地重用代码、设计良好的API的典型问题。

[
https://old.reddit.com/r/rust/comments/1fg6nsp/is_deref_right_for_this/
](
https://old.reddit.com/r/rust/comments/1fg6nsp/is_deref_right_for_this/
)
    


### TITLE

这是一个关于Rust语言中的人机工程学设计问题。作者想要在`Client`结构体中实现一个`new`方法,该方法接受一个可选的参数,参数类型是可以转换为`String`的任何类型。但是,当使用`None`调用该方法时,需要显式地注释类型,如`Client::new(None::<String>)`。这种写法显得很麻烦。作者希望找到一种更优雅、更易读的方式来实现这个功能,而不会给库的使用者带来额外的负担。

[
https://old.reddit.com/r/rust/comments/1fg2v43/ergonomics_for_optionintostring/
](
https://old.reddit.com/r/rust/comments/1fg2v43/ergonomics_for_optionintostring/
)
    


### TITLE

总结如下:

Owdle是一款每日猜谜游戏,玩家根据一些特征来猜测《守望先锋》中的英雄人物。这个游戏可以考验你对英雄背景故事的了解,也可以让你更深入地了解他们的背景。Owdle受到了许多其他类wordle游戏的启发,如metazooa、loldle、guessthegame、redactle等。但Owdle并非由暴雪娱乐公司推出或背书,《守望先锋》及其人物是暴雪的知识产权。Owdle本质上是一款根据英雄特征猜测英雄身份的每日游戏,旨在考验和增进玩家对英雄背景的了解。

[
https://owdle.guessing.day/
](
https://owdle.guessing.day/
)
    


### TITLE

这位开发者正在学习《Rust编程语言》,并使用Rust实现一个解释器。他已经完成了抽象语法树(AST)版本,并开始构建字节码版本。在实现AST版本时,他最初只是直接将代码翻译成Rust,没有太在意性能、规范写法等问题。正如预期的那样,第一个版本的性能很差,因为使用了太多克隆和智能指针。

之后,他开始优化代码,去掉不必要的克隆,尝试用更地道的Rust方式重写部分代码,性能得到了改善。但仍有进一步优化的空间。

在寻找优化方法时,他发现了一个Reddit帖子,提出了使用`typed-arena`和将字符串字面量内联的建议。但他在给`LoxValue`枚举引入生命周期时遇到了困难,因为它几乎无处不在,改动会影响到大部分代码。虽然他对生命周期有所理解,但缺乏实践经验。

经过多次尝试未果后,他暂时放弃了。但他希望能进一步提高,成为一名更出色的Rust开发者,所以他正在寻求指导和建议,以便最终解决这个问题。后来在一位用户的建议下,他使用索引而不是生命周期来处理表达式(Expr),并内联了字符串,使fib(40)的计算时间从101秒降低到64秒。

[
https://old.reddit.com/r/rust/comments/1fg1bpg/while_trying_to_implement_lifetimes_i_couldnt/
](
https://old.reddit.com/r/rust/comments/1fg1bpg/while_trying_to_implement_lifetimes_i_couldnt/
)
    


### TITLE

这篇文章介绍了一种新的高效字符串压缩算法FSST(Fast Static Symbol Table)。FSST是为了解决现有字符串压缩算法对于需要随机访问数据的应用场景效率较低的问题而提出的。

FSST的核心是构建一个符号表(symbol table),符号表中的每个符号是1到8个字节长的字符串。压缩时,FSST对待压缩的字符串进行贪婪匹配,用符号表中最长的前缀符号替换原字符串,未匹配到的部分则使用转义码表示。解压时只需根据码流查符号表即可快速重建原字符串。

文章描述了FSST的工作原理、压缩解压示例、理论压缩率,并给出了Rust伪代码实现思路。最后介绍了FSST如何通过逐代算法构建高效的符号表。

总的来说,FSST为需要高效随机访问的字符串数据提供了一种全新的压缩解决方案,在多个真实数据集上表现出2-3倍的压缩率,解压吞吐量可达数GB/秒,是一种很有前景的轻量级字符串压缩编解码器。

[
https://blog.spiraldb.com/compressing-strings-with-fsst/
](
https://blog.spiraldb.com/compressing-strings-with-fsst/
)
    


### TITLE

以下是对该内容的中文总结:

Salvo是一款使用Rust语言编写的Web框架,它注重人体工程学设计。与其他框架相比,Salvo具有以下优势:

1. 无需进行复杂类型处理,且基本不需要深入理解Rust语言知识,非常适合初学者使用。

2. 功能全面,支持HTTP3和Webtransport等新协议和功能。

3. 官方维护了许多中间件。

4. 提供了tower兼容层,可直接使用tower中间件。

最新的0.72.0版本更新包括:

1. 允许中间件直接应用于Handler,使用更加方便。

2. 实现了对元组的Writer支持。

3. 提供了SecureMaxSize中间件,可以更细粒度地控制请求的安全数据大小。

4. 修复了RequestId部分存储密钥不正确的问题。

5. 对OpenAPI支持进行了多项改进。

6. 修复了在某些情况下应该返回404但却返回405的问题。

Rust作为一种相对困难的语言,许多Web框架设计晦涩,更加剧了学习和使用的难度。当你在使用其他框架时遇到困难,想放弃Rust时,不妨试试Salvo。它会让你"留下来",给你学习Rust的信心和勇气,让你体会到成功的喜悦。

更多详细的更新信息可参考:https://github.com/salvo-rs/salvo/releases/tag/v0.72.0

[
https://old.reddit.com/r/rust/comments/1ffpgze/salvo_web_framework_0720_released/
](
https://old.reddit.com/r/rust/comments/1ffpgze/salvo_web_framework_0720_released/
)
    


### TITLE

以下是对该内容的中文总结:

Plotlars是一个Rust库,作为Plotly库的包装器,用于简化在Polars数据分析库中创建可视化的过程。它的动机是减少在Rust中生成复杂图表所需的大量样板代码和对绘图库的深入了解。Plotlars通过提供简洁的API,让开发者能够专注于数据分析和可视化,而不用过多关注底层绘图逻辑。

该库与Polars数据框紧密集成,支持创建条形图、折线图、散点图等多种类型的图表,同时还允许自定义图表外观。Plotlars还可与Jupyter笔记本环境顺利集成,结合evcxr项目,使得在Rust中进行数据科学工作变得高效便捷。

该项目基于MIT许可证,源于对Polars、Plotly、evcxr等项目的肯定,以及对Rust编程语言社区的支持和发展的贡献。通过Plotlars,数据可视化在Rust中变得更加简单高效。

[
https://github.com/alceal/plotlars
](
https://github.com/alceal/plotlars
)
    


### TITLE

这本书名为"Crafting Interpreters"(编写解释器),主要内容是教读者如何从头开始构建一个功能完备、高效的脚本语言解释器。读者将学习到关于解析和语义的高级概念,以及bytecode表示和垃圾回收等底层细节。该书从main()函数开始,一步步构建一种具有丰富语法、动态类型、垃圾回收、词法作用域、一级函数、闭包、类和继承等特性的编程语言。

这本书提供了多种形式:精美印刷版、电子书、Kindle版和网页在线免费阅读版。作者Robert Nystrom之前在Google的Dart语言团队工作,也是《Game Programming Patterns》一书的作者。他在这本书中将自己多年来构建编程语言的经验和体会娓娓道来。

[
https://craftinginterpreters.com/
](
https://craftinginterpreters.com/
)
    


### TITLE

这个仓库包含了两个Rust项目,分别是rlox-ast和rlox-bytecode。

rlox-ast是根据书籍"Crafting Interpreters"第二章实现的树遍历解释器jlox的Rust版本。该项目已经完成了书中前9章的内容,包括扫描、代码表示、解析表达式、求值表达式、语句和状态、控制流、函数、解析和绑定、类和继承。

rlox-bytecode则是根据该书第三章实现的基于字节码的解释器clox的Rust版本。目前该项目已经完成了字节码块和虚拟机的实现,其余部分如按需扫描、表达式编译、值类型、字符串、哈希表、全局/局部变量、跳转、调用和函数、闭包、垃圾回收、类和实例、方法和初始化器、超类和优化等还在进行中。

该仓库的部分代码受到了另一个Rust实现lox-rs的启发。

[
https://github.com/Emivvvvv/rlox
](
https://github.com/Emivvvvv/rlox
)
    


### TITLE

根据Reddit上的讨论,主要有以下几点原因导致使用Rust语言实现的Lox解释器比Java版本运行速度慢:

1. 内存分配和释放的开销。Rust使用引用计数进行内存管理,而Java的垃圾回收机制可能更高效。大量的malloc/free操作可能拖慢了Rust版本。

2. 哈希函数的差异。Rust默认使用了一个防DOS攻击的较慢哈希函数,而Java使用的是更快的哈希函数。更改为FNV或rustc-hash可显著提升性能。

3. 解释器的实现方式不同。Java版本可能受益于JIT编译器的优化,而Rust是AOT编译的,在循环密集型基准测试中可能逊色。

4. 数据局部性和内存布局。Rust版本使用了多层Box/Rc包裹,可能影响了数据局部性。使用Arena或Vec存储可改善这一点。

5. 解析器实现效率较低。手写的解析器可能低于生成的解析器。使用LALRPOP可能会提升解析性能。

6. 其他一些细节,如避免不必要的拷贝、内联热路径函数等也可能会带来一些提升。

总的来说,Rust版本在某些基准测试中比Java版本慢主要是由于内存管理策略、哈希函数、编译器优化能力等语言和运行时的根本差异造成的。通过一些细节优化可以缩小差距,但要完全超越可能需要采用不同的解释器实现方式。

[
https://www.reddit.com/r/ProgrammingLanguages/comments/v5d5uo/lox_interpreter_in_rust_slower_than_in_java/
](
https://www.reddit.com/r/ProgrammingLanguages/comments/v5d5uo/lox_interpreter_in_rust_slower_than_in_java/
)
    


### TITLE

以下是对给定内容的总结:

作者讨论了两种不同的方法来表示和处理数据与操作的组合。第一种方法是将数据和操作分开存储,例如[Data(2),Add]。这种方法违背了作者之前提出的"使非法状态不可表示"的原则。

第二种方法是将每个操作与其操作数嵌入在一起(称为E)。这使得类似[Data(2),Add]这样的非法状态不可能出现。从内存占用的角度来看,第一种方法和E方法应该是可比的。但是,E方法可能会有更多开销,因为每个Add等指令都需要引用其操作数,无论是常量还是变量。比如1+2+3+4需要存储、加载和加法等操作,而第一种选择只需要将其编码为1 2 3 4 Add Add Add。如果在该语言中频繁执行添加多个数字的操作,可以使用类似1 2 3 4 4 AddNMany的方式来优化。

作者认为,第二种方法在大小上应该是最小的。

[
https://old.reddit.com/u/Ok-Watercress-9624
](
https://old.reddit.com/u/Ok-Watercress-9624
)
    


### TITLE

这个版本更新包括以下几个主要内容:

1. 实现了对元组类型的Writer trait支持,允许将元组直接写入响应流中。

2. 增加了对中间件包装Handler的支持,可以方便地给Handler添加中间件功能。

3. 更新了force_https文档并添加了示例。

4. 添加了SecureMaxSize中间件来控制请求的最大大小。

5. 修复了OpenAPI文档中的一些错误和笔误。

6. 修复了RequestId头使用错误的键名的问题。

7. 为OpenAPI提取器添加了默认必填值支持。

8. 重构了OpenAPI的功能特性。

9. 添加了对非严格整数的解析支持。

10. 为OpenAPI添加了扩展支持。

11. 更新了AnyValue解析器。 

12. 为CORS中间件添加了CallNext选项来控制是否继续执行后续处理器。

13. 为OpenAPI添加了链接支持。

14. 修复了一个错误状态码被错误设置为405的问题。

此外,本次版本更新还修复了一些其他问题,并由多位贡献者参与。

[
https://github.com/salvo-rs/salvo/releases/tag/v0.72.0
](
https://github.com/salvo-rs/salvo/releases/tag/v0.72.0
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

