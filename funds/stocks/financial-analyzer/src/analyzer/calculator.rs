use crate::domain::*;
use anyhow::Result;
use rust_decimal::Decimal;
use chrono::Datelike;

pub struct RatioCalculator;

impl RatioCalculator {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_asset_structure(
        &self,
        balance_sheets: &[BalanceSheet],
    ) -> Result<AssetStructureAnalysis> {
        let years: Vec<i32> = balance_sheets
            .iter()
            .map(|bs| bs.statement.report_date.year())
            .collect();

        let mut operating_asset_ratio = Vec::new();
        let mut financial_asset_ratio = Vec::new();

        for bs in balance_sheets {
            let total_assets = bs.operating_assets.total + bs.financial_assets.total;

            if total_assets != Decimal::ZERO {
                operating_asset_ratio.push(bs.operating_assets.total / total_assets);
                financial_asset_ratio.push(bs.financial_assets.total / total_assets);
            } else {
                operating_asset_ratio.push(Decimal::ZERO);
                financial_asset_ratio.push(Decimal::ZERO);
            }
        }

        Ok(AssetStructureAnalysis {
            years,
            operating_asset_ratio,
            financial_asset_ratio,
        })
    }

    pub fn calculate_profit_ratios(
        &self,
        income_statements: &[IncomeStatement],
    ) -> Result<ProfitAnalysis> {
        let years: Vec<i32> = income_statements
            .iter()
            .map(|is| is.statement.report_date.year())
            .collect();

        let mut gross_margin = Vec::new();
        let mut core_profit_margin = Vec::new();
        let mut net_profit_margin = Vec::new();

        for is in income_statements {
            if is.revenue != Decimal::ZERO {
                gross_margin.push(is.gross_profit / is.revenue);
                core_profit_margin.push(is.core_profit / is.revenue);
                net_profit_margin.push(is.net_profit / is.revenue);
            } else {
                gross_margin.push(Decimal::ZERO);
                core_profit_margin.push(Decimal::ZERO);
                net_profit_margin.push(Decimal::ZERO);
            }
        }

        Ok(ProfitAnalysis {
            years,
            gross_margin,
            core_profit_margin,
            net_profit_margin,
        })
    }
}
