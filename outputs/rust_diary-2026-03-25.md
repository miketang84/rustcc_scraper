【Rust日报】2026-03-25


### TITLE

## wiki-route：维基百科最短路径查找工具

wiki-route 是一个用于查找任意两个维基百科文章之间最短路径的工具，可以解决"维基百科游戏"问题。

### 核心特点

- **功能**：找到连接任意两篇维基百科文章的最短超链接路径
- **架构**：采用客户端/服务器架构，通过Unix域套接字通信
- **性能**：查询响应近乎即时，因为图数据已预加载到内存中

### 工作原理

- **算法**：使用双向BFS（广度优先搜索）
  - 将维基百科文章视为有向图（文章=节点，超链接=边）
  - 同时从起点和终点进行搜索，逐层扩展
  - 当两个搜索前沿相遇时，保证找到最短路径
  - 比单向BFS快得多，因为搜索空间随深度呈指数增长

### 系统架构

- **服务器端** (`wiki-route-server`)：
  - 解析MediaWiki SQL转储文件
  - 在内存中构建图结构
  - 监听Unix套接字上的查询请求
  - 可将图数据预计算并保存为.bin文件，加快后续启动速度

- **客户端** (`wiki-route`)：
  - 轻量级CLI工具
  - 连接服务器发送查询并打印结果

### 数据来源

需要从Wikimedia数据库转储下载三个SQL文件：
- `page.sql` - 页面元数据
- `pagelinks.sql` - 页面间链接
- `linktarget.sql` - 链接目标解析表

支持英文维基（约700万文章）和简单英文维基（约28万文章）

### 使用方法

**启动服务器**：
- 首次从SQL加载：`wiki-route-server -v --sql-dir sql/ --save-dir data/`
- 从预计算数据加载：`wiki-route-server -v --data-dir data/`

**查询示例**：
```
wiki-route United_States France
# 输出：United_States -> NATO -> France
```

### 许可证
MIT许可证

[
https://github.com/michal-pielka/wiki-route
](
https://github.com/michal-pielka/wiki-route
)
    


### TITLE

# Rust安全扫描器：主动妥协检测工具

## 项目背景

一位正在学习Rust的开发者分享了他们使用Rust构建的安全扫描器项目，该工具旨在进行主动妥协检测（Proactive Compromise Detection）。

### 关键信息

- **开发语言**：Rust
- **项目性质**：安全扫描工具
- **开发状态**：仍在进行中，有大量工作需要完成
- **开发难度**：作者认为这是一个难以解决的问题

### 开发者声明

- 作者正在学习Rust，目前对该语言持积极态度
- 在开发过程中使用AI辅助进行头脑风暴和代码纠错
- 希望获得社区反馈以改进项目

### 项目意义

该项目专注于主动安全检测领域，旨在提前发现系统中的潜在安全妥协问题，这在网络安全领域具有重要的实践价值。

[
https://old.reddit.com/r/rust/comments/1s33cxn/security_scanner_for_proactive_compromise/
](
https://old.reddit.com/r/rust/comments/1s33cxn/security_scanner_for_proactive_compromise/
)
    


### TITLE

## 学习Rust：如何安全地索引字符串

### 问题背景
一位经验丰富的软件工程师在学习Rust时遇到字符串处理问题：
- 使用去年的Advent of Code练习Rust
- 之前没有处理过Unicode相关内容
- 需要从文件中读取行并解析（例如将"L34"分解为"L"和"34"）
- 虽然可以使用`as_bytes`，但想借此机会学习Unicode处理

### 初始代码
使用"Rust By Example"中的代码读取文件行：
```rust
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>
```

循环处理每一行：
```rust
for line in lines.map_while(Result::ok) {
    // 处理内容
}
```

### 主要困惑
- 确认`line`是`std::String`类型
- 如何安全地将"L34"拆分为"L"和"34"
- 看到在线资料建议使用`chars()`和迭代器，但担心获取"34"会很麻烦
- 避免使用"字符"一词，因为在Rust中存在歧义（字节 vs 字素）

