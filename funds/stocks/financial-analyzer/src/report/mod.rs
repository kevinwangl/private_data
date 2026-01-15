use crate::domain::models::{AnalysisResult, FinancialStatement, ReportType};
use anyhow::Result;
use chrono::Local;
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
}
