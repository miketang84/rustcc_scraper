【Rust日报】2026-01-24


### TITLE

# Succinctly - Rust高性能简洁数据结构库

## 项目概述
Succinctly是一个为Rust设计的高性能、空间高效的数据结构库,提供快速的rank和select操作,支持x86_64(AVX2/AVX-512)和ARM(NEON)架构,兼容`no_std`环境。

## 核心特性

1. **位向量(Bitvector)**
   - O(1) rank操作和O(log n) select操作
   - 采用Poppy风格的三级目录,仅约3%空间开销

2. **平衡括号结构** - 用于树形导航
   - RangeMin结构,O(1)操作,约6%开销

3. **半索引技术(Semi-Indexing)**
   - **JSON半索引**:SIMD加速,在AMD Zen 4上可达880 MiB/s
   - **YAML半索引**:完整的YAML 1.2解析器,250-400 MiB/s
   - **CSV/DSV半索引**:85-1676 MiB/s,支持BMI2加速

4. **jq/yq风格查询表达式** - 无需完整解析即可导航JSON和YAML

5. **跨平台支持** - 运行时检测AVX2、AVX-512、SSE4.2和ARM NEON

## 半索引优势
与传统DOM解析器不同,半索引方法:
- **内存占用减少17-46倍**(索引仅为输入的~24% vs DOM的600-800%)
- **查询更快** - 按需提取值
- **流式输出** - 无需中间分配

**权衡**:相比完整解析器验证较少

## 性能对比(1MB JSON文件)
| 库 | 解析时间 | 吞吐量 | 峰值内存 | 对比 |
|---|---|---|---|---|
| sonic-rs | 1.00ms | 810 MiB/s | 9.97MB | 快1.6倍,内存多26倍 |
| **succinctly** | **1.58ms** | **510 MiB/s** | **382KB** | **基准** |
| serde_json | 4.83ms | 167 MiB/s | 7.00MB | 慢3.1倍,内存多18倍 |
| simd-json | 5.10ms | 158 MiB/s | 17.1MB | 慢3.2倍,内存多46倍 |

## 适用场景
- 嵌入式和WASM环境
- 高吞吐量应用
- 内存受限场景
- 大规模数据查询(无需完整解析)

[
https://github.com/rust-works/succinctly
](
https://github.com/rust-works/succinctly
)
    


### TITLE

# jbundle 项目总结

## 核心功能
jbundle 是一个将 JVM 应用程序（支持 Clojure、Java、Kotlin、Scala、Groovy 等所有 JVM 语言）打包成独立可执行文件的工具。**运行生成的二进制文件无需安装 JVM**。

## 设计理念
- **问题**：传统 JVM 应用部署需要目标机器安装 JVM；GraalVM native-image 虽可生成原生二进制文件，但存在编译慢、反射配置复杂、库兼容性问题等缺点
- **解决方案**：通过 jlink 将精简的 JVM 运行时与应用 JAR 打包成单一可执行文件，零外部依赖

## 与 GraalVM 对比
| 特性 | jbundle | GraalVM |
|------|---------|---------|
| 兼容性 | 100% JVM 兼容 | 需要反射配置，部分库不支持 |
| 构建速度 | 快 | 慢（AOT 编译）|
| 文件大小 | 30-50 MB | 20-40 MB |
| 启动速度（热启动）| 200-350ms (AppCDS) / 10-50ms (CRaC) | 10-50ms |
| 设置复杂度 | 简单 | 需要配置 |

## 工作原理
1. 检测构建系统（deps.edn、pom.xml、build.gradle 等）
2. 构建 JAR 文件
3. 下载并缓存 JDK
4. 使用 jdeps 检测模块依赖
5. 通过 jlink 创建精简运行时
6. 打包成多层二进制文件：`[启动器][runtime.tar.gz][app.jar.gz][crac.tar.gz?]`

