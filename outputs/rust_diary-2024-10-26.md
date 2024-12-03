【Rust日报】2024-10-26


### TITLE

根据上述内容,这是丰田Tacoma皮卡车的官方介绍。该车型提供多种不同的车型版本,包括Tacoma TRD Off-Road Premium、Tacoma Hybrid Trailhunter、Tacoma TRD Sport、Tacoma Hybrid Limited和Tacoma Hybrid TRD Pro等。这些车型拥有不同的外观设计和内饰配置。官方介绍展示了这些车型的各种颜色选择,如Solar Octane、Bronze Oxide、Blue Crush Metallic、Wind Chill Pearl、Terra、White with Black Roof等。同时也介绍了一些车型的内饰设计和可选配件。总的来说,这是对丰田Tacoma皮卡这一车型系列的详细官方产品介绍。

[https://www.toyota.ca/toyota/en/vehicles/tacoma/overview?utm_source=Reddit&utm_medium=display&utm_campaign=FY25_Tacoma_EN_TSC-TAC-16_Mega_Post_EN&utm_content=Mega_Post_EN
](https://www.toyota.ca/toyota/en/vehicles/tacoma/overview?utm_source=Reddit&utm_medium=display&utm_campaign=FY25_Tacoma_EN_TSC-TAC-16_Mega_Post_EN&utm_content=Mega_Post_EN
)
    


### TITLE

这是Reddit上的ToyotaTacoma社区,包含了144,280名读者。该社区制定了一些规则和礼仪准则,例如:

1. 禁止骚扰、仇恨言论、垃圾邮件等行为,否则将被立即封禁。

2. 禁止发布可能会分散驾驶员注意力的照片或视频,如速度表显示超过0英里/小时。

3. 该社区不允许买卖或交易相关物品,有专门的r/ToyotaTacomaSales子reddit供此用途。

4. 帖子和评论必须与主题相关,即与卡车相关。

5. 禁止政治相关的讨论或表达政治立场。

6. 不允许询问车辆估价或好坏交易。

7. 禁止发布meme等娱乐图片。

该社区还列出了一些相关的子reddit,如Tacoma World、Toyota Tacoma Sales等。总的来说,这是一个以Toyota Tacoma卡车为中心的enthusiast社区,制定了一些规范来维护社区氛围。

[
https://old.reddit.com/r/ToyotaTacoma
](
https://old.reddit.com/r/ToyotaTacoma
)
    


### TITLE

这是一个介绍Smithay项目的手册,Smithay是一个用Rust语言构建Wayland相关软件的框架。

该项目包括3个主要组件:

1. wayland-rs仓库包含了对Wayland协议的低级别绑定,主要有wayland-client和wayland-server两个crate,分别用于客户端和服务器端应用。

2. SCTK(Smithay Client ToolKit)是一个用于编写Wayland客户端应用的crate,它建立在wayland-client之上,处理了编写客户端所需的很多基础工作。

3. Smithay是该项目的旗舰crate,旨在作为编写Wayland服务器(合成器)的框架,建立在wayland-server之上,并处理与系统的交互(输入设备、图形、udev、会话等)。

本手册的第一部分专注于客户端应用,第二部分则关注服务器端。如果感兴趣的是服务器端,建议先熟悉客户端部分,因为客户端入门相对更容易,并且很多概念在服务器端也适用。

[
https://smithay.github.io/book/
](
https://smithay.github.io/book/
)
    


### TITLE

这是一个名为Fennec的PHP工具链项目的介绍。主要内容如下:

1. Fennec是一个受Rust编程语言及其工具链启发而开发的PHP工具链,旨在为PHP开发者提供一套工具来编写更好的代码。

2. Fennec目前处于早期开发阶段,功能尚未完全实现,现有功能也可能发生变化或中断。但作者正在公开开发以与社区分享进展。

3. Fennec计划实现的核心功能包括字符串内联、词法分析器、抽象语法树、解析器、源码管理、AST遍历、名称解析、代码修复、错误报告、语义分析、符号表、代码检查、配置管理、字符串格式转换、代码格式化、静态分析、重构、代码生成、文档生成、测试运行等。

