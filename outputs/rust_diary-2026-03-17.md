【Rust日报】2026-03-17


### TITLE

## bnum - Rust 任意精度整数库

### 主要功能

`bnum` 是一个 Rust 库，提供了灵活的任意精度整数类型，支持自定义位宽和溢出行为。

### 核心特性

- **泛型整数类型**：`Integer<S, N, B, OM>` 支持：
  - `S`: 有符号/无符号
  - `N`: 位宽（任意大小）
  - `B`: 内部表示
  - `OM`: 溢出模式（wrapping/saturating/panic）

- **便捷的宏**：
  - `n!()`: 创建整数字面量，支持类型推断
  - `t!()`: 定义自定义整数类型

### 代码示例说明

**多项式函数示例**：
```rust
// p(x) = 2x³ + 3x² + 5x + 7
// 可接受任意位宽、任意符号、任意溢出行为的整数类型
```

**类型定义示例**：
- `U24w`: 24位无符号整数，溢出时环绕（wrapping）
- `I1044s`: 1044位有符号整数，溢出时饱和（saturating）
- `U753p`: 753位无符号整数，溢出时触发panic

**溢出行为演示**：
- 环绕模式：`p(U24w::MAX)` 结果溢出后自动环绕
- 饱和模式：`p(I1044s::MAX)` 结果溢出后固定为最大值
- Panic模式：`p(U753p::MAX)` 溢出会导致程序panic

[https://docs.rs/bnum/latest/bnum/
](https://docs.rs/bnum/latest/bnum/
)
    


### TITLE

## bnum v0.14.0 版本发布

这是该库迄今为止最重大的升级版本，带来了大幅简化的API、性能改进、更高的可定制性和新功能。

### 主要变更

#### 从8种整数类型统一为单一整数类型
- **重大改变**：将之前的4种无符号整数类型（BUint、BUintD32、BUintD16、BUintD8）和4种有符号整数类型（BInt、BIntD32、BIntD16、BIntD8）统一为单一的 `Integer` 类型
- 新增常量泛型参数 `S`（bool类型）来控制有符号/无符号行为
- `Integer` 内部存储为 `u8` 数组，但在计算时通过"分块"方式迭代更宽的数字类型
- 避免了位宽定制性和性能之间的权衡
- `Uint` 和 `Int` 成为 `Integer` 的别名

#### 任意位宽支持
- 新增常量泛型参数 `B`（usize类型）可指定2到2³²-1之间的任意位宽
- 之前版本只能使用8的倍数作为位宽

#### 常量泛型溢出行为控制
- 新增溢出模式参数 `OM`，支持3种行为：环绕、panic或饱和
- 默认行为：禁用溢出检查时环绕，启用时panic
- 完整参数：`Integer<S, N, B, OM>`（符号性、字节宽度、位宽度、溢出模式）

#### 简化的整数类型和值构造
- **n! 宏**：从整数字面量构造整数值
  - 示例：`let a = n!(0xabcdef_U256);`
  - 支持类型推断：`let b: I512 = n!(1234);`
  - 无效字面量会触发编译错误
  
- **t! 宏**：通过类型描述符构造特定整数类型
  - 示例：`t!(I155w)` 输出 `Integer<true, 20, 155, 0>`
  - 两个宏均为声明式宏，编译开销最小，保持零依赖

### 其他重大变更
- 移除无符号整数的 `Add<Digit>`、`Div<Digit>` 和 `Rem<Digit>` 实现
- 移除 `parse_str_radix` 方法（改用 `from_str_radix(...).unwrap()` 或 `n!` 宏）

[
https://github.com/isaacholt100/bnum/releases/tag/v0.14.0
](
https://github.com/isaacholt100/bnum/releases/tag/v0.14.0
)
    


### TITLE

## Rust开发者薪资指南2026

本指南提供了Rust开发者在不同经验水平、地区和行业的全面薪资数据（年度基本工资，以美元计）。

### 按职位划分的薪资范围

| 职位 | 全球平均 | 美国 | 欧洲 | 远程 |
|------|---------|------|------|------|
| **后端工程师** | $110K - $195K | $150K - $270K | $90K - $155K | $115K - $220K |
| **系统工程师** | $120K - $210K | $160K - $290K | $100K - $165K | $125K - $235K |
| **嵌入式工程师** | $105K - $185K | $140K - $250K | $85K - $150K | $110K - $210K |
| **基础设施工程师** | $115K - $200K | $155K - $280K | $95K - $160K | $120K - $230K |
| **区块链开发者** | $130K - $230K | $170K - $320K | $105K - $180K | $135K - $260K |
| **数据库/数据处理工程师** | $130K - $225K | $170K - $310K | $105K - $175K | $135K - $255K |