## 性能优化
- **首次运行**：解压运行时 + 生成 AppCDS 缓存（+2-5秒）
- **后续运行**：使用缓存，启动速度提升 60-95%
- **JVM 配置**：提供 `server`（默认）和 `cli`（优化短期运行）两种模式
- **AppCDS**：自动生成共享类数据归档，加速类加载

## 其他特性
- 构建错误诊断（类似 Rust 编译器的友好错误提示）
- 分层缓存机制（仅更新应用代码时复用运行时缓存）
- 支持 CRaC（Linux）实现极速启动（10-50ms）

[
https://github.com/avelino/jbundle
](
https://github.com/avelino/jbundle
)
    


### TITLE

# SatoriDB 总结

**SatoriDB** 是一个专为十亿级规模数据设计的嵌入式向量数据库，能够处理超出内存容量的工作负载，实现95%以上的召回率和可预测的延迟。

## 核心架构

**双层搜索系统：**
- **热层（RAM）**：使用量化的HNSW索引存储桶质心，通过标量量化（f32→u8）压缩数据，支持50万+桶的内存存储
- **冷层（磁盘）**：CPU固定的Glommio工作线程并行扫描选定的桶

## 主要特性

- **嵌入式**：完全进程内运行，无需外部服务
- **自动聚类**：通过k-means对向量分组，桶超过~2000个向量时自动分裂
- **高性能**：
  - 使用io_uring的CPU固定工作线程
  - AVX2/AVX-512 SIMD加速距离计算
  - 无跨核心同步
- **持久化存储**：Walrus（追加式存储）+ RocksDB索引
- **可配置持久性**：灵活的fsync调度
- **仅支持Linux**（需要io_uring，内核5.8+）

## 基本使用

```rust
let db = SatoriDb::builder("my_app")
    .workers(4)
    .fsync_ms(100)
    .build()?;

db.insert(1, vec![0.1, 0.2, 0.3])?;
let results = db.query(vec![0.15, 0.25, 0.35], 10)?;
```

## API功能
- 插入/删除向量
- K近邻查询
- 按ID批量获取
- 同步和异步API支持

**注意**：项目处于早期开发阶段（v0.1.2），API可能会变化。

[
https://github.com/nubskr/satoriDB
](
https://github.com/nubskr/satoriDB
)
    


### TITLE

# sockudo-ws 项目总结

## 项目简介
sockudo-ws 是一个为 Rust 设计的超低延迟 WebSocket 库，专门针对高频交易(HFT)应用和实时系统优化。完全兼容 Tokio 和 Axum 框架，将用于 Sockudo 高性能 WebSocket 服务器项目。

## 性能表现

### 与其他 Rust WebSocket 库对比
在 100,000 次 "Hello, World!" 消息测试中：

| 库名 | 发送 | 回显 | 接收 | 总计 |
|------|------|------|------|------|
| **sockudo-ws** | 1.2ms | 5.0ms | 3.1ms | **10.2ms** |
| fastwebsockets | 3.3ms | 5.7ms | 3.0ms | 12.0ms |
| tokio-tungstenite | 6.4ms | 18.2ms | 10.2ms | 34.8ms |

**sockudo-ws 比次快的库快约 17%**

### 与 C++ uWebSockets 对比
在多连接、大消息量测试中，sockudo-ws 的性能达到或超过行业标准 uWebSockets：
- 512字节/100连接: 232,712 msg/s (uWebSockets: 227,973)
- 1024字节/100连接: 232,072 msg/s (uWebSockets: 224,498)

## 核心特性

1. **性能优化**
   - SIMD 加速（支持 AVX2/AVX-512/SSE2/NEON 等）
   - 零拷贝解析
   - 写入批处理（减少系统调用）
   - 无锁分离流（真正的并发读写）

2. **功能支持**
   - permessage-deflate 压缩
   - 发布/订阅系统
   - HTTP/2 WebSocket (RFC 8441)
   - HTTP/3 WebSocket (RFC 9220)
   - io_uring（Linux 高性能异步 I/O）

