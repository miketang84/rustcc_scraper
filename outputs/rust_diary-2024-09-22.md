【Rust日报】2024-09-22


### TITLE

以下是对该博客文章的中文总结:

Visual Studio Code (VS Code)网页版本(vscode.dev)一直以来的目标是在浏览器中支持编辑/编译/调试等完整的开发周期。对于JavaScript和TypeScript等语言相对容易,因为浏览器自带JavaScript执行引擎。但对于其他语言就更加困难,因为需要能够在浏览器中执行(和调试)代码。

WebAssembly是一种针对虚拟机的二进制指令格式,现代浏览器都内置了WebAssembly虚拟机,同时也有工具链可以将C/C++代码编译为WebAssembly代码。为了探索WebAssembly的潜力,VS Code团队将用C/C++编写的Python解释器编译为WebAssembly,并在vscode.dev中运行。

该博客详细介绍了具体的实现方式。主要思路是利用WASI (WebAssembly System Interface)规范,WASI定义了类似操作系统的一些功能,如文件系统、套接字等。通过WASI SDK将C/C++代码编译为WebAssembly,然后在VS Code的扩展主机Worker中运行WebAssembly,并将WASI API连接到VS Code的API上,比如将printf输出连接到VS Code的终端。

文章还展示了如何在VS Code扩展中加载和运行WebAssembly代码,包括编写扩展配置清单、加载WebAssembly模块、创建WASM进程等步骤。总的来说,通过WebAssembly和WASI,VS Code网页版本可以在浏览器中执行各种语言的代码,支持完整的开发工作流程,提高了网页端开发体验。

[https://code.visualstudio.com/blogs/2023/06/05/vscode-wasm-wasi
](https://code.visualstudio.com/blogs/2023/06/05/vscode-wasm-wasi
)
    


### TITLE

这是一个vscode扩展,允许对HLSL/GLSL/WGSL着色器进行语法高亮、lint和符号提供。它使用shader-language-server通过公共验证器API对着色器进行lint。目前支持以下特性和语言:

- 语法高亮:为代码提供改进的语法高亮显示
- 诊断:在用户输入代码时提供错误和警告
- 符号提供程序:提供跳转、补全、悬停和签名功能
- 本地符号:提供所有用户创建的符号
- 内部符号:提供所有语言提供的内置符号

不同语言支持不同特性,具体如下:

语言 | 语法高亮 | 诊断 | 本地符号 | 内置符号
--- | --- | --- | --- | ---
GLSL | ✅ | ✅ | ✅ | ✅
HLSL | ✅ | ✅ | ✅ | 部分支持
WGSL | ✅ | ✅ | 不支持 | 不支持

该扩展在不同平台上的支持情况不同:
- Windows: 完整功能集
- Mac和Linux: 依赖WASI版本的服务器,与Web版本类似,请参阅Web支持部分的限制
- Web: 在vscode.dev上运行,依赖WebAssembly,无法使用DXC,而是使用功能较少的Glslang

该扩展基于PolyMeilex的vscode-wgsl扩展,做了大量修改。

[
https://github.com/antaalt/shader-validator
](
https://github.com/antaalt/shader-validator
)
    


### TITLE

这个仓库包含了一个用Rust编写的机器学习库Zyx。Zyx的主要特点是可编译后端,它可以自动生成针对CUDA和OpenCL优化的内核。Zyx采用延迟执行,只有在显式要求结果时才会执行。所有张量都是可微分的。

Zyx的语法类似于PyTorch,支持创建随机张量、进行基本运算以及自动微分等功能。它支持多种后端设备,包括CUDA、OpenCL和WGPU。Zyx会自动尝试利用所有可用设备,也可以手动配置。

文档还提供了一个简单的神经网络示例,展示了如何创建线性层、进行前向传播、计算损失、反向传播和使用优化器更新网络参数。

Zyx的目标是正确性、硬件支持和性能。目前只支持最新的Rust稳定版,需要标准库。

