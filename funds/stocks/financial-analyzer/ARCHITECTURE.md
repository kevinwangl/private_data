# 财务分析系统架构设计

## 系统概述

财务分析系统是一个基于Rust开发的命令行工具，用于自动化分析上市公司财务报表，生成专业的Excel分析报告。系统支持多数据源、数据验证、财务指标计算和估值分析。

## 核心特性

- 📊 **多数据源支持**：Mock数据、Tushare API
- ✅ **数据验证**：会计恒等式、字段完整性、数值范围检查
- 📈 **财务分析**：资产结构、利润分析、现金流分析、杠杆分析
- 💰 **估值模型**：DCF估值、唐朝估值法
- 📑 **Excel报告**：5个工作表，专业格式，公式自动计算
- 🎨 **可视化优化**：颜色高亮、千分位格式、自适应列宽

## 系统架构

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                            │
│                    (命令行接口层)                              │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────┴──────────────────────────────────────┐
│                    Application Layer                         │
│                      (应用层)                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Analyzer   │  │  Validator   │  │ Excel Writer │      │
│  │   (分析器)    │  │  (验证器)     │  │ (报告生成)    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────┴──────────────────────────────────────┐
│                    Domain Layer                              │
│                     (领域层)                                  │
│  ┌──────────────────────────────────────────────────┐       │
│  │  Financial Models (财务模型)                      │       │
│  │  - FinancialStatement                            │       │
│  │  - BalanceSheet / IncomeStatement / Cashflow    │       │
│  │  - AnalysisResult / ValuationResult             │       │
│  └──────────────────────────────────────────────────┘       │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────┴──────────────────────────────────────┐
│                 Data Source Layer                            │
│                   (数据源层)                                  │
│  ┌──────────────┐  ┌──────────────┐                         │
│  │ Mock Source  │  │Tushare Client│                         │
│  │  (模拟数据)   │  │  (真实数据)   │                         │
│  └──────────────┘  └──────────────┘                         │
└─────────────────────────────────────────────────────────────┘
```

## 模块设计

### 1. CLI模块 (`src/cli/`)
**职责**：命令行接口，参数解析

**核心功能**：
- 命令定义：`analyze` 命令
- 参数解析：股票代码、年份、数据源、输出路径
- 默认值处理：输出文件名自动生成

**技术栈**：clap 4.x

### 2. Domain模块 (`src/domain/`)
**职责**：领域模型定义

**核心类型**：
```rust
// 报表类型
enum ReportType {
    BalanceSheet,      // 资产负债表
    IncomeStatement,   // 利润表
    CashflowStatement, // 现金流量表
}

// 财务报表
struct FinancialStatement {
    stock_code: String,
    report_date: NaiveDate,
    report_type: ReportType,
    items: HashMap<String, Decimal>,
}

// 分析结果
struct AnalysisResult {
    stock_code: String,
    years: Vec<i32>,
    asset_structure: AssetStructureAnalysis,
    profit_analysis: ProfitAnalysis,
    valuation: Option<ValuationResult>,
    statements: Vec<FinancialStatement>,
}
```

### 3. Data Source模块 (`src/data_source/`)
**职责**：数据获取抽象层

**接口设计**：
```rust
#[async_trait]
pub trait DataSource: Send + Sync {
    async fn fetch_balance_sheet(&self, ...) -> Result<Vec<BalanceSheet>>;
    async fn fetch_income_statement(&self, ...) -> Result<Vec<IncomeStatement>>;
    async fn fetch_cashflow_statement(&self, ...) -> Result<Vec<CashflowStatement>>;
}
```

**实现**：
- `MockDataSource`：模拟数据，用于测试和演示
- `TushareClient`：Tushare API集成，真实数据

### 4. Validation模块 (`src/validation/`)
**职责**：数据验证

**验证规则**：
- 会计恒等式：资产 = 负债 + 所有者权益
- 必填字段检查
- 数值范围验证
- 逻辑一致性检查

**配置文件**：`config/validation_rules.toml`

### 5. Analyzer模块 (`src/analyzer/`)
**职责**：财务分析和估值计算

**子模块**：
- `calculator.rs`：财务指标计算
  - 资产结构分析（经营性/金融性资产占比）
  - 利润分析（毛利率、核心利润率、净利润率）
  - 杠杆分析（经营杠杆、财务杠杆、总杠杆）
  
- `valuation.rs`：估值模型
  - DCF估值（现金流折现）
  - 唐朝估值法（PE倍数法）

### 6. Excel模块 (`src/excel/`)
**职责**：Excel报告生成

**5个工作表**：
1. **资产&负债结构分析**：流动/非流动资产负债分类
2. **(经营性&金融性)资产&负债结构分析**：经营性/金融性分类，比率计算
3. **利润&现金流结构分析**：利润表、现金流、EBIT、杠杆、估值
4. **综合实力分析**：关键指标汇总、收益/风险评价框架
5. **资产负债表分析视角**：关键科目汇总

**格式优化**：
- 标题行：深蓝色背景 + 白色粗体
- 子标题：浅蓝色背景 + 粗体
- 重点数据：黄色高亮（资产比率、核心利润、估值）
- 数据格式：千分位、百分比
- 列宽：35字符（项目名称）、16字符（数据）
- 行高：22像素

**技术栈**：rust_xlsxwriter

### 7. Utils模块 (`src/utils/`)
**职责**：工具函数和配置管理

**功能**：
- 配置文件加载（TOML）
- 日期处理
- 数据转换

## 数据流

```
用户输入
   ↓