### 按工作经验划分的薪资

- **初级**（0-2年）：$75K - $120K
- **中级**（2-5年）：$120K - $185K
- **高级**（5-10年）：$170K - $280K
- **资深/首席**（10年以上）：$250K - $320K

### 按地区划分的薪资

- **美国**：$150K - $320K
- **欧洲**：$90K - $170K
- **英国**：$95K - $180K
- **德国**：$90K - $165K
- **荷兰**：$85K - $160K
- **远程（全球）**：$115K - $250K

### 远程与现场工作薪资对比

- **现场**（主要科技中心）：$140K - $290K
- **混合**（每周2-3天到岗）：$130K - $270K
- **远程**（完全分布式）：$115K - $250K

### 按行业划分的薪资

- **金融科技与区块链**：$155K - $320K
- **云基础设施**：$145K - $290K
- **网络安全**：$140K - $275K
- **汽车与嵌入式**：$125K - $240K
- **游戏与图形**：$115K - $220K
- **网络与电信**：$130K - $255K
- **数据库与数据处理**：$150K - $310K

### 影响薪资的关键因素

1. **经验水平** - 专业工作年限是薪资的最强预测指标
2. **地理位置** - 美国（旧金山、纽约、西雅图）薪资最高
3. **行业领域** - 金融科技、区块链和云基础设施公司支付溢价
4. **公司阶段与规模** - 后期创业公司和大型科技公司提供更高总薪酬
5. **开源贡献** - 活跃贡献者拥有更强的议价能力
6. **专业深度** - 分布式系统、编译器内部或性能优化等领域专家可获得更高薪酬

### 薪资谈判建议

1. **研究市场行情** - 利用薪资指南了解市场价值
2. **考虑总薪酬** - 关注股权、奖金、福利、远程灵活性和职业发展预算
3. **强调专业技能** - 突出Rust特定专业知识和生态系统贡献
4. **利用竞争offer** - 多个offer能增强谈判地位
5. **把握谈判时机** - 在收到书面offer后再谈判，而非初次面试时

[
https://rustjobs.dev/salary-guide
](
https://rustjobs.dev/salary-guide
)
    


### TITLE

## Rust Cookbook - 安全关键性编程指南

这是一份 Rust 编程语言的综合性实用手册，涵盖了从基础到高级的各种编程主题。

### 主要内容模块

该手册包含以下20个主要章节：

1. **算法** - 随机值生成、向量排序
2. **命令行** - 参数解析、ANSI终端
3. **压缩** - Tarball文件处理
4. **并发编程** - 显式线程、数据并行、Actor模式、自定义Future
5. **密码学** - 哈希、加密
6. **数据结构** - 位字段
7. **数据库** - SQLite、Postgres
8. **日期和时间** - 时间计算、解析和显示
9. **开发工具** - 调试、日志记录、版本控制、构建工具
10. **编码** - 字符集、CSV处理、结构化数据
11. **错误处理** - 错误变体处理
12. **文件系统** - 读写、目录遍历
13. **硬件支持** - 处理器
14. **内存管理** - 全局静态变量
15. **网络** - 服务器
16. **操作系统** - 外部命令
17. **科学计算** - 线性代数、三角函数、复数、统计学
18. **安全关键性Rust** - 无panic保证、确定性内存
19. **文本处理** - 正则表达式、字符串解析
20. **Web编程** - 链接提取、URL处理、HTTP客户端、Web API调用

### 核心重点：安全关键性编程

文档特别强调了**安全关键性Rust**编程，包括：
- **编译时无panic保证** - 确保程序运行不会崩溃
- **确定性内存管理** - 使用Heapless集合实现可预测的内存使用

[
https://rust-lang-nursery.github.io/rust-cookbook/safety_critical.html
](
https://rust-lang-nursery.github.io/rust-cookbook/safety_critical.html
)
    


### TITLE

## Rust 追踪调试（Tracing）

### 什么是 Tracing
- **tracing** 是一个用于 Rust 程序插桩的框架，用于收集结构化、基于事件的诊断信息
- 它是旧版 **log** crate 的替代方案，并具有向后兼容的适配器

### 安装依赖
在项目中添加以下 crate：
```bash
cargo add tracing tracing-subscriber
```
- 对于库项目，通常不需要 `tracing-subscriber`

