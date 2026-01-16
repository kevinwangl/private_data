use super::traits::DataSource;
use crate::domain::*;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{NaiveDate, Datelike};
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Mock数据源（用于测试）
pub struct MockDataSource;

impl MockDataSource {
    pub fn new() -> Self {
        Self
    }

    fn create_mock_balance_sheet(&self, stock_code: &str, date: NaiveDate) -> BalanceSheet {
        let mut items = HashMap::new();
        items.insert("货币资金".to_string(), Decimal::new(1000000, 0));
        items.insert("应收账款".to_string(), Decimal::new(500000, 0));
        items.insert("存货".to_string(), Decimal::new(300000, 0));
        items.insert("固定资产".to_string(), Decimal::new(2000000, 0));
        items.insert("资产总计".to_string(), Decimal::new(4000000, 0));
        items.insert("应付账款".to_string(), Decimal::new(400000, 0));
        items.insert("短期借款".to_string(), Decimal::new(600000, 0));
        items.insert("负债合计".to_string(), Decimal::new(1500000, 0));
        items.insert("所有者权益合计".to_string(), Decimal::new(2500000, 0));

        let statement = FinancialStatement {
            stock_code: stock_code.to_string(),
            report_date: date,
            report_type: ReportType::BalanceSheet,
            items,
        };

        let mut operating_assets = AssetGroup::new();
        operating_assets.add("货币资金".to_string(), Decimal::new(1000000, 0));
        operating_assets.add("应收账款".to_string(), Decimal::new(500000, 0));
        operating_assets.add("存货".to_string(), Decimal::new(300000, 0));
        operating_assets.add("固定资产".to_string(), Decimal::new(2000000, 0));

        let financial_assets = AssetGroup::new();

        let mut operating_liabilities = LiabilityGroup::new();
        operating_liabilities.add("应付账款".to_string(), Decimal::new(400000, 0));

        let mut financial_liabilities = LiabilityGroup::new();
        financial_liabilities.add("短期借款".to_string(), Decimal::new(600000, 0));

        BalanceSheet {
            statement,
            operating_assets,
            financial_assets,
            operating_liabilities,
            financial_liabilities,
        }
    }

    fn create_mock_income_statement(&self, stock_code: &str, date: NaiveDate) -> IncomeStatement {
        let mut items = HashMap::new();
        items.insert("营业收入".to_string(), Decimal::new(5000000, 0));
        items.insert("营业成本".to_string(), Decimal::new(3000000, 0));
        items.insert("税金及附加".to_string(), Decimal::new(50000, 0));
        items.insert("销售费用".to_string(), Decimal::new(300000, 0));
        items.insert("管理费用".to_string(), Decimal::new(200000, 0));
        items.insert("研发费用".to_string(), Decimal::new(150000, 0));
        items.insert("财务费用".to_string(), Decimal::new(50000, 0));
        items.insert("营业利润".to_string(), Decimal::new(1250000, 0));
        items.insert("净利润".to_string(), Decimal::new(1000000, 0));

        let statement = FinancialStatement {
            stock_code: stock_code.to_string(),
            report_date: date,
            report_type: ReportType::IncomeStatement,
            items,
        };

        IncomeStatement {
            statement,
            revenue: Decimal::new(5000000, 0),
            operating_cost: Decimal::new(3000000, 0),
            gross_profit: Decimal::new(2000000, 0),
            core_profit: Decimal::new(1250000, 0),
            net_profit: Decimal::new(1000000, 0),
        }
    }

    fn create_mock_cashflow_statement(&self, stock_code: &str, date: NaiveDate) -> CashflowStatement {
        let mut items = HashMap::new();
        items.insert("经营活动产生的现金流量净额".to_string(), Decimal::new(900000, 0));

        let statement = FinancialStatement {
            stock_code: stock_code.to_string(),
            report_date: date,
            report_type: ReportType::CashflowStatement,
            items,
        };

        CashflowStatement {
            statement,
            operating_cashflow: Decimal::new(900000, 0),
            investing_cashflow: Decimal::new(-200000, 0),
            financing_cashflow: Decimal::new(-100000, 0),
            free_cashflow: Decimal::new(700000, 0),
        }
    }
}

#[async_trait]
impl DataSource for MockDataSource {
    async fn fetch_balance_sheet(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<BalanceSheet>> {
        let mut sheets = Vec::new();
        let mut current = start_date;

        while current <= end_date {
            sheets.push(self.create_mock_balance_sheet(stock_code, current));
            current = NaiveDate::from_ymd_opt(current.year() + 1, 12, 31).unwrap();
        }

        Ok(sheets)
    }

    async fn fetch_income_statement(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<IncomeStatement>> {
        let mut statements = Vec::new();
        let mut current = start_date;

        while current <= end_date {
            statements.push(self.create_mock_income_statement(stock_code, current));
            current = NaiveDate::from_ymd_opt(current.year() + 1, 12, 31).unwrap();
        }

        Ok(statements)
    }

    async fn fetch_cashflow_statement(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<CashflowStatement>> {
        let mut statements = Vec::new();
        let mut current = start_date;

        while current <= end_date {
            statements.push(self.create_mock_cashflow_statement(stock_code, current));
            current = NaiveDate::from_ymd_opt(current.year() + 1, 12, 31).unwrap();
        }

        Ok(statements)
    }

    fn name(&self) -> &str {
        "mock"
    }
}