### 最终解决方案
在社区帮助下，作者意识到需要更好地理解字符串迭代器（不同于C++迭代器）：

```rust
let mut chars = line.chars();
let direction = chars.next().unwrap();  // 获取第一个字符
let num = chars.as_str();                // 获取剩余字符串
```

这样就成功地将方向字母和数字分离开来。

[
https://old.reddit.com/r/rust/comments/1s2ntkt/learning_rust_how_do_i_safely_index_into_strings/
](
https://old.reddit.com/r/rust/comments/1s2ntkt/learning_rust_how_do_i_safely_index_into_strings/
)
    


### TITLE

## 云优化引擎创业：Rust vs C++ 的技术选型困境

### 背景情况
- 创业者计划在**希腊**建立云优化平台创业公司
- 同时掌握 Rust 和 C++ 两种语言
- 已用 Rust 开始开发，但考虑转向 C++
- **核心顾虑**：不是技术问题，而是**招聘和商业**问题

### 关键挑战

#### 团队需求
- 初期可能独自工作1个月，之后**必须招聘2-3名程序员**
- 第一年引擎规模不会小，需要尽早招人
- 融资前预算**非常有限**，只能提供较低薪资

#### 招聘困境（希腊本地市场）
- **Rust 程序员极难找到**，C++ 程序员相对容易
- 预计需要4-6个月才能找到第一位中级 Rust 程序员
- 第一年内可能**无法组建完整的 Rust 团队**

#### 投资者考量
- 6-12个月后寻求投资时，投资者**不关心**用 Rust 还是 C++
- 投资者**关心是否有功能性团队**
- 远程招聘国外程序员可能**不受投资者欢迎**

### 两难选择

**选项A - 坚持 Rust**
- ✅ 技术优势明显，适合大型云优化引擎
- ✅ 更好的内存安全和可靠性
- ❌ 可能长达一年找不到合适程序员
- ❌ 团队组建困难

**选项B - 转向 C++**
- ✅ 招聘容易快速
- ✅ 更容易组建团队
- ❌ 内存管理问题、难以发现的 bug、安全漏洞
- ❌ 即使借助 AI 工具辅助，隐藏问题仍难避免

### 已排除方案
- ❌ 远程招聘：投资者可能不喜欢，存在沟通管理问题
- ❌ 引擎用 Rust + 其他部分用 C++：引擎太大，拆分无意义

[
https://old.reddit.com/r/rust/comments/1s2x4z3/rust_or_c_for_a_cloud_optimization_engine_not_a/
](
https://old.reddit.com/r/rust/comments/1s2x4z3/rust_or_c_for_a_cloud_optimization_engine_not_a/
)
    


### TITLE

## DefaultNew 0.1.0 发布

一个轻量级的 Rust 过程宏 crate，用于自动为结构体实现 `Default` trait。

### 主要特点

- **零依赖**：非常轻量，没有外部依赖
- **自动实现**：为带有 `new()` 函数的结构体自动生成 `Default` trait 实现
- **解决痛点**：避免手动编写重复代码来满足 clippy lint 的要求

### 使用场景

当你的结构体有 `new()` 函数时，使用 `#[derive(DefaultNew)]` 可以自动生成以下代码：

```rust
impl Default for Foo {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
```

### 限制

目前仅支持基础用例，不支持泛型

### 开发背景

作者从个人工具 crate 中提取出这个功能，认为可能对其他开发者也有用处。如果已有类似的轻量级解决方案，作者也乐于了解。

[
https://old.reddit.com/r/rust/comments/1s2o52r/defaultnew_010/
](
https://old.reddit.com/r/rust/comments/1s2o52r/defaultnew_010/
)
    


### TITLE

## 无锁持有的死锁：在没有持有锁的情况下让 Tokio Mutex 死锁

