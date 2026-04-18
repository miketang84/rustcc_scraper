【Rust日报】2026-03-23


### TITLE

## Rusqlite 使用示例

这是一个 Rust 语言中使用 SQLite 数据库的基础教程示例。

### 主要知识点

**1. 基本设置**
- 使用 `rusqlite` crate 提供的 `Connection`、`params` 和 `Result` 类型
- 定义了一个 `Person` 结构体，包含 id、name 和可选的 data 字段

**2. 数据库连接**
- 使用 `Connection::open_in_memory()` 创建内存数据库连接

**3. 创建表**
- 通过 `conn.execute()` 执行 SQL 语句创建 person 表
- 表结构包含：id (主键)、name (非空文本)、data (二进制数据)

**4. 插入数据**
- 使用参数化查询 (`?1`, `?2`) 插入数据
- 通过元组传递参数值，避免 SQL 注入

**5. 查询数据**
- 使用 `prepare()` 准备 SQL 查询语句
- 使用 `query_map()` 将查询结果映射为 Person 对象
- 通过闭包 `|row|` 从每行数据中提取字段值

**6. 遍历结果**
- 使用 for 循环遍历查询结果
- 打印每个找到的 Person 对象

### 关键特性
- 类型安全的数据库操作
- 使用 Rust 的 `Result` 类型进行错误处理
- 参数化查询防止 SQL 注入

[
https://docs.rs/rusqlite/latest/rusqlite
](
https://docs.rs/rusqlite/latest/rusqlite
)
    


### TITLE

## Danube 消息系统的持久化与存储

### 核心概念
Danube 可靠主题（reliable topics）通过持久化消息，使消费者能够回放历史记录，并确保主题在代理服务器之间移动时不会丢失数据。存储系统基于三个核心思想：
- 最近的写入存储在快速的本地预写日志（WAL）中
- 历史数据通过持久化导出的段（segments）保存
- 恢复和主题迁移依赖于嵌入式 Raft 元数据存储

### 三种存储模式

#### 1. **local（本地模式）**
- 所有数据保存在代理服务器的本地磁盘
- 无后台导出，无远程存储
- **适用场景**：单节点部署、开发环境、简单部署

#### 2. **shared_fs（共享文件系统模式）**
- 热写入数据存储在本地 WAL
- 后台导出将封闭段复制到所有代理服务器可见的共享文件系统
- **适用场景**：使用 NFS 或共享 POSIX 卷的本地多代理集群

#### 3. **object_store（对象存储模式）**
- 热写入数据存储在本地 WAL
- 后台导出将封闭段推送到云对象存储（S3、GCS、Azure Blob）
- **适用场景**：云原生多代理部署

### 关键配置项

#### **WAL（预写日志）设置**
- 缓存容量、文件同步间隔
- 批处理字节数上限
- 文件轮转阈值（大小和时间）

#### **本地保留策略**
- 控制本地 WAL 文件的清理时机
- 基于时间（默认 48 小时）和大小（默认 20GB）的清理策略
- 仅删除本地 WAL 文件，不删除持久化段对象

#### **持久化后端配置**
- shared_fs：配置共享段目录
- object_store：配置云存储后端、存储桶、区域等参数

### 选择建议
| 需求 | 推荐模式 |
|------|---------|
| 最简单的单代理部署 | local |
| 多代理 + 共享磁盘 | shared_fs |
| 多代理 + 云原生 | object_store |

[
https://danube-docs.dev-state.com/concepts/persistence/
](
https://danube-docs.dev-state.com/concepts/persistence/
)
    


### TITLE

## Danube Messaging - 云原生消息平台

Danube 是一个用 Rust 构建的开源分布式消息代理平台，具有轻量级、云原生和成本效益的特点。

### 核心特性

- **嵌入式 Raft 共识**：基于 openraft 实现元数据复制，无需依赖 ETCD 或 ZooKeeper
- **可靠消息传递**：结合本地预写日志（WAL）、持久化段存储和元数据驱动恢复
- **低延迟分发**：支持本地磁盘、共享文件系统和对象存储
- **基于 Tokio 构建**：异步高性能运行时

### 快速开始方式

**1. Docker Compose 集群部署**
```bash
mkdir danube-docker && cd danube-docker
curl -O [docker-compose.yml]
curl -O [danube_broker.yml]
docker-compose up -d
```
包含：3个高可用 Broker、Prometheus 监控、danube-cli 工具