4. 作者希望社区提出建议、撰写文档、贡献代码、资助项目、设计Logo等方式参与Fennec的开发。

5. Fennec受到了多个Rust编写的其他工具和项目的启发,如Clippy、OXC、php-rust-tools等。

6. 作者对一些已经为PHP社区做出重大贡献的工具如PHP CS Fixer、Psalm、PHPStan等表示感谢。

7. Fennec采用MIT和Apache 2.0双重许可证。

总的来说,这是一个雄心勃勃的PHP工具链项目,希望最终为PHP开发者提供一站式的编码辅助工具。

[
http://github.com/carthage-software/fennec
](
http://github.com/carthage-software/fennec
)
    


### TITLE

这是一个名为Motion的裸机物理引擎的介绍。Motion是一个用Rust编写的物理引擎,可以轻松快速地进行模拟。

文档首先展示了如何建立一个简单的事件循环,用于模拟物理过程。然后介绍了如何创建一个更复杂的2D对象,并设置其位置、大小、形状和速度等参数。

最后解释了为什么选择Rust编程语言来开发Motion。Rust是一种运行速度快、高效的语言,非常适合开发物理引擎。同时Rust也很灵活,可以让Motion在各种环境中使用。

[
https://github.com/juanperias/motion
](
https://github.com/juanperias/motion
)
    


### TITLE

ContainerYard 是一种声明式、可重复和可重用的分散式方法,用于定义容器。它类似于 Nix flakes 和 Containerfiles(又称 Dockerfiles)的结合。ContainerYard 将 Containerfiles 分解为模块,每个模块代表容器的一些特定功能,例如 rust 模块定义了 rust 的安装。模块还支持 Tera 模板。一个 yard.yaml 文件用于将模块组合成 Containerfiles。

通过运行 `yard build` 命令,可以根据 yard.yaml 中定义的模块生成 Containerfile。每个模块由一个 Containerfile Tera 模板和一个 yard-module.yaml 文件组成,后者定义了模板的配置选项和依赖项。

ContainerYard 旨在促进一个可组合 Containerfile 模块的库生态系统。用户可以导入这些模块而无需太多配置。需要 Rust?只需将其添加到 yard.yaml 文件中。需要 Flutter?同样操作。使用 ContainerYard,您无需再定义某些 Containerfile 配置。如果您确实需要定制,ContainerYard 不会阻碍您,因为一切都基于 Containerfile,输出是纯 Containerfile。

ContainerYard 与 Nix Flakes 的区别在于,Nix Flakes 以开发人员灵活性为代价保证了可重复性,而 ContainerYard 是去中心化的,允许用户轻松使用不同的包管理器和上游源,因此牺牲了一些可重复性保证,但获得了完全的开发人员灵活性。ContainerYard 非常简单,建立在熟悉的开发人员工具(Containerfiles 和 Tera 模板)之上。

[
https://github.com/mcmah309/containeryard
](
https://github.com/mcmah309/containeryard
)
    


### TITLE

这是一个GitHub仓库的链接,名为yard_module_repository,由用户mcmah309创建。该仓库包含几个目录:bases、dependent、independent,以及许可证文件LICENSE和自述文件README.md。仓库目前有3个星星,可以查看代码、提交问题、拉取请求等。该仓库使用Apache-2.0许可证,是一个公开的代码仓库。总的来说,这是一个存储某些模块化代码的代码仓库。

[
https://github.com/mcmah309/yard_module_repository
](
https://github.com/mcmah309/yard_module_repository
)
    


### TITLE

这是一个用Rust编写的嵌入式脚本语言基准测试项目。它对比了几种不同的Rust嵌入式脚本解决方案在各种场景下的性能表现。测试用例包括创建对象、调用函数、排序对象等。该项目提供了详细的基准测试结果分析,旨在帮助开发者选择最适合自己需求的嵌入式脚本语言解决方案。