### 将日志消息输出到控制台

#### 基本用法
1. **tracing** crate 提供宏来发出日志事件
2. **tracing-subscriber** crate 配置事件的发送位置
3. 调用 `tracing_subscriber::fmt::init()` 安装默认追踪订阅器

#### 日志级别（从高到低）
- `error!()` - 错误
- `warn!()` - 警告
- `info!()` - 信息（默认级别）
- `debug!()` - 调试
- `trace!()` - 追踪

### 关键特点
- **默认日志级别**：INFO（会丢弃更低级别的事件）
- 默认配置下，`debug!` 和 `trace!` 消息不会显示
- 可通过设置 **RUST_LOG** 环境变量来配置更详细的日志级别：
  ```bash
  RUST_LOG=trace cargo run
  ```

### 输出格式
日志输出包含时间戳、级别、模块名和消息内容，例如：
```
2024-12-01T07:56:14.778440Z ERROR tracing_console: This is an error!
```

[
https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/tracing.html
](
https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/tracing.html
)
    


### TITLE

## 在 Rust 中实现自定义 Future

这段代码演示了如何在 Rust 中手动实现一个自定义的 `Future`，通过创建一个简单的延迟（Delay）future 来说明核心概念。

### 主要知识点

#### 1. **`Pin<&mut Self>` 的作用**
- 保证结构体在内存中不会被移动
- 本例中 `Delay` 只包含 `Instant` 和 `Duration`（都是 `Unpin` 类型），编译器自动实现了 `Unpin`，因此固定（pinning）操作实际上是空操作
- 如果结构体包含自引用借用（如手写的异步状态机），固定会防止首次轮询后的移动，避免借用失效

#### 2. **`Poll::Pending` vs `Poll::Ready`**
- 返回 `Poll::Pending` 告诉执行器"我还没完成，稍后唤醒我"
- 返回 `Poll::Ready(())` 表示 future 已完成

#### 3. **`cx.waker()` 唤醒机制**
- 这是安排重新轮询的关键机制
- 必须调用 `wake()`，否则执行器永远不会再次轮询，future 会挂起
- 生产环境的计时器会向反应器（reactor）注册；本例为简化直接请求立即重新轮询

### 代码实现

- **`Delay` 结构体**：包含一个截止时间（deadline）
- **`poll` 方法**：检查当前时间是否达到截止时间
  - 已到达 → 返回 `Poll::Ready(())`
  - 未到达 → 调用 `cx.waker().wake_by_ref()` 并返回 `Poll::Pending`
- **使用示例**：在异步主函数中等待 10 毫秒，验证实际经过的时间

### 注意事项

这是一个教学示例，采用忙轮询（busy-polling）方式，实际应用中应使用定时器轮（timer wheel）或反应器模式以提高效率。

[
https://rust-lang-nursery.github.io/rust-cookbook/concurrency/custom_future.html
](
https://rust-lang-nursery.github.io/rust-cookbook/concurrency/custom_future.html
)
    


### TITLE

## Rust Actor 模式实现驾驶员位置追踪系统

这是一个使用 Tokio 异步运行时实现的 Actor 模式示例，用于管理驾驶员位置信息。

### 核心组件

**Message 枚举**
- 定义了 Actor 接收的命令类型
- `UpdateLocation`: 更新驾驶员位置（包含驾驶员ID、经纬度）
- `GetDriverStatus`: 查询驾驶员状态（通过 oneshot 通道返回结果）

**DriverStatus 结构体**
- 存储驾驶员状态信息：ID、经纬度坐标、更新次数

**DriverActor（Actor 实体）**
- 拥有数据的所有权，运行在独立的任务中
- 使用 HashMap 存储所有驾驶员状态
- 无需 `Arc<Mutex>`，因为只有一个任务访问数据
- `run()` 方法：循环接收并处理消息
- `handle_message()` 方法：处理具体的消息逻辑

**DriverHandle（句柄）**
- 可克隆的句柄，持有 `mpsc::Sender`
- 在应用程序中传递和共享
- 提供异步方法：
  - `update_location()`: 发送位置更新命令
  - `get_driver_status()`: 查询驾驶员状态并等待响应

### 关键特性

- **并发安全**：通过消息传递避免共享内存，无需显式锁
- **可扩展**：Handle 可克隆并发送到多个任务
- **异步通信**：使用 mpsc 通道进行命令发送，oneshot 通道获取响应
- **简洁的 API**：外部代码通过 Handle 与 Actor 交互，隐藏实现细节