**2. 单节点本地运行**
```bash
./danube-broker --single-node --data-dir ~/danube-data
```
- 无需 Docker 和配置文件
- Broker 运行在 127.0.0.1:6650
- 管理端口 127.0.0.1:50051
- 数据持久化到本地目录

### 集群特性

- **水平扩展**：零停机时间快速添加 Broker
- **智能负载均衡**：自动主题分配和重新平衡
- **高可用性**：自动领导者选举、故障转移和主题协调
- **无外部依赖**：元数据存储完全嵌入式

### 项目结构

包含多个模块：broker、client、admin、CLI、core、持久化存储、Raft 实现等，最新版本 v0.9.0 进行了持久化和单节点 broker 的重大改进。

[
https://github.com/danube-messaging/danube
](
https://github.com/danube-messaging/danube
)
    


### TITLE

## Kreuzberg v4.5.2 版本更新说明

这是一个专注于PDF文档提取和处理的开源项目的重要更新版本。

### 修复的问题

- **PDF文字分割问题**：修复了Pdfium文本提取时在单词中间插入多余空格的问题（如"shall be active"被错误提取为"s hall a b e active"）。通过字符级间隙分析（字体大小×0.33阈值）进行页面级重新处理，将ISO 21111-10测试文档的乱码行从406行减少到0行。

- **Markdown下划线转义**：移除了对提取文本中下划线的错误转义（如`CTC_ARP_01`不再被转义为`CTC\_ARP\_01`）

- **页眉页脚泄漏**：修复了运行页眉（如ISO 21111-10:2021(E)）和版权页脚泄漏到文档正文的问题，添加了模糊匹配来检测重复的页眉页脚文本

- **R批处理函数NULL参数问题**：修复了R包装器批处理函数传递多余NULL参数导致的错误

- **Elixir Windows ORT DLL问题**：修复了Windows CI上OCR/布局/嵌入功能的DLL加载问题

### 新增功能

**缓存功能增强**：
- 所有文件类型（PDF、Office、HTML、归档等）现在都支持缓存
- 缓存命名空间隔离，支持多租户环境
- 每个请求可自定义缓存TTL
- 缓存命名空间删除和统计功能
- 多工作进程环境下的清理安全性改进

**OCR语言支持**：
- 内置英语OCR数据（eng.traineddata）
- `cache warm`命令可下载约120种语言的tessdata文件
- 改进的tessdata路径解析机制

**CLI命令增强**：
- 新增`embed`命令：从文本生成向量嵌入
- 新增`chunk`命令：将文本分割成块
- 新增`completions`命令：生成shell自动补全
- 新增`--log-level`全局标志
- 彩色输出支持（尊重NO_COLOR环境变量）
- 27个新的提取覆盖标志

**API和MCP工具**：
- 新REST端点：`POST /detect`、`GET /version`、`GET /cache/manifest`、`POST /cache/warm`
- 新MCP工具：`get_version`、`cache_manifest`、`cache_warm`、`embed_text`、`chunk_text`

**其他改进**：
- 表格提取的跟踪日志
- TATR模型可用性检查
- 改进的CLI验证机制

### 架构变更

- 使用`ExtractionOverrides`结构体简化配置架构
- 移除MCP工具中基于trait的重复代码
- 改进批处理命令标志支持

[
https://github.com/kreuzberg-dev/kreuzberg/releases
](
https://github.com/kreuzberg-dev/kreuzberg/releases
)
    


### TITLE

## Mamba: 使用选择性状态空间的线性时间序列建模

### 研究背景
- 当前深度学习的基础模型几乎都基于Transformer架构及其核心注意力模块
- 现有的次二次时间复杂度架构(如线性注意力、门控卷积、循环模型和结构化状态空间模型SSM)在处理长序列时虽能提高计算效率,但在语言等重要模态上表现不如注意力机制

### 关键创新点
- **识别核心问题**: 发现这些模型的主要弱点是无法进行基于内容的推理
- **选择性SSM机制**: 让SSM参数成为输入的函数,使模型能够根据当前token选择性地传播或遗忘序列信息
- **硬件感知算法**: 设计了循环模式下的硬件感知并行算法,克服了无法使用高效卷积的限制
- **简化架构**: 提出了不含注意力机制或MLP块的端到端神经网络架构Mamba