项目的主要内容包括:

1. 基准测试代码
2. 测试脚本样例
3. Cargo配置文件 
4. README文档
5. 基准测试结果图表
6. Python脚本用于运行基准测试

总的来说,这是一个针对Rust嵌入式脚本语言进行全面性能评测的项目。

[
https://github.com/khvzak/script-bench-rs
](
https://github.com/khvzak/script-bench-rs
)
    


### TITLE

该更新日志记录了mlua(一个Rust语言的Lua绑定库)从0.9.0版本到0.10.0版本的变更内容,主要包括以下几个方面:

1. 性能优化,如更快的表遍历、序列化、Lua函数调用等。

2. Luau支持,包括buffer类型、二进制模块、包管理等。

3. 错误处理改进,如Error::chain、Value::is_error等。

4. 类型支持增强,如Either、OsString/OsStr、PathBuf/Path等。

5. API更新,如Lua::scope暂时移除后重新添加、Lua::replace_registry_value改变等。

6. 内部重构,如移除owned类型、使类型真正可发送(Send)等。

7. WebAssembly支持。

8. 实验性特性,如vector元表、OwnedThread等。

9. 一些bug修复。

总的来说,这是一个重大版本升级,带来了性能提升、更多功能支持以及内部重构等多方面改进。

[
https://github.com/mlua-rs/mlua/blob/main/CHANGELOG.md
](
https://github.com/mlua-rs/mlua/blob/main/CHANGELOG.md
)
    


### TITLE

这是一个包含两个函数的 LLVM IR 代码片段。

第一个函数 `@func` 接受一个 `i32` 类型的参数 `%0`。它比较 `%0` 是否等于 5。如果相等,则将 `%1` 变量存储为 0,否则不做任何操作。最后它返回 `%1` 的值。

第二个函数 `@main` 首先将常量 5 存储在 `%0` 中,然后调用 `@func` 函数并将结果存储在 `%1` 中。接着它将 `%1` 加 1 并将结果作为程序的返回值。

该代码片段还包含一些注释和运行指令,用于编译和执行该程序。它们指定了如何使用 `cargo` 和 `gcc` 来编译并运行该程序。

[
https://github.com/Cr0a3/ygen/blob/main/tests%2Fshared_vars1.yl#L7-L33
](
https://github.com/Cr0a3/ygen/blob/main/tests%2Fshared_vars1.yl#L7-L33
)
    


### TITLE

这是一个名为Ygen的项目,旨在提供一个用于构建快速、干净编译器的内存安全API工具包。该项目的主要关注点是简单性,API设计类似于LLVM,并广泛使用traits来实现函数重载。该项目目前还处于早期开发阶段,可能存在bug和错误编译。文中提供了一个简单的示例,展示了如何使用Ygen构建一个add函数。Ygen目前支持x64架构的完整IR,但暂不支持完整的ISA。该项目由Cr0a3拥有,采用Apache 2.0许可证。

[
https://github.com/Cr0a3/ygen
](
https://github.com/Cr0a3/ygen
)
    


### TITLE

这是Compiler Explorer的隐私政策总结。主要内容包括:

1. Compiler Explorer是由Matt Godbolt和一些志愿者在GitHub上维护的非商业项目,旨在保护用户数据隐私。

2. 用户输入的源代码会被发送到服务器进行编译和执行,输出结果会返回浏览器显示。源代码仅在编译期间暂存,之后会被删除,除非出现需要调试的问题才可能最多保留一周。

3. 分享代码时,短链接对应的界面状态和代码会存储在服务器上,管理员可访问。较长的URL则将状态编码在URL中。

4. 应用程序和Web日志会记录半匿名的IP地址,但不包含其他个人身份信息。如果使用短链接,理论上代码可能会暴露在Web日志中。

5. Amazon的相关系统日志会保留IP地址,用于调试和统计,最长保留一个月后永久删除。

6. 如果浏览器出错,会使用第三方服务上报错误及一些浏览器信息,不包括源代码。