文档还警告说,Zyx违背了很多整洁代码的原则,包括抽象、包装、虚函数、泛型、生命周期等,以换取更好的性能。同时也使用了一些不安全代码、全局变量和代码重复,如果你不能接受这些,请不要使用Zyx。

最后,文档提到了行为准则、贡献方式和双重许可证(MIT和Apache 2.0)。

[
https://github.com/zk4x/zyx
](
https://github.com/zk4x/zyx
)
    


### TITLE

这个存储库是一个名为 Hyperion 的雄心勃勃的 Minecraft 项目,旨在打破由 EVE Online 保持的同时8825人连线的吉尼斯世界纪录,让10,000名玩家同时在一个 Minecraft 世界中进行 PvP。该架构采用 ECS 驱动,使用 Rust 语言的 Flecs 库。该项目旨在创建一个类似于 Overcast Network 的大型活动(但与其无关)。

该仓库列出了项目的各个功能区域及其完成状态,包括照明、方块机制、世界生成、渲染、物品栏、战斗、世界持久性、物理、游戏机制、音频和模块化等。一些核心功能如放置/破坏方块、PvP 等尚未实现。

该项目提供了调试模式和发布模式的构建说明。开发者还建立了一个 Discord 服务器来讨论最新进展并招募新的贡献者。总的来说,这是一个极具野心的大规模多人 Minecraft 游戏项目。

[
https://github.com/andrewgazelka/hyperion
](
https://github.com/andrewgazelka/hyperion
)
    


### TITLE

这是一个关于Flecs Rust API的介绍。Flecs是一个快速、轻量级的实体组件系统(ECS),可用于构建游戏和模拟,支持数百万实体。Flecs Rust API是对Flecs C API的包装,为Rust开发者提供了直观、简化的接口。主要特点包括:

1. 高性能,可移植,多语言绑定。
2. 全面支持实体关系。
3. 支持分层和预制体。
4. 高效的数据存储,每帧可处理数百万实体。
5. 支持数百个组件,数万种原型。
6. 自动组件注册。
7. 多核心调度,无锁。
8. 反射框架,支持JSON序列化等。
9. 强大的查询语言。
10. 性能分析附加组件。

该项目目前处于Alpha阶段,核心功能已移植,但API仍在实验中。未来将实现与C++ API同等功能,并解决任何安全性问题。项目旨在提供安全、惯用和高效的Rust API接口。欢迎反馈和贡献。

[
https://github.com/Indra-db/Flecs-Rust
](
https://github.com/Indra-db/Flecs-Rust
)
    


### TITLE

根据吉尼斯世界纪录的信息,在2020年10月6日,EVE Online这款在线游戏创造了最大规模的玩家对战纪录。这场名为"Fury at FWST-8"的战役中,有8,825名玩家参与其中。这是帕皮联盟(PAPI)试图在Delve地区建立一个前沿基地的尝试,而帝国则努力阻止这一行动。战斗最终导致6,746艘舰船和一个Keepstar前哨基地被摧毁,损失了362艘资本级舰船,总计约合18,712美元的成本。这一纪录代表了大规模多人在线游戏中最激烈的玩家对抗性战役。

[
https://www.guinnessworldrecords.com/world-records/105603-largest-videogame-pvp-battle
](
https://www.guinnessworldrecords.com/world-records/105603-largest-videogame-pvp-battle
)
    


### TITLE

这位开发者热爱Rust语言,希望能够使用Rust来编程Arduino板。他有一个涉及人脸识别和网络应用的项目想要实现,但对硬件电路部分缺乏经验。他找到了一个Arduino兼容开发板,但不太清楚如何开始学习使用Rust编程Arduino。他希望能获得一些学习资源和学习路径的建议,以便正确地学习如何将Rust与Arduino硬件集成,整合各种组件到系统中。任何相关的帮助对他都将非常有用。