### 性能优势
- **推理速度**: 吞吐量比Transformers高5倍
- **可扩展性**: 序列长度呈线性扩展,可处理百万级长度序列
- **模型表现**: Mamba-3B模型在预训练和下游评估中优于同等规模的Transformers,并可匹敌两倍规模的Transformers
- **跨模态能力**: 在语言、音频和基因组学等多个模态上达到最先进性能

### 应用价值
作为通用序列模型骨干网络,Mamba为处理长序列数据提供了高效且性能优越的解决方案

[
https://arxiv.org/abs/2312.00752
](
https://arxiv.org/abs/2312.00752
)
    


### TITLE

## Mamba-RS: Rust实现的Mamba选择性状态空间模型

### 项目概述
这是一个用Rust语言实现的Mamba SSM（选择性状态空间模型），支持可选的CUDA GPU加速。项目支持推理和训练，包括通过递归SSM状态的完整反向传播（BPTT），并提供自定义CUDA核心用于GPU加速的前向和反向传播。

### 主要特性
- **推理优化**：零内存分配的单步递归前向传播（CPU上约200微秒）
- **完整训练支持**：通过SSM隐藏状态的完整BPTT反向传播
- **预热机制**：支持从历史上下文预热递归状态
- **CUDA加速**：为SSM递归、conv1d和融合激活函数提供自定义核心
- **独立运行**：无需PyTorch、Burn或Candle等框架依赖
- **单精度浮点**：原生f32，在Ampere/Hopper架构上支持TF32张量核心

### 架构设计
模型采用多层结构，每层包含：
- 输入投影和残差连接
- RMS归一化
- 门控机制和卷积层
- SSM递归计算（h = A*h + B*x, y = C*h + D*x）
- 输出投影和残差相加

### 性能表现
**CPU推理**（GH200 ARM）：
- 小型配置（64维，2层）：61微秒
- 默认配置（128维，3层）：375微秒
- 大型配置（512维，6层）：13.6毫秒

**GPU性能**（H100，TF32）：批量推理延迟约10-25微秒

### 核心优势
- 手动推导的解析梯度，无需自动微分依赖
- 跨时间步的BPTT支持
- 零分配推理路径
- 扁平连续权重缓冲区，便于优化器融合
- 兼容CUDA Graph捕获

### 与Python版本的差异
| 特性 | Python版本 | Rust版本 |
|------|-----------|---------|
| 反向传播 | PyTorch自动微分 | 手动BPTT |
| 核心实现 | Triton + CUDA C++ | CUDA NVRTC运行时编译 |
| 框架依赖 | PyTorch | 独立运行 |
| 精度 | fp16/bf16/fp32 | f32（GPU上TF32）|

### 许可证
采用MIT或Apache-2.0双重许可。

[
https://github.com/silvermpx/mamba-rs
](
https://github.com/silvermpx/mamba-rs
)
    


### TITLE

## Arnis - 真实地理数据生成Minecraft世界

### 项目简介
Arnis是一个免费开源项目，能够将真实世界的地理、地形和建筑转换为复杂且精确的Minecraft世界（支持Java版1.17+和基岩版）。该项目处理OpenStreetMap的地理空间数据和海拔数据，生成详细的Minecraft地形和建筑表现。用户可以轻松生成家乡、大城市和自然景观。

### 主要特点
- **真实地理映射**：基于OpenStreetMap和海拔数据生成准确的Minecraft世界
- **大规模数据处理**：能够处理大范围的地理数据
- **跨平台支持**：可在Windows、macOS和Linux上运行
- **浏览器版本**：MapSmith提供无需安装的浏览器版本，支持移动端

### 使用方法
1. 下载最新版本或自行编译
2. 在地图上使用矩形工具选择区域
3. 选择Minecraft世界
4. 点击"开始生成"
5. 可自定义设置（世界比例、出生点、建筑内部生成等）

### 开源项目目标
- **模块化**：各组件清晰分离，便于维护和扩展
- **性能优化**：保持良好的生成速度
- **完善文档**：提供详细的代码文档
- **用户友好**：注重易用性
- **跨平台**：全平台流畅运行

### 贡献方式
- 修复bug、改进性能、添加新功能、完善文档
- Fork仓库，提交Pull Request
- 支持命令行和GUI两种构建方式
- 支持Nix环境直接运行