总的来说,尽管会短暂保存源代码用于编译,但做了多方面的努力来保护用户隐私和数据安全。

[
https://godbolt.org/z/1bY3eq4EE
](
https://godbolt.org/z/1bY3eq4EE
)
    


### TITLE

这个Reddit帖子是在询问有没有一个命令行界面的TOML编辑工具。TOML是一种用于配置文件的简单格式。

作者没有找到专门的TOML CLI编辑工具,但猜测Cargo(Rust的包管理器和构建工具)自带的cargo add和cargo remove命令可以用于编辑Cargo.toml文件。

他想要一个这样的工作流程:使用cargo-generate创建一个库项目,然后将它添加到根Cargo.toml文件的[workspaces]部分中,这样就可以将该库作为工作空间的一部分进行管理和构建。

总的来说,这个帖子在寻求一个方便编辑TOML文件的命令行工具,特别是能够将生成的库项目整合到多工作空间的根Cargo.toml中。

[
https://old.reddit.com/r/rust/comments/1gcwdly/toml_edit_cli_tool/
](
https://old.reddit.com/r/rust/comments/1gcwdly/toml_edit_cli_tool/
)
    


### TITLE

这篇内容主要介绍了丰田Tacoma皮卡车型,特别是TRD Pro和全新的Trailhunter版本。文中强调Tacoma卓越的越野性能和动力,无论是攀登山峰、穿越泥地还是穿越茂密的森林,它都是探索野外的绝佳伴侣。

TRD Pro版搭载强大的混合动力系统,马力达326匹,扭矩高达465磅英尺,同时配备专业的越野减震器和人体工学座椅,确保高速行驶时的舒适性和操控稳定性。另外,它也具备强劲的牵引力,最高可拖5,950磅。

全新Trailhunter版则专为越野探险而打造,搭载Old Man Emu专业越野悬挂,18寸全地形轮毂,并配备多角度地形显示系统和爬行控制系统,让您尽情征服最极限的地形。

总之,无论是TRD Pro还是Trailhunter,Tacoma都是征服野外、追求自由的不二之选,让您尽情挑战自我极限,体验前所未有的冒险乐趣。

[
https://old.reddit.com/user/Toyota_Canada/comments/1fi7whf/toyota_tacoma_is_your_invitation_to_the_wild/
](
https://old.reddit.com/user/Toyota_Canada/comments/1fi7whf/toyota_tacoma_is_your_invitation_to_the_wild/
)
    


### TITLE

这位初学者来自其他编程语言背景,习惯了避免代码深度嵌套的编码风格,比如提前返回、继续等。然而在Rust中,他发现常见的模式像`if let`链会导致嵌套很深的代码。虽然可以用`?`操作符减轻部分情况,但在实现trait时无法改变函数签名。

他目前的做法是在不返回Option/Result的函数中,对可能的None/Err提前unwrap并返回,但觉得这可能不太idimatic。他想征求Rust经验丰富的开发者的意见,是否有更好的方式避免这种深度嵌套,还是就"接受"这种风格。

[
https://old.reddit.com/r/rust/comments/1gc9r2x/how_to_avoid_deeply_nested_if_let_chains/
](
https://old.reddit.com/r/rust/comments/1gc9r2x/how_to_avoid_deeply_nested_if_let_chains/
)
    


### TITLE

这是一个开源项目名为"maudify",旨在将HTML转换为Maud(Rust语言的HTML模板引擎)格式。项目由Stuart Hildreth创建并使用MIT许可证。该仓库包含Rust源代码、Cargo配置文件、README文件和LICENSE文件。用户可以fork该项目,给出star评价,查看文件树结构等。该项目还提供了代码克隆的多种方式,包括HTTPS、SSH、GitHub CLI和其他图形界面工具。总的来说,这是一个帮助Rust开发者将HTML转换为Maud模板的实用工具。

[
https://github.com/stuarth/maudify/
](
https://github.com/stuarth/maudify/
)
    


### TITLE

