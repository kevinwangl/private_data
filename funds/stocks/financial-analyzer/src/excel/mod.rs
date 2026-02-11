use crate::domain::*;
use crate::analyzer::ValuationResult;
use anyhow::Result;
use rust_xlsxwriter::*;
use rust_decimal::prelude::ToPrimitive;
use std::path::Path;

mod helpers;
mod descriptions;
mod sheet_builder;
mod enhanced_sensitivity;
mod enhanced_profit_cashflow;
mod enhanced_balance_sheet;
mod enhanced_comprehensive;
pub use helpers::{DataHelper, ExcelFormatter};
use descriptions::IndicatorDescriptions;
use sheet_builder::SheetBuilder;

pub struct ExcelWriter;

impl ExcelWriter {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, result: &AnalysisResult, output_path: &Path) -> Result<()> {
        let stock_code = &result.stock_code;
        let mut workbook = Workbook::new();

        // 原版sheets（保留用于对比）
        self.write_sheet1_asset_liability(&mut workbook, result)?;
        self.write_sheet2_operating_financial(&mut workbook, result)?;
        self.write_sheet3_profit_cashflow(&mut workbook, result)?;
        self.write_sheet4_comprehensive(&mut workbook, result)?;
        self.write_sheet5_balance_perspective(&mut workbook, result)?;
        
        // 如果有敏感性分析结果，添加敏感性分析工作表
        if result.sensitivity.is_some() {
            self.write_sheet6_sensitivity(&mut workbook, result)?;
        }
        
        // 优化版sheets（新增）
        enhanced_balance_sheet::write_enhanced_balance_sheet(&mut workbook, result, stock_code)?;
        enhanced_profit_cashflow::write_enhanced_profit_cashflow_sheet(&mut workbook, result, stock_code)?;
        enhanced_comprehensive::write_enhanced_comprehensive_sheet(&mut workbook, result, stock_code)?;
        
        if result.sensitivity.is_some() {
            enhanced_sensitivity::write_enhanced_sensitivity_sheet(&mut workbook, result, stock_code)?;
        }