[
https://old.reddit.com/r/rust/comments/1fmwgux/using_rust_for_working_with_arduino_uno_r3/
](
https://old.reddit.com/r/rust/comments/1fmwgux/using_rust_for_working_with_arduino_uno_r3/
)
    


### TITLE

这篇内容是关于一个名为 shader-validator 的 VSCode 扩展程序的介绍。该扩展基于语言服务器协议 (LSP)，支持以下功能:

- 诊断: 依赖验证器 API (glslang 用于 GLSL、dxc 用于 HLSL、naga 用于 WGSL)
- 符号: 提供跳转、悬停、签名和自动补全功能
- 语法高亮: 比 VSCode 内置的语法高亮更好

该扩展也支持 VSCode 的网页版 vscode.dev。

扩展的核心是一个用 Rust 编写的语言服务器,遵循 LSP 协议,可以与任何支持该协议的 IDE 集成。为了支持网页版,服务器可以编译为 WASI 格式,在 VSCode 中使用新增的 WASI 支持运行。因为 dxc 编译器不支持 WASI,所以 HLSL 的支持在网页版中会回退到 glslang(支持 HLSL 但功能较少)。

未来的开发计划包括:
- 添加 HLSL 所有内置函数
- 改进对 WGSL 的支持
- 改进符号提供器(可能使用 tree-sitter)

总的来说,这是一个增强 VSCode 中shader编写体验的扩展程序。

[
https://old.reddit.com/r/GraphicsProgramming/comments/1fms2ps/shadervalidator_a_shader_language_server_for_hlsl/
](
https://old.reddit.com/r/GraphicsProgramming/comments/1fms2ps/shadervalidator_a_shader_language_server_for_hlsl/
)
    


### TITLE

这篇文章介绍了作者创建的一个用Rust编写的机器学习库zyx。与传统的PyTorch等库采用手写优化内核的方式不同,zyx采用了自动优化内核的策略。它只包含有限的基础指令集,如元素运算、数据移动和归约运算,然后通过搜索方式在不同硬件上获得最大性能。

作者认为,这种自动优化策略能够很好地支持各种硬件,而无需为每个平台和设备手动优化。他提出了一些需要完成的工作,如寄存器分块优化、Tensor核心支持等,以使zyx能够达到PyTorch的性能水平。

最后,作者邀请读者对这个自动优化的理念提供反馈,也欢迎大家为zyx做出贡献,使其成为一个稳定可靠的机器学习库选择。他还提到使用AI辅助完成了部分文档的撰写工作。

[
https://old.reddit.com/r/rust/comments/1fmu9dy/searched_vs_hardcoded_code_in_ml_libraries/
](
https://old.reddit.com/r/rust/comments/1fmu9dy/searched_vs_hardcoded_code_in_ml_libraries/
)
    


### TITLE

这篇文章是关于 git-cliff 2.6.0 版本的新特性介绍。git-cliff 是一个命令行工具,用于从 Git 历史记录生成高度可定制的变更日志。主要新增功能包括:

1. 弃用了一些集成字段,改为使用 commit.remote 字段。
2. 增加了 --use-branch-tags 选项,支持为不同分支生成不同的变更日志。
3. 新增 render_always 选项,允许在无更改时仍渲染变更日志。
4. 可从配置文件指定输出文件路径。
5. 改进了 TypeScript API,完善了选项文档和 skipCommit 选项支持数组输入。
6. 去除提交信息末尾的换行符,方便使用正则表达式处理。
7. 优化了示例模板,使之更加直观和符合约定。
8. 其他一些错误修复和改进。
9. 感谢贡献者并呼吁更多贡献。
10. 提供了赞助开发者的途径。

[
https://git-cliff.org/blog/2.6.0/
](
https://git-cliff.org/blog/2.6.0/
)
    


### TITLE