这是一个来自Reddit的rust子版块的帖子,主题是关于如何使用Rust编写Wayland合成器(compositor)。

作者表示想学习如何使用Rust制作Wayland合成器,但找不到相关资源。他/她提到smithay是一个主要用于制作Wayland合成器的Rust库,它有一本书,但这本书很短且不完整。

作者在寻求有编写Wayland合成器经验的人的建议,希望获得开始学习的指导和资源推荐。

[
https://old.reddit.com/r/rust/comments/1gcqkq1/are_there_any_resources_on_how_to_make_a_wayland/
](
https://old.reddit.com/r/rust/comments/1gcqkq1/are_there_any_resources_on_how_to_make_a_wayland/
)
    


### TITLE

这个GitHub仓库包含了一个用Rust语言编写的API,名为Space Monitor。它的主要功能是在macOS系统上订阅实时变化,以获取当前活动的虚拟桌面(Space)索引。该项目受到George Christou用Swift编写的WhichSpace项目的很大启发。

该仓库提供了示例代码,演示了如何以异步和同步的方式获取当前Space的编号。它也解释了为什么需要开发这个库 - 作者正在进行一个有趣的项目,需要更好地管理macOS上的Space和窗口,这需要实时查询虚拟显示ID对应的Space索引。

该库依赖于CoreGraphics和Cocoa等macOS原生API,使用FFI(外部函数接口)从Rust调用C语言API。它实际上是对一些低级macOS内部API的Rust绑定,使得访问这些API变得简单高效。文档还提供了用Swift实现的简单示例代码。

不过,由于该库使用了苹果的私有、未公开API,如果将它集成到应用程序中,可能会被拒绝上架App Store。但用户仍然可以从外部安装该应用程序。

[
https://github.com/Schachte/space-monitor-rs
](
https://github.com/Schachte/space-monitor-rs
)
    


### TITLE

这是一个关于一个名为 Fennec 的新 PHP 工具链项目的介绍。Fennec 受到了 Rust 工具链和理念的启发,旨在为 PHP 带来 Rust 生态系统的稳健性和以开发者为中心的设计理念,具有诸如lint检查、代码格式化和静态分析等功能。

该项目仍处于非常早期阶段,目前只实现了基本的 lint 检查和少数检查规则。但长期目标是让 Fennec 成为一个完整的工具链,用于现代化 PHP 工具。

作者正在开源这个项目,希望能获得反馈和建议,帮助将 Rust 工具的理念带入 PHP 生态。代码仓库位于 github.com/carthage-software/fennec。

[
https://old.reddit.com/r/rust/comments/1gcfm2i/fennec_an_earlystage_oxidized_toolchain_for_php/
](
https://old.reddit.com/r/rust/comments/1gcfm2i/fennec_an_earlystage_oxidized_toolchain_for_php/
)
    


### TITLE

这是一位开发者分享了自己一个使用Rust编写的裸机物理引擎项目。该项目目前实现了简单的基础系统和碰撞检测系统,但仍在不断改进和添加新功能。他提供了项目的crates.io网址和GitHub仓库链接,并希望能获得反馈意见,以便进一步完善这个物理引擎。这个项目展示了Rust在系统级编程、游戏开发等领域的潜力,同时也体现了开源社区的互帮互助精神。

[
https://old.reddit.com/r/rust/comments/1gcsvmy/my_bare_metal_physics_engine_in_rust/
](
https://old.reddit.com/r/rust/comments/1gcsvmy/my_bare_metal_physics_engine_in_rust/
)
    


### TITLE

该内容讨论了Rust编程语言中Option<Option<T>>类型的大小优化问题。具体来说:

1. 在Rust中,Option<Option<T>>的大小通常比预期的Option<T>大,因为它需要存储两个标记位而不是一个。

2.这引发了关于Rust类型系统的根本限制还是优化机会被错过的讨论。

3. 一些人认为这是类型系统的固有限制,因为编译器无法在编译时确定嵌套Option是否为None。

