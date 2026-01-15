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
    // 资产科目
    #[serde(rename = "NOTES_RECEIVABLE")]
    notes_receivable: Option<f64>,
    #[serde(rename = "PREPAYMENTS")]
    prepayments: Option<f64>,
    #[serde(rename = "INTANGIBLE_ASSETS")]
    intangible_assets: Option<f64>,
    #[serde(rename = "TRADING_FINANCIAL_ASSETS")]
    trading_financial_assets: Option<f64>,
    #[serde(rename = "LONG_TERM_EQUITY_INVESTMENT")]
    long_term_equity_investment: Option<f64>,
    #[serde(rename = "HELD_TO_MATURITY_INVESTMENTS")]
    held_to_maturity_investments: Option<f64>,
    #[serde(rename = "INVESTMENT_PROPERTY")]
    investment_property: Option<f64>,
    #[serde(rename = "LONG_TERM_RECEIVABLES")]
    long_term_receivables: Option<f64>,
    #[serde(rename = "INTEREST_RECEIVABLE")]
    interest_receivable: Option<f64>,
    #[serde(rename = "DIVIDEND_RECEIVABLE")]
    dividend_receivable: Option<f64>,
    #[serde(rename = "DEFERRED_TAX_ASSETS")]
    deferred_tax_assets: Option<f64>,
    #[serde(rename = "NON_CURRENT_ASSETS_DUE_WITHIN_ONE_YEAR")]
    non_current_assets_due_within_one_year: Option<f64>,
    #[serde(rename = "OTHER_NON_CURRENT_ASSETS")]
    other_non_current_assets: Option<f64>,
    // 负债科目
    #[serde(rename = "NOTES_PAYABLE")]
    notes_payable: Option<f64>,
    #[serde(rename = "ACCOUNTS_PAYABLE")]
    accounts_payable: Option<f64>,
    #[serde(rename = "ADVANCE_RECEIPTS")]
    advance_receipts: Option<f64>,
    #[serde(rename = "EMPLOYEE_PAYABLE")]
    employee_payable: Option<f64>,
    #[serde(rename = "TAX_PAYABLE")]
    tax_payable: Option<f64>,
    #[serde(rename = "CONTRACT_LIABILITIES")]
    contract_liabilities: Option<f64>,
    #[serde(rename = "DEFERRED_TAX_LIABILITIES")]
    deferred_tax_liabilities: Option<f64>,
    #[serde(rename = "DEFERRED_REVENUE")]
    deferred_revenue: Option<f64>,
    #[serde(rename = "INTEREST_PAYABLE")]
    interest_payable: Option<f64>,
    #[serde(rename = "DIVIDEND_PAYABLE")]
    dividend_payable: Option<f64>,
    #[serde(rename = "BONDS_PAYABLE")]
    bonds_payable: Option<f64>,
    #[serde(rename = "TRADING_FINANCIAL_LIABILITIES")]
    trading_financial_liabilities: Option<f64>,
    #[serde(rename = "LONG_TERM_PAYABLE")]
    long_term_payable: Option<f64>,
    #[serde(rename = "LONG_TERM_LOAN")]
    long_term_loan: Option<f64>,
    #[serde(rename = "SHORT_TERM_LOAN")]
    short_term_loan: Option<f64>,
    #[serde(rename = "NON_CURRENT_LIABILITIES_DUE_WITHIN_ONE_YEAR")]
    non_current_liabilities_due_within_one_year: Option<f64>,
    #[serde(rename = "CURRENT_LIABILITIES")]
    current_liabilities: Option<f64>,
    #[serde(rename = "NON_CURRENT_LIABILITIES")]
    non_current_liabilities: Option<f64>,
    #[serde(rename = "CURRENT_ASSETS")]
    current_assets: Option<f64>,
    #[serde(rename = "NON_CURRENT_ASSETS")]
    non_current_assets: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct AkshareIncomeStatement {
    #[serde(rename = "REPORT_DATE")]
    report_date: String,
    #[serde(rename = "TOTAL_OPERATE_INCOME")]
    revenue: Option<f64>,
    #[serde(rename = "OPERATE_COST")]
    operating_cost: Option<f64>,
    #[serde(rename = "MAIN_OPERATE_COST")]
    main_operate_cost: Option<f64>,
    #[serde(rename = "OPERATE_PROFIT")]
    operate_profit: Option<f64>,
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
    #[serde(rename = "BIZ_ADMIN_EXPENSE")]
    biz_admin_expense: Option<f64>,
    #[serde(rename = "RD_EXPENSE")]
    rd_expense: Option<f64>,
    #[serde(rename = "OTHER_INCOME")]
    other_income: Option<f64>,
    #[serde(rename = "INVEST_INCOME")]
    invest_income: Option<f64>,
    #[serde(rename = "FAIR_VALUE_CHANGE")]
    fair_value_change: Option<f64>,
    #[serde(rename = "ASSET_DISPOSAL_INCOME")]
    asset_disposal_income: Option<f64>,
    #[serde(rename = "ASSET_IMPAIRMENT_LOSS")]
    asset_impairment_loss: Option<f64>,
    #[serde(rename = "CREDIT_IMPAIRMENT_LOSS")]
    credit_impairment_loss: Option<f64>,
    #[serde(rename = "NON_OPERATING_INCOME")]
    non_operating_income: Option<f64>,
    #[serde(rename = "NON_OPERATING_EXPENSE")]
    non_operating_expense: Option<f64>,
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
    #[serde(rename = "CAPEX")]
    capex: Option<f64>,
    #[serde(rename = "INVEST_PAY_CASH")]
    invest_pay_cash: Option<f64>,
    #[serde(rename = "RECEIVE_INVEST_CASH")]
    receive_invest_cash: Option<f64>,
    #[serde(rename = "RECEIVE_LOAN_CASH")]
    receive_loan_cash: Option<f64>,
    #[serde(rename = "REPAY_DEBT_CASH")]
    repay_debt_cash: Option<f64>,
    #[serde(rename = "DISTRIBUTE_DIVIDEND_CASH")]
    distribute_dividend_cash: Option<f64>,
    #[serde(rename = "PAY_OTHER_FINANCE_CASH")]
    pay_other_finance_cash: Option<f64>,
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
        'TOTAL_EQUITY': safe_float(row.get('所有者权益(或股东权益)合计')) or safe_float(row.get('所有者权益合计')),
        'MONETARYFUNDS': safe_float(row.get('货币资金')),
        'FIXED_ASSETS': safe_float(row.get('固定资产净额')) or safe_float(row.get('固定资产及清理合计')),
        'ACCOUNTS_RECE': safe_float(row.get('应收账款')),
        'INVENTORY': safe_float(row.get('存货')),
        'SHARE_CAPITAL': safe_float(row.get('实收资本(或股本)')) or safe_float(row.get('股本')),
        'NOTES_RECEIVABLE': safe_float(row.get('应收票据')),
        'PREPAYMENTS': safe_float(row.get('预付款项')),
        'INTANGIBLE_ASSETS': safe_float(row.get('无形资产')),
        'TRADING_FINANCIAL_ASSETS': safe_float(row.get('交易性金融资产')),
        'LONG_TERM_EQUITY_INVESTMENT': safe_float(row.get('长期股权投资')),
        'HELD_TO_MATURITY_INVESTMENTS': safe_float(row.get('持有至到期投资')),
        'INVESTMENT_PROPERTY': safe_float(row.get('投资性房地产')),
        'LONG_TERM_RECEIVABLES': safe_float(row.get('长期应收款')),
        'INTEREST_RECEIVABLE': safe_float(row.get('应收利息')),
        'DIVIDEND_RECEIVABLE': safe_float(row.get('应收股利')),
        'DEFERRED_TAX_ASSETS': safe_float(row.get('递延所得税资产')) or safe_float(row.get('递延税款借项')),
        'NON_CURRENT_ASSETS_DUE_WITHIN_ONE_YEAR': safe_float(row.get('一年内到期的非流动资产')),
        'OTHER_NON_CURRENT_ASSETS': safe_float(row.get('其他非流动资产')),
        'NOTES_PAYABLE': safe_float(row.get('应付票据')) or safe_float(row.get('应付票据及应付账款')),
        'ACCOUNTS_PAYABLE': safe_float(row.get('应付账款')),
        'ADVANCE_RECEIPTS': safe_float(row.get('预收款项')),
        'EMPLOYEE_PAYABLE': safe_float(row.get('应付职工薪酬')),
        'TAX_PAYABLE': safe_float(row.get('应交税费')),
        'CONTRACT_LIABILITIES': safe_float(row.get('合同负债')),
        'DEFERRED_TAX_LIABILITIES': safe_float(row.get('递延所得税负债')) or safe_float(row.get('递延税款贷项')),
        'DEFERRED_REVENUE': safe_float(row.get('长期递延收益')) or safe_float(row.get('递延收益')),
        'INTEREST_PAYABLE': safe_float(row.get('应付利息')),
        'DIVIDEND_PAYABLE': safe_float(row.get('应付股利')),
        'BONDS_PAYABLE': safe_float(row.get('应付债券')) or safe_float(row.get('应付债券款')),
        'TRADING_FINANCIAL_LIABILITIES': safe_float(row.get('交易性金融负债')),
        'LONG_TERM_PAYABLE': safe_float(row.get('长期应付款')),
        'LONG_TERM_LOAN': safe_float(row.get('长期借款')),
        'SHORT_TERM_LOAN': safe_float(row.get('短期借款')),
        'NON_CURRENT_LIABILITIES_DUE_WITHIN_ONE_YEAR': safe_float(row.get('一年内到期的非流动负债')),
        'CURRENT_LIABILITIES': safe_float(row.get('流动负债合计')),
        'NON_CURRENT_LIABILITIES': safe_float(row.get('非流动负债合计')),
        'CURRENT_ASSETS': safe_float(row.get('流动资产合计')),
        'NON_CURRENT_ASSETS': safe_float(row.get('非流动资产合计')),
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
        'TOTAL_OPERATE_INCOME': safe_float(row.get('营业总收入')) or safe_float(row.get('营业收入')),
        'OPERATE_COST': safe_float(row.get('营业总成本')) or safe_float(row.get('营业支出')),
        'MAIN_OPERATE_COST': safe_float(row.get('营业成本')),
        'OPERATE_PROFIT': safe_float(row.get('营业利润')),
        'NETPROFIT': safe_float(row.get('净利润')),
        'TAX': safe_float(row.get('营业税金及附加')),
        'FINANCE_EXPENSE': safe_float(row.get('财务费用')),
        'SALES_EXPENSE': safe_float(row.get('销售费用')) or 0,
        'ADMIN_EXPENSE': safe_float(row.get('管理费用')) or safe_float(row.get('业务及管理费')),
        'BIZ_ADMIN_EXPENSE': safe_float(row.get('业务及管理费')),
        'RD_EXPENSE': safe_float(row.get('研发费用')),
        'OTHER_INCOME': safe_float(row.get('其他收益')),
        'INVEST_INCOME': safe_float(row.get('投资收益')),
        'FAIR_VALUE_CHANGE': safe_float(row.get('公允价值变动收益')),
        'ASSET_DISPOSAL_INCOME': safe_float(row.get('资产处置收益')),
        'ASSET_IMPAIRMENT_LOSS': safe_float(row.get('资产减值损失')),
        'CREDIT_IMPAIRMENT_LOSS': safe_float(row.get('信用减值损失')),
        'NON_OPERATING_INCOME': safe_float(row.get('营业外收入')) or safe_float(row.get('加:营业外收入')),
        'NON_OPERATING_EXPENSE': safe_float(row.get('营业外支出')) or safe_float(row.get('减:营业外支出')),
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
        'CAPEX': safe_float(row.get('购建固定资产、无形资产和其他长期资产所支付的现金')) or safe_float(row.get('购建固定资产、无形资产和其他长期资产支付的现金')),
        'INVEST_PAY_CASH': safe_float(row.get('投资所支付的现金')),
        'RECEIVE_INVEST_CASH': safe_float(row.get('吸收投资收到的现金')),
        'RECEIVE_LOAN_CASH': safe_float(row.get('取得借款收到的现金')),
        'REPAY_DEBT_CASH': safe_float(row.get('偿还债务支付的现金')),
        'DISTRIBUTE_DIVIDEND_CASH': safe_float(row.get('分配股利、利润或偿付利息所支付的现金')),
        'PAY_OTHER_FINANCE_CASH': safe_float(row.get('支付其他与筹资活动有关的现金')),
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
            
            // 资产科目
            items_map.insert("应收票据".to_string(), Decimal::from_f64_retain(item.notes_receivable.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("预付款项".to_string(), Decimal::from_f64_retain(item.prepayments.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("无形资产".to_string(), Decimal::from_f64_retain(item.intangible_assets.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("交易性金融资产".to_string(), Decimal::from_f64_retain(item.trading_financial_assets.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("长期股权投资".to_string(), Decimal::from_f64_retain(item.long_term_equity_investment.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("持有至到期投资".to_string(), Decimal::from_f64_retain(item.held_to_maturity_investments.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("投资性房地产".to_string(), Decimal::from_f64_retain(item.investment_property.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("长期应收款".to_string(), Decimal::from_f64_retain(item.long_term_receivables.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("应收利息".to_string(), Decimal::from_f64_retain(item.interest_receivable.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("应收股利".to_string(), Decimal::from_f64_retain(item.dividend_receivable.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("递延所得税资产".to_string(), Decimal::from_f64_retain(item.deferred_tax_assets.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("一年内到期的非流动资产".to_string(), Decimal::from_f64_retain(item.non_current_assets_due_within_one_year.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("其他非流动资产".to_string(), Decimal::from_f64_retain(item.other_non_current_assets.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            
            // 负债科目
            items_map.insert("应付票据".to_string(), Decimal::from_f64_retain(item.notes_payable.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("应付账款".to_string(), Decimal::from_f64_retain(item.accounts_payable.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("预收款项".to_string(), Decimal::from_f64_retain(item.advance_receipts.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("应付职工薪酬".to_string(), Decimal::from_f64_retain(item.employee_payable.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("应交税费".to_string(), Decimal::from_f64_retain(item.tax_payable.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("合同负债".to_string(), Decimal::from_f64_retain(item.contract_liabilities.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("递延所得税负债".to_string(), Decimal::from_f64_retain(item.deferred_tax_liabilities.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("递延收益".to_string(), Decimal::from_f64_retain(item.deferred_revenue.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("应付利息".to_string(), Decimal::from_f64_retain(item.interest_payable.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("应付股利".to_string(), Decimal::from_f64_retain(item.dividend_payable.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("应付债券".to_string(), Decimal::from_f64_retain(item.bonds_payable.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("交易性金融负债".to_string(), Decimal::from_f64_retain(item.trading_financial_liabilities.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("长期应付款".to_string(), Decimal::from_f64_retain(item.long_term_payable.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("长期借款".to_string(), Decimal::from_f64_retain(item.long_term_loan.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("短期借款".to_string(), Decimal::from_f64_retain(item.short_term_loan.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("一年内到期的非流动负债".to_string(), Decimal::from_f64_retain(item.non_current_liabilities_due_within_one_year.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("流动负债合计".to_string(), Decimal::from_f64_retain(item.current_liabilities.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("非流动负债合计".to_string(), Decimal::from_f64_retain(item.non_current_liabilities.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("流动资产合计".to_string(), Decimal::from_f64_retain(item.current_assets.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("非流动资产合计".to_string(), Decimal::from_f64_retain(item.non_current_assets.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));

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
            let biz_admin_expense = Decimal::from_f64_retain(item.biz_admin_expense.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
            let rd_expense = Decimal::from_f64_retain(item.rd_expense.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
            let main_cost = Decimal::from_f64_retain(item.main_operate_cost.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
            let operate_profit = Decimal::from_f64_retain(item.operate_profit.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);

            let mut items_map = HashMap::new();
            items_map.insert("营业总收入".to_string(), revenue);
            items_map.insert("营业总成本".to_string(), operating_cost);
            items_map.insert("营业成本".to_string(), main_cost);
            items_map.insert("营业利润".to_string(), operate_profit);
            items_map.insert("净利润".to_string(), net_profit);
            items_map.insert("税金及附加".to_string(), tax);
            items_map.insert("财务费用".to_string(), finance_expense);
            items_map.insert("销售费用".to_string(), sales_expense);
            items_map.insert("管理费用".to_string(), admin_expense);
            items_map.insert("业务及管理费".to_string(), biz_admin_expense);
            items_map.insert("研发费用".to_string(), rd_expense);
            items_map.insert("其他收益".to_string(), Decimal::from_f64_retain(item.other_income.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("投资收益".to_string(), Decimal::from_f64_retain(item.invest_income.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("公允价值变动收益".to_string(), Decimal::from_f64_retain(item.fair_value_change.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("资产处置收益".to_string(), Decimal::from_f64_retain(item.asset_disposal_income.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("资产减值损失".to_string(), Decimal::from_f64_retain(item.asset_impairment_loss.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("信用减值损失".to_string(), Decimal::from_f64_retain(item.credit_impairment_loss.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("营业外收入".to_string(), Decimal::from_f64_retain(item.non_operating_income.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("营业外支出".to_string(), Decimal::from_f64_retain(item.non_operating_expense.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));

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
                gross_profit: revenue - main_cost,  // 毛利 = 营业总收入 - 营业成本
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
            items_map.insert("投资活动产生的现金流量净额".to_string(), investing_cashflow);
            items_map.insert("筹资活动产生的现金流量净额".to_string(), financing_cashflow);
            items_map.insert("购建固定资产、无形资产和其他长期资产支付的现金".to_string(), Decimal::from_f64_retain(item.capex.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("投资支付的现金".to_string(), Decimal::from_f64_retain(item.invest_pay_cash.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("吸收投资收到的现金".to_string(), Decimal::from_f64_retain(item.receive_invest_cash.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("取得借款收到的现金".to_string(), Decimal::from_f64_retain(item.receive_loan_cash.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("偿还债务支付的现金".to_string(), Decimal::from_f64_retain(item.repay_debt_cash.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("分配股利、利润或偿付利息支付的现金".to_string(), Decimal::from_f64_retain(item.distribute_dividend_cash.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));
            items_map.insert("支付其他与筹资活动有关的现金".to_string(), Decimal::from_f64_retain(item.pay_other_finance_cash.unwrap_or(0.0)).unwrap_or(Decimal::ZERO));

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
