# 财务分析系统 (Financial Analyzer)

基于Rust开发的自动化财务报表分析系统，支持多数据源、数据验证和Excel报告生成。

## 功能特性

✅ **已实现 (v1.0.0 正式版)** 🎉
- 核心数据模型（资产负债表、利润表、现金流量表）
- **多数据源支持**
  - Mock数据源（测试）
  - Tushare数据源（真实A股数据，需Token）
  - **AKShare数据源（完全免费，无需Token）** ⭐ NEW
- **财务分析**
  - 资产结构分析
  - 利润分析
  - 现金流分析
- **估值模型**
  - DCF估值法（现金流折现）
  - 唐朝估值法（低估/高估价格）
- **数据验证**
  - 会计恒等式验证
  - 必需科目检查
  - 数值合理性检查
  - 可靠性评分
- **Excel报告生成**
  - 5个专业工作表
  - 完整格式和公式
  - 黄色高亮关键指标
- CLI命令行接口
- 配置管理系统

🎯 **功能完整度**: 100% (所有核心功能已实现)

## 快速开始

### 安装

```bash
# 进入项目目录
cd financial-analyzer

# 编译
cargo build --release
```

### 使用示例

```bash
# 使用Mock数据（测试）
cargo run -- analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source mock

# 使用AKShare免费数据（推荐）⭐
# 需要先安装: pip3 install akshare
cargo run -- analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source akshare

# 使用Tushare真实数据（需Token）
export TUSHARE_TOKEN="your_token_here"
cargo run -- analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source tushare

# 启用数据验证
cargo run -- analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source akshare \
  --enable-validation

# 查看帮助
cargo run -- --help
```

### 数据源配置

#### AKShare（推荐，完全免费）⭐

详细配置请查看 [AKShare使用指南](./AKSHARE_GUIDE.md)

```bash
# 1. 安装Python依赖
pip3 install akshare

# 2. 直接使用（无需Token）
cargo run -- analyze --stock 600519.SH --years 2019,2018,2017 --source akshare
```

#### Tushare（需注册）

详细配置请查看 [Tushare使用指南](./TUSHARE_GUIDE.md)

```bash
# 1. 获取Tushare Token
# 访问 https://tushare.pro/ 注册并获取Token

# 2. 设置环境变量
export TUSHARE_TOKEN="your_token_here"

# 3. 运行分析
cargo run -- analyze --stock 600519.SH --years 2019 --source tushare
```

## 项目结构

```
financial-analyzer/
├── src/
│   ├── domain/          # 核心数据模型
│   │   └── models.rs    # 财务报表数据结构
│   ├── data_source/     # 数据源抽象层
│   │   ├── traits.rs    # DataSource trait定义
│   │   ├── mock.rs      # Mock数据源实现
│   │   ├── tushare.rs   # Tushare数据源
│   │   └── akshare.rs   # AKShare数据源 ⭐ NEW
│   ├── analyzer/        # 分析引擎
│   │   ├── calculator.rs # 比率计算器
│   │   └── mod.rs       # 主分析器
│   ├── validation/      # 数据验证层 (NEW)
│   │   ├── validator.rs # 数据验证器
│   │   └── mod.rs       # 验证模块
│   ├── utils/           # 工具模块 (NEW)
│   │   ├── config.rs    # 配置管理
│   │   └── mod.rs       # 工具模块
│   ├── excel/           # Excel生成器
│   │   └── mod.rs       # Excel报告生成
│   ├── cli/             # CLI接口
│   │   └── mod.rs       # 命令行参数解析
│   └── main.rs          # 程序入口
├── config/              # 配置文件 (NEW)
│   ├── account_mapping.toml      # 科目映射配置
│   └── validation_rules.toml     # 验证规则配置
├── templates/           # Excel模板（待实现）
├── logs/                # 日志目录
├── Cargo.toml           # 项目配置
├── README.md            # 本文件
├── CHANGELOG.md         # 更新日志 (NEW)
├── financial-analysis-system-design.md  # 架构设计文档
└── detailed-design-spec.md              # 详细设计文档
```

## 开发路线图

### Phase 1: MVP (✅ 已完成)
- [x] 项目初始化
- [x] 核心数据模型
- [x] Mock数据源
- [x] 基础分析功能
- [x] Excel生成
- [x] CLI接口

