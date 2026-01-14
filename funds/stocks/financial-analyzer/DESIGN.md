# 财务分析系统详细设计

## 1. 数据模型设计

### 1.1 核心数据结构

#### FinancialStatement（财务报表）
```rust
pub struct FinancialStatement {
    pub stock_code: String,           // 股票代码
    pub report_date: NaiveDate,       // 报告日期
    pub report_type: ReportType,      // 报表类型
    pub items: HashMap<String, Decimal>, // 科目数据
}
```

**设计要点**：
- 使用 `Decimal` 类型保证财务数据精度
- `HashMap` 存储科目数据，灵活支持不同科目
- 统一的数据结构支持三种报表类型

#### AnalysisResult（分析结果）
```rust
pub struct AnalysisResult {
    pub stock_code: String,
    pub years: Vec<i32>,
    pub asset_structure: AssetStructureAnalysis,
    pub profit_analysis: ProfitAnalysis,
    pub valuation: Option<ValuationResult>,
    pub statements: Vec<FinancialStatement>,
}
```

**设计要点**：
- 包含原始报表数据，支持Excel公式引用
- 分析结果分模块存储
- 估值结果可选（可能计算失败）

### 1.2 分析结果结构

#### AssetStructureAnalysis（资产结构分析）
```rust
pub struct AssetStructureAnalysis {
    pub years: Vec<i32>,
    pub operating_asset_ratio: Vec<Decimal>,  // 经营性资产占比
    pub financial_asset_ratio: Vec<Decimal>,  // 金融性资产占比
}
```

#### ProfitAnalysis（利润分析）
```rust
pub struct ProfitAnalysis {
    pub years: Vec<i32>,
    pub gross_margin: Vec<Decimal>,        // 毛利率
    pub core_profit_margin: Vec<Decimal>,  // 核心利润率
    pub net_profit_margin: Vec<Decimal>,   // 净利润率
}
```

#### ValuationResult（估值结果）
```rust
pub struct ValuationResult {
    pub dcf: DCFValuation,
    pub tangchao: TangchaoValuation,
}

pub struct DCFValuation {
    pub enterprise_value: Decimal,  // 企业价值
    pub price_per_share: Decimal,   // 每股价格
}

pub struct TangchaoValuation {
    pub low_estimate: Decimal,   // 低估买入点
    pub high_estimate: Decimal,  // 高估卖出点
}
```

## 2. 数据源设计

### 2.1 DataSource接口

```rust
#[async_trait]
pub trait DataSource: Send + Sync {
    async fn fetch_balance_sheet(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<BalanceSheet>>;

    async fn fetch_income_statement(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<IncomeStatement>>;

    async fn fetch_cashflow_statement(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<CashflowStatement>>;
}
```

### 2.2 MockDataSource实现

**用途**：测试和演示

**数据生成规则**：
```rust
// 资产负债表
货币资金: 1,000,000
固定资产: 2,000,000
应收账款: 500,000
存货: 300,000
短期借款: 600,000
长期借款: 900,000
所有者权益: 2,500,000

// 利润表
营业收入: 5,000,000
营业成本: 3,000,000
净利润: 800,000

// 现金流量表
经营活动现金流: 900,000
```

### 2.3 TushareClient实现

**API端点**：
- 资产负债表：`balancesheet`
- 利润表：`income`
- 现金流量表：`cashflow`

**字段映射**：
```rust
// Tushare字段 -> 系统字段
"total_assets" -> "资产总计"
"total_liab" -> "负债合计"
"total_hldr_eqy_exc_min_int" -> "所有者权益合计"
"revenue" -> "营业总收入"
"oper_cost" -> "营业总成本"
"n_income" -> "净利润"
```

**错误处理**：
- 网络超时：重试3次
- API限流：等待后重试
- 数据缺失：返回错误信息

## 3. 验证器设计

### 3.1 验证规则

#### 会计恒等式验证
```rust
资产总计 = 负债合计 + 所有者权益合计
容差: 0.01 (1分钱)
```

#### 必填字段验证
```toml
[balance_sheet.required_fields]
fields = [
    "资产总计",
    "负债合计",
    "所有者权益合计",
    "货币资金",
    "固定资产"
]
```

#### 数值范围验证
```toml
[balance_sheet.value_ranges]
"资产总计" = { min = 0.0 }
"负债合计" = { min = 0.0 }
"货币资金" = { min = 0.0 }
```

