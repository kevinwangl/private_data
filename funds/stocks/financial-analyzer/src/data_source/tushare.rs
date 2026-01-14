use super::traits::DataSource;
use crate::domain::*;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Tushare API响应
#[derive(Debug, Deserialize)]
struct TushareResponse {
    code: i32,
    msg: Option<String>,
    data: Option<TushareData>,
}

#[derive(Debug, Deserialize)]
struct TushareData {
    fields: Vec<String>,
    items: Vec<Vec<serde_json::Value>>,
}

/// Tushare客户端
pub struct TushareClient {
    api_url: String,
    token: String,
    client: reqwest::Client,
}

impl TushareClient {
    pub fn new(token: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            api_url: "http://api.tushare.pro".to_string(),
            token,
            client,
        })
    }

    pub fn from_env() -> Result<Self> {
        let token = std::env::var("TUSHARE_TOKEN")
            .map_err(|_| anyhow!("TUSHARE_TOKEN环境变量未设置"))?;
        Self::new(token)
    }

    /// 调用Tushare API
    async fn call_api(&self, api_name: &str, params: serde_json::Value) -> Result<TushareData> {
        let request_body = serde_json::json!({
            "api_name": api_name,
            "token": self.token,
            "params": params,
            "fields": ""
        });

        let response = self
            .client
            .post(&self.api_url)
            .json(&request_body)
            .send()
            .await?;

        let result: TushareResponse = response.json().await?;

        if result.code != 0 {
            return Err(anyhow!(
                "Tushare API错误: {}",
                result.msg.unwrap_or_else(|| "未知错误".to_string())
            ));
        }

        result.data.ok_or_else(|| anyhow!("API返回数据为空"))
    }

    /// 解析资产负债表数据
    fn parse_balance_sheet(&self, data: TushareData, stock_code: &str) -> Result<Vec<BalanceSheet>> {
        let mut sheets = Vec::new();

        for item in data.items {
            let mut items_map = HashMap::new();
            
            // 解析字段
            for (i, field) in data.fields.iter().enumerate() {
                if let Some(value) = item.get(i) {
                    if let Some(num) = value.as_f64() {
                        items_map.insert(field.clone(), Decimal::from_f64_retain(num).unwrap_or(Decimal::ZERO));
                    }
                }
            }

            // 获取报告日期
            let end_date_str = item.get(data.fields.iter().position(|f| f == "end_date").unwrap_or(0))
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("缺少报告日期"))?;

            let report_date = NaiveDate::parse_from_str(end_date_str, "%Y%m%d")?;

            let statement = FinancialStatement {
                stock_code: stock_code.to_string(),
                report_date,
                report_type: ReportType::BalanceSheet,
                items: items_map,
            };

            // 简化分类（实际应使用配置文件）
            let mut operating_assets = AssetGroup::new();
            let mut financial_assets = AssetGroup::new();
            let mut operating_liabilities = LiabilityGroup::new();
            let mut financial_liabilities = LiabilityGroup::new();

            sheets.push(BalanceSheet {
                statement,
                operating_assets,
                financial_assets,
                operating_liabilities,
                financial_liabilities,
            });
        }

        Ok(sheets)
    }

    /// 解析利润表数据
    fn parse_income_statement(&self, data: TushareData, stock_code: &str) -> Result<Vec<IncomeStatement>> {
        let mut statements = Vec::new();

        for item in data.items {
            let mut items_map = HashMap::new();
            
            for (i, field) in data.fields.iter().enumerate() {
                if let Some(value) = item.get(i) {
                    if let Some(num) = value.as_f64() {
                        items_map.insert(field.clone(), Decimal::from_f64_retain(num).unwrap_or(Decimal::ZERO));
                    }
                }
            }

            let end_date_str = item.get(data.fields.iter().position(|f| f == "end_date").unwrap_or(0))
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("缺少报告日期"))?;

            let report_date = NaiveDate::parse_from_str(end_date_str, "%Y%m%d")?;

            let revenue = items_map.get("revenue").copied().unwrap_or(Decimal::ZERO);
            let operating_cost = items_map.get("oper_cost").copied().unwrap_or(Decimal::ZERO);
            let net_profit = items_map.get("n_income").copied().unwrap_or(Decimal::ZERO);

            let statement = FinancialStatement {
                stock_code: stock_code.to_string(),
                report_date,
                report_type: ReportType::IncomeStatement,
                items: items_map,
            };

            statements.push(IncomeStatement {
                statement,
                revenue,
                operating_cost,
                gross_profit: revenue - operating_cost,
                core_profit: net_profit,
                net_profit,
            });
        }

        Ok(statements)
    }
}

#[async_trait]
impl DataSource for TushareClient {
    async fn fetch_balance_sheet(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<BalanceSheet>> {
        let params = serde_json::json!({
            "ts_code": stock_code,
            "start_date": start_date.format("%Y%m%d").to_string(),
            "end_date": end_date.format("%Y%m%d").to_string(),
        });

        let data = self.call_api("balancesheet", params).await?;
        self.parse_balance_sheet(data, stock_code)
    }

    async fn fetch_income_statement(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<IncomeStatement>> {
        let params = serde_json::json!({
            "ts_code": stock_code,
            "start_date": start_date.format("%Y%m%d").to_string(),
            "end_date": end_date.format("%Y%m%d").to_string(),
        });

        let data = self.call_api("income", params).await?;
        self.parse_income_statement(data, stock_code)
    }

    async fn fetch_cashflow_statement(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<CashflowStatement>> {
        let params = serde_json::json!({
            "ts_code": stock_code,
            "start_date": start_date.format("%Y%m%d").to_string(),
            "end_date": end_date.format("%Y%m%d").to_string(),
        });

        let data = self.call_api("cashflow", params).await?;
        
        // 简化实现
        let mut statements = Vec::new();
        for item in data.items {
            let end_date_str = item.get(data.fields.iter().position(|f| f == "end_date").unwrap_or(0))
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("缺少报告日期"))?;

            let report_date = NaiveDate::parse_from_str(end_date_str, "%Y%m%d")?;

            let statement = FinancialStatement {
                stock_code: stock_code.to_string(),
                report_date,
                report_type: ReportType::CashflowStatement,
                items: HashMap::new(),
            };

            statements.push(CashflowStatement {
                statement,
                operating_cashflow: Decimal::ZERO,
                investing_cashflow: Decimal::ZERO,
                financing_cashflow: Decimal::ZERO,
                free_cashflow: Decimal::ZERO,
            });
        }

        Ok(statements)
    }

    fn name(&self) -> &str {
        "tushare"
    }
}