### 示例执行流程

主函数演示了多任务并发更新驾驶员位置，然后查询不同驾驶员的状态，包括不存在的驾驶员（返回 None）。

[
https://rust-lang-nursery.github.io/rust-cookbook/concurrency/actor.html
](
https://rust-lang-nursery.github.io/rust-cookbook/concurrency/actor.html
)
    


### TITLE

## Rust Analyzer 更新总结

本次更新包含多项改进和修复：

### 主要更新内容

- **UTF-8 标识符支持**：#21793（首次贡献）处理 NameGenerator::suggest_name 中的多字节 UTF-8 标识符
- **代码结构优化**：#21767 从 EditionedFileId 中移除 crate
- **关联类型处理**：#21785 允许重复的关联类型简写解析，如果它们指向同一个关联类型
- **泛型推断增强**：#21820 为 TraitRef 及其关联类型推断泛型参数
- **命名规范验证**：#21794 验证联合类型（union types）的命名约定
- **文件监视改进**：#21771 使文件监视器支持递归监视

[
https://rust-analyzer.github.io/thisweek/2026/03/16/changelog-319.html
](
https://rust-analyzer.github.io/thisweek/2026/03/16/changelog-319.html
)
    


### TITLE

## SPiCa - 基于eBPF的内核级Rootkit检测引擎

### 项目简介
SPiCa是一个用Rust编写的高性能eBPF rootkit检测引擎。其设计理念受初音未来歌曲《SPiCa》启发，如同守护之星一般监视系统。架构上采用"双星系统"：两个独立的观测通道围绕同一进程空间运行，各自基于不同的物理机制，形成无法通过攻击单一通道就能使其失效的检测系统。

### 核心架构

SPiCa维护两个独立的观测通道和一个用户空间差异分析引擎：

**通道1 - BTF跟踪点（sched_switch）**
- 附加到内核sched_switch BTF跟踪点的eBPF程序
- 每次进程被调度到CPU时触发
- 通过CO-RE直接从内核内存读取task_struct，绕过可被hook的helper函数
- 提取pid、tgid和comm信息并通过RingBuf推送到用户空间

**通道2 - NMI性能事件（硬件CPU周期计数器）**
- 通过硬件PMU周期计数器驱动的非屏蔽中断（NMI）触发
- NMI无法被软件级的cli/sti指令屏蔽
- 抵御可击败商业EDR产品的软件hook攻击

**混淆层 - 运行时PID掩码**
- 使用64位密钥对PID值进行XOR加密后再写入ring buffer
- 密钥从/dev/urandom生成，每小时轮换一次
- 击败rootkit的PID过滤攻击（如Singularity）
- 与NMI通道形成正交防御机制

**差异分析引擎（用户空间）**
- 基于Tokio的用户空间FSM
- 读取两个ring buffer和/proc
- 交叉关联三种检测信号进行异常识别

### 技术特点
- 通过直接读取内核内存建立CPU执行事件的"ground truth"
- 刻意绕过可被rootkit hook的helper函数
- 实施"内核主权"原则
- 使用硬件级防护机制（NMI）抵御软件级攻击

### 项目信息
- 许可证：GPL-2.0
- 最新版本：v2.1（2026年3月）
- 主要更新：修复竞态条件、停止选择性PID过滤、添加硬件验证

[
https://github.com/0xkirisame/spica
](
https://github.com/0xkirisame/spica
)
    


### TITLE

## Rust数据库：寻找类似MariaDB的解决方案

一位开发者在Reddit的Rust社区发帖，咨询是否存在用Rust语言开发的数据库系统，希望找到功能上可以与MariaDB或PostgreSQL相媲美的替代方案。

### 关键需求

发帖者特别关注以下几个方面：

- **完整的SQL支持** - 需要全面的SQL语法和功能
- **ACID合规性** - 必须满足事务处理的原子性、一致性、隔离性和持久性
- **生产就绪** - 能够在生产环境中稳定使用
- **活跃的社区和维护** - 有持续的开发支持和社区参与

### 问题背景

发帖者希望社区成员能够推荐完全符合或部分符合这些标准的Rust数据库项目，表明了对Rust生态系统中企业级数据库解决方案的需求和兴趣。

[
https://old.reddit.com/r/rust/comments/1rvf5ew/is_there_a_rustbased_database_similar_to_mariadb/
](
https://old.reddit.com/r/rust/comments/1rvf5ew/is_there_a_rustbased_database_similar_to_mariadb/
)
    


