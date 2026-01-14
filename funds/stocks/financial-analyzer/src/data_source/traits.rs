use crate::domain::*;
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;

/// 数据源统一接口
#[async_trait]
pub trait DataSource: Send + Sync {
    /// 获取资产负债表
    async fn fetch_balance_sheet(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<BalanceSheet>>;

    /// 获取利润表
    async fn fetch_income_statement(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<IncomeStatement>>;

    /// 获取现金流量表
    async fn fetch_cashflow_statement(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<CashflowStatement>>;

    /// 数据源名称
    fn name(&self) -> &str;
}