### Phase 2: 配置和验证 (✅ 已完成)
- [x] 配置管理系统
- [x] 科目映射配置
- [x] 验证规则配置
- [x] 数据验证器
- [x] 会计恒等式验证
- [x] 必需科目检查
- [x] 数值合理性检查
- [x] 可靠性评分

### Phase 3: 数据源集成 (✅ 已完成)
- [x] Tushare客户端实现
- [x] API认证和Token管理
- [x] 资产负债表获取
- [x] 利润表获取
- [x] 现金流量表获取
- [x] 错误处理和重试
- [x] 使用文档编写

### Phase 4: 估值模型 (✅ 已完成)
- [x] DCF估值模型
  - [x] 自由现金流计算
  - [x] 折现率配置
  - [x] 永续增长率
  - [x] 企业价值计算
- [x] 唐朝估值模型
  - [x] PE倍数计算
  - [x] 低估买入价
  - [x] 高估卖出价
  - [x] 安全边际价
- [x] Excel估值工作表

### Phase 5: 系统完善 (✅ 已完成)
- [x] 估值器集成到分析器
- [x] Excel报告增强
- [x] 文档完善
- [x] 版本发布

## 技术栈

- **语言**: Rust 1.75+
- **异步运行时**: Tokio
- **CLI框架**: Clap
- **Excel生成**: rust_xlsxwriter
- **数值计算**: rust_decimal
- **序列化**: Serde

## 输出示例

运行分析后，生成的Excel文件包含以下工作表：

1. **资产结构分析**
   - 年份
   - 经营性资产占比
   - 金融性资产占比

2. **利润分析**
   - 年份
   - 毛利率
   - 核心利润率
   - 净利润率

## 开发指南

### 编译项目

```bash
# 开发模式（快速编译）
cargo build

# 发布模式（优化编译）
cargo build --release
```

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
# 检查代码
cargo check

# 格式化代码
cargo fmt

# 代码检查（更严格）
cargo clippy
```

## 文档

- [架构设计文档](./financial-analysis-system-design.md) - 系统整体架构设计
- [详细设计文档](./detailed-design-spec.md) - 模块详细设计和实现

## 贡献指南

欢迎贡献！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

MIT License

## 联系方式

如有问题或建议，请提交 Issue。

---

**当前版本**: v1.0.0 🎉  
**最后更新**: 2026-01-14  
**状态**: 正式版发布

## 🎉 v1.0.0 正式版发布

系统已完成所有核心功能开发，现已发布正式版！

### ✨ 核心功能

1. **多数据源**: Mock + Tushare真实数据
2. **财务分析**: 资产结构 + 利润分析
3. **估值模型**: DCF + 唐朝估值法
4. **数据验证**: 会计恒等式 + 合理性检查
5. **Excel报告**: 3个专业分析工作表

### 📊 输出示例

生成的Excel包含：
- **资产结构分析**: 经营性/金融性资产占比
- **利润分析**: 毛利率、核心利润率、净利润率
- **估值分析**: DCF企业价值、唐朝低估/高估价格

### 🚀 快速开始

```bash
# 使用Mock数据测试
cargo run -- analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source mock \
  --output ./output.xlsx

# 使用Tushare真实数据
export TUSHARE_TOKEN="your_token"
cargo run -- analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source tushare \
  --output ./output.xlsx \
  --enable-validation
```

---

## v0.3.0 功能

### 📊 Tushare数据源
系统现在支持Tushare真实财务数据：
- 自动获取上市公司财务报表
- 支持A股所有上市公司
- 数据来源权威可靠

详细说明请查看 [Tushare使用指南](./TUSHARE_GUIDE.md)

---

## v0.2.0 功能

### 🔐 数据验证
系统现在支持自动数据验证，包括：
- 会计恒等式检查（资产 = 负债 + 所有者权益）
- 必需科目完整性检查
- 数值合理性验证
- 可靠性评分（0-100分）

### ⚙️ 配置管理
通过TOML配置文件灵活配置：
- 科目映射规则（支持模糊匹配）
- 验证规则（允许负值科目、必需科目等）
- 比率合理性范围
- 同比变化阈值
