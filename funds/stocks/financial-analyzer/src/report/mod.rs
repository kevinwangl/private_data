use crate::domain::models::{AnalysisResult, FinancialStatement, ReportType};
use anyhow::Result;
use chrono::Local;
use rust_decimal::prelude::ToPrimitive;
use std::fs::File;
use std::io::Write;

pub struct TextReporter;

impl TextReporter {
    pub fn generate(result: &AnalysisResult, stock_code: &str, output_path: &str) -> Result<String> {
        let mut report = String::new();
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        let years = &result.asset_structure.years;
        
        // 标题
        report.push_str(&format!("{}\n", "=".repeat(100)));
        report.push_str(&format!("财务分析报告: {}\n", stock_code));
        report.push_str(&format!("生成时间: {}\n", now));
        report.push_str(&format!("{}\n\n", "=".repeat(100)));
        
        // Sheet1: 资产&负债结构分析
        Self::append_sheet1(&mut report, result, years);
        
        // Sheet2: (经营性&金融性)资产&负债结构分析
        Self::append_sheet2(&mut report, result, years);
        
        // Sheet3: 利润&现金流结构分析
        Self::append_sheet3(&mut report, result, years);
        
        // Sheet4: 综合实力分析
        Self::append_sheet4(&mut report, result, years);
        
        // Sheet5: 敏感性分析（如果有）
        if result.sensitivity.is_some() {
            Self::append_sensitivity(&mut report, result);
        }
        
        // 输出到控制台
        println!("{}", report);
        
        // 保存到文件
        let txt_path = output_path.replace(".xlsx", ".txt");
        let mut file = File::create(&txt_path)?;
        file.write_all(report.as_bytes())?;
        println!("📝 文本报告已保存到: {}", txt_path);
        
        Ok(report)
    }
    
    fn append_sheet1(report: &mut String, result: &AnalysisResult, years: &[i32]) {
        report.push_str("【Sheet1: 资产&负债结构分析】\n");
        report.push_str(&format!("{}\n", "=".repeat(100)));
        Self::append_header(report, years);
        
        report.push_str("\n--- 流动资产 ---\n");
        let items = ["货币资金", "应收账款", "存货", "预付款项", "应收票据"];
        Self::append_balance_items(report, result, &items);
        
        report.push_str("\n--- 非流动资产 ---\n");
        let items = ["固定资产", "无形资产", "长期股权投资", "投资性房地产"];
        Self::append_balance_items(report, result, &items);
        
        report.push_str("\n--- 资产合计 ---\n");
        Self::append_balance_items(report, result, &["资产总计"]);
        
        report.push_str("\n--- 流动负债 ---\n");
        let items = ["短期借款", "应付账款", "应付票据", "预收款项", "合同负债", "应付职工薪酬", "应交税费"];
        Self::append_balance_items(report, result, &items);
        
        report.push_str("\n--- 非流动负债 ---\n");
        let items = ["长期借款", "应付债券", "递延所得税负债"];
        Self::append_balance_items(report, result, &items);
        
        report.push_str("\n--- 负债及权益 ---\n");
        let items = ["负债合计", "所有者权益合计"];
        Self::append_balance_items(report, result, &items);
        report.push_str("\n");
    }
    
    fn append_sheet2(report: &mut String, result: &AnalysisResult, years: &[i32]) {
        report.push_str("【Sheet2: (经营性&金融性)资产&负债结构分析】\n");
        report.push_str(&format!("{}\n", "=".repeat(100)));
        Self::append_header(report, years);
        
        report.push_str("\n--- 经营性资产 ---\n");
        let items = ["货币资金", "固定资产", "应收票据", "应收账款", "预付款项", "存货", "无形资产"];
        Self::append_balance_items(report, result, &items);
        
        report.push_str("\n--- 金融性资产(投资性资产) ---\n");
        let items = ["交易性金融资产", "长期股权投资", "投资性房地产", "递延所得税资产"];
        Self::append_balance_items(report, result, &items);
        
        report.push_str("\n--- 资产合计 ---\n");
        Self::append_balance_items(report, result, &["资产总计"]);
        
        report.push_str("\n--- 经营性负债 ---\n");
        let items = ["应付票据", "应付账款", "预收款项", "应付职工薪酬", "应交税费", "合同负债", "递延所得税负债"];
        Self::append_balance_items(report, result, &items);
        
        report.push_str("\n--- 金融性负债 ---\n");
        let items = ["短期借款", "长期借款", "应付债券", "交易性金融负债", "一年内到期的非流动负债"];
        Self::append_balance_items(report, result, &items);
        
        report.push_str("\n--- 负债及权益 ---\n");
        let items = ["负债合计", "所有者权益合计"];
        Self::append_balance_items(report, result, &items);
        report.push_str("\n");
    }
    