### 项目认可
该项目在2024年12月获得广泛关注后，已被多个学术和媒体出版物认可，包括AWS、教育应用和技术媒体报道。

[
https://github.com/louis-e/arnis
](
https://github.com/louis-e/arnis
)
    


### TITLE

## TideORM CLI - Rust ORM 命令行工具

TideORM CLI 是一个为 TideORM（强大的 Rust ORM）设计的综合命令行界面工具。

### 安装方式
```bash
cargo install tideorm-cli
```

### 核心功能

#### 1. **项目初始化与模型生成**
- 初始化新项目
- 生成模型（支持字段、关系、时间戳、软删除等）
- 自动生成迁移文件和种子文件

#### 2. **TideORM Studio（Web UI）**
- 海洋主题的可视化 Web 界面
- 运行在 localhost:8080（可自定义端口）
- 包含以下模块：
  - 📊 仪表板 - 快速操作和命令历史
  - 🏗️ 模型生成器 - 可视化表单创建模型
  - 📦 迁移管理器 - 创建、运行、回滚迁移
  - 🌱 种子管理器 - 创建和执行数据库种子
  - ⚡ 查询工作台 - 交互式 SQL 编辑器

#### 3. **数据库迁移命令**
- `migrate run` - 运行待执行的迁移
- `migrate up/down` - 向上/向下迁移
- `migrate fresh` - 删除所有表并重新运行
- `migrate reset` - 回滚所有迁移
- `migrate refresh` - 重置并重新迁移
- `migrate status` - 查看迁移状态

#### 4. **模型生成选项**
- **字段类型**：string, text, i32, i64, f32, f64, bool, datetime, date, time, uuid, json, decimal
- **字段修饰符**：nullable, unique, indexed, primary_key, auto_increment, default
- **关系类型**：belongs_to, has_many
- **特殊功能**：
  - 可翻译字段（translatable）
  - 附件支持（单个/多个文件）
  - 软删除、时间戳、令牌化

### 配置文件
使用 `tideorm.toml` 配置项目、数据库连接、路径、迁移和种子选项等。

[
https://github.com/mohamadzoh/tideorm-cli
](
https://github.com/mohamadzoh/tideorm-cli
)
    


### TITLE

## TideORM - Rust 开发者友好的 ORM 框架

TideORM 是一个为 Rust 设计的现代化 ORM 框架，提供简洁优雅的语法和强大的数据库操作能力。

### 核心特性

- **简洁的模型定义** - 使用 `#[tideorm::model(table = "...")]` 属性宏轻松定义模型
- **字段级关系声明** - 支持 `HasOne`、`HasMany`、`BelongsTo` 和 `HasManyThrough` 关系，直接在模型字段中定义
- **异步优先** - 基于现代 async/await 工作流构建
- **自动模式同步** - 开发阶段自动管理数据表结构
- **多数据库支持** - 支持 PostgreSQL、MySQL 和 SQLite
- **查询构建器** - 流畅的过滤、OR 分组、联接、联合、CTE 和窗口函数
- **性能分析和日志** - 内置查询日志、执行计数器和慢查询统计
- **数据生命周期工具** - 迁移、数据填充、验证、回调、软删除和事务
- **可选模块** - 通过 feature flags 提供附件、翻译和全文搜索功能
- **令牌化** - 安全的记录 ID 编码/解码辅助工具

### 主要功能

**模型关系支持：**
- 一对一关系（HasOne）
- 一对多关系（HasMany）
- 反向关系（BelongsTo）
- 多对多关系（HasManyThrough）

**CRUD 操作：**
- 创建记录（save）
- 查询数据（query、where、order、limit）
- 复杂查询（支持 OR 条件、模糊匹配）
- 关系加载（load）
- 更新记录（update）
- 删除记录（destroy）

**性能分析：**
- GlobalProfiler - 记录实际执行查询的聚合时间
- Profiler - 构建手动分析报告

### 技术特点

- 关系辅助字段（HasOne<T>、HasMany<T>）仅在运行时使用，不会出现在 JSON 序列化中
- 支持全局状态重置，方便测试和重新配置
- 性能分析器可观察所有主要执行路径，包括查询构建器读取、原始 SQL、聚合查询、全文搜索等

[
https://github.com/mohamadzoh/tideorm
](
https://github.com/mohamadzoh/tideorm
)
    


