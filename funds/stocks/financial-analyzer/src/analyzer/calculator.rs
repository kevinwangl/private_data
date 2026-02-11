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

    pub fn calculate_leverage(
        &self,
        income_statements: &[IncomeStatement],
    ) -> Result<LeverageAnalysis> {
        let years: Vec<i32> = income_statements
            .iter()
            .map(|is| is.statement.report_date.year())
            .collect();

        let mut operating_leverage = Vec::new();
        let mut financial_leverage = Vec::new();
        let mut total_leverage = Vec::new();

        for i in 0..income_statements.len() {
            let current = &income_statements[i];
            
            // 计算EBIT（息税前利润）= 营业利润
            let ebit = current.core_profit;
            
            // 获取利息费用
            let interest_expense = current.statement.items
                .get("财务费用")
                .or_else(|| current.statement.items.get("利息费用"))
                .copied()
                .unwrap_or(Decimal::ZERO);
            
            // 计算EBT（税前利润）= EBIT - 利息费用
            let ebt = ebit - interest_expense;
            
            // 如果有上一年数据，计算杠杆
            if i + 1 < income_statements.len() {
                let prev = &income_statements[i + 1];
                let prev_ebit = prev.core_profit;
                let prev_interest = prev.statement.items
                    .get("财务费用")
                    .or_else(|| prev.statement.items.get("利息费用"))
                    .copied()
                    .unwrap_or(Decimal::ZERO);
                let prev_ebt = prev_ebit - prev_interest;
                
                // 经营杠杆 DOL = (EBIT变化率) / (销售收入变化率)
                let revenue_change = if prev.revenue != Decimal::ZERO {
                    (current.revenue - prev.revenue) / prev.revenue
                } else {
                    Decimal::ZERO
                };
                
                let ebit_change = if prev_ebit != Decimal::ZERO {
                    (ebit - prev_ebit) / prev_ebit
                } else {
                    Decimal::ZERO
                };
                
                let dol = if revenue_change != Decimal::ZERO && revenue_change.abs() > Decimal::new(1, 4) {
                    ebit_change / revenue_change
                } else {
                    Decimal::ZERO
                };
                
                // 财务杠杆 DFL = (EPS变化率) / (EBIT变化率) ≈ EBIT / EBT
                let dfl = if ebt != Decimal::ZERO && ebit != Decimal::ZERO {
                    ebit / ebt
                } else {
                    Decimal::ONE
                };
                
                // 总杠杆 DTL = DOL × DFL
                let dtl = dol * dfl;
                
                operating_leverage.push(dol);
                financial_leverage.push(dfl);
                total_leverage.push(dtl);
            } else {
                // 第一年（最早年份）没有对比数据
                operating_leverage.push(Decimal::ZERO);
                financial_leverage.push(Decimal::ONE);
                total_leverage.push(Decimal::ZERO);
            }
        }

        Ok(LeverageAnalysis {
            years,
            operating_leverage,
            financial_leverage,
            total_leverage,
        })
    }
}