CLI解析参数
   ↓
选择数据源 (Mock/Tushare)
   ↓
获取财务报表数据
   ↓
数据验证 (可选)
   ↓
财务分析计算
   ├─ 资产结构分析
   ├─ 利润分析
   └─ 估值计算
   ↓
生成Excel报告
   ├─ 5个工作表
   ├─ 数据填充
   ├─ 公式生成
   └─ 格式美化
   ↓
保存文件
```

## 配置文件

### 1. `config/account_mapping.toml`
科目映射配置，定义财务科目的分类：
- 经营性资产/负债
- 金融性资产/负债

### 2. `config/validation_rules.toml`
验证规则配置：
- 必填字段列表
- 数值范围限制
- 会计恒等式容差

### 3. `config/data_sources.toml`
数据源配置：
- Tushare API端点
- 超时设置
- 重试策略

## 技术栈

### 核心依赖
- **Rust 1.70+**：系统编程语言
- **Tokio**：异步运行时
- **clap 4.x**：命令行解析
- **rust_xlsxwriter**：Excel生成
- **reqwest**：HTTP客户端
- **serde**：序列化/反序列化
- **rust_decimal**：高精度数值计算
- **chrono**：日期时间处理
- **anyhow**：错误处理
- **tracing**：日志记录

### 开发工具
- **cargo**：包管理和构建
- **rustfmt**：代码格式化
- **clippy**：代码检查

## 性能特性

- **异步I/O**：使用Tokio实现高效的网络请求
- **零拷贝**：Rust所有权系统避免不必要的数据复制
- **编译优化**：Release模式下启用LTO和优化
- **内存安全**：编译时保证内存安全，无GC开销

## 扩展性设计

### 1. 数据源扩展
实现 `DataSource` trait 即可添加新数据源：
```rust
pub struct NewDataSource;

#[async_trait]
impl DataSource for NewDataSource {
    // 实现接口方法
}
```

### 2. 验证规则扩展
在 `validation_rules.toml` 中添加新规则

### 3. 分析指标扩展
在 `Calculator` 中添加新的计算方法

### 4. 估值模型扩展
在 `Valuator` 中添加新的估值算法

### 5. Excel格式扩展
在 `ExcelWriter` 中添加新的工作表生成方法

## 安全性

- **类型安全**：Rust强类型系统防止类型错误
- **内存安全**：编译时保证无内存泄漏、无悬垂指针
- **并发安全**：Send/Sync trait保证线程安全
- **错误处理**：Result类型强制错误处理
- **环境变量**：敏感信息（API Token）通过环境变量配置

## 部署

### 编译
```bash
cargo build --release
```

### 运行
```bash
./target/release/financial-analyzer analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source tushare
```

### 配置
```bash
export TUSHARE_TOKEN=your_token_here
```

## 未来规划

1. **Web界面**：提供Web UI进行交互式分析
2. **数据库支持**：缓存历史数据，减少API调用
3. **批量分析**：支持多只股票批量分析
4. **自定义模板**：用户自定义Excel模板
5. **图表生成**：在Excel中生成图表
6. **PDF导出**：支持PDF格式报告
7. **实时监控**：定期自动分析并发送报告

## 版本历史

- **v1.0.0** (2026-01-14)：完整功能版本，5个工作表，格式优化
- **v0.3.0** (2026-01-14)：添加Tushare集成和估值模型
- **v0.2.0** (2026-01-14)：添加配置和验证功能
- **v0.1.0** (2026-01-13)：MVP版本，基础分析功能