### TITLE

## Rust新手寻求MySQL数据库连接方案

### 背景
- 作者有超过10年的Python开发经验
- 因无法忍受动态类型而转向Rust
- 习惯使用原生/预处理SQL查询，从未使用过ORM
- 不反对ORM，但希望保持简单

### 核心问题
**询问Rust中连接MySQL数据库的事实标准是什么？**
- 是否应该使用官方的"mysql" crate？

[
https://old.reddit.com/r/rust/comments/1s0keho/absolutely_new_to_rust_and_want_connect_to_a/
](
https://old.reddit.com/r/rust/comments/1s0keho/absolutely_new_to_rust_and_want_connect_to_a/
)
    


### TITLE

## Rust SQLite 库的选择和使用体验

帖主目前在使用 **rusqlite**（原生 SQLite）配合 **r2d2** 和 **r2d2_sqlite** 连接池。

### 使用中遇到的问题

- **内存管理问题**：rusqlite 使用普通的 malloc，在其工作场景下启动时会占用 400MB 内存，且系统 malloc 不会快速释放页面。帖主希望像主项目那样使用 jemalloc 来减少内存占用，但未找到覆盖方法。

### 考虑的替代方案

帖主对以下特性和库感兴趣：

- **Diesel**：想了解其编译时检查功能，以及编译时间表现
- **迁移体验**：询问是否有人从 rusqlite 迁移到 diesel 或其他库，迁移过程如何
- **动态查询**：听说 diesel 的"动态查询很痛苦"，想确认是否属实

### 其他选项

- **SQLx**：听说口碑不错

[
https://old.reddit.com/r/rust/comments/1s0iucz/what_crate_do_you_use_for_sqlite_and_how_is_using/
](
https://old.reddit.com/r/rust/comments/1s0iucz/what_crate_do_you_use_for_sqlite_and_how_is_using/
)
    


### TITLE

## Danube v0.9.0 发布：持久化存储重构和零配置单节点模式

Danube 是一个用 Rust 编写的开源分布式消息代理。v0.9.0 版本发布了两项重大更新：

### 持久化存储重构

`danube-persistent-storage` 模块采用全新的密封段架构，包含三层设计：
- 每个主题的本地 WAL（预写日志），用于快速写入
- 不可变的持久化段，用于历史数据读取
- Raft 元数据，用于段跟踪和主题迁移

现支持三种存储模式：
- **local**：仅在代理磁盘上使用 WAL（最简单，适合单节点）
- **shared_fs**：WAL + 后台导出到共享文件系统（如 NFS）
- **object_store**：WAL + 通过 OpenDAL 后台导出到 S3/GCS/Azure Blob

**主题可以在代理之间迁移**，并保持完整的偏移量连续性。旧代理会密封并导出剩余的 WAL 数据，新代理从持久化段恢复。

存储模型详解：https://danube-docs.dev-state.com/concepts/persistence/

### 零配置单节点模式

现在只需一条命令即可启动代理：
```
danube-broker --single-node
```

无需配置文件，系统会自动生成默认设置（本地主机端口、本地存储、单节点 Raft），首次启动时自动初始化，并在重启后保持数据持久化。非常适合本地开发和测试。

**技术栈**：OpenRaft（共识算法）、OpenDAL（对象存储抽象）、tonic（gRPC）、tokio（异步运行时）

[
https://old.reddit.com/r/rust/comments/1s1b9hv/danube_v090_persistent_storage_revamp_and/
](
https://old.reddit.com/r/rust/comments/1s1b9hv/danube_v090_persistent_storage_revamp_and/
)
    


### TITLE

## Kreuzberg v4.5 重大更新发布

Kreuzberg 团队发布了 v4.5 版本，这是一个重要的里程碑更新。

**项目简介：**
Kreuzberg 是一个开源（MIT 许可）的文档智能框架，支持 12 种编程语言。它用 Rust 编写，为 Python、TypeScript/Node.js、PHP、Ruby、Java、C#、Go、Elixir、R、C 和 WASM 提供原生绑定。可以从 88+ 种格式中提取文本、结构和元数据，运行 OCR，生成嵌入向量，专为 AI 流程和大规模文档处理而构建。

### 核心更新亮点

**文档结构理解能力：**
- 不仅提取文本，现在还能理解文档结构（布局/表格）
- 集成了 Docling 的 RT-DETR v2（Docling Heron）模型
- 将其嵌入到 Rust 原生管道中

