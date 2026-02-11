# 财务分析系统 - 架构图

## 1. 整体分层架构

```
┌─────────────────────────────────────────────────────────────┐
│                     CLI Layer (cli/)                        │
│                  命令行接口 - 用户交互                        │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                  Presentation Layer                         │
│  ┌──────────────────────┐    ┌──────────────────────┐      │
│  │   Excel Generator    │    │   Text Reporter      │      │
│  │   (excel/)           │    │   (report/)          │      │
│  │   - 6个工作表         │    │   - TXT格式          │      │
│  │   - 格式化           │    │   - 格式化输出        │      │
│  └──────────────────────┘    └──────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   Business Layer                            │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         Financial Analyzer (analyzer/)               │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │  │
│  │  │   Ratio     │  │  Valuator   │  │ Sensitivity │  │  │
│  │  │ Calculator  │  │  - DCF      │  │  Analysis   │  │  │
│  │  │             │  │  - Tangchao │  │             │  │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  │  │
│  └──────────────────────────────────────────────────────┘  │
│                              ↓                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         Data Validator (validation/)                 │  │
│  │  - 会计恒等式  - 必需科目  - 数值范围  - 可靠性评分   │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                    Data Source Layer                        │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              DataSource Trait                        │  │
│  └──────────────────────────────────────────────────────┘  │
│         ↓                  ↓                  ↓             │
│  ┌──────────┐      ┌──────────┐      ┌──────────┐         │
│  │  Mock    │      │ Tushare  │      │ AKShare  │ ⭐      │
│  │  (测试)   │      │ (需Token)│      │  (免费)   │         │
│  └──────────┘      └──────────┘      └──────────┘         │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                    Domain Layer                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              Core Models (domain/)                   │  │
│  │  - BalanceSheet  - IncomeStatement  - CashflowStatement│
│  │  - AnalysisResult  - ValuationResult                │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                  Infrastructure Layer                       │
│  ┌──────────────────────┐    ┌──────────────────────┐      │
│  │   Config Manager     │    │   Utilities          │      │
│  │   (utils/)           │    │   - Helpers          │      │
│  │   - TOML配置         │    │   - Formatters       │      │
│  └──────────────────────┘    └──────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

## 2. 数据流图

```
用户输入
  ↓
CLI解析参数
  ↓
选择数据源 (Mock/Tushare/AKShare)
  ↓
获取财务数据 (资产负债表/利润表/现金流量表)
  ↓
数据验证 (可选)
  ├─ 会计恒等式检查
  ├─ 必需科目检查
  ├─ 数值合理性检查
  └─ 可靠性评分
  ↓
财务分析
  ├─ 资产结构分析
  ├─ 利润分析
  ├─ 杠杆分析
  ├─ 估值分析 (DCF + 唐朝)
  └─ 敏感性分析
  ↓
生成报告
  ├─ Excel报告 (6个工作表)
  └─ TXT报告
  ↓
输出文件
```

## 3. 模块依赖关系

```
main.rs
  ├─→ cli/          (命令行解析)
  ├─→ data_source/  (数据获取)
  │    ├─→ domain/  (数据模型)
  │    └─→ utils/   (配置)
  ├─→ validation/   (数据验证)
  │    ├─→ domain/
  │    └─→ utils/
  ├─→ analyzer/     (财务分析)
  │    ├─→ domain/
  │    └─→ validation/
  ├─→ excel/        (Excel生成)
  │    ├─→ domain/
  │    └─→ analyzer/
  └─→ report/       (文本报告)
       ├─→ domain/
       └─→ analyzer/
```

## 4. Excel模块内部结构（需要重构）

```
excel/
├── mod.rs (1,105行) ❌ 过大！
│   ├─ ExcelWriter
│   ├─ write_sheet1_asset_liability()
│   ├─ write_sheet2_operating_financial()
│   ├─ write_sheet3_profit_cashflow()
│   ├─ write_sheet4_comprehensive()
│   ├─ write_sheet5_balance_perspective()
│   └─ write_sheet6_sensitivity()
│
├── enhanced.rs (183行)
│   └─ EnhancedExcelWriter
│       ├─ write_balance_sheet_analysis()
│       ├─ write_income_analysis()
│       ├─ write_cashflow_analysis()
│       ├─ write_comprehensive_analysis()
│       └─ write_valuation_analysis()
│
├── enhanced_balance_sheet.rs (213行)
├── enhanced_profit_cashflow.rs (261行)
├── enhanced_comprehensive.rs (251行)
├── enhanced_sensitivity.rs (190行)
├── helpers.rs (210行)
├── descriptions.rs (61行)
└── sheet_builder.rs (86行)