### TITLE

## bnum v0.14.0 发布 - 重大升级

bnum 是一个 Rust 库，提供任意位宽的固定大小有符号和无符号整数类型。v0.14.0 版本是迄今为止最大的升级。

### 主要变化

**1. 全新的完全泛型整数类型**
- 引入单一的 `Integer` 类型，通过常量泛型参数指定：
  - **符号性**：有符号或无符号
  - **位宽**：2 到 2^32 - 1 之间的任意 `usize` 值
  - **溢出行为**：环绕(wrapping)、恐慌(panicking)或饱和(saturating)

- 泛型符号性支持编写更通用的函数（可以是 `const` 函数，这是 trait 目前无法实现的）
- 内部使用 `u8` 数组存储，操作时组合成更宽的数字，实现空间效率最大化同时保持或提升性能
- API 比以前更简单，不再需要针对不同数字宽度使用多种类型
- 泛型溢出行为比标准库的 `Saturating` 和 `Wrapping` 类型更简洁

**2. 两个便捷的新宏**
- `n!` 宏：在编译时将整数字面量转换为 `Integer` 值
- `t!` 宏：提供可读的方式创建具体的 `Integer` 类型实例

**3. 使用示例**
代码展示了如何编写适用于任意符号、位宽和溢出行为的泛型多项式函数，以及如何创建不同配置的整数类型（如 24 位环绕算术、1044 位饱和算术、753 位溢出恐慌等）。

[
https://old.reddit.com/r/rust/comments/1rv6j3f/bnum_v0140_huge_improvements/
](
https://old.reddit.com/r/rust/comments/1rv6j3f/bnum_v0140_huge_improvements/
)
    


### TITLE

## Rust开发者薪资指南发布

RustJobs.dev的Alex分享了一份新的Rust开发者薪资指南。

### 核心要点：

- **背景动机**：在与招聘Rust工程师的公司和求职开发者合作多年后，发现双方都难以清晰了解这一领域的薪酬水平

- **指南内容**：涵盖不同地区、经验水平和行业的薪资范围，基于多年来观察到的招聘活动和候选人期望制定

- **访问链接**：https://rustjobs.dev/salary-guide

- **未来计划**：
  - 这是初始版本，将持续改进
  - 征求社区反馈以确保信息准确性
  - 考虑撰写"如何获得Rust职位"的实用指南

- **目标用户**：
  - 评估自身市场价值的工程师
  - 为offer设定基准的招聘公司

[
https://old.reddit.com/r/rust/comments/1rv1ywm/rust_developer_salary_guide/
](
https://old.reddit.com/r/rust/comments/1rv1ywm/rust_developer_salary_guide/
)
    


### TITLE

## Rust单体仓库构建系统咨询

### 背景
- 用户目前使用Node.js和小型Rust微服务项目
- 一直使用Lerna维护集中式单体仓库(monorepo)
- 通过自定义库管理多个微服务间的依赖和类型处理，有助于快速扩展项目，避免重复编写代码和重复安装依赖

### 遇到的问题
- Lerna和Nx虽然是被广泛采用的解决方案，但存在以下缺点：
  - 体积庞大，内部依赖众多
  - 随着项目增长，运行缓慢，开发体验(DX)滞后

### 寻求帮助
- 希望迁移到基于Rust的单体仓库解决方案
- 正在寻求相关建议和帮助

[
https://old.reddit.com/r/rust/comments/1rvenc5/best_monorepo_build_system_in_rust/
](
https://old.reddit.com/r/rust/comments/1rvenc5/best_monorepo_build_system_in_rust/
)
    


### TITLE

## Rust 的借用检查器本身不是难点——难的是数据流设计

### 核心观点

作者在深入使用 Rust 后发现，**借用检查器（borrow checker）本身并不是真正的难题**。

### 真正的挑战

- **难点在于**：设计出能让借用检查器满意的数据流方式
- **与其他语言的区别**：
  - 其他语言：先写逻辑，后处理 bug
  - Rust：必须在架构设计阶段就**预先考虑**所有权、可变性和生命周期，而不能事后补救

### 实际影响

- 很多时候"修复"问题**不是改一行代码**，而是需要**重构整个实现方式**：
  - 拆分结构体
  - 改变函数边界
  - 避免共享可变状态等

### 设计理念转变