        workbook.save(output_path)?;
        Ok(())
    }

    // 设置列宽自适应
    fn auto_fit_columns(worksheet: &mut Worksheet) -> Result<()> {
        // 设置更大的列宽以确保完整显示大数字
        worksheet.set_column_width(0, 20)?;   // A列 - 分类
        worksheet.set_column_width(1, 35)?;   // B列 - 项目名称（更宽）
        worksheet.set_column_width(2, 30)?;   // C列 - 数据（增加到30）
        worksheet.set_column_width(3, 30)?;   // D列 - 数据
        worksheet.set_column_width(4, 30)?;   // E列 - 数据
        worksheet.set_column_width(5, 30)?;   // F列 - 数据
        worksheet.set_column_width(6, 20)?;   // G列 - 分类
        worksheet.set_column_width(7, 35)?;   // H列 - 项目名称
        for col in 8..15 {
            worksheet.set_column_width(col, 30)?;  // 其他数据列（增加到30）
        }
        Ok(())
    }
    
    // 设置行高
    fn set_row_heights(worksheet: &mut Worksheet, start_row: u32, end_row: u32) -> Result<()> {
        for row in start_row..=end_row {
            worksheet.set_row_height(row, 22)?;  // 增加行高到22
        }
        Ok(())
    }

    // 格式定义
    fn create_formats() -> (Format, Format, Format, Format, Format, Format, Format, Format) {
        // 标题格式 - 深蓝色背景，白色粗体
        let header_fmt = Format::new()
            .set_bold()
            .set_font_size(12)
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
            .set_align(FormatAlign::Center)
            .set_border(FormatBorder::Thin);
        
        // 子标题格式 - 浅蓝色背景
        let subheader_fmt = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0xD9E1F2))
            .set_border(FormatBorder::Thin);
        
        // 数据格式 - 千分位，保留2位小数
        let number_fmt = Format::new()
            .set_num_format("#,##0.00")
            .set_border(FormatBorder::Thin);
        
        // 百分比格式 - 突出显示
        let percent_fmt = Format::new()
            .set_num_format("0.00%")
            .set_border(FormatBorder::Thin);
        
        // 重点数据格式 - 黄色背景（百分比）
        let highlight_fmt = Format::new()
            .set_num_format("0.00%")
            .set_background_color(Color::RGB(0xFFFF00))
            .set_bold()
            .set_border(FormatBorder::Thin);
        
        // 正向指标格式 - 绿色
        let positive_fmt = Format::new()
            .set_num_format("0.00%")
            .set_font_color(Color::RGB(0x00B050))
            .set_bold()
            .set_border(FormatBorder::Thin);
        
        // 公式格式 - 保留2位小数
        let formula_fmt = Format::new()
            .set_num_format("#,##0.00")
            .set_background_color(Color::RGB(0xF2F2F2))
            .set_border(FormatBorder::Thin);
        
        // 黄色高亮的数字格式（不是百分比）
        let highlight_number_fmt = Format::new()
            .set_num_format("#,##0.00")
            .set_background_color(Color::RGB(0xFFFF00))
            .set_bold()
            .set_border(FormatBorder::Thin);
        
        (header_fmt, subheader_fmt, number_fmt, percent_fmt, highlight_fmt, positive_fmt, formula_fmt, highlight_number_fmt)
    }

    // Helper: Get account value from statements
    fn get_account_value(&self, statements: &[FinancialStatement], year_idx: usize, account: &str) -> f64 {
        if year_idx >= statements.len() {
            return 0.0;
        }
        statements[year_idx]
            .items
            .get(account)
            .and_then(|v| v.to_f64())
            .unwrap_or(0.0)
    }

    // Sheet 2: (经营性&金融性)资产&负债结构分析
    fn write_sheet2_operating_financial(&self, workbook: &mut Workbook, result: &AnalysisResult) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("(经营性&金融性)资产&负债结构分析")?;

        let years = &result.asset_structure.years;
        let num_years = years.len().min(3); // 最多显示3年
        
        // 创建格式
        let (header_fmt, subheader_fmt, number_fmt, percent_fmt, highlight_fmt, _, _, _) = Self::create_formats();
        
        // Headers - 使用标题格式，根据实际年份数量写入
        for (i, year) in years.iter().take(num_years).enumerate() {
            worksheet.write_string_with_format(1, 2 + i as u16, year.to_string(), &header_fmt)?;
            worksheet.write_string_with_format(1, 8 + i as u16, year.to_string(), &header_fmt)?;
        }
        worksheet.write_string_with_format(1, 7, "项目", &header_fmt)?;

        worksheet.write_string_with_format(2, 1, "项目", &subheader_fmt)?;
        worksheet.write_string_with_format(2, 7, "项目", &subheader_fmt)?;

        // Left side - Assets with data
        let asset_mapping = vec![
            ("经营性资产", "货币资金", "货币资金"),
            ("经营性资产", "固定资产", "固定资产"),
            ("经营性资产", "应收票据", "应收票据"),
            ("经营性资产", "应收账款", "应收账款"),
            ("经营性资产", "预付款项", "预付款项"),
            ("经营性资产", "存货", "存货"),
            ("经营性资产", "无形资产", "无形资产"),
            ("金融性资产\n（投资性资产）", "交易性金融资产", "交易性金融资产"),
            ("金融性资产\n（投资性资产）", "长期股权投资", "长期股权投资"),
            ("金融性资产\n（投资性资产）", "持有至到期投资", "持有至到期投资"),
            ("金融性资产\n（投资性资产）", "投资性房地产", "投资性房地产"),
            ("金融性资产\n（投资性资产）", "长期应收款", "长期应收款"),
            ("金融性资产\n（投资性资产）", "应收利息", "应收利息"),
            ("金融性资产\n（投资性资产）", "应收股利", "应收股利"),
            ("金融性资产\n（投资性资产）", "递延所得税资产", "递延所得税资产"),
            ("金融性资产\n（投资性资产）", "一年内到期的非流动资产", "一年内到期的非流动资产"),
            ("金融性资产\n（投资性资产）", "其他非流动资产", "其他非流动资产"),
        ];

        for (i, (cat, item, account)) in asset_mapping.iter().enumerate() {
            let row = 3 + i as u32;
            worksheet.write_string(row, 0, *cat)?;
            worksheet.write_string(row, 1, *item)?;
            
            // Fill data for 3 years with number format
            for (year_idx, _year) in years.iter().enumerate() {
                let value = self.get_balance_sheet_value(&result.statements, year_idx, account);
                if value != 0.0 {
                    worksheet.write_number_with_format(row, 2 + year_idx as u16, value, &number_fmt)?;
                }
            }
        }

        worksheet.write_string_with_format(20, 1, "资产合计", &subheader_fmt)?;
        for (year_idx, _year) in years.iter().enumerate() {
            let value = self.get_balance_sheet_value(&result.statements, year_idx, "资产总计");
            if value != 0.0 {
                worksheet.write_number_with_format(20, 2 + year_idx as u16, value, &number_fmt)?;
            }
        }
        
        // Calculate and write ratios directly - 使用高亮格式
        worksheet.write_string_with_format(21, 0, "资产比率", &subheader_fmt)?;
        worksheet.write_string_with_format(21, 1, "经营性资产占总资产比率", &subheader_fmt)?;
        worksheet.write_formula_with_format(21, 2, "=IF(C21=0,0,SUM(C4:C10)/C21)", &highlight_fmt)?;
        worksheet.write_formula_with_format(21, 3, "=IF(D21=0,0,SUM(D4:D10)/D21)", &highlight_fmt)?;
        worksheet.write_formula_with_format(21, 4, "=IF(E21=0,0,SUM(E4:E10)/E21)", &highlight_fmt)?;

        worksheet.write_string_with_format(22, 0, "资产比率", &subheader_fmt)?;
        worksheet.write_string_with_format(22, 1, "金融性资产占总资产比率", &subheader_fmt)?;
        worksheet.write_formula_with_format(22, 2, "=IF(C21=0,0,SUM(C11:C19)/C21)", &highlight_fmt)?;
        worksheet.write_formula_with_format(22, 3, "=IF(D21=0,0,SUM(D11:D19)/D21)", &highlight_fmt)?;
        worksheet.write_formula_with_format(22, 4, "=IF(E21=0,0,SUM(E11:E19)/E21)", &highlight_fmt)?;

        // Right side - Liabilities with data
        let liability_mapping = vec![
            ("经营性负债", "应付票据", "应付票据"),
            ("经营性负债", "应付账款", "应付账款"),
            ("经营性负债", "预收款项", "预收款项"),
            ("经营性负债", "应付职工薪酬", "应付职工薪酬"),
            ("经营性负债", "应交税费", "应交税费"),
            ("经营性负债", "合同负债", "合同负债"),
            ("经营性负债", "递延所得税负债", "递延所得税负债"),
            ("经营性负债", "递延收益-非流动负债", "递延收益"),
            ("金融性负债", "应付利息", "应付利息"),
            ("金融性负债", "应付股利", "应付股利"),
            ("金融性负债", "应付债券", "应付债券"),
            ("金融性负债", "交易性金融负债", "交易性金融负债"),
            ("金融性负债", "长期应付款合计", "长期应付款"),
            ("金融性负债", "长期借款", "长期借款"),
            ("金融性负债", "短期借款", "短期借款"),
            ("金融性负债", "一年内到期的非流动负债", "一年内到期的非流动负债"),
        ];

        for (i, (cat, item, account)) in liability_mapping.iter().enumerate() {
            let row = 3 + i as u32;
            worksheet.write_string(row, 6, *cat)?;
            worksheet.write_string(row, 7, *item)?;
            
            // Fill data for 3 years with number format
            for (year_idx, _year) in years.iter().enumerate() {
                let value = self.get_balance_sheet_value(&result.statements, year_idx, account);
                if value != 0.0 {
                    worksheet.write_number_with_format(row, 8 + year_idx as u16, value, &number_fmt)?;
                }
            }
        }

        worksheet.write_string_with_format(19, 7, "负债合计", &subheader_fmt)?;
        for (year_idx, _year) in years.iter().enumerate() {
            let value = self.get_balance_sheet_value(&result.statements, year_idx, "负债合计");
            if value != 0.0 {
                worksheet.write_number_with_format(19, 8 + year_idx as u16, value, &number_fmt)?;
            }
        }

        // 添加股东权益
        worksheet.write_string_with_format(20, 7, "股东权益合计", &subheader_fmt)?;
        for (year_idx, _year) in years.iter().enumerate() {
            let value = self.get_balance_sheet_value(&result.statements, year_idx, "所有者权益合计");
            if value != 0.0 {
                worksheet.write_number_with_format(20, 8 + year_idx as u16, value, &number_fmt)?;
            }
        }

        worksheet.write_string_with_format(21, 6, "负债比率", &subheader_fmt)?;
        worksheet.write_string_with_format(21, 7, "经营性负债占总负债比率", &subheader_fmt)?;
        worksheet.write_formula_with_format(21, 8, "=IF(I20=0,0,SUM(I4:I11)/I20)", &highlight_fmt)?;
        worksheet.write_formula_with_format(21, 9, "=IF(J20=0,0,SUM(J4:J11)/J20)", &highlight_fmt)?;
        worksheet.write_formula_with_format(21, 10, "=IF(K20=0,0,SUM(K4:K11)/K20)", &highlight_fmt)?;

        worksheet.write_string_with_format(22, 6, "负债比率", &subheader_fmt)?;
        worksheet.write_string_with_format(22, 7, "金融性负债占总负债比率", &subheader_fmt)?;
        worksheet.write_formula_with_format(22, 8, "=IF(I20=0,0,SUM(I12:I19)/I20)", &highlight_fmt)?;
        worksheet.write_formula_with_format(22, 9, "=IF(J20=0,0,SUM(J12:J19)/J20)", &highlight_fmt)?;
        worksheet.write_formula_with_format(22, 10, "=IF(K20=0,0,SUM(K12:K19)/K20)", &highlight_fmt)?;

        // 设置列宽和行高
        Self::auto_fit_columns(worksheet)?;
        Self::set_row_heights(worksheet, 0, 25)?;

        Ok(())
    }

    // Helper: Get balance sheet value
    fn get_balance_sheet_value(&self, statements: &[FinancialStatement], year_idx: usize, account: &str) -> f64 {
        statements
            .iter()
            .filter(|s| s.report_type == ReportType::BalanceSheet)
            .nth(year_idx)
            .and_then(|s| s.items.get(account))
            .and_then(|v| v.to_f64())
            .unwrap_or(0.0)
    }

    // Helper: Get income statement value
    fn get_income_value(&self, statements: &[FinancialStatement], year_idx: usize, account: &str) -> f64 {
        statements
            .iter()
            .filter(|s| s.report_type == ReportType::IncomeStatement)
            .nth(year_idx)
            .and_then(|s| s.items.get(account))
            .and_then(|v| v.to_f64())
            .unwrap_or(0.0)
    }

    // Helper: Get cashflow statement value
    fn get_cashflow_value(&self, statements: &[FinancialStatement], year_idx: usize, account: &str) -> f64 {
        statements
            .iter()
            .filter(|s| s.report_type == ReportType::CashflowStatement)
            .nth(year_idx)
            .and_then(|s| s.items.get(account))
            .and_then(|v| v.to_f64())
            .unwrap_or(0.0)
    }



    // Sheet 3: 利润&现金流结构分析
    fn write_sheet3_profit_cashflow(&self, workbook: &mut Workbook, result: &AnalysisResult) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("利润&现金流结构分析")?;

        // 设置列宽
        worksheet.set_column_width(0, 20.0)?;
        worksheet.set_column_width(1, 35.0)?;
        for col in 2..16 {
            worksheet.set_column_width(col, 30.0)?;
        }

        let years = &result.asset_structure.years;
        let num_years = years.len().min(3);
        let (header_fmt, subheader_fmt, number_fmt, percent_fmt, highlight_fmt, _, _, highlight_number_fmt) = Self::create_formats();

        // === 左侧：利润表项目 ===
        worksheet.write_string_with_format(1, 1, "项目", &subheader_fmt)?;
        for (i, year) in years.iter().take(num_years).enumerate() {
            worksheet.write_string_with_format(1, 2 + i as u16, year.to_string(), &header_fmt)?;
        }

        let income_items = vec![
            ("营业总收入", "营业总收入"), ("营业成本", "营业成本"), ("营业总成本", "营业总成本"),
            ("税金及附加", "税金及附加"), ("销售费用", "销售费用"),
            ("管理费用", "管理费用"), ("研发费用", "研发费用"),
            ("财务费用", "财务费用"), ("资产减值损失", "资产减值损失"),
            ("信用减值损失", "信用减值损失"), ("其他收益", "其他收益"),
            ("投资收益", "投资收益"), ("公允价值变动收益", "公允价值变动收益"),
            ("资产处置收益", "资产处置收益"),
        ];

        for (i, (label, account)) in income_items.iter().enumerate() {
            let row = 2 + i as u32;
            worksheet.write_string(row, 1, *label)?;
            for (year_idx, _) in years.iter().take(num_years).enumerate() {
                let value = self.get_income_value(&result.statements, year_idx, account);
                worksheet.write_number_with_format(row, 2 + year_idx as u16, value, &number_fmt)?;
            }
        }

        // 营业外收入/支出、持续经营净利润
        worksheet.write_string(16, 1, "营业外收入")?;
        worksheet.write_string(17, 1, "营业外支出")?;
        worksheet.write_string(18, 1, "持续经营净利润")?;
        for (year_idx, _) in years.iter().take(num_years).enumerate() {
            let extra_income = self.get_income_value(&result.statements, year_idx, "营业外收入");
            let extra_expense = self.get_income_value(&result.statements, year_idx, "营业外支出");
            let net_profit = self.get_income_value(&result.statements, year_idx, "净利润");
            worksheet.write_number_with_format(16, 2 + year_idx as u16, extra_income, &number_fmt)?;
            worksheet.write_number_with_format(17, 2 + year_idx as u16, extra_expense, &number_fmt)?;
            worksheet.write_number_with_format(18, 2 + year_idx as u16, net_profit, &number_fmt)?;
        }

        // 计算指标 - 修正后的公式
        // Row 3=营业总收入, Row 4=营业成本, Row 5=营业总成本, Row 6=税金, Row 7=销售费用, Row 8=管理费用, Row 9=研发费用, Row 10=财务费用
        worksheet.write_string_with_format(19, 1, "毛利", &subheader_fmt)?;
        worksheet.write_string_with_format(20, 1, "毛利率", &subheader_fmt)?;
        worksheet.write_string_with_format(21, 1, "核心利润", &subheader_fmt)?;
        worksheet.write_string_with_format(22, 1, "核心利润率", &subheader_fmt)?;
        worksheet.write_string_with_format(23, 1, "核心利润获现率", &subheader_fmt)?;
        worksheet.write_string_with_format(24, 1, "销售费用率", &subheader_fmt)?;
        worksheet.write_string_with_format(25, 1, "管理费用率", &subheader_fmt)?;
        worksheet.write_string_with_format(26, 1, "营业外收入占比", &subheader_fmt)?;
        worksheet.write_string_with_format(27, 1, "净利润营收占比", &subheader_fmt)?;

        for i in 0..num_years {
            let col = 2 + i as u16;
            let col_letter = (b'C' + i as u8) as char;
            // 毛利 = 营业总收入(Row3) - 营业成本(Row4)  【修正：使用营业成本而非营业总成本】
            let f1 = format!("={}3-{}4", col_letter, col_letter);
            // 毛利率 = 毛利(Row20) / 营业总收入(Row3)
            let f2 = format!("=IF({}3=0,0,{}20/{}3)", col_letter, col_letter, col_letter);
            // 核心利润 = 毛利(Row20) - 税金(Row6) - 销售费用(Row7) - 管理费用(Row8) - 研发费用(Row9) - 财务费用(Row10)
            // 【修正：基于毛利计算，避免重复扣除】
            let f3 = format!("={}20-{}6-{}7-{}8-{}9-{}10", col_letter, col_letter, col_letter, col_letter, col_letter, col_letter);
            // 核心利润率 = 核心利润(Row22) / 营业总收入(Row3)
            let f4 = format!("=IF({}3=0,0,{}22/{}3)", col_letter, col_letter, col_letter);
            // 核心利润获现率 = 经营现金流(I4) / 核心利润
            let cashflow_col = (b'I' + i as u8) as char;
            let f5 = format!("=IF({}22=0,0,{}4/{}22)", col_letter, cashflow_col, col_letter);
            // 销售费用率 = 销售费用(Row7) / 营业总收入(Row3)
            let f6 = format!("=IF({}3=0,0,{}7/{}3)", col_letter, col_letter, col_letter);
            // 管理费用率 = 管理费用(Row8) / 营业总收入(Row3)
            let f7 = format!("=IF({}3=0,0,{}8/{}3)", col_letter, col_letter, col_letter);
            // 营业外收入占比 = 营业外收入(Row17) / 营业总收入(Row3)
            let f8 = format!("=IF({}3=0,0,{}17/{}3)", col_letter, col_letter, col_letter);
            // 净利润营收占比 = 净利润(Row19) / 营业总收入(Row3)
            let f9 = format!("=IF({}3=0,0,{}19/{}3)", col_letter, col_letter, col_letter);
            
            worksheet.write_formula_with_format(19, col, f1.as_str(), &highlight_number_fmt)?;
            worksheet.write_formula_with_format(20, col, f2.as_str(), &percent_fmt)?;
            worksheet.write_formula_with_format(21, col, f3.as_str(), &highlight_number_fmt)?;
            worksheet.write_formula_with_format(22, col, f4.as_str(), &percent_fmt)?;
            worksheet.write_formula_with_format(23, col, f5.as_str(), &number_fmt)?;
            worksheet.write_formula_with_format(24, col, f6.as_str(), &percent_fmt)?;
            worksheet.write_formula_with_format(25, col, f7.as_str(), &percent_fmt)?;
            worksheet.write_formula_with_format(26, col, f8.as_str(), &percent_fmt)?;
            worksheet.write_formula_with_format(27, col, f9.as_str(), &percent_fmt)?;
        }

        // === 中间：经营现金流分析 ===
        // 添加年份标题
        for (i, year) in years.iter().take(num_years).enumerate() {
            worksheet.write_string_with_format(1, 8 + i as u16, year.to_string(), &header_fmt)?;
        }
        
        worksheet.write_string_with_format(2, 6, "经营现金流", &subheader_fmt)?;
        worksheet.write_string(3, 7, "经营活动产生的现金流量净额")?;
        for (year_idx, _) in years.iter().take(num_years).enumerate() {
            let value = self.get_cashflow_value(&result.statements, year_idx, "经营活动产生的现金流量净额");
            worksheet.write_number_with_format(3, 8 + year_idx as u16, value, &number_fmt)?;
        }

        worksheet.write_string_with_format(4, 6, "投资现金流", &subheader_fmt)?;
        worksheet.write_string(4, 7, "资本性支出")?;
        // 资本性支出 = 固定资产期末 - 固定资产期初 (简化版，因为折旧数据不可用)
        // 使用公式引用资产负债表的固定资产数据
        for i in 0..num_years {
            let col = 8 + i as u16;
            if i == 0 {
                // 第一年：期末固定资产 - 上一年固定资产（如果有的话，否则用现金流数据）
                let value = self.get_cashflow_value(&result.statements, i, "购建固定资产、无形资产和其他长期资产支付的现金");
                worksheet.write_number_with_format(4, col, value, &number_fmt)?;
            } else {
                // 后续年份：用公式计算
                let value = self.get_cashflow_value(&result.statements, i, "购建固定资产、无形资产和其他长期资产支付的现金");
                worksheet.write_number_with_format(4, col, value, &number_fmt)?;
            }
        }

        worksheet.write_string(5, 7, "投资支付的现金")?;
        for (year_idx, _) in years.iter().take(num_years).enumerate() {
            let value = self.get_cashflow_value(&result.statements, year_idx, "投资支付的现金");
            worksheet.write_number_with_format(5, 8 + year_idx as u16, value, &number_fmt)?;
        }

        worksheet.write_string(6, 7, "投资活动产生的现金流量净额")?;
        for (year_idx, _) in years.iter().take(num_years).enumerate() {
            let value = self.get_cashflow_value(&result.statements, year_idx, "投资活动产生的现金流量净额");
            worksheet.write_number_with_format(6, 8 + year_idx as u16, value, &number_fmt)?;
        }

        worksheet.write_string_with_format(7, 6, "筹资现金流", &subheader_fmt)?;
        let financing_items = vec![
            ("吸收投资收到的现金", "吸收投资收到的现金"),
            ("取得借款收到的现金", "取得借款收到的现金"),
            ("偿还债务支付的现金", "偿还债务支付的现金"),
            ("分配股利、利润或偿付利息支付的现金", "分配股利、利润或偿付利息支付的现金"),
            ("支付其他与筹资活动有关的现金", "支付其他与筹资活动有关的现金"),
            ("筹资活动产生的现金流量净额", "筹资活动产生的现金流量净额"),
        ];

        for (i, (label, account)) in financing_items.iter().enumerate() {
            let row = 8 + i as u32;
            worksheet.write_string(row, 7, *label)?;
            for (year_idx, _) in years.iter().take(num_years).enumerate() {
                let value = self.get_cashflow_value(&result.statements, year_idx, account);
                worksheet.write_number_with_format(row, 8 + year_idx as u16, value, &number_fmt)?;
            }
        }

        worksheet.write_string_with_format(14, 6, "自由现金流", &subheader_fmt)?;
        worksheet.write_string(14, 7, "自由现金流")?;
        for i in 0..num_years {
            let col = 8 + i as u16;
            let col_letter = (b'I' + i as u8) as char;
            let formula = format!("={}4-{}5", col_letter, col_letter);
            worksheet.write_formula_with_format(14, col, formula.as_str(), &highlight_number_fmt)?;
        }

        // === 右上：分项数据 ===
        worksheet.write_string_with_format(1, 13, "分项", &subheader_fmt)?;
        for (i, year) in years.iter().take(num_years).enumerate() {
            worksheet.write_string_with_format(1, 14 + i as u16, year.to_string(), &header_fmt)?;
        }

        worksheet.write_string(2, 13, "营业收入")?;
        worksheet.write_string(3, 13, "资本性支出")?;
        worksheet.write_string(4, 13, "核心利润")?;
        worksheet.write_string(5, 13, "持续经营净利润")?;

        for i in 0..num_years {
            let col = 14 + i as u16;
            let src_col = (b'C' + i as u8) as char;
            let f1 = format!("={}3", src_col);  // 营业总收入在第3行
            let f2 = format!("={}5", (b'I' + i as u8) as char);  // 资本性支出在I5
            let f3 = format!("={}21", src_col);  // 核心利润在第21行
            let f4 = format!("={}18", src_col);  // 持续经营净利润在第18行
            
            worksheet.write_formula_with_format(2, col, f1.as_str(), &number_fmt)?;
            worksheet.write_formula_with_format(3, col, f2.as_str(), &number_fmt)?;
            worksheet.write_formula_with_format(4, col, f3.as_str(), &number_fmt)?;
            worksheet.write_formula_with_format(5, col, f4.as_str(), &number_fmt)?;
        }

        // DCF估值和EBIT分析
        self.write_valuation_section(worksheet, result, &number_fmt, &subheader_fmt, &highlight_number_fmt)?;
        
        // 杠杆分析
        self.write_leverage_section(worksheet, result, &number_fmt, &subheader_fmt)?;
        
        Self::set_row_heights(worksheet, 0, 35)?;
        Ok(())
    }

    fn write_valuation_section(&self, worksheet: &mut Worksheet, result: &AnalysisResult, 
                               number_fmt: &Format, subheader_fmt: &Format, highlight_number_fmt: &Format) -> Result<()> {
        
        // DCF section - 自由现金流在Row 15 (I15)
        worksheet.write_string_with_format(15, 5, "DCF估值", subheader_fmt)?;
        worksheet.write_string(15, 6, "基准FCF(最近一年)")?;
        worksheet.write_formula_with_format(15, 7, "=I15", number_fmt)?;

        worksheet.write_string(16, 5, "DCF估值")?;
        worksheet.write_string(16, 6, "折现率(r)")?;
        worksheet.write_number(16, 7, 0.08)?;

        worksheet.write_string(17, 5, "DCF估值")?;
        worksheet.write_string(17, 6, "永续年金增长率(g)")?;
        worksheet.write_number(17, 7, 0.04)?;

        worksheet.write_string(18, 5, "DCF估值")?;
        worksheet.write_string(18, 6, "FCF增长率(G)")?;
        // FCF增长率：计算2年复合增长率，但限制在-10%到15%之间
        worksheet.write_formula_with_format(18, 7, "=IF(OR(K15=0,K15<0,I15<0),0.1,MIN(0.15,MAX(-0.1,POWER(I15/K15,1/2)-1)))", number_fmt)?;

        worksheet.write_string(19, 5, "DCF估值")?;
        worksheet.write_string(19, 6, "总股本")?;
        // 从资产负债表获取实际总股本
        let share_capital = self.get_balance_sheet_value(&result.statements, 0, "股本");
        let paid_in_capital = self.get_balance_sheet_value(&result.statements, 0, "实收资本(或股本)");
        let total_shares = if share_capital > 0.0 {
            share_capital
        } else if paid_in_capital > 0.0 {
            paid_in_capital
        } else {
            100_000_000.0  // 如果没有数据，默认1亿股
        };
        worksheet.write_number_with_format(19, 7, total_shares, &number_fmt)?;

        worksheet.write_string(20, 5, "DCF估值")?;
        worksheet.write_string(20, 6, "第一年价值")?;
        worksheet.write_formula_with_format(20, 7, "=H16*(1+H19)/POWER(1+H17,1)", &number_fmt)?;

        worksheet.write_string(21, 5, "DCF估值")?;
        worksheet.write_string(21, 6, "第二年价值")?;
        worksheet.write_formula_with_format(21, 7, "=H16*POWER(1+H19,2)/POWER(1+H17,2)", &number_fmt)?;

        worksheet.write_string(22, 5, "DCF估值")?;
        worksheet.write_string(22, 6, "第三年价值")?;
        worksheet.write_formula_with_format(22, 7, "=H16*POWER(1+H19,3)/POWER(1+H17,3)", &number_fmt)?;

        worksheet.write_string(23, 5, "DCF估值")?;
        worksheet.write_string(23, 6, "永续年金价值")?;
        // 修正公式：永续年金 = (第3年FCF * (1+g)) / (r-g) / (1+r)^3
        // 第3年FCF = H16 * (1+H19)^3
        // 第4年FCF = 第3年FCF * (1+H18)
        // 永续年金现值 = 第4年FCF / (r-g) / (1+r)^3
        worksheet.write_formula_with_format(23, 7, "=(H16*POWER(1+H19,3)*(1+H18))/(H17-H18)/POWER(1+H17,3)", &number_fmt)?;

        worksheet.write_string(24, 5, "DCF估值")?;
        worksheet.write_string(24, 6, "永续经营三年后DCF价值")?;
        worksheet.write_formula_with_format(24, 7, "=SUM(H21:H24)", &number_fmt)?;

        // 创建黄色高亮的数字格式（不是百分比）
        let highlight_number_fmt = Format::new()
            .set_num_format("#,##0.00")
            .set_background_color(Color::RGB(0xFFFF00))
            .set_bold()
            .set_border(FormatBorder::Thin);

        worksheet.write_string_with_format(25, 5, "DCF估值", &subheader_fmt)?;
        worksheet.write_string_with_format(25, 6, "永续经营3年后企业股价", &subheader_fmt)?;
        worksheet.write_formula_with_format(25, 7, "=H25/H20", &highlight_number_fmt)?;

        // Tangchao section
        worksheet.write_string_with_format(27, 5, "唐朝估值", &subheader_fmt)?;
        worksheet.write_string(27, 6, "净利润增长率")?;
        worksheet.write_number(27, 7, 0.1)?;  // 改为数字

        worksheet.write_string_with_format(28, 5, "唐朝估值", &subheader_fmt)?;
        worksheet.write_string(28, 6, "无风险收益率(低估区域)")?;
        worksheet.write_number(28, 7, 0.04)?;  // 改为数字
        worksheet.write_formula_with_format(28, 9, "=1/H29", &number_fmt)?;

        worksheet.write_string_with_format(29, 5, "唐朝估值", &subheader_fmt)?;
        worksheet.write_string(29, 6, "无风险收益率(高估区域)")?;
        worksheet.write_number(29, 7, 0.02)?;  // 改为数字
        worksheet.write_formula_with_format(29, 9, "=1/H30", &number_fmt)?;

        worksheet.write_string_with_format(30, 5, "唐朝估值", &subheader_fmt)?;
        worksheet.write_string_with_format(30, 6, "低估买入点", &subheader_fmt)?;
        worksheet.write_formula_with_format(30, 7, "=(C19*POWER(1+H28,3))*J29", &highlight_number_fmt)?;
        worksheet.write_formula_with_format(30, 8, "=H31/H20", &highlight_number_fmt)?;

        worksheet.write_string_with_format(31, 5, "唐朝估值", &subheader_fmt)?;
        worksheet.write_string(31, 6, "再打个7折")?;
        worksheet.write_formula_with_format(31, 7, "=H31*0.7", &highlight_number_fmt)?;
        worksheet.write_formula_with_format(31, 8, "=H32/H20", &highlight_number_fmt)?;

        worksheet.write_string_with_format(32, 5, "唐朝估值", &subheader_fmt)?;
        worksheet.write_string_with_format(32, 6, "高估卖出点", &subheader_fmt)?;
        worksheet.write_formula_with_format(32, 7, "=(C19*POWER(1+H28,3))*J30", &highlight_number_fmt)?;
        worksheet.write_formula_with_format(32, 8, "=H33/H20", &highlight_number_fmt)?;

        Ok(())
    }
    
    fn write_leverage_section(&self, worksheet: &mut Worksheet, result: &AnalysisResult,
                             number_fmt: &Format, subheader_fmt: &Format) -> Result<()> {
        if result.leverage_analysis.is_some() {
            let years = &result.asset_structure.years;
            let num_years = years.len().min(3);
            
            // 杠杆分析标题（从第35行开始）
            worksheet.write_string_with_format(35, 5, "杠杆分析", subheader_fmt)?;
            
            // 年份标题
            for (i, year) in years.iter().take(num_years).enumerate() {
                worksheet.write_number_with_format(35, 6 + i as u16, *year as f64, subheader_fmt)?;
            }
            
            // 经营杠杆 DOL = EBIT变化率 / 收入变化率
            // 营业总收入在第3行，持续经营净利润在第19行（作为EBIT替代）
            worksheet.write_string(36, 5, "经营杠杆(DOL)")?;
            for i in 0..num_years {
                let col = 6 + i as u16;
                let data_col = (b'C' + i as u8) as char;
                let prev_data_col = (b'C' + i as u8 + 1) as char;
                
                if i < num_years - 1 {
                    // DOL = (净利润变化率) / (收入变化率)
                    let formula = format!(
                        "=IF(AND({}19<>0,{}3<>0,ABS(({}3/{}3)-1)>0.0001),(({}19/{}19)-1)/(({}3/{}3)-1),0)",
                        prev_data_col, prev_data_col,
                        data_col, prev_data_col,
                        data_col, prev_data_col,
                        data_col, prev_data_col
                    );
                    worksheet.write_formula_with_format(36, col, formula.as_str(), number_fmt)?;
                } else {
                    worksheet.write_string(36, col, "-")?;
                }
            }
            
            // 财务杠杆 DFL = EBIT / (EBIT - 利息费用)
            // 持续经营净利润在第19行，财务费用在第10行
            worksheet.write_string(37, 5, "财务杠杆(DFL)")?;
            for i in 0..num_years {
                let col = 6 + i as u16;
                let data_col = (b'C' + i as u8) as char;
                
                // DFL = 净利润 / (净利润 - 财务费用)
                // 注意：财务费用为负数表示利息收入，所以用减法
                let formula = format!(
                    "=IF(AND({}19<>0,({}19-{}10)<>0),{}19/({}19-{}10),1)",
                    data_col, data_col, data_col, data_col, data_col, data_col
                );
                worksheet.write_formula_with_format(37, col, formula.as_str(), number_fmt)?;
            }
            
            // 总杠杆 DTL = DOL × DFL
            worksheet.write_string(38, 5, "总杠杆(DTL)")?;
            for i in 0..num_years {
                let col = 6 + i as u16;
                let col_letter = (b'G' + i as u8) as char;
                
                let formula = format!("={}37*{}38", col_letter, col_letter);
                worksheet.write_formula_with_format(38, col, formula.as_str(), number_fmt)?;
            }
        }
        
        Ok(())
    }

    // Sheet 1: 资产&负债结构分析 (简化版，引用sheet2)
    fn write_sheet1_asset_liability(&self, workbook: &mut Workbook, result: &AnalysisResult) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("资产&负债结构分析")?;
        
        let years = &result.asset_structure.years;
        let (header_fmt, subheader_fmt, number_fmt, _, _, _, _, _) = Self::create_formats();
        
        // Headers - 至少需要1年数据
        if !years.is_empty() {
            worksheet.write_string_with_format(1, 3, years[0].to_string(), &header_fmt)?;
        }
        if years.len() > 1 {
            worksheet.write_string_with_format(1, 4, years[1].to_string(), &header_fmt)?;
        }
        if years.len() > 2 {
            worksheet.write_string_with_format(1, 5, years[2].to_string(), &header_fmt)?;
        }
        
        worksheet.write_string_with_format(2, 1, "项目", &subheader_fmt)?;
        
        // 流动资产
        worksheet.write_string_with_format(3, 0, "流动资产", &subheader_fmt)?;
        worksheet.write_string(3, 1, "货币资金")?;
        if !years.is_empty() {
            worksheet.write_formula_with_format(3, 3, "='(经营性&金融性)资产&负债结构分析'!C4", &number_fmt)?;
        }
        if years.len() > 1 {
            worksheet.write_formula_with_format(3, 4, "='(经营性&金融性)资产&负债结构分析'!D4", &number_fmt)?;
        }
        if years.len() > 2 {
            worksheet.write_formula_with_format(3, 5, "='(经营性&金融性)资产&负债结构分析'!E4", &number_fmt)?;
        }
        
        worksheet.write_string_with_format(4, 0, "流动资产", &subheader_fmt)?;
        worksheet.write_string(4, 1, "应收账款")?;
        if !years.is_empty() {
            worksheet.write_formula_with_format(4, 3, "='(经营性&金融性)资产&负债结构分析'!C7", &number_fmt)?;
        }
        if years.len() > 1 {
            worksheet.write_formula_with_format(4, 4, "='(经营性&金融性)资产&负债结构分析'!D7", &number_fmt)?;
        }
        if years.len() > 2 {
            worksheet.write_formula_with_format(4, 5, "='(经营性&金融性)资产&负债结构分析'!E7", &number_fmt)?;
        }
        
        worksheet.write_string_with_format(5, 0, "流动资产", &subheader_fmt)?;
        worksheet.write_string(5, 1, "存货")?;
        if !years.is_empty() {
            worksheet.write_formula_with_format(5, 3, "='(经营性&金融性)资产&负债结构分析'!C9", &number_fmt)?;
        }
        if years.len() > 1 {
            worksheet.write_formula_with_format(5, 4, "='(经营性&金融性)资产&负债结构分析'!D9", &number_fmt)?;
        }
        if years.len() > 2 {
            worksheet.write_formula_with_format(5, 5, "='(经营性&金融性)资产&负债结构分析'!E9", &number_fmt)?;
        }
        
        worksheet.write_string_with_format(8, 0, "非流动资产", &subheader_fmt)?;
        worksheet.write_string(8, 1, "固定资产")?;
        worksheet.write_formula_with_format(8, 3, "='(经营性&金融性)资产&负债结构分析'!C5", &number_fmt)?;
        worksheet.write_formula_with_format(8, 4, "='(经营性&金融性)资产&负债结构分析'!D5", &number_fmt)?;
        worksheet.write_formula_with_format(8, 5, "='(经营性&金融性)资产&负债结构分析'!E5", &number_fmt)?;
        
        // 设置列宽和行高
        Self::auto_fit_columns(worksheet)?;
        Self::set_row_heights(worksheet, 0, 10)?;
        
        Ok(())
    }

    // Sheet 4: 综合实力分析 (简化版)
    fn write_sheet4_comprehensive(&self, workbook: &mut Workbook, result: &AnalysisResult) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("综合实力分析")?;
        
        let years = &result.asset_structure.years;
        let (header_fmt, subheader_fmt, number_fmt, _, _highlight_fmt, _, _, _) = Self::create_formats();
        
        let highlight_number_fmt = Format::new()
            .set_num_format("#,##0.00")
            .set_background_color(Color::RGB(0xFFFF00))
            .set_bold()
            .set_border(FormatBorder::Thin);
        
        let percent_fmt = Format::new()
            .set_num_format("0.00%")
            .set_border(FormatBorder::Thin);
        
        // Headers
        let num_years = years.len().min(3);
        for (i, year) in years.iter().take(num_years).enumerate() {
            worksheet.write_string_with_format(2, 3 + i as u16, year.to_string(), &header_fmt)?;
        }
        
        worksheet.write_string_with_format(3, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string_with_format(3, 1, "项目", &subheader_fmt)?;
        
        // ROE和ROA - 净利润现在在C19
        worksheet.write_string_with_format(4, 0, "盈利能力", &subheader_fmt)?;
        worksheet.write_string_with_format(4, 1, "ROE(净资产收益率)", &subheader_fmt)?;
        worksheet.write_formula_with_format(4, 3, "=IF('(经营性&金融性)资产&负债结构分析'!I21=0,0,'利润&现金流结构分析'!C19/'(经营性&金融性)资产&负债结构分析'!I21)", &percent_fmt)?;
        worksheet.write_formula_with_format(4, 4, "=IF('(经营性&金融性)资产&负债结构分析'!J21=0,0,'利润&现金流结构分析'!D19/'(经营性&金融性)资产&负债结构分析'!J21)", &percent_fmt)?;
        worksheet.write_formula_with_format(4, 5, "=IF('(经营性&金融性)资产&负债结构分析'!K21=0,0,'利润&现金流结构分析'!E19/'(经营性&金融性)资产&负债结构分析'!K21)", &percent_fmt)?;
        
        worksheet.write_string_with_format(5, 0, "盈利能力", &subheader_fmt)?;
        worksheet.write_string_with_format(5, 1, "ROA(总资产收益率)", &subheader_fmt)?;
        worksheet.write_formula_with_format(5, 3, "=IF('(经营性&金融性)资产&负债结构分析'!C21=0,0,'利润&现金流结构分析'!C19/'(经营性&金融性)资产&负债结构分析'!C21)", &percent_fmt)?;
        worksheet.write_formula_with_format(5, 4, "=IF('(经营性&金融性)资产&负债结构分析'!D21=0,0,'利润&现金流结构分析'!D19/'(经营性&金融性)资产&负债结构分析'!D21)", &percent_fmt)?;
        worksheet.write_formula_with_format(5, 5, "=IF('(经营性&金融性)资产&负债结构分析'!E21=0,0,'利润&现金流结构分析'!E19/'(经营性&金融性)资产&负债结构分析'!E21)", &percent_fmt)?;
        
        // 关键指标
        worksheet.write_string_with_format(7, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string(7, 1, "货币资金")?;
        worksheet.write_formula_with_format(7, 3, "='(经营性&金融性)资产&负债结构分析'!C4", &number_fmt)?;
        worksheet.write_formula_with_format(7, 4, "='(经营性&金融性)资产&负债结构分析'!D4", &number_fmt)?;
        worksheet.write_formula_with_format(7, 5, "='(经营性&金融性)资产&负债结构分析'!E4", &number_fmt)?;
        
        worksheet.write_string_with_format(8, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string(8, 1, "存货")?;
        worksheet.write_formula_with_format(8, 3, "='(经营性&金融性)资产&负债结构分析'!C9", &number_fmt)?;
        worksheet.write_formula_with_format(8, 4, "='(经营性&金融性)资产&负债结构分析'!D9", &number_fmt)?;
        worksheet.write_formula_with_format(8, 5, "='(经营性&金融性)资产&负债结构分析'!E9", &number_fmt)?;
        
        worksheet.write_string_with_format(9, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string(9, 1, "固定资产")?;
        worksheet.write_formula_with_format(9, 3, "='(经营性&金融性)资产&负债结构分析'!C5", &number_fmt)?;
        worksheet.write_formula_with_format(9, 4, "='(经营性&金融性)资产&负债结构分析'!D5", &number_fmt)?;
        worksheet.write_formula_with_format(9, 5, "='(经营性&金融性)资产&负债结构分析'!E5", &number_fmt)?;
        
        worksheet.write_string_with_format(10, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string(10, 1, "核心利润")?;
        // 核心利润现在在C22
        worksheet.write_formula_with_format(10, 3, "='利润&现金流结构分析'!C22", &highlight_number_fmt)?;
        worksheet.write_formula_with_format(10, 4, "='利润&现金流结构分析'!D22", &highlight_number_fmt)?;
        worksheet.write_formula_with_format(10, 5, "='利润&现金流结构分析'!E22", &highlight_number_fmt)?;
        
        worksheet.write_string_with_format(11, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string(11, 1, "经营活动产生的现金流量净额")?;
        worksheet.write_formula_with_format(11, 3, "='利润&现金流结构分析'!I4", &number_fmt)?;
        worksheet.write_formula_with_format(11, 4, "='利润&现金流结构分析'!J4", &number_fmt)?;
        worksheet.write_formula_with_format(11, 5, "='利润&现金流结构分析'!K4", &number_fmt)?;
        
        worksheet.write_string_with_format(12, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string(12, 1, "资产总计")?;
        worksheet.write_formula_with_format(12, 3, "='(经营性&金融性)资产&负债结构分析'!C21", &number_fmt)?;
        worksheet.write_formula_with_format(12, 4, "='(经营性&金融性)资产&负债结构分析'!D21", &number_fmt)?;
        worksheet.write_formula_with_format(12, 5, "='(经营性&金融性)资产&负债结构分析'!E21", &number_fmt)?;
        
        // 设置列宽和行高
        Self::auto_fit_columns(worksheet)?;
        Self::set_row_heights(worksheet, 0, 30)?;
        
        Ok(())
    }

    // Sheet 5: 资产负债表分析视角
    fn write_sheet5_balance_perspective(&self, workbook: &mut Workbook, result: &AnalysisResult) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("资产负债表分析视角")?;
        
        let years = &result.asset_structure.years;
        let num_years = years.len().min(3);
        let (header_fmt, subheader_fmt, number_fmt, _, _, _, _, _) = Self::create_formats();
        
        worksheet.write_string_with_format(0, 0, "科目", &header_fmt)?;
        for (i, year) in years.iter().take(num_years).enumerate() {
            worksheet.write_string_with_format(0, 1 + i as u16, year.to_string(), &header_fmt)?;
        }
        
        // Key balance sheet items
        let items = vec![
            "资产总计", "流动资产合计", "非流动资产合计",
            "负债合计", "流动负债合计", "非流动负债合计",
            "所有者权益合计", "货币资金", "应收账款", "存货",
            "固定资产", "短期借款", "长期借款",
        ];
        
        for (i, item) in items.iter().enumerate() {
            worksheet.write_string_with_format(1 + i as u32, 0, *item, &subheader_fmt)?;
            for (year_idx, _) in years.iter().enumerate() {
                let value = self.get_balance_sheet_value(&result.statements, year_idx, item);
                if value != 0.0 {
                    worksheet.write_number_with_format(1 + i as u32, 1 + year_idx as u16, value, &number_fmt)?;
                }
            }
        }
        
        // 设置列宽和行高
        Self::auto_fit_columns(worksheet)?;
        Self::set_row_heights(worksheet, 0, 15)?;
        
        Ok(())
    }

    // Sheet 6: 敏感性分析
    fn write_sheet6_sensitivity(&self, workbook: &mut Workbook, result: &AnalysisResult) -> Result<()> {
        let mut worksheet = workbook.add_worksheet();
        worksheet.set_name("敏感性分析")?;
        
        let data = DataHelper::new(&result.statements);
        let sensitivity = result.sensitivity.as_ref().unwrap();
        
        let (header_fmt, subheader_fmt, number_fmt, percent_fmt, _, _, _, _) = Self::create_formats();
        
        // 获取基础数据
        let latest_fcf = data.get_cashflow_opt(0, "经营活动产生的现金流量净额").unwrap_or(0.0) 
            - data.get_cashflow_opt(0, "购建固定资产、无形资产和其他长期资产支付的现金").unwrap_or(0.0);
        let latest_net_profit = data.get_income_opt(0, "净利润").unwrap_or(0.0);
        let total_shares = data.get_balance_opt(0, "股本").unwrap_or(100_000_000.0);
        
        // 标题
        worksheet.write_string_with_format(0, 0, "敏感性分析 - 可编辑参数", &header_fmt)?;
        worksheet.merge_range(0, 0, 0, 2, "敏感性分析 - 可编辑参数", &header_fmt)?;
        
        // 参数部分
        let mut row = 2u32;
        worksheet.write_string_with_format(row, 0, "参数名称", &subheader_fmt)?;
        worksheet.write_string_with_format(row, 1, "参数值", &subheader_fmt)?;
        worksheet.write_string_with_format(row, 2, "说明", &subheader_fmt)?;
        
        row += 1;
        let r_row = row;
        worksheet.write_string(row, 0, "折现率(r)")?;
        worksheet.write_number_with_format(row, 1, sensitivity.params.discount_rate, &percent_fmt)?;
        worksheet.write_string(row, 2, "DCF估值使用")?;
        
        row += 1;
        let g_row = row;
        worksheet.write_string(row, 0, "永续增长率(g)")?;
        worksheet.write_number_with_format(row, 1, sensitivity.params.perpetual_growth_rate, &percent_fmt)?;
        worksheet.write_string(row, 2, "DCF估值使用")?;
        
        row += 1;
        let fcf_g_row = row;
        worksheet.write_string(row, 0, "FCF增长率(G)")?;
        worksheet.write_number_with_format(row, 1, sensitivity.params.fcf_growth_rate, &percent_fmt)?;
        worksheet.write_string(row, 2, "DCF估值使用")?;
        
        row += 1;
        let np_g_row = row;
        worksheet.write_string(row, 0, "净利润增长率")?;
        worksheet.write_number_with_format(row, 1, sensitivity.params.net_profit_growth_rate, &percent_fmt)?;
        worksheet.write_string(row, 2, "唐朝估值使用")?;
        
        row += 1;
        let low_rf_row = row;
        worksheet.write_string(row, 0, "无风险收益率(低估)")?;
        worksheet.write_number_with_format(row, 1, sensitivity.params.low_risk_free_rate, &percent_fmt)?;
        worksheet.write_string(row, 2, "唐朝估值使用")?;
        
        row += 1;
        let high_rf_row = row;
        worksheet.write_string(row, 0, "无风险收益率(高估)")?;
        worksheet.write_number_with_format(row, 1, sensitivity.params.high_risk_free_rate, &percent_fmt)?;
        worksheet.write_string(row, 2, "唐朝估值使用")?;
        
        // 基础数据部分
        row += 2;
        worksheet.write_string_with_format(row, 0, "基础数据（最近一年）", &header_fmt)?;
        worksheet.merge_range(row, 0, row, 2, "基础数据（最近一年）", &header_fmt)?;
        
        row += 1;
        worksheet.write_string_with_format(row, 0, "数据项", &subheader_fmt)?;
        worksheet.write_string_with_format(row, 1, "数值", &subheader_fmt)?;
        worksheet.write_string_with_format(row, 2, "单位", &subheader_fmt)?;
        
        row += 1;
        let fcf_row = row;
        worksheet.write_string(row, 0, "自由现金流(FCF)")?;
        worksheet.write_number_with_format(row, 1, latest_fcf, &number_fmt)?;
        worksheet.write_string(row, 2, "元")?;
        
        row += 1;
        let np_row = row;
        worksheet.write_string(row, 0, "净利润")?;
        worksheet.write_number_with_format(row, 1, latest_net_profit, &number_fmt)?;
        worksheet.write_string(row, 2, "元")?;
        
        row += 1;
        let shares_row = row;
        worksheet.write_string(row, 0, "总股本")?;
        worksheet.write_number_with_format(row, 1, total_shares, &number_fmt)?;
        worksheet.write_string(row, 2, "股")?;
        
        // 估值结果部分（使用公式）
        row += 2;
        worksheet.write_string_with_format(row, 0, "估值结果（自动计算）", &header_fmt)?;
        worksheet.merge_range(row, 0, row, 2, "估值结果（自动计算）", &header_fmt)?;
        
        row += 1;
        worksheet.write_string_with_format(row, 0, "估值方法", &subheader_fmt)?;
        worksheet.write_string_with_format(row, 1, "估值结果", &subheader_fmt)?;
        worksheet.write_string_with_format(row, 2, "单位", &subheader_fmt)?;
        
        // DCF计算公式
        row += 1;
        worksheet.write_string(row, 0, "DCF企业价值")?;
        worksheet.write_formula_with_format(row, 1, 
            format!("=B{fcf}*(1+B{g_fcf})/(1+B{r})+B{fcf}*(1+B{g_fcf})^2/(1+B{r})^2+B{fcf}*(1+B{g_fcf})^3/(1+B{r})^3+B{fcf}*(1+B{g_fcf})^3*(1+B{g})/(B{r}-B{g})/(1+B{r})^3",
                fcf = fcf_row + 1, r = r_row + 1, g = g_row + 1, g_fcf = fcf_g_row + 1).as_str(), 
            &number_fmt)?;
        worksheet.write_string(row, 2, "元")?;
        
        row += 1;
        let dcf_value_row = row - 1;
        worksheet.write_string(row, 0, "DCF每股价值")?;
        worksheet.write_formula_with_format(row, 1, 
            format!("=B{}/B{}", dcf_value_row + 1, shares_row + 1).as_str(), 
            &number_fmt)?;
        worksheet.write_string(row, 2, "元/股")?;
        
        // 唐朝估值公式
        row += 1;
        worksheet.write_string(row, 0, "唐朝低估价")?;
        worksheet.write_formula_with_format(row, 1, 
            format!("=B{np}*(1+B{g})^3/B{rf}/B{shares}",
                np = np_row + 1, g = np_g_row + 1, rf = low_rf_row + 1, shares = shares_row + 1).as_str(),
            &number_fmt)?;
        worksheet.write_string(row, 2, "元/股")?;
        
        row += 1;
        worksheet.write_string(row, 0, "唐朝高估价")?;
        worksheet.write_formula_with_format(row, 1, 
            format!("=B{np}*(1+B{g})^3/B{rf}/B{shares}",
                np = np_row + 1, g = np_g_row + 1, rf = high_rf_row + 1, shares = shares_row + 1).as_str(),
            &number_fmt)?;
        worksheet.write_string(row, 2, "元/股")?;
        
        row += 1;
        let low_price_row = row - 2;
        worksheet.write_string(row, 0, "唐朝安全边际价")?;
        worksheet.write_formula_with_format(row, 1, 
            format!("=B{}*0.7", low_price_row + 1).as_str(), 
            &number_fmt)?;
        worksheet.write_string(row, 2, "元/股")?;
        
        // 使用说明
        row += 2;
        worksheet.write_string_with_format(row, 0, "使用说明", &header_fmt)?;
        worksheet.merge_range(row, 0, row, 2, "使用说明", &header_fmt)?;
        
        row += 1;
        worksheet.write_string(row, 0, "1. 直接修改上方参数值，估值结果会自动更新 ✅")?;
        worksheet.merge_range(row, 0, row, 2, "1. 直接修改上方参数值，估值结果会自动更新 ✅", &Format::new())?;
        
        row += 1;
        worksheet.write_string(row, 0, "2. 参数说明：")?;
        worksheet.merge_range(row, 0, row, 2, "2. 参数说明：", &Format::new())?;
        
        row += 1;
        worksheet.write_string(row, 0, "   - 折现率：反映投资风险，通常8%-12%")?;
        worksheet.merge_range(row, 0, row, 2, "   - 折现率：反映投资风险，通常8%-12%", &Format::new())?;
        
        row += 1;
        worksheet.write_string(row, 0, "   - 永续增长率：长期稳定增长率，通常2%-5%")?;
        worksheet.merge_range(row, 0, row, 2, "   - 永续增长率：长期稳定增长率，通常2%-5%", &Format::new())?;
        
        row += 1;
        worksheet.write_string(row, 0, "   - FCF增长率：自由现金流增长率")?;
        worksheet.merge_range(row, 0, row, 2, "   - FCF增长率：自由现金流增长率", &Format::new())?;
        
        row += 1;
        worksheet.write_string(row, 0, "   - 净利润增长率：用于唐朝估值法")?;
        worksheet.merge_range(row, 0, row, 2, "   - 净利润增长率：用于唐朝估值法", &Format::new())?;
        
        // 设置列宽
        worksheet.set_column_width(0, 30)?;
        worksheet.set_column_width(1, 20)?;
        worksheet.set_column_width(2, 30)?;
        
        Ok(())
    }
}