ene-kafka是一个易于使用的Rust语言Kafka客户端,旨在简化从Rust(微)服务与Kafka的通信并使其更加直观。ene-kafka的目标是加速Rust中与Kafka相关项目(如事件驱动服务)的交付速度,并拥有可扩展的Kafka客户端。目前Rust的Kafka选择相当有限,ene-kafka使用rdkafka作为底层Kafka客户端。

从消费者端来看,ene-kafka采用消费者->调度器->处理程序的架构。作为库用户,您主要需要定义事件处理程序(当特定类型的事件到来时要做什么)并配置消费者。

最新版本优化了处理程序和调度器的代码生成,消除了冗余步骤。之前的做法是为每个处理程序类型生成一个枚举变体,在调度时搜索处理程序向量。新版本直接在调度器中为每个处理程序生成结构字段,在调度时检查每个字段,找到正确的处理程序。这种方式更加简单高效。

ene-kafka是作者学习Rust宏系统、Kafka和Rust的重要项目,采用Apache或MIT许可,欢迎反馈、贡献或使用。

[
https://old.reddit.com/r/rust/comments/1fmt4u7/new_update_of_enekafka_an_easy_to_use_kafka/
](
https://old.reddit.com/r/rust/comments/1fmt4u7/new_update_of_enekafka_an_easy_to_use_kafka/
)
    


### TITLE

这位开发者正在寻求建议,希望找到最适合开发60fps波形可视化应用程序的Rust GUI库。他的具体需求是将帧缓冲区水平分割成两部分,上半部分显示过去5秒的音频输入波形,下半部分显示经过低通滤波后的数据。

他已经了解了minifb和plotters两个库,并且用它们创建了一些基本的东西。他认为plotters非常酷,但是它似乎更侧重于静态渲染或低帧率可视化,而不是60fps/实时动画。而minifb则很灵活,但需要自己做更多工作。

他有多年使用Rust的经验,但在GUI编程和可视化方面只了解最基本的知识。因此,他希望能获得一些建议,了解哪个库最适合实现他的需求。

[
https://old.reddit.com/r/rust/comments/1fn2bvl/which_gui_library_can_you_recommend_for_60_fps/
](
https://old.reddit.com/r/rust/comments/1fn2bvl/which_gui_library_can_you_recommend_for_60_fps/
)
    


### TITLE

该仓库介绍了一个名为 Peggen 的解析表达式语法(PEG)的解析器生成器。它使用内联宏来指定 PEG 操作。以下是该仓库的一些主要内容:

1. 路线图和需求帮助
   - 提出了一些优化和改进的建议,如优化规则分派、更精简的标记表示、自定义错误处理等。

2. 与其他解析器的区别
   - 与 PEST、Chumsky、LALRPOP 等解析器进行了比较,说明了 Peggen 在生成 AST、左递归处理、错误处理等方面的优势和差异。

3. 性能比较 
   - 与 Chumsky 在 JSON 解析任务上进行了基准测试,显示 Peggen 具有更好的性能。

4. JSON 解析器教程
   - 分步骤介绍了如何使用 Peggen 构建一个 JSON 解析器,包括格式化字符串、正则表达式的使用、重复模式的处理等。

5. 实际示例
   - 给出了一个使用 Peggen 生成的 JSON 解析器的最终代码示例。

总的来说,该仓库介绍了 Peggen 解析器生成器的特性、与其他解析器的对比,并通过 JSON 解析器示例向读者展示了如何使用 Peggen 构建解析器。

[
https://github.com/Y-jiji/peggen
](
https://github.com/Y-jiji/peggen
)
    


### TITLE

总结:

这位高中生想开始从事更加严肃的编程项目,而不仅仅是练习编程。他之前用Python做过一些机器学习,用C++参加过编程竞赛,但从未真正开发过任何东西。他想知道Rust是否适合作为开始构建更大型项目的编程语言。作为一名热衷于学习新事物的高中生,他希望能够在Rust中开启自己的项目开发之旅。