Rust 实际上是在**强制开发者采用更函数式/面向数据的设计风格**，即使你一开始并没有这个打算。

[
https://old.reddit.com/r/rust/comments/1rw12u6/rusts_borrow_checker_isnt_the_hard_part_its/
](
https://old.reddit.com/r/rust/comments/1rw12u6/rusts_borrow_checker_isnt_the_hard_part_its/
)
    


### TITLE

## asmkit-rs 文档示例

### 主要功能
这是一个 Rust 汇编工具库的使用示例，展示了如何动态生成和执行机器码。

### 核心组件
- **CodeBuffer**: 代码缓冲区，用于存储生成的汇编指令
- **Assembler**: 汇编器，用于编写汇编代码
- **JitAllocator**: JIT(即时编译)内存分配器

### 代码示例说明
该示例演示了创建一个函数来执行 SIMD 向量加法操作：

1. **初始化**: 创建代码缓冲区和汇编器
2. **参数设置**: 使用寄存器 RDI(目标)、RSI(参数0)、RDX(参数1)
3. **汇编指令**:
   - 从第一个参数地址加载 4 个整数到 XMM0 寄存器
   - 从第二个参数地址加载 4 个整数到 XMM1 寄存器
   - 执行向量加法操作
   - 将结果存储到目标地址
   - 返回函数
4. **JIT 执行**: 
   - 使用双重映射机制分配可执行内存
   - 通过 `span.rw()` 写入代码，通过 `span.rx()` 执行代码
5. **测试**: 在 x86_64 Unix 平台上执行生成的函数，将 [4,3,2,1] 和 [1,5,2,8] 相加，输出 [5,8,4,9]

### 平台要求
仅支持 x86_64 架构的 Unix/SystemV 平台

[
https://docs.rs/asmkit-rs
](
https://docs.rs/asmkit-rs
)
    


### TITLE

## Rust Cookbook 新增多个实用示例

这是一则关于 Rust 官方 Cookbook 更新的公告，介绍了几个新添加的编程示例和改进。

### 主要更新内容

**并发编程新示例：**
- **Actor 模式**：新增了 Actor 并发模式的实现示例
- **自定义 Future**：添加了如何创建自定义 Future 的教程

**开发工具更新：**
- **Tracing 日志库**：加入了 tracing crate 的使用示例，用于应用程序的调试和追踪

**新增专题章节：**
- **安全关键型 Rust**：新增了一个专门讲解安全关键应用场景的章节

**依赖更新：**
- 所有使用 `rand` 随机数库的示例已更新至 0.10 版本

### 致谢

感谢以下贡献者：i-a-m-d、JayanAXHF、jungseoklee 和 Joshka

---

*Happy Cooking（祝编程愉快）*

[
https://old.reddit.com/r/rust/comments/1rvqvaw/a_few_new_recipes_in_the_cookbook/
](
https://old.reddit.com/r/rust/comments/1rvqvaw/a_few_new_recipes_in_the_cookbook/
)
    


### TITLE

## Avian Physics 0.6 发布

这是一篇关于 Avian Physics 0.6 版本发布的技术博客文章。Avian 是一个为 Bevy 游戏引擎开发的基于 ECS 的 2D 和 3D 物理引擎。

### 主要亮点

**Move-and-Slide（移动与滑动）功能**
- 实现了运动学角色控制器（KCC）的核心移动和碰撞算法
- 允许物体沿期望的速度矢量移动，遇到障碍物时会沿其表面滑动
- 提供了相关工具：
  - 针对角色移动优化的形状投射
  - 从初始重叠中恢复的穿透修复功能
  - 基于 GJK 算法的速度投影
  - 支持自定义位置积分

**关节马达（Joint Motors）**
- 旋转关节和棱柱关节现在支持马达
- 提供速度控制和位置控制两种模式

**BVH 宽相位优化**
- 宽相位碰撞检测现在使用 OBVHS
- 大幅提升大型场景和静态几何体的性能

**空间查询优化**
- 空间查询现在重用宽相位使用的 BVH
- 显著降低系统开销

### 使用示例

文章提供了简单的运动学角色控制器实现示例，展示了如何：
- 根据线性速度移动角色
- 碰撞时沿障碍物滑动
- 处理几何体重叠
- 记录移动过程中接触的实体

### 未来计划

- 通用数值和 SIMD 支持
- 代码清理和优化
- 更多角色控制器功能

[
https://joonaa.dev/blog/12/avian-0-6
](
https://joonaa.dev/blog/12/avian-0-6
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