### 3.2 验证流程

```
1. 检查必填字段
   ↓
2. 检查数值范围
   ↓
3. 检查会计恒等式
   ↓
4. 生成验证报告
   - errors: 严重错误
   - warnings: 警告信息
```

## 4. 分析器设计

### 4.1 Calculator（计算器）

#### 资产结构分析
```rust
fn calculate_asset_structure(balance_sheets: &[BalanceSheet]) -> Result<AssetStructureAnalysis> {
    // 1. 提取资产总计
    let total_assets = get_value("资产总计");
    
    // 2. 计算经营性资产
    let operating_assets = sum([
        "货币资金", "固定资产", "应收票据",
        "应收账款", "预付款项", "存货", "无形资产"
    ]);
    
    // 3. 计算金融性资产
    let financial_assets = sum([
        "交易性金融资产", "长期股权投资",
        "持有至到期投资", "投资性房地产", ...
    ]);
    
    // 4. 计算占比
    operating_ratio = operating_assets / total_assets;
    financial_ratio = financial_assets / total_assets;
}
```

#### 利润分析
```rust
fn calculate_profit_ratios(income_statements: &[IncomeStatement]) -> Result<ProfitAnalysis> {
    // 毛利率
    gross_margin = (营业收入 - 营业成本) / 营业收入;
    
    // 核心利润率
    core_profit = 营业收入 - 营业成本 - 税金及附加 
                  - 销售费用 - 管理费用 - 研发费用 - 财务费用;
    core_profit_margin = core_profit / 营业收入;
    
    // 净利润率
    net_profit_margin = 净利润 / 营业收入;
}
```

#### 杠杆分析
```rust
// EBIT（息税前利润）
EBIT = 净利润 + 财务费用 + 税金及附加;

// 经营杠杆
operating_leverage = (EBIT + 销售费用 + 管理费用 + 研发费用) / EBIT;

// 财务杠杆
financial_leverage = EBIT / (EBIT - 财务费用);

// 总杠杆
total_leverage = operating_leverage * financial_leverage;
```

### 4.2 Valuator（估值器）

#### DCF估值模型
```rust
fn calculate_dcf(income_statements: &[IncomeStatement], 
                 cashflow_statements: &[CashflowStatement]) -> Result<DCFValuation> {
    // 1. 计算自由现金流均值
    fcf_avg = average(经营活动现金流[3年]);
    
    // 2. 参数设置
    discount_rate = 0.08;      // 折现率 8%
    growth_rate = 0.10;        // 增长率 10%
    perpetual_growth = 0.03;   // 永续增长率 3%
    
    // 3. 计算未来3年现金流现值
    pv_year1 = fcf_avg * (1 + growth_rate) / (1 + discount_rate)^1;
    pv_year2 = fcf_avg * (1 + growth_rate)^2 / (1 + discount_rate)^2;
    pv_year3 = fcf_avg * (1 + growth_rate)^3 / (1 + discount_rate)^3;
    
    // 4. 计算永续年金价值
    terminal_value = (sum(pv_year1:3) * (1 + perpetual_growth)) 
                     / (discount_rate - perpetual_growth);
    
    // 5. 企业价值
    enterprise_value = sum(pv_year1:3) + terminal_value;
    
    // 6. 每股价格
    price_per_share = enterprise_value / total_shares;
}
```

#### 唐朝估值法
```rust
fn calculate_tangchao(income_statements: &[IncomeStatement]) -> Result<TangchaoValuation> {
    // 1. 获取最新净利润
    net_profit = income_statements[0].净利润;
    
    // 2. 参数设置
    growth_rate = 0.10;           // 净利润增长率 10%
    low_risk_rate = 0.04;         // 低估区域无风险收益率 4%
    high_risk_rate = 0.02;        // 高估区域无风险收益率 2%
    
    // 3. 计算3年后净利润
    future_profit = net_profit * (1 + growth_rate)^3;
    
    // 4. 计算估值
    low_pe = 1 / low_risk_rate;   // 低估PE = 25倍
    high_pe = 1 / high_risk_rate; // 高估PE = 50倍
    
    low_estimate = future_profit * low_pe;
    high_estimate = future_profit * high_pe;
    
    // 5. 安全边际（7折）
    safe_buy_point = low_estimate * 0.7;
}
```

## 5. Excel生成器设计