3. **可靠性**
   - 通过全部 517 个 Autobahn 测试用例
   - 经过 libFuzzer 模糊测试

## 安装使用

可通过 Cargo.toml 添加依赖，支持多种特性组合：
- 基础版本
- 带压缩（permessage-deflate）
- 带 HTTP/2/HTTP/3 支持
- 带 TLS 支持
- 带 io_uring 支持
- 完整功能版本

## 适用场景
- 高频交易系统
- 实时通信应用
- 需要极低延迟的 WebSocket 服务

[
https://github.com/sockudo/sockudo-ws
](
https://github.com/sockudo/sockudo-ws
)
    


### TITLE

# 植物病害检测研究项目总结

**学生：** Warre Snaet | **机构：** Howest MCT

## 项目概述
使用Rust语言在边缘设备(Jetson)上进行植物病害分类的半监督学习研究。

## 📁 项目结构
- **plantvillage_ssl/** - 半监督学习实现(伪标签法)✅已完成
- **incremental_learning/** - 增量学习(5→6类、30→31类实验)
- **pytorch_reference/** - Python参考实现用于对比
- **benchmarks/** - 框架性能对比脚本
- **research/** - 文献研究、合同、会议记录

## 🚀 使用方法

### 1. 下载数据集
```bash
./download_plantvillage.sh
```

### 2. 半监督学习训练
```bash
cd plantvillage_ssl
cargo build --release
./target/release/plantvillage_ssl ssl-train \
  --data-dir data/plantvillage/organized \
  --labeled-ratio 0.3 \
  --epochs 30 --cuda
```

### 3. 增量学习
```bash
cd incremental_learning
cargo build --release
./target/release/plant-incremental experiment \
  --method lwf \
  --base-classes 5 \
  --new-classes 1
```

## 🎯 研究问题
1. **半监督学习：** 伪标签法在边缘设备上的效率如何?
2. **增量学习：** 5→6类比30→31类更难吗?
3. **数据效率：** 每个新类别需要多少图像?

## 📖 文档
- 安装和用户指南
- 文献综述

[
https://github.com/SnaetWarre/Research_Project_Rust_Semi-Supervised_Learning
](
https://github.com/SnaetWarre/Research_Project_Rust_Semi-Supervised_Learning
)
    


### TITLE

# 智能疾病检测系统总结

这是一个基于深度学习的植物疾病检测项目，主要特点如下：

## 核心架构
- **框架**：使用 Burn 框架（Rust 深度学习库）构建
- **模型类型**：卷积神经网络（CNN）

## 网络结构
采用渐进式特征提取设计：
1. **4个卷积块**：逐层增加通道数（3→32→64→128→256）
2. **标准化处理**：每层配备 BatchNorm 和 ReLU 激活函数
3. **降采样**：使用 2×2 最大池化层
4. **分类头**：全局平均池化 + 两层全连接层
5. **输出**：38 个类别的疾病分类

## 技术特色
- 使用 Rust 语言实现，具有高性能和内存安全特性
- 经典的 CNN 架构，适合图像分类任务
- 模型结构清晰，便于理解和部署

## 应用场景
用于识别植物的38种不同疾病类型，可辅助农业病害诊断。

[
https://snaetwarre.github.io/My-Portofolio/blog/intelligent-disease-detection.html
](
https://snaetwarre.github.io/My-Portofolio/blog/intelligent-disease-detection.html
)
    


### TITLE

# mmdr - 超快速 Mermaid 图表渲染器总结

## 核心特点
**mmdr** 是一个纯 Rust 实现的 Mermaid 图表渲染工具，性能比官方 mermaid-cli 快 **500-1000倍**，无需浏览器依赖。

## 性能对比
| 图表类型 | mmdr | mermaid-cli | 加速比 |
|---------|------|-------------|--------|
| 流程图 | 2.75ms | 2,636ms | 958倍 |
| 类图 | 3.19ms | 2,381ms | 746倍 |
| 状态图 | 2.45ms | 2,647ms | 1,080倍 |
| 时序图 | 2.47ms | 2,444ms | 990倍 |

## 为什么这么快?
- **mermaid-cli**: 每次渲染都启动 Chromium 浏览器，有 2-3 秒启动开销
- **mmdr**: 纯 Rust 原生解析和渲染，直接输出 SVG/PNG，无浏览器、Node.js、Puppeteer

## 实际应用场景对比
- 50 个图表的 CI/CD: mermaid-cli ~2分钟 vs mmdr <1秒
- 实时编辑器预览: mermaid-cli 卡顿 vs mmdr 即时响应

## 安装方式
```bash
cargo install --path .           # 源码安装
brew install mmdr                # macOS/Linux
scoop install mmdr               # Windows
yay -S mmdr-bin                  # Arch Linux
```

## 快速使用
```bash
echo 'flowchart LR; A-->B-->C' | mmdr -e svg
mmdr -i diagram.mmd -o output.svg -e svg
mmdr -i README.md -o ./diagrams/ -e svg
```

## 支持的功能
- **图表类型**: 流程图、类图、状态图、时序图
- **节点形状**: 矩形、圆角、菱形、六边形、圆柱等 12+ 种
- **边样式**: 实线、虚线、粗线、带标签、多种箭头装饰
- **高级特性**: 子图、嵌套、样式定制、主题配置

## 技术实现
纯 Rust 管道: 解析 → 中间表示 → 布局(dagre算法) → 渲染SVG → 转PNG

[
https://github.com/1jehuang/mermaid-rs-renderer
](
https://github.com/1jehuang/mermaid-rs-renderer
)
    


### TITLE

# Succinctly：基于简洁数据结构的快速 jq/yq 替代工具

一位开发者发布了用 Rust 编写的 **Succinctly** 项目，这是一个使用简洁数据结构（通过 rank/select 半索引技术）实现 jq 和 yq 功能的库和 CLI 工具。

## 主要特性

**已实现功能：**
- 支持大多数 jq 和 yq 查询模式（reduce、limit、recurse、regex、路径函数等）
- JSON 解析速度约 880 MiB/s，YAML 解析速度约 250-400 MiB/s
- 支持基于位置的导航功能，可用于 IDE 集成

**尚未实现：**
- input/inputs（从标准输入流式处理多个 JSON 值）
- 超过内存大小的文件流式处理
- 部分高级 YAML 边缘情况

## 性能对比

**vs jq（AMD Ryzen 9 7950X）：**
- 速度提升 **1.7-1.8倍**
- 内存使用仅为 jq 的 **7-30%**（100MB 文件仅用 104MB vs 1GB）

**vs yq（Apple M1 Max）：**
- 速度提升 **7-10倍**
- 内存使用仅为 yq 的 **9-16%**（100MB 文件仅用 573MB vs 6GB）

## 技术亮点

- **硬件优化**：x86_64 使用 AVX2 SIMD、POPCNT、BMI2；ARM 使用 NEON 指令
- **简洁数据结构**：通过在原始文本上创建轻量级索引而非完整 DOM，实现 O(1) 节点导航，内存占用降低 6-10倍
- 支持 `no_std` 环境

项目已在 GitHub 和 Crates.io 开源，开发者欢迎反馈和 bug 报告。

[
https://old.reddit.com/r/rust/comments/1qleizg/succinctly_a_fast_jqyq_alternative_built_on/
](
https://old.reddit.com/r/rust/comments/1qleizg/succinctly_a_fast_jqyq_alternative_built_on/
)
    


### TITLE

# Reddit讨论摘要：Rust trait方法可见性问题

**核心问题：**
作者希望在trait中实现某些方法对外部用户不可见，但对trait实现者可见的功能。虽然理解Rust中trait方法应该公开的设计哲学，但在某些场景下仍需要"半私有"的方法。

**现有的解决方案：**
1. **实现结构在crate内部时**：将私有部分做成私有模块中的trait，内部添加blanket implementation
2. **辅助方法**：不将辅助方法定义为trait的一部分，而是定义为独立的私有函数

**具体使用场景举例：**
- 有一个`read_text`方法，它先调用`read_header`读取头部信息并修改状态，然后执行标准操作
- `read_header`需要由实现者提供具体定义（且实现者可能在crate外部）
- 但`read_header`应该只在trait内部使用，不对trait用户暴露
- 只有`read_text`应该对用户可见

**作者的困惑：**
如何让方法"只对实现者公开"而对trait用户隐藏，同时实现者可能在crate外部？

作者提到了一个可能的方案（将trait拆分但不够理想）：拆分为内部和公开trait，但这样无法强制实现者遵循约束。

[
https://old.reddit.com/r/rust/comments/1qm1lxz/trait_method_visibility_workarounds_public_to_the/
](
https://old.reddit.com/r/rust/comments/1qm1lxz/trait_method_visibility_workarounds_public_to_the/
)
    


### TITLE

# jbundle：用 Rust 打包 JVM 应用的命令行工具

## 项目背景
一位开发者用 Rust 开发了一个工具，解决了 Java 生态系统中的一个痛点：**如何在目标机器上分发 JVM 应用而无需预装 Java**。

## 解决的问题
- 传统方案是使用 GraalVM native-image，但存在诸多问题：
  - 反射配置复杂
  - 库兼容性问题
  - 编译时间超过 10 分钟
- 大多数开发者最后只能分发 JAR 文件，并附带"请安装 Java 21"的说明

## 解决方案
jbundle 采用不同方法：
- 将 JAR 文件 + 精简的 JVM 运行时（通过 jlink 创建）打包成**单个自解压可执行文件**
- 无需 AOT 编译，无需反射配置，100% JVM 兼容
- 最终生成 30-50 MB 的单个二进制文件

## 为什么选择 Rust
- **速度快**：打包仅需几秒钟
- **无运行时依赖**：单个静态二进制文件
- **跨平台**：支持 Linux x64/ARM64、macOS x64/ARM64

## 技术亮点
- 多层二进制格式，带内容哈希缓存
- 类似 rustc 风格的结构化错误诊断
- 使用 flate2 压缩、reqwest 下载 JDK、clap 处理 CLI
- 约 2500 行 Rust 代码

## 项目状态
- 目前缺少 Windows 支持
- 欢迎贡献和代码结构反馈
- **GitHub**: https://github.com/avelino/jbundle

[
https://old.reddit.com/r/rust/comments/1qm43ge/jbundle_a_rust_cli_to_package_jvm_apps_into/
](
https://old.reddit.com/r/rust/comments/1qm43ge/jbundle_a_rust_cli_to_package_jvm_apps_into/
)
    


### TITLE

# SatoriDB - 从零开始构建的向量数据库

一位开发者分享了他开发的 **satoriDB** 项目，这是一个用 Rust 编写的嵌入式向量数据库。

## 核心理念
- **模仿 SQLite 的使用体验**：不需要运行独立服务器，只需添加到 Cargo.toml 即可使用
- 解决现有向量数据库(如 Qdrant、Milvus、Weaviate)需要 Docker 容器、网络配置、HTTP/gRPC 序列化等复杂部署问题

## 技术架构亮点
1. **双层设计**：
   - 内存层：使用 HNSW 索引的量化质心作为路由器
   - 磁盘层：存储完整精度的 f32 向量，支持并行扫描

2. **性能优化**：
   - 基于 Glommio 的无共享、每核心一线程架构，减少上下文切换和互斥锁竞争
   - 自定义 WAL(Walrus)，支持 Linux 上的 io_uring 异步批量 I/O
   - L2 距离计算使用手写的 AVX2、FMA 和 AVX-512 指令集
   - 使用 RocksDB 处理元数据存储

3. **开发中功能**：正在集成对象存储支持

项目地址：https://github.com/nubskr/satoriDB

[
https://old.reddit.com/r/rust/comments/1qm0vsq/i_built_sqlite_for_vectors_from_scratch/
](
https://old.reddit.com/r/rust/comments/1qm0vsq/i_built_sqlite_for_vectors_from_scratch/
)
    


### TITLE

# Rust 闭包总结

这段代码演示了 **Rust 闭包的所有权和移动语义**：

## 关键点：

1. **值捕获（Capture by Value）**
   - 闭包通过 `drop(last_word)` 强制按值捕获变量 `last_word`
   - 所有权从外部作用域移动到闭包内部

2. **所有权转移的影响**
   - 闭包定义后，外部无法再访问 `last_word`（已被移动）
   - 调用闭包后，`last_word` 在闭包内被 drop，彻底销毁

3. **闭包只能调用一次**
   - 因为闭包内部消费了 `last_word`，闭包本身也变成了 `FnOnce` 类型
   - 第二次调用会报错：`use of moved value`

## 核心概念：
展示了 Rust 的 **所有权系统** 如何在闭包中工作，以及 `FnOnce` trait（只能调用一次的闭包）的实际应用场景。

[
https://antoine.vandecreme.net/blog/rust-closures/
](
https://antoine.vandecreme.net/blog/rust-closures/
)
    


### TITLE

# 内容总结

一位开发者用 Rust 语言创建了一个高性能的 WebSocket 库，主要特点包括：

## 核心亮点
- **性能卓越**：基准测试中性能可与 uWebSockets 媲美，在 Rust 生态中是最流行库中速度最快的
- **功能丰富**：
  - 支持压缩扩展
  - 支持基于 HTTP/2 和 HTTP/3 的 WebSocket
  - 集成了发布-订阅（pubsub）系统

## 技术细节
- 优化方面深受 uWebSockets 启发
- 正在开发 napi-rs 绑定（用于与 Node.js 集成）

项目地址：https://github.com/sockudo/sockudo-ws

[
https://old.reddit.com/r/rust/comments/1qlykf8/i_made_a_very_fast_websockets_library_in_rust/
](
https://old.reddit.com/r/rust/comments/1qlykf8/i_made_a_very_fast_websockets_library_in_rust/
)
    


### TITLE

# Rust构建的离线机器学习应用：植物病害检测AI

一位开发者分享了使用Rust构建的植物病害检测AI项目，主要亮点如下：

## 核心数据
- **应用大小**：仅24MB（相比之下PyTorch版本需要7.1GB）
- **推理速度**：
  - 桌面GPU (RTX 3060)：0.39毫秒/帧，达2,579 FPS
  - iPhone 12：约80毫秒
- **功能**：识别38种植物病害，使用30%标注数据进行半监督学习

## 技术选择
**为什么用Rust而非Python？**
- 目标用户是农民，需要在现有设备上离线运行
- PyTorch问题：7GB依赖、3秒冷启动、安装复杂
- Burn框架优势：编译成单一二进制文件，支持多平台（GPU/CPU/浏览器）

## 技术实现
- **框架**：使用Burn机器学习框架
- **模型**：标准CNN架构（4个卷积块：32→64→128→256通道）
- **半监督学习**：通过伪标签技术，用30%标注数据达到相当于60%全标注数据的准确率
- **部署**：使用Tauri 2.0打包成原生iOS应用，在iPhone上直接运行Rust代码

## 主要收获
- Burn框架已可用于生产环境
- Tauri移动端表现优秀，一套代码跨平台
- WASM性能超出预期
- 编译时间较长（release版本5分钟以上）

项目提供了完整的源代码和技术博客链接。

[
https://old.reddit.com/r/rust/comments/1qlpaj5/built_a_24mb_offline_ml_app_with_burn_tauri_runs/
](
https://old.reddit.com/r/rust/comments/1qlpaj5/built_a_24mb_offline_ml_app_with_burn_tauri_runs/
)
    


### TITLE

# Servo浏览器引擎2025年12月进展总结

## 主要新功能

**多窗口支持**
- Servo 0.0.4版本现已支持多窗口功能
- 注意：macOS版本存在已知问题，可能无法直接打开新窗口

**代理支持**
- 新增HTTP代理基础支持
- 可通过http_proxy、HTTP_PROXY环境变量或--pref参数设置代理

**证书管理**
- 默认使用系统根证书
- 可选择使用Mozilla根证书或自定义证书

## Web平台功能改进

**CSS功能**
- 支持contrast-color()颜色值
- 支持供应商前缀CSS属性（如-moz-transform）
- 支持HTML表格相关的background和bgcolor属性

**加密API (SubtleCrypto)**
- 完整支持ChaCha20-Poly1305、RSA-OAEP、RSA-PSS、RSASSA-PKCS1-v1_5
- 新增ML-KEM的importKey()支持

**其他Web功能**
- 部分支持<meta charset>和编码嗅探
- 支持可读字节流的tee()方法
- 支持window.clientInformation

## 开发者工具改进

- 开发工具新增网络安全选项卡，可检查TLS详情
- 兼容Firefox 145
- 优化IPC资源使用

## 嵌入API改进

- 新增SiteDataManager API管理localStorage、sessionStorage和cookies
- 新增NetworkManager API管理缓存
- 新增SimpleDialog类型处理alert()、confirm()、prompt()
- Web控制台消息现可通过ServoDelegate访问

## 其他改进

- 修复多个渲染bug
- 改进多种事件的一致性（wheel、hashchange、dblclick、resize等）
- Windows版servoshell现可显示--help和日志输出

**活动预告**：团队将参加FOSDEM 2026并发表演讲

[
https://servo.org/blog/2026/01/23/december-in-servo/
](
https://servo.org/blog/2026/01/23/december-in-servo/
)
    


### TITLE

# SIMD 编程在纯 Rust 中的应用

## 主要内容概述

### AMD Zen 5 的性能突破
- AMD Zen 5 CPU（AWS m8a 实例）性能表现惊人
- 在 m8a.2xlarge 虚拟实例上：
  - ChaCha20 达到 5.1 GB/s
  - ChaCha12 达到 6.7 GB/s  
  - BLAKE3 达到 10.8 GB/s
- **关键突破**：Zen 5 是首个拥有完整 512 位数据通路的 AMD CPU，可以充分利用 AVX-512 指令而不会降频

### SIMD 技术简介
**SIMD**（单指令多数据流）：
- 允许 CPU 在更大的数据向量上操作
- 传统 CPU 处理最多 64 位数据（标量指令）
- SIMD 指令可处理高达 512 位数据（向量指令）
- 可以用更少的指令完成更多计算

### SIMD 工作流程（三步骤）
1. **加载（Load）**：将数据从内存加载到向量寄存器
2. **计算（Compute）**：执行加法、异或、减法等操作
3. **存储（Store）**：将结果存回内存

### Rust 的优势
- 可以用纯 Rust 编写 SIMD 加速代码，无需处理汇编语言
- 不需要 nightly 版本
- AVX-512 代码可以在不到一天的工作中实现 10 倍以上的性能提升
- 支持 x86、ARM64 和 WebAssembly 平台

### 优化要点
- 最小化内存加载/存储操作（延迟成本高）
- 将数据保持在 SIMD 寄存器中
- 将输入分块并行处理多个数据块

[
https://kerkour.com/introduction-rust-simd
](
https://kerkour.com/introduction-rust-simd
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