### 性能优势

在 171 个 PDF 文档的基准测试中（包括学术论文、政府和法律文档、发票、OCR 扫描等）：
- **结构 F1 分数**：Kreuzberg 42.1% vs Docling 41.7%
- **文本 F1 分数**：Kreuzberg 88.9% vs Docling 86.7%
- **平均处理时间**：Kreuzberg 1,032 毫秒/文档 vs Docling 2,894 毫秒/文档
- **速度提升**：平均快 2.8 倍，内存开销更小，无需 Python 依赖

### 技术特性

- 支持 17 种文档元素类型分类
- 表格检测和结构预测（使用 TATR 模型）
- 使用 pdfium 直接从 PDF 原生文本层提取文本，保留字符位置、字体元数据
- 自动 OCR 回退（针对无文本层的页面）
- 支持 PDF/A 标记结构树
- 自动修复字体 CMap 表错误
- 多后端 OCR 管道，包含 PaddleOCR v2（18,000+ 字符多语言模型）
- 提取结果缓存

**项目链接：** https://kreuzberg.dev/

[
https://old.reddit.com/r/rust/comments/1s0eyn5/kreuzberg_v450_we_loved_doclings_model_so_much/
](
https://old.reddit.com/r/rust/comments/1s0eyn5/kreuzberg_v450_we_loved_doclings_model_so_much/
)
    


### TITLE

## Rust中处理循环依赖的问题：解析Firefox书签JSON

### 背景问题
用户在使用 `serde` 解析Firefox书签JSON数据时遇到了循环依赖问题。Firefox书签包含以下几种类型：
- **内容根节点**（编译时已知，数量有限）
- **文件夹**（用户管理，运行时才知道）
- **书签**
- **分隔符**

这些对象分为3种类型标识：
- `text/x-moz-place-container`：内容根和文件夹
- `text/x-moz-place-separator`：分隔符
- `text/x-moz-place`：书签

### 核心矛盾
- 内容根节点有 `root` 字段，而文件夹没有
- 用户希望将内容根节点与普通文件夹在编译时区分开
- 但是这样做会产生**循环依赖**：`ContainerNode` 包含 `Vec<Node>`，而 `Node` 又包含 `ContainerNode`
- 分离后还会导致 `ContainerNode` 不再是 `Node` 的变体，无法支持嵌套文件夹结构

### 当前解决思路
用户考虑将内容根节点实现为结构体字段，并手动实现反序列化逻辑来避免这个问题。同时也在寻求其他可能的解决方案。

### 问题实质
这是一个在保持类型安全的同时处理**递归数据结构**和**编译时类型区分**之间的权衡问题。

[
https://old.reddit.com/r/rust/comments/1s1bn30/how_do_you_deal_with_circular_dependency_when/
](
https://old.reddit.com/r/rust/comments/1s1bn30/how_do_you_deal_with_circular_dependency_when/
)
    


### TITLE

## Mamba-rs: 使用 Rust 实现的 Mamba SSM 及训练支持

### 项目概述
开发者分享了一个用 Rust 编写的 Mamba（选择性状态空间模型）完整实现，这是基于 Gu & Dao 论文的工作。

### 核心功能
- **完整的 Mamba v1 实现**：支持推理和训练
- **手动推导的反向传播**：通过递归 SSM 状态实现 BPTT，无需自动微分
- **自定义 CUDA 核心**：用于 GPU 加速（SSM 递归、conv1d、融合操作）
- **零内存分配推理路径**：优化内存使用
- **Burn-in 支持**：从历史上下文预热隐藏状态
- **独立 crate**：无框架依赖

### 开发动机
- 项目需求中 Python 的性能开销无法接受
- 现有 Rust 实现仅支持推理（如 candle 和 mamba.rs），缺乏训练支持和自定义 CUDA 核心
- 反向传播是最大挑战：需手动解析推导每个梯度并通过有限差分验证

### 性能表现
**CPU 推理基准**（GH200 ARM Neoverse V2）：
- d_model=64, 2层: 61 μs/步
- d_model=128, 3层: 375 μs/步
- d_model=256, 4层: 2.2 ms/步
- d_model=512, 6层: 13.6 ms/步