### 5.1 格式定义

```rust
// 标题格式
header_fmt = Format::new()
    .set_bold()
    .set_font_size(12)
    .set_background_color(0x4472C4)  // 深蓝色
    .set_font_color(Color::White)
    .set_align(FormatAlign::Center)
    .set_border(FormatBorder::Thin);

// 子标题格式
subheader_fmt = Format::new()
    .set_bold()
    .set_background_color(0xD9E1F2)  // 浅蓝色
    .set_border(FormatBorder::Thin);

// 数据格式
number_fmt = Format::new()
    .set_num_format("#,##0")  // 千分位
    .set_border(FormatBorder::Thin);

// 百分比格式
percent_fmt = Format::new()
    .set_num_format("0.00%")
    .set_border(FormatBorder::Thin);

// 高亮格式
highlight_fmt = Format::new()
    .set_num_format("0.00%")
    .set_background_color(0xFFFF00)  // 黄色
    .set_bold()
    .set_border(FormatBorder::Thin);
```

### 5.2 工作表结构

#### Sheet 1: 资产&负债结构分析
```
行1: 空
行2: 标题行（年份）
行3: 子标题（项目）
行4-6: 流动资产（货币资金、应收账款、存货）
行9: 非流动资产（固定资产）
```

#### Sheet 2: (经营性&金融性)资产&负债结构分析
```
列A-B: 资产分类和项目
列C-E: 3年数据
列G-H: 负债分类和项目
列I-K: 3年数据

行21-22: 资产比率（黄色高亮）
行20-21: 负债比率（黄色高亮）
```

#### Sheet 3: 利润&现金流结构分析
```
左侧（列B-E）:
- 利润表数据
- 现金流数据
- 毛利、核心利润（黄色高亮）

中间（列F-I）:
- DCF估值
- 唐朝估值（黄色高亮）

右侧（列J-M）:
- EBIT
- 经营杠杆
- 财务杠杆
- 总杠杆
```

#### Sheet 4: 综合实力分析
```
上部: 关键指标汇总
- 货币资金、存货、固定资产
- 核心利润（黄色高亮）
- 经营现金流、资产总计

下部: 评价框架
- 收益评价指标
- 风险评价指标
```

#### Sheet 5: 资产负债表分析视角
```
简化的资产负债表关键科目
- 资产总计、流动/非流动资产
- 负债合计、流动/非流动负债
- 所有者权益
- 关键科目（货币资金、应收账款、存货等）
```

### 5.3 列宽和行高

```rust
// 列宽设置
worksheet.set_column_width(0, 20)?;   // A列 - 分类
worksheet.set_column_width(1, 35)?;   // B列 - 项目名称
worksheet.set_column_width(2, 16)?;   // C列 - 数据
worksheet.set_column_width(3, 16)?;   // D列 - 数据
worksheet.set_column_width(4, 16)?;   // E列 - 数据
// ... 其他列

// 行高设置
for row in 0..=end_row {
    worksheet.set_row_height(row, 22)?;
}
```

### 5.4 公式生成

```rust
// 毛利公式
worksheet.write_formula(20, 2, "=C3-C4")?;

// 毛利率公式（带错误处理）
worksheet.write_formula_with_format(
    21, 2, 
    "=IF(C3=0,0,C20/C3)", 
    &highlight_fmt
)?;

// 资产比率公式
worksheet.write_formula_with_format(
    21, 2,
    "=IF(C21=0,0,SUM(C4:C10)/C21)",
    &highlight_fmt
)?;

// DCF估值公式
worksheet.write_formula(
    25, 7,
    "=H25/H20"  // 企业价值 / 总股本
)?;

// 唐朝估值公式
worksheet.write_formula_with_format(
    30, 7,
    "=(C19*POWER(1+H28,3))*J29",  // 未来净利润 * PE倍数
    &highlight_fmt
)?;
```

## 6. 错误处理设计

### 6.1 错误类型

```rust
#[derive(Debug, thiserror::Error)]
pub enum AnalysisError {
    #[error("数据源错误: {0}")]
    DataSourceError(String),
    
    #[error("验证失败: {0}")]
    ValidationError(String),
    
    #[error("计算错误: {0}")]
    CalculationError(String),
    
    #[error("Excel生成错误: {0}")]
    ExcelError(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),
}
```

### 6.2 错误传播