[
https://old.reddit.com/r/rust/comments/1fmxpm9/should_i_start_with_rust/
](
https://old.reddit.com/r/rust/comments/1fmxpm9/should_i_start_with_rust/
)
    


### TITLE

这是关于 Gitoxide 项目的最新进展总结。主要内容包括:

1. 开源维护时间达到了156小时,其中48小时用于 Gitoxide 项目。

2. 引入了一个新的内部工具 (it tool),用于生成更好的测试用例,以改进 Gitoxide 重命名跟踪功能。

3. 根据 GitButler 的需求,对 API 进行了多项改进,使其更加方便实用。

4. GitButler 的性能得到显著提升,分支详情生成速度提高2.6倍。

5. 关注了 Gitoxide CLI 中潜在的安全风险,计划采取缓解措施。

6. 正在调查通过执行 git 命令来滥用 Gitoxide 的风险。

7. 社区贡献者正在实现 "blame" 算法,旨在革新 gitui 中的 blame 功能。

8. 在柏林举办了 Git 见面会,介绍了多个项目。

9. 关于 Sovereign Tech Fund 的进展缓慢。

10. 发现并修复了 Gitoxide 处理 .gitignore 排除规则的问题。

总的来说,是对 Gitoxide 项目在多个方面的最新进展的总结,包括性能优化、新功能开发、安全考虑和社区合作等。

[
https://github.com/Byron/gitoxide/discussions/1602
](
https://github.com/Byron/gitoxide/discussions/1602
)
    


### TITLE

这是关于一个名为 rustc_codegen_clr 的 Rust 到 .NET 编译器项目的文章。该项目旨在实现一个 Rust 编译器后端,将 Rust 代码编译成 .NET 公共中间语言(CIL),从而允许在 .NET 运行时上执行 Rust 代码。文章主要讨论了 Rust 中恐慌(panic)和展开(unwind)的实现细节。

主要内容包括:

1. 介绍了 rustc_codegen_clr 项目的工作原理,它如何将 Rust 中级中间表示(MIR)转换为 .NET CIL代码。

2. 解释了恐慌(panic)和展开(unwind)的区别。恐慌是 Rust 语言的一个特性,当程序逻辑出错时会触发;而展开是实现恐慌的一种方式,涉及遍历调用栈并清理堆栈上的数据。

3. 详细介绍了展开的工作原理,包括如何通过"清理块"(cleanup block)清理每个堆栈帧上的数据。

4. 使用伪代码示例说明了清理块是如何生成和工作的。

5. 比较了 Rust 中的展开实现与 C# 中的 finally 和 using 语句的异同。

总的来说,这篇文章着重解释了 Rust 到 .NET 编译器如何在 .NET 平台上实现 Rust 语言的关键特性 —— 恐慌和展开。

[
https://fractalfir.github.io/generated_html/rustc_codegen_clr_v0_2_1.html
](
https://fractalfir.github.io/generated_html/rustc_codegen_clr_v0_2_1.html
)
    


### TITLE

这是一个关于 Hyperion 项目的介绍,Hyperion 是一个高性能的 Minecraft 游戏引擎,使用 Rust 语言构建,基于 flecs 库。该项目的目标是打破 EVE Online 8825 玩家的在线记录,因为原生的 Minecraft 实现只能支持几百个玩家。

与其他 Minecraft 服务器项目不同,Hyperion 采用了模块化设计,只实现需要的功能,从而专注于支持大型定制活动,如 Hypixel 等。根据目前的性能,Hyperion 估计可以支持约 5 万名同时在线玩家。该项目已经与一些创作者取得联系,他们有意使用 Hyperion 进行 YouTube 或直播内容制作。相关信息包括 GitHub 仓库链接和 Discord 服务器链接。

[
https://old.reddit.com/r/rust/comments/1fmznnh/hyperion_10k_player_minecraft_game_engine/
](
https://old.reddit.com/r/rust/comments/1fmznnh/hyperion_10k_player_minecraft_game_engine/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