4. 另一些人则认为这可能是一个被忽视的优化机会,因为在某些情况下,编译器可以推断嵌套Option实际上是Some(None),从而优化内存使用。

5. 总的来说,这个问题揭示了类型系统设计和编译器优化之间的权衡,同时也反映了Rust社区对语言改进的持续探索。

[
https://old.reddit.com/r/rust/comments/1gchzlt/why_cant_rust_optimize_the_size_of_optionoptiont/
](
https://old.reddit.com/r/rust/comments/1gchzlt/why_cant_rust_optimize_the_size_of_optionoptiont/
)
    


### TITLE

ContainerYard是一种声明式、可重现和可重用的去中心化方法,用于定义容器。它类似于Nix flakes结合了Containerfiles(也称为Dockerfiles)的概念。

Containerfiles被分割成可重用和可配置的"模块"部分,可以组合这些模块来构建Containerfiles。例如,您可以从远程存储库获取基础模块(如Ubuntu基础映像)、Git配置模块和Bash风格模块,然后将它们组合成不同的Containerfiles。

这种方法的优点是,您可以一次性编写配置,然后在将来轻松重用和增量改进它。配置文件使用YAML格式,包含输入(远程模块)、输出(Containerfiles组合)和构建钩子等部分。

ContainerYard项目托管在GitHub上,地址为https://github.com/mcmah309/containeryard。

[
https://old.reddit.com/r/rust/comments/1gcx5b9/containeryard_a_declarative_reproducible_and/
](
https://old.reddit.com/r/rust/comments/1gcx5b9/containeryard_a_declarative_reproducible_and/
)
    


### TITLE

该帖子宣布了一个名为 Statement 的 Rust 开源库。Statement 是一个非常灵活的事件驱动状态机库,旨在管理某种类型的多个实体的状态。与一些其他 Rust 状态机库不同,Statement 在运行时定义状态机,并且可以将副作用附加到状态转换上,从而在实体改变状态时驱动应用程序。作者希望能收到建设性的反馈意见,并邀请大家查看该库的 crates.io 页面。

[
https://old.reddit.com/r/rust/comments/1gcxqyc/announcing_statement/
](
https://old.reddit.com/r/rust/comments/1gcxqyc/announcing_statement/
)
    


### TITLE

这是一份Rust语言中mlua库的新版本0.10.0发布公告。mlua是一个高级的Lua绑定库,支持Lua 5.4/5.3/5.2/5.1(包括LuaJIT)和Roblox Luau。该版本的重点是改善开发者体验和多线程支持。

最值得注意的变化包括:

1. 移除了Lua生命周期,Lua及其子类型现在都是'static的。
2. 通过send特性标志,实现了Send + Sync。
3. 添加了函数trait,可以将Rust函数包装为任意数量参数。
4. 通过Lua::exec_raw支持低级别的Lua C API。

该版本的完整更改日志可查看CHANGELOG,新版本比v0.9更快,性能基准测试见benchmarks。

[
https://old.reddit.com/r/rust/comments/1gcihk3/announcing_mlua_v010/
](
https://old.reddit.com/r/rust/comments/1gcihk3/announcing_mlua_v010/
)
    


### TITLE

总结:

这是一位13岁的年轻开发者的作品,他最近完成了一个名为ygen的编译器项目。ygen现在支持55%的LLVM IR节点,并针对生成的汇编代码和WebAssembly后端进行了优化。他创建了一个Pull Request,将ygen集成到Godbolt在线编译器浏览器中,因此无需安装即可在线尝试。他分享了ygen的GitHub仓库链接,并鼓励大家为其加星支持。他还介绍了一些命令行标志,如-O开启优化,-triple="wasm64-unkown"使用实验性WebAssembly后端。他提供了一个IR代码示例作为参考。最后,他表示由于是从手机上撰写此帖,如果格式有问题请谅解。

[
https://old.reddit.com/r/rust/comments/1gctr27/ygen_now_landed_in_godbolt/
](
https://old.reddit.com/r/rust/comments/1gctr27/ygen_now_landed_in_godbolt/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

