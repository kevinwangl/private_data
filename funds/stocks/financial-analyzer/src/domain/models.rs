use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 报告类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReportType {
    BalanceSheet,
    IncomeStatement,
    CashflowStatement,
}

impl ReportType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BalanceSheet => "balance_sheet",
            Self::IncomeStatement => "income_statement",
            Self::CashflowStatement => "cashflow_statement",
        }
    }
}

/// 财务报表基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialStatement {
    pub stock_code: String,
    pub report_date: NaiveDate,
    pub report_type: ReportType,
    pub items: HashMap<String, Decimal>,
}

/// 资产组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetGroup {
    pub items: HashMap<String, Decimal>,
    pub total: Decimal,
}

impl AssetGroup {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            total: Decimal::ZERO,
        }
    }

    pub fn add(&mut self, name: String, amount: Decimal) {
        self.total += amount;
        self.items.insert(name, amount);
    }
}

/// 负债组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiabilityGroup {
    pub items: HashMap<String, Decimal>,
    pub total: Decimal,
}

impl LiabilityGroup {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            total: Decimal::ZERO,
        }
    }

    pub fn add(&mut self, name: String, amount: Decimal) {
        self.total += amount;
        self.items.insert(name, amount);
    }
}

/// 资产负债表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSheet {
    pub statement: FinancialStatement,
    pub operating_assets: AssetGroup,
    pub financial_assets: AssetGroup,
    pub operating_liabilities: LiabilityGroup,
    pub financial_liabilities: LiabilityGroup,
}

/// 利润表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeStatement {
    pub statement: FinancialStatement,
    pub revenue: Decimal,
    pub operating_cost: Decimal,
    pub gross_profit: Decimal,
    pub core_profit: Decimal,
    pub net_profit: Decimal,
}

/// 现金流量表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashflowStatement {
    pub statement: FinancialStatement,
    pub operating_cashflow: Decimal,
    pub investing_cashflow: Decimal,
    pub financing_cashflow: Decimal,
    pub free_cashflow: Decimal,
}

/// 分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub stock_code: String,
    pub years: Vec<i32>,
    pub asset_structure: AssetStructureAnalysis,
    pub profit_analysis: ProfitAnalysis,
    pub leverage_analysis: Option<LeverageAnalysis>,
    pub valuation: Option<crate::analyzer::ValuationResult>,
    pub statements: Vec<FinancialStatement>,  // 添加原始报表数据
    pub sensitivity: Option<crate::analyzer::SensitivityResult>,  // 敏感性分析结果
}

/// 资产结构分析
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetStructureAnalysis {
    pub years: Vec<i32>,
    pub operating_asset_ratio: Vec<Decimal>,
    pub financial_asset_ratio: Vec<Decimal>,
}

/// 利润分析
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitAnalysis {
    pub years: Vec<i32>,
    pub gross_margin: Vec<Decimal>,
    pub core_profit_margin: Vec<Decimal>,
    pub net_profit_margin: Vec<Decimal>,
}

/// 杠杆分析
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeverageAnalysis {
    pub years: Vec<i32>,
    pub operating_leverage: Vec<Decimal>,  // 经营杠杆 DOL
    pub financial_leverage: Vec<Decimal>,  // 财务杠杆 DFL
    pub total_leverage: Vec<Decimal>,      // 总杠杆 DTL
}
