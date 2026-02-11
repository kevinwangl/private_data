use crate::data_source::DataSource;
use crate::domain::*;
use crate::validation::{DataValidator, ValidationResult};
use anyhow::Result;
use chrono::Datelike;
use rust_decimal::Decimal;

mod calculator;
mod valuation;
mod sensitivity;
#[cfg(test)]
mod tests;

use calculator::RatioCalculator;
pub use valuation::{Valuator, ValuationResult, ValuationParams};
pub use sensitivity::{SensitivityParams, SensitivityResult};

/// 财务分析器
/// 
/// 负责执行完整的财务分析流程，包括：
/// - 资产结构分析
/// - 利润分析
/// - 杠杆分析
/// - 估值分析
/// - 敏感性分析
/// 
/// # Examples
/// 
/// ```no_run
/// use financial_analyzer::analyzer::FinancialAnalyzer;
/// use financial_analyzer::domain::FinancialStatement;
/// 
/// let statements = vec![/* ... */];
/// let analyzer = FinancialAnalyzer::new(statements);
/// let result = analyzer.analyze()?;
/// ```
pub struct FinancialAnalyzer {
    calculator: RatioCalculator,
    validator: Option<DataValidator>,
    valuator: Valuator,
}

impl FinancialAnalyzer {
    /// 创建新的财务分析器实例
    /// 
    /// 使用默认的估值参数
    pub fn new() -> Self {
        Self {
            calculator: RatioCalculator::new(),
            validator: None,
            valuator: Valuator::with_default(),
        }
    }

    /// 设置数据验证器
    /// 
    /// # Arguments
    /// 
    /// * `validator` - 数据验证器实例
    pub fn with_validator(mut self, validator: DataValidator) -> Self {
        self.validator = Some(validator);
        self
    }

    /// 设置估值参数
    /// 
    /// # Arguments
    /// 
    /// * `params` - 估值参数（DCF和唐朝估值法）
    pub fn with_valuation_params(mut self, params: ValuationParams) -> Self {
        self.valuator = Valuator::new(params);
        self
    }

    /// 执行财务分析
    /// 
    /// # Arguments
    /// 
    /// * `stock_code` - 股票代码（如 600519.SH）
    /// * `years` - 分析年份列表
    /// * `data_source` - 数据源实现
    /// 
    /// # Returns
    /// 
    /// 返回完整的分析结果，包括：
    /// - 财务报表数据
    /// - 资产结构分析
    /// - 利润分析
    /// - 杠杆分析
    /// - 估值结果
    pub async fn analyze(
        &self,
        stock_code: &str,
        years: Vec<i32>,
        data_source: &dyn DataSource,
    ) -> Result<AnalysisResult> {
        // 如果years为空，使用默认的最近3年
        let years = if years.is_empty() {
            let current_year = chrono::Local::now().year();
            vec![current_year - 1, current_year - 2, current_year - 3]
        } else {
            years
        };
        
        // 获取数据
        let start_date = chrono::NaiveDate::from_ymd_opt(years[years.len() - 1], 12, 31).unwrap();
        let end_date = chrono::NaiveDate::from_ymd_opt(years[0], 12, 31).unwrap();

        let balance_sheets = data_source
            .fetch_balance_sheet(stock_code, start_date, end_date)
            .await?;

        let income_statements = data_source
            .fetch_income_statement(stock_code, start_date, end_date)
            .await?;

        let cashflow_statements = data_source
            .fetch_cashflow_statement(stock_code, start_date, end_date)
            .await?;

        // 数据验证（如果启用）
        if let Some(validator) = &self.validator {
            for bs in &balance_sheets {
                let validation = validator.validate_balance_sheet(bs);
                if !validation.is_valid {
                    tracing::warn!(
                        "资产负债表验证失败 ({}): {} 个错误",
                        bs.statement.report_date,
                        validation.errors.len()
                    );
                    for error in &validation.errors {
                        tracing::warn!("  - {}: {}", error.field, error.message);
                    }
                }
            }

            for is in &income_statements {
                let validation = validator.validate_income_statement(is);
                if !validation.is_valid {
                    tracing::warn!(
                        "利润表验证失败 ({}): {} 个错误",
                        is.statement.report_date,
                        validation.errors.len()
                    );
                }
            }
        }

        // 计算分析指标
        let asset_structure = self.calculator.calculate_asset_structure(&balance_sheets)?;
        let profit_analysis = self.calculator.calculate_profit_ratios(&income_statements)?;
        let leverage_analysis = self.calculator.calculate_leverage(&income_statements).ok();

        // 自动获取总股本
        let total_shares = balance_sheets.first()
            .and_then(|bs| bs.statement.items.get("股本"))
            .copied()
            .unwrap_or_else(|| {
                tracing::warn!("未找到股本数据，使用默认值1亿股");
                Decimal::new(100_000_000, 0)
            });

        // 更新估值器的总股本
        let mut valuator = Valuator::new(self.valuator.params.clone());
        valuator.params.total_shares = total_shares;

        // 计算估值
        let valuation = valuator.calculate(&income_statements, &cashflow_statements)?;

        // 合并所有报表
        let mut statements = Vec::new();
        statements.extend(balance_sheets.into_iter().map(|bs| bs.statement));
        statements.extend(income_statements.into_iter().map(|is| is.statement));
        statements.extend(cashflow_statements.into_iter().map(|cs| cs.statement));

        Ok(AnalysisResult {
            stock_code: stock_code.to_string(),
            years,
            asset_structure,
            profit_analysis,
            leverage_analysis,
            valuation: Some(valuation),
            statements,
            sensitivity: None,  // 默认不计算敏感性分析
        })
    }