```rust
// 使用 ? 操作符传播错误
let data = data_source.fetch_balance_sheet(...).await?;

// 使用 anyhow::Context 添加上下文
let result = calculate_ratios(&data)
    .context("计算财务比率时出错")?;
```

## 7. 配置管理

### 7.1 配置文件结构

```toml
# config/account_mapping.toml
[operating_assets]
accounts = [
    "货币资金",
    "固定资产",
    "应收票据",
    "应收账款",
    "预付款项",
    "存货",
    "无形资产"
]

[financial_assets]
accounts = [
    "交易性金融资产",
    "长期股权投资",
    "持有至到期投资",
    # ...
]
```

### 7.2 配置加载

```rust
pub struct Config {
    pub account_mapping: AccountMapping,
    pub validation_rules: ValidationRules,
    pub data_sources: DataSourceConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        let account_mapping = load_toml("config/account_mapping.toml")?;
        let validation_rules = load_toml("config/validation_rules.toml")?;
        let data_sources = load_toml("config/data_sources.toml")?;
        
        Ok(Config {
            account_mapping,
            validation_rules,
            data_sources,
        })
    }
}
```

## 8. 性能优化

### 8.1 异步并发

```rust
// 并发获取三种报表
let (balance_sheets, income_statements, cashflow_statements) = tokio::join!(
    data_source.fetch_balance_sheet(...),
    data_source.fetch_income_statement(...),
    data_source.fetch_cashflow_statement(...),
);
```

### 8.2 内存优化

- 使用 `&str` 而不是 `String` 传递参数
- 避免不必要的克隆
- 使用迭代器而不是中间集合

### 8.3 编译优化

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

## 9. 测试策略

### 9.1 单元测试

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_gross_margin() {
        let revenue = Decimal::from(1000);
        let cost = Decimal::from(600);
        let margin = calculate_gross_margin(revenue, cost);
        assert_eq!(margin, Decimal::from_str("0.4").unwrap());
    }
}
```

### 9.2 集成测试

```rust
#[tokio::test]
async fn test_full_analysis_flow() {
    let data_source = MockDataSource::new();
    let analyzer = FinancialAnalyzer::new();
    
    let result = analyzer
        .analyze("600519.SH", vec![2019, 2018, 2017], &data_source)
        .await
        .unwrap();
    
    assert!(result.asset_structure.operating_asset_ratio[0] > Decimal::ZERO);
}
```

## 10. 部署和运维

### 10.1 编译发布

```bash
# 编译release版本
cargo build --release

# 生成可执行文件
# target/release/financial-analyzer
```

### 10.2 环境配置

```bash
# 设置Tushare Token
export TUSHARE_TOKEN=your_token_here

# 设置日志级别
export RUST_LOG=info
```

### 10.3 使用示例

```bash
# 基础使用（Mock数据）
./financial-analyzer analyze --stock 600519.SH --years 2019,2018,2017

# 使用Tushare数据
./financial-analyzer analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source tushare

# 启用数据验证
./financial-analyzer analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --enable-validation

# 指定输出文件
./financial-analyzer analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --output 茅台分析.xlsx
```

## 11. 维护和扩展

### 11.1 添加新的财务指标

1. 在 `Calculator` 中添加计算方法
2. 在 `AnalysisResult` 中添加字段
3. 在 Excel生成器中添加显示逻辑

### 11.2 添加新的估值模型

1. 在 `valuation.rs` 中实现新模型
2. 在 `ValuationResult` 中添加字段
3. 在 Excel中添加新的估值区域

### 11.3 添加新的数据源

1. 实现 `DataSource` trait
2. 在 CLI 中添加新的数据源选项
3. 更新配置文件

## 12. 已知限制

1. **数据源依赖**：依赖外部API的可用性和准确性
2. **科目映射**：不同公司可能使用不同的科目名称
3. **估值假设**：估值模型基于固定参数，实际应根据行业调整
4. **单线程Excel生成**：Excel生成是单线程的
5. **内存限制**：大量年份数据可能占用较多内存

## 13. 最佳实践

1. **数据验证**：生产环境建议启用 `--enable-validation`
2. **错误处理**：检查命令返回值和日志输出
3. **API限流**：使用Tushare时注意API调用频率限制
4. **数据缓存**：频繁分析同一股票时考虑缓存数据
5. **配置管理**：根据实际需求调整配置文件
