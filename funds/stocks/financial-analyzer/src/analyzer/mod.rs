use crate::data_source::DataSource;
use crate::domain::*;
use crate::validation::{DataValidator, ValidationResult};
use anyhow::Result;
use chrono::Datelike;

mod calculator;
mod valuation;

use calculator::RatioCalculator;
pub use valuation::{Valuator, ValuationResult, ValuationParams};

pub struct FinancialAnalyzer {
    calculator: RatioCalculator,
    validator: Option<DataValidator>,
    valuator: Valuator,
}

impl FinancialAnalyzer {
    pub fn new() -> Self {
        Self {
            calculator: RatioCalculator::new(),
            validator: None,
            valuator: Valuator::with_default(),
        }
    }

    pub fn with_validator(mut self, validator: DataValidator) -> Self {
        self.validator = Some(validator);
        self
    }

    pub fn with_valuation_params(mut self, params: ValuationParams) -> Self {
        self.valuator = Valuator::new(params);
        self
    }

    pub async fn analyze(
        &self,
        stock_code: &str,
        years: Vec<i32>,
        data_source: &dyn DataSource,
    ) -> Result<AnalysisResult> {
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

        // 计算估值
        let valuation = self.valuator.calculate(&income_statements, &cashflow_statements)?;

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
            valuation: Some(valuation),
            statements,
        })
    }
}