**GPU SGEMM**（GH200 H100, TF32 张量核心, d_model=128）：
- 批次=1: 25 μs
- 批次=128: 10 μs
- 批次=256: 10 μs

### 项目资源
- GitHub: https://github.com/silvermpx/mamba-rs
- crates.io: https://crates.io/crates/mamba-rs
- 论文链接: https://arxiv.org/abs/2312.00752

[
https://old.reddit.com/r/rust/comments/1s18ct5/mambars_mamba_ssm_in_rust_with_training_support/
](
https://old.reddit.com/r/rust/comments/1s18ct5/mambars_mamba_ssm_in_rust_with_training_support/
)
    


### TITLE

## Rust中如何防止内部可变性

### 问题描述
开发者遇到以下场景：

```rust
type Foo = ...;

fn bar(foo: Foo) -> Option<Foo> {
  ...
}
```

### 需求
- 当函数 `bar` 返回 `Some(foo)` 时，需要**保证** `foo` 没有被 `bar` 函数修改
- 当返回 `None` 时，允许销毁 `foo`
- 希望 `Foo` 类型能够：
  - 拥有数据的所有权并可以销毁它
  - 但**不能**通过 `Foo` 修改其内部数据（防止内部可变性）

### 核心问题
如何在 Rust 中设计一个类型，使其拥有数据所有权但禁止通过该类型进行可变操作？

[
https://old.reddit.com/r/rust/comments/1s0r6w2/prevent_interior_mutability/
](
https://old.reddit.com/r/rust/comments/1s0r6w2/prevent_interior_mutability/
)
    


### TITLE

## Rust Analyzer 更新日志 #320

**发布日期**: 2026年3月23日 (v0.3.2836)

### 主要新功能

**代码重构与修复**
- 将测试占位符从 `test_name` 改为 `executable_arg`
- 修复内联类型别名代码辅助中的生命周期参数尖括号问题
- 修复命名结构体与元组结构体相互转换时的重叠编辑问题
- 修复元组结构体模式的预期类型分析
- 支持 `rust-project.json` 中更多可运行类型

**代码补全改进**
- 函数参数补全时保留限定符
- 支持闭包中的后缀 `let` 补全
- 改进后缀补全后的缩进处理
- 避免后缀补全后添加多余分号

**代码辅助功能增强**
- 在 `generate_trait_from_impl` 中提取文档注释
- 为空结构体提供 `no_such_field` 修复建议
- 添加 `non_exhaustive_let` 的修复功能
- 支持在 let-chains 中使用 `inline_local_variable`
- 在 `let-else` 语句上提供 `unwrap_block` 建议
- 在 `for` 和 `while` 表达式上提供 `add_label_to_loop`
- 多个新的代码辅助选项(如 `unwrap_tuple`、`extract_variable` 等)

**其他改进**
- 支持嵌套生命周期处理
- 支持多属性包装/解包
- 支持部分选择的导入合并
- 在 scip 中并行预加载缓存

### 内部改进
- 多个辅助功能迁移到 SyntaxEditor
- 使用 SyntaxFactory 替换部分 ast::make 调用
- 修复 IR 类型打印问题
- 优化测试性能
- 添加 codecov.io 徽章

[
https://rust-analyzer.github.io/thisweek/2026/03/23/changelog-320.html
](
https://rust-analyzer.github.io/thisweek/2026/03/23/changelog-320.html
)
    


### TITLE

## Rust项目激发的学习计划

一位开发者在GitHub上发现了名为Arnis的项目后深受启发。Arnis能够利用OpenStreetMap数据将真实世界的地理信息转换为Minecraft世界。

### 项目构想

受Arnis启发，该开发者计划构建类似项目：将CAD/3D文件（.obj、.stl格式）转换为Minecraft结构，并保持准确的比例缩放。目标用户包括建筑师、工程师和学生，帮助他们在Minecraft中可视化设计作品。

### 个人背景

- 完成了CS50x课程
- 在Zulip项目中有2个被合并的PR
- 掌握Git和Linux基础知识
- 了解一些C语言基础

### 学习计划

1. 学习Rust编程语言（通过Rust Book）
2. 深入理解Arnis项目代码库
3. 为Rust开源项目做贡献
4. 同时开发自己的CAD转换项目

### 时间目标

计划在8个月内完成上述学习和项目开发，目标是获得一份远程实习机会。开发者询问这个计划是否现实可行。