    fn append_sheet3(report: &mut String, result: &AnalysisResult, years: &[i32]) {
        report.push_str("【Sheet3: 利润&现金流结构分析】\n");
        report.push_str(&format!("{}\n", "=".repeat(100)));
        Self::append_header(report, years);
        
        report.push_str("\n--- 利润表 ---\n");
        let items = ["营业总收入", "营业总成本", "税金及附加", "销售费用", "管理费用", "研发费用", "财务费用"];
        Self::append_income_items(report, result, &items);
        
        report.push_str("\n--- 其他收益 ---\n");
        let items = ["其他收益", "投资收益", "公允价值变动收益", "资产处置收益", "资产减值损失", "信用减值损失"];
        Self::append_income_items(report, result, &items);
        
        report.push_str("\n--- 营业外收支 ---\n");
        let items = ["营业外收入", "营业外支出"];
        Self::append_income_items(report, result, &items);
        
        report.push_str("\n--- 净利润 ---\n");
        Self::append_income_items(report, result, &["净利润"]);
        
        report.push_str("\n--- 现金流量表 ---\n");
        let items = [
            ("经营活动现金流量净额", "经营活动产生的现金流量净额"),
            ("投资活动现金流量净额", "投资活动产生的现金流量净额"),
            ("筹资活动现金流量净额", "筹资活动产生的现金流量净额"),
            ("资本支出(购建固定资产等)", "购建固定资产、无形资产和其他长期资产支付的现金"),
        ];
        for (display, account) in items {
            let values: Vec<String> = (0..3).map(|i| Self::get_cashflow_value(&result.statements, i, account)).collect();
            report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", display, values[0], values[1], values[2]));
        }
        
        report.push_str("\n--- 财务比率 ---\n");
        Self::append_ratios(report, result);
        
        report.push_str("\n--- 杠杆分析 ---\n");
        Self::append_leverage(report, result);
        
        report.push_str("\n--- DCF估值 ---\n");
        Self::append_dcf(report, result);
        
        report.push_str("\n--- 唐朝估值 ---\n");
        Self::append_tangchao(report, result);
        report.push_str("\n");
    }
    