### 问题概述
在一个使用 Tokio 的 Rust 程序中，团队遇到了一个反直觉的死锁现象：
- 使用单个 `tokio::sync::Mutex` 保护共享状态
- 4个异步任务启动，3个完成，第1个永久挂起
- **关键矛盾**：日志显示互斥锁已经释放，但任务仍然被阻塞
- 这不是经典的在 `.await` 中持有 `std::mutex` 的错误

### 复现代码要点
- 使用 `tokio::sync::Mutex` 被4个 worker 并发访问
- 实现了 `PausableFuture` 包装器，收到信号时停止轮询内部 future
- 使用单线程的 `current_thread` 运行时（多线程运行时也有类似行为）

### 执行流程
1. 主线程获取锁
2. 启动4个 worker，每个都有独立的停止标志
3. 让出执行权，让所有 worker 开始等待锁
4. **关键操作**：停止 worker 1 的轮询
5. 主线程释放锁

### 实际结果
```
main released the lock
worker 0: acquired lock
worker 0: released lock
worker 0: done
worker 1: DEADLOCKED
worker 2: DEADLOCKED
worker 3: DEADLOCKED
```

**异常现象**：主线程和 worker 0 都已释放锁，没有人持有锁，但其他线程仍然死锁！

### 实际生产环境背景
- 发生在基于 DataFusion 构建的查询引擎中
- 只在特定工作负载中出现
- 涉及暂停和恢复流的操作

### 调试方向：Coffman 条件
文章开始检查死锁的 Coffman 条件：
1. **互斥**：使用 mutex，满足 ✅
2. **持有并等待**：只有一个资源...（内容未完）

这是一个深入 Tokio 内部机制的技术案例，展示了异步编程中的微妙陷阱。

[
https://www.e6data.com/blog/deadlocking-tokio-mutex-without-holding-lock
](
https://www.e6data.com/blog/deadlocking-tokio-mutex-without-holding-lock
)
    


### TITLE

## Rust 为什么不提供 HashMap 的 map 宏？

### 主要观点

- **历史遗留问题**：作者表示从未真正理解为什么 Rust 没有提供 HashMap 的宏，并且自己多次编写过类似功能

### 现代解决方案

- **collection macros 已经过时**：由于现在每个集合都实现了 `From<[T; N]>` trait，collection 宏基本上已经不再必要

- **推荐的替代方法**：使用 `HashMap::from()` 配合数组语法：
  ```rust
  let map = HashMap::from([
      (key1, value1),
      (key2, value2),
      (key3, value3),
  ]);
  ```

- **实践建议**：作者通常在可能的情况下使用这种模式来替代宏

[
https://old.reddit.com/r/rust/comments/1s2ez82/why_doesnt_rust_provide_a_map_macro_for_hashmap/
](
https://old.reddit.com/r/rust/comments/1s2ez82/why_doesnt_rust_provide_a_map_macro_for_hashmap/
)
    


### TITLE

## VectorWare在GPU上实现Rust线程调度

VectorWare公司宣布成功在GPU上使用Rust的`std::thread`，这是朝着使用熟悉的Rust抽象来编写高性能GPU应用程序愿景迈出的重要一步。

### 核心要点

**CPU与GPU的执行模型差异**
- CPU程序从单线程开始，按需生成额外线程，程序员控制并发的时机和方式
- GPU程序由一个或多个内核组成，每个内核启动时会并行运行大量实例，并发是硬件运行方式固有的特性

**GPU编程的挑战**
- 大多数GPU编程模型使用函数作为入口点，但该函数会被硬件并行启动数千次
- 编程模型与执行模型之间的不匹配是GPU编程困难的部分原因
- 程序员需要手动维护正确的索引和避免竞态条件等不变量

**Rust在GPU上的现状**
- 当前GPU内核需要使用`unsafe`和原始指针，因为成千上万的实例同时运行并接收相同的指针
- 内核边界被视为FFI边界，没有编译器安全保证
- Rust的所有权模型是围绕CPU执行模型设计的，GPU执行模型对该语言来说是陌生的

**VectorWare的目标**
- 让GPU代码看起来像普通的Rust代码，能够与Rust生态系统原生集成
- 将Rust的安全保证扩展到GPU
- 避免创建需要学习新抽象的独立GPU编程模型

