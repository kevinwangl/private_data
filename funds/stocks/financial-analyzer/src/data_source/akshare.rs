use super::traits::DataSource;
use crate::domain::*;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{NaiveDate, Datelike};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

/// AKShare数据源（通过Python脚本调用）
pub struct AkshareClient {
    python_path: String,
}

#[derive(Debug, Deserialize)]
struct AkshareBalanceSheet {
    #[serde(rename = "REPORT_DATE")]
    report_date: String,
    #[serde(rename = "TOTAL_ASSETS")]
    total_assets: Option<f64>,
    #[serde(rename = "TOTAL_LIABILITIES")]
    total_liabilities: Option<f64>,
    #[serde(rename = "TOTAL_EQUITY")]
    total_equity: Option<f64>,
    #[serde(rename = "MONETARYFUNDS")]
    monetary_funds: Option<f64>,
    #[serde(rename = "FIXED_ASSETS")]
    fixed_assets: Option<f64>,
    #[serde(rename = "ACCOUNTS_RECE")]
    accounts_receivable: Option<f64>,
    #[serde(rename = "INVENTORY")]
    inventory: Option<f64>,
    #[serde(rename = "SHARE_CAPITAL")]
    share_capital: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct AkshareIncomeStatement {
    #[serde(rename = "REPORT_DATE")]
    report_date: String,
    #[serde(rename = "TOTAL_OPERATE_INCOME")]
    revenue: Option<f64>,
    #[serde(rename = "OPERATE_COST")]
    operating_cost: Option<f64>,
    #[serde(rename = "NETPROFIT")]
    net_profit: Option<f64>,
    #[serde(rename = "TAX")]
    tax: Option<f64>,
    #[serde(rename = "FINANCE_EXPENSE")]
    finance_expense: Option<f64>,
    #[serde(rename = "SALES_EXPENSE")]
    sales_expense: Option<f64>,
    #[serde(rename = "ADMIN_EXPENSE")]
    admin_expense: Option<f64>,
    #[serde(rename = "RD_EXPENSE")]
    rd_expense: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct AkshareCashflow {
    #[serde(rename = "REPORT_DATE")]
    report_date: String,
    #[serde(rename = "OPERATE_CASH_FLOW")]
    operating_cashflow: Option<f64>,
    #[serde(rename = "INVEST_CASH_FLOW")]
    investing_cashflow: Option<f64>,
    #[serde(rename = "FINANCE_CASH_FLOW")]
    financing_cashflow: Option<f64>,
}

impl AkshareClient {
    pub fn new() -> Self {
        Self {
            python_path: "python3".to_string(),
        }
    }

    /// 调用Python脚本获取数据
    fn call_python_script(&self, script: &str) -> Result<String> {
        let output = Command::new(&self.python_path)
            .arg("-c")
            .arg(script)
            .output()
            .map_err(|e| anyhow!("执行Python失败: {}. 请确保已安装Python3和akshare库 (pip3 install akshare)", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            
            // 检查是否是lxml解析器问题
            if error.contains("lxml") || error.contains("FeatureNotFound") {
                return Err(anyhow!(
                    "AKShare依赖错误: 请安装完整依赖\n\
                    解决方案:\n\
                    1. pip3 install beautifulsoup4 lxml html5lib\n\
                    2. 或使用: pip3 install --upgrade akshare beautifulsoup4 lxml\n\
                    \n原始错误: {}", error
                ));
            }
            
            return Err(anyhow!("Python脚本执行错误: {}", error));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// 获取资产负债表
    fn fetch_balance_sheet_data(&self, stock_code: &str) -> Result<Vec<AkshareBalanceSheet>> {
        // 转换股票代码格式: 600519 -> sh600519
        let sina_code = if stock_code.starts_with('6') {
            format!("sh{}", stock_code)
        } else {
            format!("sz{}", stock_code)
        };
        
        let script = format!(
            r#"
import akshare as ak
import json
import math
df = ak.stock_financial_report_sina(stock='{}', symbol='资产负债表')
result = []
for _, row in df.iterrows():
    def safe_float(val):
        try:
            f = float(val or 0)
            return 0.0 if (math.isnan(f) or math.isinf(f)) else f
        except:
            return 0.0
    
    result.append({{
        'REPORT_DATE': str(row['报告日']),
        'TOTAL_ASSETS': safe_float(row.get('资产总计')),
        'TOTAL_LIABILITIES': safe_float(row.get('负债合计')),
        'TOTAL_EQUITY': safe_float(row.get('所有者权益(或股东权益)合计')),
        'MONETARYFUNDS': safe_float(row.get('货币资金')),
        'FIXED_ASSETS': safe_float(row.get('固定资产')),
        'ACCOUNTS_RECE': safe_float(row.get('应收账款')),
        'INVENTORY': safe_float(row.get('存货')),
        'SHARE_CAPITAL': safe_float(row.get('实收资本(或股本)')),
    }})
print(json.dumps(result))
"#,
            sina_code
        );

        let json_str = self.call_python_script(&script)?;
        serde_json::from_str(&json_str)
            .map_err(|e| anyhow!("解析资产负债表JSON失败: {}", e))
    }

    /// 获取利润表
    fn fetch_income_statement_data(&self, stock_code: &str) -> Result<Vec<AkshareIncomeStatement>> {
        let sina_code = if stock_code.starts_with('6') {
            format!("sh{}", stock_code)
        } else {
            format!("sz{}", stock_code)
        };
        
        let script = format!(
            r#"
import akshare as ak
import json
import math
df = ak.stock_financial_report_sina(stock='{}', symbol='利润表')
result = []
def safe_float(val):
    try:
        f = float(val or 0)
        return 0.0 if (math.isnan(f) or math.isinf(f)) else f
    except:
        return 0.0

for _, row in df.iterrows():
    result.append({{
        'REPORT_DATE': str(row['报告日']),
        'TOTAL_OPERATE_INCOME': safe_float(row.get('营业总收入')),
        'OPERATE_COST': safe_float(row.get('营业总成本')),
        'NETPROFIT': safe_float(row.get('净利润')),
        'TAX': safe_float(row.get('营业税金及附加')),
        'FINANCE_EXPENSE': safe_float(row.get('财务费用')),
        'SALES_EXPENSE': safe_float(row.get('销售费用')),
        'ADMIN_EXPENSE': safe_float(row.get('管理费用')),
        'RD_EXPENSE': safe_float(row.get('研发费用')),
    }})
print(json.dumps(result))
"#,
            sina_code
        );

        let json_str = self.call_python_script(&script)?;
        serde_json::from_str(&json_str)
            .map_err(|e| anyhow!("解析利润表JSON失败: {}", e))
    }

    /// 获取现金流量表
    fn fetch_cashflow_statement_data(&self, stock_code: &str) -> Result<Vec<AkshareCashflow>> {
        let sina_code = if stock_code.starts_with('6') {
            format!("sh{}", stock_code)
        } else {
            format!("sz{}", stock_code)
        };
        
        let script = format!(
            r#"
import akshare as ak
import json
import math
df = ak.stock_financial_report_sina(stock='{}', symbol='现金流量表')
result = []
def safe_float(val):
    try:
        f = float(val or 0)
        return 0.0 if (math.isnan(f) or math.isinf(f)) else f
    except:
        return 0.0

for _, row in df.iterrows():
    result.append({{
        'REPORT_DATE': str(row['报告日']),
        'OPERATE_CASH_FLOW': safe_float(row.get('经营活动产生的现金流量净额')),
        'INVEST_CASH_FLOW': safe_float(row.get('投资活动产生的现金流量净额')),
        'FINANCE_CASH_FLOW': safe_float(row.get('筹资活动产生的现金流量净额')),
    }})
print(json.dumps(result))
"#,
            sina_code
        );

        let json_str = self.call_python_script(&script)?;
        serde_json::from_str(&json_str)
            .map_err(|e| anyhow!("解析现金流量表JSON失败: {}", e))
    }

    /// 转换股票代码格式 (600519.SH -> 600519)
    fn convert_stock_code(&self, code: &str) -> String {
        code.split('.').next().unwrap_or(code).to_string()
    }
}

#[async_trait]
impl DataSource for AkshareClient {
    async fn fetch_balance_sheet(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<BalanceSheet>> {
        let code = self.convert_stock_code(stock_code);
        let data = self.fetch_balance_sheet_data(&code)?;

        let mut sheets = Vec::new();
        for item in data {
            let report_date = NaiveDate::parse_from_str(&item.report_date, "%Y-%m-%d")
                .or_else(|_| NaiveDate::parse_from_str(&item.report_date, "%Y%m%d"))?;

            // 只保留年报数据（12月31日）且在日期范围内
            if report_date.month() != 12 || report_date.day() != 31 {
                continue;
            }
            if report_date < start_date || report_date > end_date {
                continue;
            }

            let mut items_map = HashMap::new();
            items_map.insert("资产总计".to_string(), Decimal::from_f64_retain(item.total_assets.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("负债合计".to_string(), Decimal::from_f64_retain(item.total_liabilities.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("所有者权益合计".to_string(), Decimal::from_f64_retain(item.total_equity.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("货币资金".to_string(), Decimal::from_f64_retain(item.monetary_funds.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("固定资产".to_string(), Decimal::from_f64_retain(item.fixed_assets.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("应收账款".to_string(), Decimal::from_f64_retain(item.accounts_receivable.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("存货".to_string(), Decimal::from_f64_retain(item.inventory.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("股本".to_string(), Decimal::from_f64_retain(item.share_capital.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("实收资本(或股本)".to_string(), Decimal::from_f64_retain(item.share_capital.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));

            let statement = FinancialStatement {
                stock_code: stock_code.to_string(),
                report_date,
                report_type: ReportType::BalanceSheet,
                items: items_map,
            };

            sheets.push(BalanceSheet {
                statement,
                operating_assets: AssetGroup::new(),
                financial_assets: AssetGroup::new(),
                operating_liabilities: LiabilityGroup::new(),
                financial_liabilities: LiabilityGroup::new(),
            });
        }

        Ok(sheets)
    }

    async fn fetch_income_statement(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<IncomeStatement>> {
        let code = self.convert_stock_code(stock_code);
        let data = self.fetch_income_statement_data(&code)?;

        let mut statements = Vec::new();
        for item in data {
            let report_date = NaiveDate::parse_from_str(&item.report_date, "%Y-%m-%d")
                .or_else(|_| NaiveDate::parse_from_str(&item.report_date, "%Y%m%d"))?;

            // 只保留年报数据（12月31日）且在日期范围内
            if report_date.month() != 12 || report_date.day() != 31 {
                continue;
            }
            if report_date < start_date || report_date > end_date {
                continue;
            }

            let revenue = Decimal::from_f64_retain(item.revenue.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
            let operating_cost = Decimal::from_f64_retain(item.operating_cost.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
            let net_profit = Decimal::from_f64_retain(item.net_profit.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
            let tax = Decimal::from_f64_retain(item.tax.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
            let finance_expense = Decimal::from_f64_retain(item.finance_expense.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
            let sales_expense = Decimal::from_f64_retain(item.sales_expense.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
            let admin_expense = Decimal::from_f64_retain(item.admin_expense.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
            let rd_expense = Decimal::from_f64_retain(item.rd_expense.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);

            let mut items_map = HashMap::new();
            items_map.insert("营业总收入".to_string(), revenue);
            items_map.insert("营业总成本".to_string(), operating_cost);
            items_map.insert("净利润".to_string(), net_profit);
            items_map.insert("税金及附加".to_string(), tax);
            items_map.insert("财务费用".to_string(), finance_expense);
            items_map.insert("销售费用".to_string(), sales_expense);
            items_map.insert("管理费用".to_string(), admin_expense);
            items_map.insert("研发费用".to_string(), rd_expense);

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

    async fn fetch_cashflow_statement(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<CashflowStatement>> {
        let code = self.convert_stock_code(stock_code);
        let data = self.fetch_cashflow_statement_data(&code)?;

        let mut statements = Vec::new();
        for item in data {
            let report_date = NaiveDate::parse_from_str(&item.report_date, "%Y-%m-%d")
                .or_else(|_| NaiveDate::parse_from_str(&item.report_date, "%Y%m%d"))?;

            // 只保留年报数据（12月31日）且在日期范围内
            if report_date.month() != 12 || report_date.day() != 31 {
                continue;
            }
            if report_date < start_date || report_date > end_date {
                continue;
            }

            let operating_cashflow = Decimal::from_f64_retain(item.operating_cashflow.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
            let investing_cashflow = Decimal::from_f64_retain(item.investing_cashflow.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
            let financing_cashflow = Decimal::from_f64_retain(item.financing_cashflow.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);

            let mut items_map = HashMap::new();
            items_map.insert("经营活动产生的现金流量净额".to_string(), operating_cashflow);

            let statement = FinancialStatement {
                stock_code: stock_code.to_string(),
                report_date,
                report_type: ReportType::CashflowStatement,
                items: items_map,
            };

            statements.push(CashflowStatement {
                statement,
                operating_cashflow,
                investing_cashflow,
                financing_cashflow,
                free_cashflow: operating_cashflow + investing_cashflow,
            });
        }

        Ok(statements)
    }

    fn name(&self) -> &str {
        "akshare"
    }
}