    fn append_sheet4(report: &mut String, result: &AnalysisResult, years: &[i32]) {
        report.push_str("【Sheet4: 综合实力分析】\n");
        report.push_str(&format!("{}\n", "=".repeat(100)));
        Self::append_header(report, years);
        
        report.push_str("\n--- 盈利能力 ---\n");
        // ROE
        let roe_values: Vec<String> = (0..3).map(|i| {
            if let (Some(equity), Some(net_profit)) = (
                Self::get_raw_balance_value(&result.statements, i, "所有者权益合计"),
                Self::get_raw_income_value(&result.statements, i, "净利润")
            ) {
                if equity > 0.0 { format!("{:.2}%", net_profit / equity * 100.0) } else { "-".to_string() }
            } else { "-".to_string() }
        }).collect();
        report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "ROE (净资产收益率)", roe_values[0], roe_values[1], roe_values[2]));
        
        // ROA
        let roa_values: Vec<String> = (0..3).map(|i| {
            if let (Some(assets), Some(net_profit)) = (
                Self::get_raw_balance_value(&result.statements, i, "资产总计"),
                Self::get_raw_income_value(&result.statements, i, "净利润")
            ) {
                if assets > 0.0 { format!("{:.2}%", net_profit / assets * 100.0) } else { "-".to_string() }
            } else { "-".to_string() }
        }).collect();
        report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "ROA (总资产收益率)", roa_values[0], roa_values[1], roa_values[2]));
        
        // 净利润率
        let npm_values: Vec<String> = (0..3).map(|i| {
            if let (Some(revenue), Some(net_profit)) = (
                Self::get_raw_income_value(&result.statements, i, "营业总收入"),
                Self::get_raw_income_value(&result.statements, i, "净利润")
            ) {
                if revenue > 0.0 { format!("{:.2}%", net_profit / revenue * 100.0) } else { "-".to_string() }
            } else { "-".to_string() }
        }).collect();
        report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "净利润率", npm_values[0], npm_values[1], npm_values[2]));
        
        report.push_str("\n--- 关键指标 ---\n");
        let items = ["货币资金", "存货", "固定资产", "资产总计"];
        Self::append_balance_items(report, result, &items);
        
        report.push_str("\n--- 核心利润与现金流 ---\n");
        Self::append_income_items(report, result, &["净利润"]);
        let values: Vec<String> = (0..3).map(|i| Self::get_cashflow_value(&result.statements, i, "经营活动产生的现金流量净额")).collect();
        report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "经营活动现金流量净额", values[0], values[1], values[2]));
        report.push_str("\n");
    }
    
    fn append_header(report: &mut String, years: &[i32]) {
        report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "项目",
            years.get(0).unwrap_or(&0), years.get(1).unwrap_or(&0), years.get(2).unwrap_or(&0)));
        report.push_str(&format!("{}\n", "-".repeat(100)));
    }
    
    fn append_balance_items(report: &mut String, result: &AnalysisResult, items: &[&str]) {
        for item in items {
            let values: Vec<String> = (0..3).map(|i| Self::get_balance_value(&result.statements, i, item)).collect();
            report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", item, values[0], values[1], values[2]));
        }
    }
    
    fn append_income_items(report: &mut String, result: &AnalysisResult, items: &[&str]) {
        for item in items {
            let values: Vec<String> = (0..3).map(|i| Self::get_income_value(&result.statements, i, item)).collect();
            report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", item, values[0], values[1], values[2]));
        }
    }
    
    fn append_ratios(report: &mut String, result: &AnalysisResult) {
        // 毛利率 - 3年数据
        let gross_margins: Vec<String> = (0..3).map(|i| {
            if let (Some(revenue), Some(cost)) = (
                Self::get_raw_income_value(&result.statements, i, "营业总收入"),
                Self::get_raw_income_value(&result.statements, i, "营业成本")
            ) {
                if revenue > 0.0 { format!("{:.2}%", (revenue - cost) / revenue * 100.0) } else { "-".to_string() }
            } else { "-".to_string() }
        }).collect();
        report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "毛利率", gross_margins[0], gross_margins[1], gross_margins[2]));
        
        // 净利润率 - 3年数据
        let npm: Vec<String> = (0..3).map(|i| {
            if let (Some(revenue), Some(net_profit)) = (
                Self::get_raw_income_value(&result.statements, i, "营业总收入"),
                Self::get_raw_income_value(&result.statements, i, "净利润")
            ) {
                if revenue > 0.0 { format!("{:.2}%", net_profit / revenue * 100.0) } else { "-".to_string() }
            } else { "-".to_string() }
        }).collect();
        report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "净利润率", npm[0], npm[1], npm[2]));
        
        // 销售费用率 - 3年数据（保险公司无此项）
        let sales_ratio: Vec<String> = (0..3).map(|i| {
            if let (Some(revenue), Some(sales_exp)) = (
                Self::get_raw_income_value(&result.statements, i, "营业总收入"),
                Self::get_raw_income_value(&result.statements, i, "销售费用")
            ) {
                if revenue > 0.0 && sales_exp > 0.0 { format!("{:.2}%", sales_exp / revenue * 100.0) } else { "-".to_string() }
            } else { "-".to_string() }
        }).collect();
        // 只有当有数据时才输出
        if sales_ratio.iter().any(|s| s != "-") {
            report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "销售费用率", sales_ratio[0], sales_ratio[1], sales_ratio[2]));
        }
        
        // 管理费用率 - 3年数据（保险公司用业务及管理费）
        let admin_ratio: Vec<String> = (0..3).map(|i| {
            let revenue = Self::get_raw_income_value(&result.statements, i, "营业总收入");
            let admin_exp = Self::get_raw_income_value(&result.statements, i, "管理费用");
            if let (Some(rev), Some(exp)) = (revenue, admin_exp) {
                if rev > 0.0 && exp > 0.0 { return format!("{:.2}%", exp / rev * 100.0); }
            }
            "-".to_string()
        }).collect();
        if admin_ratio.iter().any(|s| s != "-") {
            report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "管理费用率", admin_ratio[0], admin_ratio[1], admin_ratio[2]));
        }
        
        // 业务及管理费率 - 保险公司专用
        let biz_admin_ratio: Vec<String> = (0..3).map(|i| {
            let revenue = Self::get_raw_income_value(&result.statements, i, "营业总收入");
            let biz_exp = Self::get_raw_income_value(&result.statements, i, "业务及管理费");
            if let (Some(rev), Some(exp)) = (revenue, biz_exp) {
                if rev > 0.0 && exp > 0.0 { return format!("{:.2}%", exp / rev * 100.0); }
            }
            "-".to_string()
        }).collect();
        if biz_admin_ratio.iter().any(|s| s != "-") {
            report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "业务及管理费率", biz_admin_ratio[0], biz_admin_ratio[1], biz_admin_ratio[2]));
        }
    }
    
    fn append_leverage(report: &mut String, result: &AnalysisResult) {
        if let Some(leverage) = &result.leverage_analysis {
            use rust_decimal::prelude::ToPrimitive;
            
            let dol: Vec<String> = leverage.operating_leverage.iter()
                .map(|v| {
                    let val = v.to_f64().unwrap_or(0.0);
                    if val.abs() < 0.01 { "-".to_string() } else { format!("{:.2}", val) }
                })
                .collect();
            
            let dfl: Vec<String> = leverage.financial_leverage.iter()
                .map(|v| {
                    let val = v.to_f64().unwrap_or(1.0);
                    if val.abs() < 0.01 { "-".to_string() } else { format!("{:.2}", val) }
                })
                .collect();
            
            let dtl: Vec<String> = leverage.total_leverage.iter()
                .map(|v| {
                    let val = v.to_f64().unwrap_or(0.0);
                    if val.abs() < 0.01 { "-".to_string() } else { format!("{:.2}", val) }
                })
                .collect();
            
            if dol.len() >= 3 && dfl.len() >= 3 && dtl.len() >= 3 {
                report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "经营杠杆(DOL)", dol[0], dol[1], dol[2]));
                report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "财务杠杆(DFL)", dfl[0], dfl[1], dfl[2]));
                report.push_str(&format!("{:<30} {:>18} {:>18} {:>18}\n", "总杠杆(DTL)", dtl[0], dtl[1], dtl[2]));
            }
            
            // 添加计算公式说明
            report.push_str("\n--- 杠杆计算公式说明 ---\n");
            
            // 获取最近一年的数据用于示例
            if let (Some(revenue_0), Some(revenue_1)) = (
                Self::get_raw_income_value(&result.statements, 0, "营业总收入"),
                Self::get_raw_income_value(&result.statements, 1, "营业总收入")
            ) {
                if let (Some(profit_0), Some(profit_1)) = (
                    Self::get_raw_income_value(&result.statements, 0, "净利润"),
                    Self::get_raw_income_value(&result.statements, 1, "净利润")
                ) {
                    let revenue_change = (revenue_0 / revenue_1 - 1.0) * 100.0;
                    let profit_change = (profit_0 / profit_1 - 1.0) * 100.0;
                    
                    report.push_str("经营杠杆(DOL) = 净利润变化率 / 收入变化率\n");
                    report.push_str(&format!("  最近一年计算: {:.2}% / {:.2}% = {}\n", 
                        profit_change, revenue_change, dol[0]));
                }
            }
            
            if let Some(profit) = Self::get_raw_income_value(&result.statements, 0, "净利润") {
                if let Some(interest) = Self::get_raw_income_value(&result.statements, 0, "财务费用") {
                    let ebt = profit - interest;
                    report.push_str("\n财务杠杆(DFL) = 净利润 / (净利润 - 财务费用)\n");
                    report.push_str(&format!("  最近一年计算: {:.2}亿 / ({:.2}亿 - {:.2}亿) = {:.2}亿 / {:.2}亿 = {}\n",
                        profit / 100_000_000.0,
                        profit / 100_000_000.0,
                        interest / 100_000_000.0,
                        profit / 100_000_000.0,
                        ebt / 100_000_000.0,
                        dfl[0]));
                }
            }
            
            report.push_str("\n总杠杆(DTL) = DOL × DFL\n");
            report.push_str(&format!("  最近一年计算: {} × {} = {}\n", dol[0], dfl[0], dtl[0]));
        }
    }
    
    fn append_dcf(report: &mut String, result: &AnalysisResult) {
        report.push_str("折现率(r): 8%\n");
        report.push_str("永续增长率(g): 4%\n");
        
        if let (Some(op_cf), Some(capex)) = (
            Self::get_raw_cashflow_value(&result.statements, 0, "经营活动产生的现金流量净额"),
            Self::get_raw_cashflow_value(&result.statements, 0, "购建固定资产、无形资产和其他长期资产支付的现金")
        ) {
            let fcf = op_cf - capex;
            report.push_str(&format!("基准FCF (最近一年): {}\n", Self::format_number(fcf)));
            
            // 简单DCF计算
            let r = 0.08;
            let g = 0.04;
            let growth = 0.1;
            let y1 = fcf * (1.0 + growth) / (1.0 + r);
            let y2 = fcf * (1.0 + growth).powi(2) / (1.0 + r).powi(2);
            let y3 = fcf * (1.0 + growth).powi(3) / (1.0 + r).powi(3);
            let terminal = fcf * (1.0 + growth).powi(3) * (1.0 + g) / (r - g) / (1.0 + r).powi(3);
            let total = y1 + y2 + y3 + terminal;
            
            report.push_str(&format!("第1年现值: {}\n", Self::format_number(y1)));
            report.push_str(&format!("第2年现值: {}\n", Self::format_number(y2)));
            report.push_str(&format!("第3年现值: {}\n", Self::format_number(y3)));
            report.push_str(&format!("永续年金现值: {}\n", Self::format_number(terminal)));
            report.push_str(&format!("企业价值: {}\n", Self::format_number(total)));
            
            if let Some(shares) = Self::get_raw_balance_value(&result.statements, 0, "股本") {
                if shares > 0.0 {
                    report.push_str(&format!("每股价值: {:.2}元\n", total / shares));
                }
            }
        }
    }
    
    fn append_tangchao(report: &mut String, result: &AnalysisResult) {
        report.push_str("净利润增长率: 10%\n");
        report.push_str("无风险收益率(低估): 4% (PE=25)\n");
        report.push_str("无风险收益率(高估): 2% (PE=50)\n");
        
        if let Some(net_profit) = Self::get_raw_income_value(&result.statements, 0, "净利润") {
            let future_profit = net_profit * 1.1_f64.powi(3);
            let low_value = future_profit * 25.0;
            let high_value = future_profit * 50.0;
            
            report.push_str(&format!("3年后净利润: {}\n", Self::format_number(future_profit)));
            report.push_str(&format!("低估买入点: {}\n", Self::format_number(low_value)));
            report.push_str(&format!("7折买入点: {}\n", Self::format_number(low_value * 0.7)));
            report.push_str(&format!("高估卖出点: {}\n", Self::format_number(high_value)));
            
            if let Some(shares) = Self::get_raw_balance_value(&result.statements, 0, "股本") {
                if shares > 0.0 {
                    report.push_str(&format!("低估股价: {:.2}元\n", low_value / shares));
                    report.push_str(&format!("7折股价: {:.2}元\n", low_value * 0.7 / shares));
                    report.push_str(&format!("高估股价: {:.2}元\n", high_value / shares));
                }
            }
        }
    }
    
    fn get_balance_value(statements: &[FinancialStatement], year_idx: usize, account: &str) -> String {
        Self::get_raw_balance_value(statements, year_idx, account)
            .map(Self::format_number).unwrap_or_else(|| "-".to_string())
    }
    
    fn get_raw_balance_value(statements: &[FinancialStatement], year_idx: usize, account: &str) -> Option<f64> {
        statements.iter().filter(|s| s.report_type == ReportType::BalanceSheet).nth(year_idx)
            .and_then(|s| s.items.get(account)).map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)).filter(|&v| v != 0.0)
    }
    
    fn get_income_value(statements: &[FinancialStatement], year_idx: usize, account: &str) -> String {
        Self::get_raw_income_value(statements, year_idx, account)
            .map(Self::format_number).unwrap_or_else(|| "-".to_string())
    }
    
    fn get_raw_income_value(statements: &[FinancialStatement], year_idx: usize, account: &str) -> Option<f64> {
        statements.iter().filter(|s| s.report_type == ReportType::IncomeStatement).nth(year_idx)
            .and_then(|s| s.items.get(account)).map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)).filter(|&v| v != 0.0)
    }
    
    fn get_cashflow_value(statements: &[FinancialStatement], year_idx: usize, account: &str) -> String {
        Self::get_raw_cashflow_value(statements, year_idx, account)
            .map(Self::format_number).unwrap_or_else(|| "-".to_string())
    }
    
    fn get_raw_cashflow_value(statements: &[FinancialStatement], year_idx: usize, account: &str) -> Option<f64> {
        statements.iter().filter(|s| s.report_type == ReportType::CashflowStatement).nth(year_idx)
            .and_then(|s| s.items.get(account)).map(|d| d.to_string().parse::<f64>().unwrap_or(0.0))
    }
    
    fn format_number(n: f64) -> String {
        if n.abs() >= 1_000_000_000.0 { format!("{:.2}亿", n / 100_000_000.0) }
        else if n.abs() >= 10_000.0 { format!("{:.2}万", n / 10_000.0) }
        else { format!("{:.2}", n) }
    }
    
    fn append_sensitivity(report: &mut String, result: &AnalysisResult) {
        report.push_str("\n【敏感性分析】\n");
        report.push_str(&format!("{}\n", "=".repeat(100)));
        
        let sensitivity = result.sensitivity.as_ref().unwrap();
        
        report.push_str("\n--- 敏感性参数 ---\n");
        report.push_str(&format!("{:<30} {:>18}\n", "参数名称", "参数值"));
        report.push_str(&format!("{}\n", "-".repeat(50)));
        report.push_str(&format!("{:<30} {:>17.2}%\n", "折现率(r)", sensitivity.params.discount_rate * 100.0));
        report.push_str(&format!("{:<30} {:>17.2}%\n", "永续年金增长率(g)", sensitivity.params.perpetual_growth_rate * 100.0));
        report.push_str(&format!("{:<30} {:>17.2}%\n", "FCF增长率(G)", sensitivity.params.fcf_growth_rate * 100.0));
        report.push_str(&format!("{:<30} {:>17.2}%\n", "净利润增长率", sensitivity.params.net_profit_growth_rate * 100.0));
        report.push_str(&format!("{:<30} {:>17.2}%\n", "无风险收益率(低估区域)", sensitivity.params.low_risk_free_rate * 100.0));
        report.push_str(&format!("{:<30} {:>17.2}%\n", "无风险收益率(高估区域)", sensitivity.params.high_risk_free_rate * 100.0));
        
        report.push_str("\n--- 估值结果 ---\n");
        report.push_str(&format!("{:<30} {:>18} {:>10}\n", "估值方法", "估值结果", "单位"));
        report.push_str(&format!("{}\n", "-".repeat(60)));
        
        let dcf_value = sensitivity.dcf_enterprise_value.to_f64().unwrap_or(0.0);
        let dcf_price = sensitivity.dcf_price_per_share.to_f64().unwrap_or(0.0);
        let low_price = sensitivity.tangchao_low_estimate.to_f64().unwrap_or(0.0);
        let high_price = sensitivity.tangchao_high_estimate.to_f64().unwrap_or(0.0);
        let safety_price = sensitivity.tangchao_safety_margin_price.to_f64().unwrap_or(0.0);
        
        report.push_str(&format!("{:<30} {:>18} {:>10}\n", "DCF企业价值", Self::format_number(dcf_value), "元"));
        report.push_str(&format!("{:<30} {:>18.2} {:>10}\n", "DCF每股价值", dcf_price, "元/股"));
        report.push_str(&format!("{:<30} {:>18.2} {:>10}\n", "唐朝低估价", low_price, "元/股"));
        report.push_str(&format!("{:<30} {:>18.2} {:>10}\n", "唐朝高估价", high_price, "元/股"));
        report.push_str(&format!("{:<30} {:>18.2} {:>10}\n", "唐朝安全边际价", safety_price, "元/股"));
        
        // 获取基础FCF用于详细计算说明
        let base_fcf = result.statements.iter()
            .find(|s| s.report_type == crate::domain::ReportType::CashflowStatement)
            .and_then(|s| {
                let operating = s.items.get("经营活动产生的现金流量净额").copied().unwrap_or_default();
                let capex = s.items.get("购建固定资产、无形资产和其他长期资产支付的现金").copied().unwrap_or_default();
                Some((operating - capex).to_f64().unwrap_or(0.0) / 100_000_000.0)
            })
            .unwrap_or(0.0);
        
        let r = sensitivity.params.discount_rate;
        let g = sensitivity.params.perpetual_growth_rate;
        let fcf_g = sensitivity.params.fcf_growth_rate;
        
        // 计算3年现值
        let fcf1 = base_fcf * (1.0 + fcf_g);
        let fcf2 = base_fcf * (1.0 + fcf_g).powi(2);
        let fcf3 = base_fcf * (1.0 + fcf_g).powi(3);
        let pv1 = fcf1 / (1.0 + r);
        let pv2 = fcf2 / (1.0 + r).powi(2);
        let pv3 = fcf3 / (1.0 + r).powi(3);
        let pv_sum = pv1 + pv2 + pv3;
        
        // 计算终值
        let terminal_value = fcf3 * (1.0 + g) / (r - g);
        let pv_terminal = terminal_value / (1.0 + r).powi(3);
        
        report.push_str("\n--- 计算公式说明 ---\n");
        report.push_str("DCF估值法（现金流折现模型）：\n");
        report.push_str(&format!("  基础FCF(最近一年): {:.2}亿元\n", base_fcf));
        report.push_str(&format!("  前3年现值: {:.2}亿 + {:.2}亿 + {:.2}亿 = {:.2}亿元\n", pv1, pv2, pv3, pv_sum));
        report.push_str(&format!("  终值现值: {:.2}亿元\n", pv_terminal));
        report.push_str(&format!("  企业价值 = {:.2}亿 + {:.2}亿 = {:.2}亿元\n", pv_sum, pv_terminal, pv_sum + pv_terminal));
        report.push_str(&format!("  每股价值 = {:.2}亿 / 总股本 = {:.2}元/股\n\n", pv_sum + pv_terminal, dcf_price));
        
        report.push_str("唐朝估值法（PE倍数法）：\n");
        report.push_str(&format!("  3年后净利润 = 当前净利润 × (1 + {}%)^3\n", sensitivity.params.net_profit_growth_rate * 100.0));
        report.push_str(&format!("  低估PE = 1 / {}% = {:.0}倍\n", sensitivity.params.low_risk_free_rate * 100.0, 1.0 / sensitivity.params.low_risk_free_rate));
        report.push_str(&format!("  高估PE = 1 / {}% = {:.0}倍\n", sensitivity.params.high_risk_free_rate * 100.0, 1.0 / sensitivity.params.high_risk_free_rate));
        report.push_str(&format!("  低估价 = 3年后净利润 × 低估PE / 总股本 = {:.2}元/股\n", low_price));
        report.push_str(&format!("  高估价 = 3年后净利润 × 高估PE / 总股本 = {:.2}元/股\n", high_price));
        report.push_str(&format!("  安全边际价 = 低估价 × 0.7 = {:.2} × 0.7 = {:.2}元/股\n", low_price, safety_price));
        
        report.push_str("\n--- 使用说明 ---\n");
        report.push_str("1. 可以通过修改参数重新运行分析，观察估值结果变化\n");
        report.push_str("2. 参数说明：\n");
        report.push_str("   - 折现率：反映投资风险，通常8%-12%\n");
        report.push_str("   - 永续增长率：长期稳定增长率，通常2%-5%\n");
        report.push_str("   - FCF增长率：自由现金流增长率\n");
        report.push_str("   - 净利润增长率：用于唐朝估值法\n");
        report.push_str("   - 无风险收益率：用于计算PE倍数\n");
        report.push_str(&format!("{}\n\n", "=".repeat(100)));
    }
}