[
https://www.vectorware.com/blog/threads-on-gpu
](
https://www.vectorware.com/blog/threads-on-gpu
)
    


### TITLE

## Rust 移除 `contains` 方法引发的讨论

### 核心问题
Reddit 用户抱怨 Rust 移除了 `Option` 类型的 `contains` 方法，表达了对这一决定的不满。

### 主要观点

**影响：**
- 现在必须写更冗长的代码：`my_option.is_some_and(|my_value| my_value == my_non_option_value)`
- 原本可以用更简洁的 `contains` 方法

**移除原因（据作者回忆）：**
- 官方认为该方法是不必要的，因为他们稳定化了 `is_some_and` 方法

**作者的反驳：**
- 虽然可以用 `my_option == Some(my_value)` 来替代
- 但作者仍然认为 `contains` 方法可读性更好
- 不理解为什么要移除这个更符合人体工程学的方法

### 争议焦点
作者质疑移除 `contains` 方法的必要性，认为即使有替代方案，`contains` 依然是更易读、更符合直觉的 API 设计。

[
https://old.reddit.com/r/rust/comments/1s31jfu/i_really_hate_that_they_removed_the/
](
https://old.reddit.com/r/rust/comments/1s31jfu/i_really_hate_that_they_removed_the/
)
    


### TITLE

## Wikipedia游戏求解器

一位开发者创建了名为 **wiki-route** 的工具，用于找到任意两个维基百科条目之间的最短路径。

### 主要特点

- **算法**：使用双向广度优先搜索（bidirectional BFS）算法
- **性能**：能在毫秒/秒级时间内找到连接路径
- **技术实现**：
  - 解析实际的 MediaWiki 数据库转储文件
  - 在内存中构建有向图

### 示例

从"杰弗里·爱泼斯坦"到"Rust编程语言"的最短路径（在 simple.wikipedia.org 上）：

杰弗里·爱泼斯坦 → NBC新闻 → Peacock流媒体服务 → Rust编程语言

### 项目链接

GitHub仓库：https://github.com/michal-pielka/wiki-route

[
https://old.reddit.com/r/rust/comments/1s2wvri/wikipedia_game_solver/
](
https://old.reddit.com/r/rust/comments/1s2wvri/wikipedia_game_solver/
)
    


### TITLE

## Fyrox 游戏引擎 1.0.0 发布

Fyrox 游戏引擎经过 7 年开发后，正式发布了首个稳定版本 1.0.0。这是一个用 Rust 编写的现代游戏引擎，帮助开发者通过原生编辑器轻松创建 2D 和 3D 游戏，类似于 Unity，但使用 Rust 语言开发。

### 主要特性

**项目导出 CLI 工具**
- 现在可以为 Fyrox 游戏配置 CI/CD（持续集成/持续交付）
- 每个项目都包含一个名为 `export-cli` 的特殊 crate
- 提供与编辑器导出功能相同的命令行界面
- 对团队开发和自动化构建非常重要

### CLI 工具选项

**基础选项：**
- `-h, --help`: 显示详细使用说明
- `-V, --version`: 打印 CLI 版本
- `--target_platform`: 目标平台（pc/wasm/android）
- `--build_target`: 构建目标架构
- `--destination_folder`: 构建输出文件夹（默认 `./build/`）

**高级选项：**
- `--include_used_assets`: 仅包含使用过的资源
- `--ignored_extensions`: 指定要忽略的文件扩展名
- `-r, --run_after_build`: 构建后自动运行项目
- `-o, --open_destination_folder`: 构建后打开目标文件夹
- `-c, --convert_assets`: 将资源转换为发布版本（默认启用）
- `-e, --enable_optimization`: 启用所有优化（默认启用）

### 其他信息

- 相比上一版本（0.36）有巨大改进
- 建议查看候选发布版本（RC1 和 RC2）的发行说明
- 开发团队规模较小，资金有限，欢迎社区反馈和贡献

[
https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0/
](
https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