    /// 计算敏感性分析
    pub fn calculate_sensitivity(
        &self,
        result: &mut AnalysisResult,
        params: SensitivityParams,
    ) -> Result<()> {
        // 从原始报表中提取数据
        let income_statements: Vec<_> = result.statements.iter()
            .filter(|s| s.report_type == crate::domain::ReportType::IncomeStatement)
            .cloned()
            .collect();
        
        let cashflow_statements: Vec<_> = result.statements.iter()
            .filter(|s| s.report_type == crate::domain::ReportType::CashflowStatement)
            .cloned()
            .collect();

        // 获取总股本（从资产负债表中读取）
        let total_shares = result.statements.iter()
            .find(|s| s.report_type == crate::domain::ReportType::BalanceSheet)
            .and_then(|s| s.items.get("股本"))
            .copied()
            .unwrap_or_else(|| {
                tracing::warn!("敏感性分析：未找到股本数据，使用默认值1亿股");
                Decimal::new(100_000_000, 0)
            });

        // 使用新参数创建临时估值器
        let temp_valuator = Valuator::new(params.to_valuation_params(total_shares));
        
        // 从FinancialStatement构造IncomeStatement和CashflowStatement
        let income_stmts: Vec<IncomeStatement> = income_statements.iter()
            .map(|s| {
                let revenue = s.items.get("营业收入").copied().unwrap_or(Decimal::ZERO);
                let operating_cost = s.items.get("营业成本").copied().unwrap_or(Decimal::ZERO);
                let gross_profit = revenue - operating_cost;
                let core_profit = s.items.get("营业利润").copied().unwrap_or(Decimal::ZERO);
                let net_profit = s.items.get("净利润").copied().unwrap_or(Decimal::ZERO);
                
                IncomeStatement {
                    statement: s.clone(),
                    revenue,
                    operating_cost,
                    gross_profit,
                    core_profit,
                    net_profit,
                }
            })
            .collect();
        
        let cashflow_stmts: Vec<CashflowStatement> = cashflow_statements.iter()
            .map(|s| {
                let operating_cashflow = s.items.get("经营活动产生的现金流量净额").copied().unwrap_or(Decimal::ZERO);
                let investing_cashflow = s.items.get("投资活动产生的现金流量净额").copied().unwrap_or(Decimal::ZERO);
                let financing_cashflow = s.items.get("筹资活动产生的现金流量净额").copied().unwrap_or(Decimal::ZERO);
                let capex = s.items.get("购建固定资产、无形资产和其他长期资产支付的现金").copied().unwrap_or(Decimal::ZERO);
                let free_cashflow = operating_cashflow - capex;
                
                CashflowStatement {
                    statement: s.clone(),
                    operating_cashflow,
                    investing_cashflow,
                    financing_cashflow,
                    free_cashflow,
                }
            })
            .collect();

        // 计算估值
        let valuation = temp_valuator.calculate(&income_stmts, &cashflow_stmts)?;

        // 保存敏感性分析结果
        result.sensitivity = Some(SensitivityResult {
            params,
            dcf_enterprise_value: valuation.dcf.enterprise_value,
            dcf_price_per_share: valuation.dcf.price_per_share,
            tangchao_low_estimate: valuation.tangchao.low_estimate,
            tangchao_high_estimate: valuation.tangchao.high_estimate,
            tangchao_safety_margin_price: valuation.tangchao.safety_margin_price,
        });

        Ok(())
    }
}