问题: 
- mod.rs 过大，难以维护
- 代码重复（格式化逻辑）
- 缺乏抽象
```

## 5. 建议的Excel模块重构结构

```
excel/
├── mod.rs (100行以内)
│   ├─ pub use writer::ExcelWriter;
│   ├─ pub use enhanced::EnhancedExcelWriter;
│   └─ pub use helpers::*;
│
├── writer.rs (主生成器)
│   └─ ExcelWriter::generate()
│
├── sheets/ (原版工作表)
│   ├── asset_liability.rs
│   ├── operating_financial.rs
│   ├── profit_cashflow.rs
│   ├── comprehensive.rs
│   ├── balance_perspective.rs
│   └── sensitivity.rs
│
├── enhanced/ (优化版工作表)
│   ├── mod.rs
│   ├── writer.rs (EnhancedExcelWriter)
│   ├── balance_sheet.rs
│   ├── profit_cashflow.rs
│   ├── comprehensive.rs
│   └── sensitivity.rs
│
├── common/ (公共逻辑)
│   ├── formats.rs (格式创建)
│   ├── styles.rs (样式定义)
│   └── layout.rs (布局工具)
│
├── helpers.rs (数据辅助)
├── descriptions.rs (指标说明)
└── sheet_builder.rs (工作表构建器)
```

## 6. 核心类图

```
┌─────────────────────────────────────────┐
│         FinancialAnalyzer               │
├─────────────────────────────────────────┤
│ - statements: Vec<FinancialStatement>   │
│ - validator: Option<DataValidator>      │
│ - valuation_params: ValuationParams     │
├─────────────────────────────────────────┤
│ + new() -> Self                         │
│ + with_validator() -> Self              │
│ + with_valuation_params() -> Self       │
│ + analyze() -> Result<AnalysisResult>   │
└─────────────────────────────────────────┘
              ↓ uses
┌─────────────────────────────────────────┐
│         RatioCalculator                 │
├─────────────────────────────────────────┤
│ + calculate_asset_structure()           │
│ + calculate_profit_ratios()             │
│ + calculate_leverage()                  │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│            Valuator                     │
├─────────────────────────────────────────┤
│ - params: ValuationParams               │
├─────────────────────────────────────────┤
│ + calculate() -> ValuationResult        │
│ + calculate_dcf() -> DCFValuation       │
│ + calculate_tangchao() -> TangchaoVal   │
└─────────────────────────────────────────┘
```

## 7. 配置文件结构

```
config/
├── account_mapping.toml
│   └── 科目映射规则
│       ├─ balance_sheet
│       ├─ income_statement
│       └─ cashflow_statement
│
└── validation_rules.toml
    ├─ allow_negative (允许负值科目)
    ├─ required_accounts (必需科目)
    ├─ ratio_ranges (比率范围)
    └─ yoy_thresholds (同比阈值)
```

## 8. 输出文件结构

```
analyzer-report/
├── {stock_code}_财务分析.xlsx
│   ├─ Sheet1: 资产负债表分析(优化版)
│   ├─ Sheet2: 利润&现金流分析(优化版)
│   ├─ Sheet3: 现金流分析
│   ├─ Sheet4: 综合实力分析(优化版)
│   ├─ Sheet5: 估值分析
│   └─ Sheet6: 敏感性分析(优化版) ⭐
│
└── {stock_code}_财务分析.txt
    ├─ 基本信息
    ├─ 资产负债表
    ├─ 利润表
    ├─ 现金流量表
    ├─ 财务比率
    ├─ 杠杆分析
    ├─ 估值分析
    └─ 敏感性分析
```

## 9. 技术栈

```
核心语言: Rust 1.75+
├─ 异步运行时: tokio
├─ CLI框架: clap
├─ Excel生成: rust_xlsxwriter
├─ 数值计算: rust_decimal
├─ 序列化: serde
├─ 错误处理: anyhow
├─ 日志: tracing
└─ 配置: toml
```

## 10. 性能特征

```
编译时间: ~2秒 (增量编译)
运行时间: ~3-5秒 (含数据获取)
内存占用: ~50MB
二进制大小: ~10MB (release)
并发支持: ✅ (tokio异步)
```