[
https://old.reddit.com/r/rust/comments/1s0tqg3/rust_really_caught_my_eyeand_now_i_want_to_invest/
](
https://old.reddit.com/r/rust/comments/1s0tqg3/rust_really_caught_my_eyeand_now_i_want_to_invest/
)
    


### TITLE

## TideORM：为 Rust 构建的 ActiveRecord 风格 ORM

作者开发了一个名为 TideORM 的项目——这是一个构建在 SeaORM 之上的 ORM 层，为 Rust 带来了 ActiveRecord 风格的 API。

### 项目目标
让来自 Laravel 或 Rails 等框架的开发者在使用 Rust 进行后端开发时感到更加熟悉，同时保持 Rust 的性能和安全性优势。

### 主要特点
- **ActiveRecord 风格的模型**：采用熟悉的 ActiveRecord 模式
- **简洁优雅的语法**：提供清晰且富有表现力的代码写法
- **基于 SeaORM 构建**：确保稳定性和异步支持

### 项目链接
- 核心库：https://github.com/mohamadzoh/tideorm
- CLI 工具：https://github.com/mohamadzoh/tideorm-cli

作者希望获得社区反馈以改进项目。

[
https://old.reddit.com/r/rust/comments/1s17jm2/tideorm_activerecordstyle_orm_for_rust_built_on/
](
https://old.reddit.com/r/rust/comments/1s17jm2/tideorm_activerecordstyle_orm_for_rust_built_on/
)
    


### TITLE

# 特性携带值的设想

## 概述
这篇文章探讨了一个假设性的语言设计概念：如果特性(traits)可以携带值会怎样？

## 核心机制

### 声明全局能力名称
- 使用 `capability my_capability;` 声明一个全局名称

### 隐式参数传递
- 可以在 where 约束中写入 `my_capability: Type`
- 这类似于隐式参数的工作方式
- 编译器会自动传递相应的值
- 如果找不到合适的值，编译器会报错

### 作用域内提供值
- 通过以下语法为特定作用域提供值：
  ```
  with my_capability = some_value() { ... }
  ```
- 在代码块内，该能力将携带指定的值

## 设计意义
这种机制允许在编译时通过类型系统管理和传递上下文相关的值，类似于依赖注入或上下文传递模式，但以更加类型安全和编译器辅助的方式实现。

[
https://nadrieril.github.io/blog/2026/03/22/what-if-traits-carried-values.html
](
https://nadrieril.github.io/blog/2026/03/22/what-if-traits-carried-values.html
)
    


### TITLE

## 最大化极简的视图类型提案

这篇博文提出了一个关于Rust语言视图类型（View Types）的极简提案，旨在解决借用检查器中的字段访问限制问题。

### 核心问题

在Rust中，当一个方法借用`self`时，编译器假定它可能访问所有字段，这导致了不必要的借用冲突。文章通过`MessageProcessor`示例说明：

- 当使用`self.messages.drain(..)`时会产生可变借用
- 随后调用`self.process_message()`会报错，因为编译器认为可能修改包括`messages`在内的任何字段
- 实际上`process_message`只访问`statistics`字段

### 解决方案：视图类型

**语法扩展：**
- 在结构体类型后添加字段列表：`StructName { field1, field2 }`
- `MessageProcessor { statistics }` 表示只能访问`statistics`字段
- `MessageProcessor { .. }` 表示可访问所有字段（等同于当前行为）

**关键特性：**

1. **尊重隐私性**：只能在可见范围内指定字段
2. **可用于self参数**：
   ```rust
   fn process_message(&mut self { statistics }, message: String)
   ```
3. **显式限制借用**：
   ```rust
   let messages = &mut some_variable { messages };
   ```

### 优势

- 让借用检查器明确知道方法访问哪些字段
- 避免不必要的代码重构
- 使合理的代码能够直接通过编译

[
https://smallcultfollowing.com/babysteps/blog/2026/03/21/view-types-max-min/
](
https://smallcultfollowing.com/babysteps/blog/2026/03/21/view-types-max-min/
)
    


--

From 日报小组 Mike

社区学习交流平台订阅：

- [Rustcc论坛: 支持rss](https://rustcc.cn/)
- [微信公众号：Rust语言中文社区](https://rustcc.cn/article?id=ed7c9379-d681-47cb-9532-0db97d883f62)

