use crate::domain::*;
use crate::analyzer::ValuationResult;
use anyhow::Result;
use rust_xlsxwriter::*;
use rust_decimal::prelude::ToPrimitive;
use std::path::Path;

pub struct ExcelWriter;

impl ExcelWriter {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, result: &AnalysisResult, output_path: &Path) -> Result<()> {
        let mut workbook = Workbook::new();

        self.write_sheet1_asset_liability(&mut workbook, result)?;
        self.write_sheet2_operating_financial(&mut workbook, result)?;
        self.write_sheet3_profit_cashflow(&mut workbook, result)?;
        self.write_sheet4_comprehensive(&mut workbook, result)?;
        self.write_sheet5_balance_perspective(&mut workbook, result)?;

        workbook.save(output_path)?;
        Ok(())
    }

    // 设置列宽自适应
    fn auto_fit_columns(worksheet: &mut Worksheet) -> Result<()> {
        // 设置更大的列宽以确保完整显示
        worksheet.set_column_width(0, 20)?;   // A列 - 分类
        worksheet.set_column_width(1, 35)?;   // B列 - 项目名称（更宽）
        worksheet.set_column_width(2, 16)?;   // C列 - 数据
        worksheet.set_column_width(3, 16)?;   // D列 - 数据
        worksheet.set_column_width(4, 16)?;   // E列 - 数据
        worksheet.set_column_width(5, 16)?;   // F列 - 数据
        worksheet.set_column_width(6, 20)?;   // G列 - 分类
        worksheet.set_column_width(7, 35)?;   // H列 - 项目名称
        for col in 8..15 {
            worksheet.set_column_width(col, 16)?;  // 其他数据列
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
    fn create_formats() -> (Format, Format, Format, Format, Format, Format, Format) {
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
        
        // 数据格式 - 千分位
        let number_fmt = Format::new()
            .set_num_format("#,##0")
            .set_border(FormatBorder::Thin);
        
        // 百分比格式 - 突出显示
        let percent_fmt = Format::new()
            .set_num_format("0.00%")
            .set_border(FormatBorder::Thin);
        
        // 重点数据格式 - 黄色背景
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
        
        // 公式格式
        let formula_fmt = Format::new()
            .set_background_color(Color::RGB(0xF2F2F2))
            .set_border(FormatBorder::Thin);
        
        (header_fmt, subheader_fmt, number_fmt, percent_fmt, highlight_fmt, positive_fmt, formula_fmt)
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
        
        // 创建格式
        let (header_fmt, subheader_fmt, number_fmt, percent_fmt, highlight_fmt, _, _) = Self::create_formats();
        
        // Headers - 使用标题格式
        worksheet.write_string_with_format(1, 2, years[0].to_string(), &header_fmt)?;
        worksheet.write_string_with_format(1, 3, years[1].to_string(), &header_fmt)?;
        worksheet.write_string_with_format(1, 4, years[2].to_string(), &header_fmt)?;
        worksheet.write_string_with_format(1, 7, "项目", &header_fmt)?;
        worksheet.write_string_with_format(1, 8, years[0].to_string(), &header_fmt)?;
        worksheet.write_string_with_format(1, 9, years[1].to_string(), &header_fmt)?;
        worksheet.write_string_with_format(1, 10, years[2].to_string(), &header_fmt)?;

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
        
        // Calculate operating asset ratio for each year
        for (year_idx, _year) in years.iter().enumerate() {
            let total_assets = self.get_balance_sheet_value(&result.statements, year_idx, "资产总计");
            if total_assets > 0.0 {
                let operating_assets: f64 = ["货币资金", "固定资产", "应收票据", "应收账款", "预付款项", "存货", "无形资产"]
                    .iter()
                    .map(|acc| self.get_balance_sheet_value(&result.statements, year_idx, acc))
                    .sum();
                let ratio = operating_assets / total_assets;
                worksheet.write_number_with_format(21, 2 + year_idx as u16, ratio, &highlight_fmt)?;
            } else {
                worksheet.write_number_with_format(21, 2 + year_idx as u16, 0.0, &highlight_fmt)?;
            }
        }

        worksheet.write_string_with_format(22, 0, "资产比率", &subheader_fmt)?;
        worksheet.write_string_with_format(22, 1, "金融性资产占总资产比率", &subheader_fmt)?;
        
        // Calculate financial asset ratio for each year
        for (year_idx, _year) in years.iter().enumerate() {
            let total_assets = self.get_balance_sheet_value(&result.statements, year_idx, "资产总计");
            if total_assets > 0.0 {
                let financial_assets: f64 = ["交易性金融资产", "长期股权投资", "持有至到期投资", "投资性房地产",
                    "长期应收款", "应收利息", "应收股利", "递延所得税资产", "一年内到期的非流动资产", "其他非流动资产"]
                    .iter()
                    .map(|acc| self.get_balance_sheet_value(&result.statements, year_idx, acc))
                    .sum();
                let ratio = financial_assets / total_assets;
                worksheet.write_number_with_format(22, 2 + year_idx as u16, ratio, &percent_fmt)?;
            } else {
                worksheet.write_number_with_format(22, 2 + year_idx as u16, 0.0, &percent_fmt)?;
            }
        }

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

        worksheet.write_string_with_format(20, 6, "负债比率", &subheader_fmt)?;
        worksheet.write_string_with_format(20, 7, "经营性负债占总负债比率", &subheader_fmt)?;
        worksheet.write_formula_with_format(20, 8, "=IF(I20=0,0,SUM(I4:I11)/I20)", &highlight_fmt)?;
        worksheet.write_formula_with_format(20, 9, "=IF(J20=0,0,SUM(J4:J11)/J20)", &highlight_fmt)?;
        worksheet.write_formula_with_format(20, 10, "=IF(K20=0,0,SUM(K4:K11)/K20)", &highlight_fmt)?;

        worksheet.write_string_with_format(21, 6, "负债比率", &subheader_fmt)?;
        worksheet.write_string_with_format(21, 7, "金融性负债占总负债比率", &subheader_fmt)?;
        worksheet.write_formula_with_format(21, 8, "=IF(I20=0,0,SUM(I12:I19)/I20)", &highlight_fmt)?;
        worksheet.write_formula_with_format(21, 9, "=IF(J20=0,0,SUM(J12:J19)/J20)", &highlight_fmt)?;
        worksheet.write_formula_with_format(21, 10, "=IF(K20=0,0,SUM(K12:K19)/K20)", &highlight_fmt)?;

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

        let years = &result.asset_structure.years;

        // Headers
        worksheet.write_string(1, 1, "项目")?;
        worksheet.write_string(1, 2, years[0].to_string())?;
        worksheet.write_string(1, 3, years[1].to_string())?;
        worksheet.write_string(1, 4, years[2].to_string())?;

        // Income statement items with data
        let income_items = vec![
            ("营业总收入", "营业总收入"),
            ("营业总成本", "营业总成本"),
            ("税金及附加", "税金及附加"),
            ("销售费用", "销售费用"),
            ("管理费用", "管理费用"),
            ("研发费用", "研发费用"),
            ("财务费用", "财务费用"),
            ("资产减值损失", "资产减值损失"),
            ("信用减值损失", "信用减值损失"),
            ("其他收益", "其他收益"),
            ("投资收益", "投资收益"),
            ("公允价值变动收益", "公允价值变动收益"),
            ("资产处置收益", "资产处置收益"),
        ];

        let (_, _, number_fmt, _, _, _, _) = Self::create_formats();
        
        for (i, (label, account)) in income_items.iter().enumerate() {
            let row = 2 + i as u32;
            worksheet.write_string(row, 1, *label)?;
            for (year_idx, _) in years.iter().enumerate() {
                let value = self.get_income_value(&result.statements, year_idx, account);
                if value != 0.0 {
                    worksheet.write_number_with_format(row, 2 + year_idx as u16, value, &number_fmt)?;
                }
            }
        }

        // Cashflow items
        worksheet.write_string(15, 1, "经营活动产生的现金流量净额")?;
        worksheet.write_string(16, 1, "投资活动产生的现金流量净额")?;
        for (year_idx, _) in years.iter().enumerate() {
            let op_cf = self.get_cashflow_value(&result.statements, year_idx, "经营活动产生的现金流量净额");
            let inv_cf = self.get_cashflow_value(&result.statements, year_idx, "投资活动产生的现金流量净额");
            if op_cf != 0.0 {
                worksheet.write_number_with_format(15, 2 + year_idx as u16, op_cf, &number_fmt)?;
            }
            if inv_cf != 0.0 {
                worksheet.write_number_with_format(16, 2 + year_idx as u16, inv_cf, &number_fmt)?;
            }
        }

        worksheet.write_string(17, 1, "营业外收入")?;
        worksheet.write_string(18, 1, "营业外支出")?;
        worksheet.write_string(19, 1, "持续经营净利润")?;
        for (year_idx, _) in years.iter().enumerate() {
            let extra_income = self.get_income_value(&result.statements, year_idx, "营业外收入");
            let extra_expense = self.get_income_value(&result.statements, year_idx, "营业外支出");
            let net_profit = self.get_income_value(&result.statements, year_idx, "净利润");
            if extra_income != 0.0 {
                worksheet.write_number_with_format(17, 2 + year_idx as u16, extra_income, &number_fmt)?;
            }
            if extra_expense != 0.0 {
                worksheet.write_number_with_format(18, 2 + year_idx as u16, extra_expense, &number_fmt)?;
            }
            if net_profit != 0.0 {
                worksheet.write_number_with_format(19, 2 + year_idx as u16, net_profit, &number_fmt)?;
            }
        }

        // Calculated ratios - 使用格式
        let (_, subheader_fmt, number_fmt, percent_fmt, highlight_fmt, _, _) = Self::create_formats();
        
        worksheet.write_string_with_format(20, 1, "毛利", &subheader_fmt)?;
        worksheet.write_formula_with_format(20, 2, "=C3-C4", &highlight_fmt)?;
        worksheet.write_formula_with_format(20, 3, "=D3-D4", &highlight_fmt)?;
        worksheet.write_formula_with_format(20, 4, "=E3-E4", &highlight_fmt)?;

        worksheet.write_string_with_format(21, 1, "毛利率", &subheader_fmt)?;
        worksheet.write_formula_with_format(21, 2, "=IF(C3=0,0,C20/C3)", &highlight_fmt)?;
        worksheet.write_formula_with_format(21, 3, "=IF(D3=0,0,D20/D3)", &highlight_fmt)?;
        worksheet.write_formula_with_format(21, 4, "=IF(E3=0,0,E20/E3)", &highlight_fmt)?;

        worksheet.write_string_with_format(22, 1, "核心利润", &subheader_fmt)?;
        worksheet.write_formula_with_format(22, 2, "=C3-C4-C5-C6-C7-C8-C9", &highlight_fmt)?;
        worksheet.write_formula_with_format(22, 3, "=D3-D4-D5-D6-D7-D8-D9", &highlight_fmt)?;
        worksheet.write_formula_with_format(22, 4, "=E3-E4-E5-E6-E7-E8-E9", &highlight_fmt)?;

        worksheet.write_string_with_format(23, 1, "核心利润率", &subheader_fmt)?;
        worksheet.write_formula_with_format(23, 2, "=IF(C3=0,0,C22/C3)", &highlight_fmt)?;
        worksheet.write_formula_with_format(23, 3, "=IF(D3=0,0,D22/D3)", &highlight_fmt)?;
        worksheet.write_formula_with_format(23, 4, "=IF(E3=0,0,E22/E3)", &highlight_fmt)?;

        // EBIT和杠杆分析
        let years = &result.asset_structure.years;
        worksheet.write_string_with_format(15, 9, "项目", &subheader_fmt)?;
        worksheet.write_string_with_format(15, 10, years[0].to_string(), &subheader_fmt)?;
        worksheet.write_string_with_format(15, 11, years[1].to_string(), &subheader_fmt)?;
        worksheet.write_string_with_format(15, 12, years[2].to_string(), &subheader_fmt)?;

        worksheet.write_string_with_format(16, 9, "EBIT", &subheader_fmt)?;
        worksheet.write_formula_with_format(16, 10, "=C19+C9+C5", &number_fmt)?;
        worksheet.write_formula_with_format(16, 11, "=D19+D9+D5", &number_fmt)?;
        worksheet.write_formula_with_format(16, 12, "=E19+E9+E5", &number_fmt)?;

        worksheet.write_string_with_format(17, 9, "经营杠杆", &subheader_fmt)?;
        worksheet.write_formula_with_format(17, 10, "=IF(K17=0,0,(K17+C6+C7+C8)/K17)", &percent_fmt)?;
        worksheet.write_formula_with_format(17, 11, "=IF(L17=0,0,(L17+D6+D7+D8)/L17)", &percent_fmt)?;
        worksheet.write_formula_with_format(17, 12, "=IF(M17=0,0,(M17+E6+E7+E8)/M17)", &percent_fmt)?;

        worksheet.write_string_with_format(18, 9, "财务杠杆", &subheader_fmt)?;
        worksheet.write_formula_with_format(18, 10, "=IF((K17-C9)=0,0,K17/(K17-C9))", &percent_fmt)?;
        worksheet.write_formula_with_format(18, 11, "=IF((L17-D9)=0,0,L17/(L17-D9))", &percent_fmt)?;
        worksheet.write_formula_with_format(18, 12, "=IF((M17-E9)=0,0,M17/(M17-E9))", &percent_fmt)?;

        worksheet.write_string_with_format(19, 9, "总杠杆", &subheader_fmt)?;
        worksheet.write_formula_with_format(19, 10, "=K18*K19", &percent_fmt)?;
        worksheet.write_formula_with_format(19, 11, "=L18*L19", &percent_fmt)?;
        worksheet.write_formula_with_format(19, 12, "=M18*M19", &percent_fmt)?;

        // DCF & Tangchao valuation sections (formulas only)
        self.write_valuation_section(worksheet, result)?;
        
        // 设置列宽和行高
        Self::auto_fit_columns(worksheet)?;
        Self::set_row_heights(worksheet, 0, 35)?;

        Ok(())
    }

    fn write_valuation_section(&self, worksheet: &mut Worksheet, result: &AnalysisResult) -> Result<()> {
        let (_, subheader_fmt, number_fmt, _, highlight_fmt, _, _) = Self::create_formats();
        
        // DCF section
        worksheet.write_string_with_format(15, 5, "DCF估值", &subheader_fmt)?;
        worksheet.write_string(15, 6, "自由现金流均值(FCF)")?;
        worksheet.write_formula(15, 7, "=SUM(C16:E16)/3")?;

        worksheet.write_string(16, 5, "DCF估值")?;
        worksheet.write_string(16, 6, "折现率(r)")?;
        worksheet.write_string(16, 7, "8%")?;

        worksheet.write_string(17, 5, "DCF估值")?;
        worksheet.write_string(17, 6, "永续年金增长率(g)")?;
        worksheet.write_string(17, 7, "3%")?;

        worksheet.write_string(18, 5, "DCF估值")?;
        worksheet.write_string(18, 6, "自由现金流增长率(G)")?;
        worksheet.write_string(18, 7, "10%")?;

        worksheet.write_string(19, 5, "DCF估值")?;
        worksheet.write_string(19, 6, "总股本")?;
        worksheet.write_string(19, 7, "100")?;

        worksheet.write_string(20, 5, "DCF估值")?;
        worksheet.write_string(20, 6, "第一年价值")?;
        worksheet.write_formula(20, 7, "=H16*(1+H19)/POWER(1+H17,1)")?;

        worksheet.write_string(21, 5, "DCF估值")?;
        worksheet.write_string(21, 6, "第二年价值")?;
        worksheet.write_formula(21, 7, "=H16*POWER(1+H19,2)/POWER(1+H17,2)")?;

        worksheet.write_string(22, 5, "DCF估值")?;
        worksheet.write_string(22, 6, "第三年价值")?;
        worksheet.write_formula(22, 7, "=H16*POWER(1+H19,3)/POWER(1+H17,3)")?;

        worksheet.write_string(23, 5, "DCF估值")?;
        worksheet.write_string(23, 6, "永续年金价值")?;
        worksheet.write_formula(23, 7, "=(SUM(H21:H23)*(1+H18))/(H17-H18)")?;

        worksheet.write_string(24, 5, "DCF估值")?;
        worksheet.write_string(24, 6, "永续经营三年后DCF价值")?;
        worksheet.write_formula(24, 7, "=SUM(H21:H24)")?;

        worksheet.write_string_with_format(25, 5, "DCF估值", &subheader_fmt)?;
        worksheet.write_string_with_format(25, 6, "永续经营3年后企业股价", &subheader_fmt)?;
        worksheet.write_formula_with_format(25, 7, "=H25/H20", &highlight_fmt)?;

        // Tangchao section
        worksheet.write_string_with_format(27, 5, "唐朝估值", &subheader_fmt)?;
        worksheet.write_string(27, 6, "净利润增长率")?;
        worksheet.write_string(27, 7, "10%")?;

        worksheet.write_string_with_format(28, 5, "唐朝估值", &subheader_fmt)?;
        worksheet.write_string(28, 6, "无风险收益率(低估区域)")?;
        worksheet.write_string(28, 7, "4%")?;
        worksheet.write_formula(28, 9, "=1/H29")?;

        worksheet.write_string_with_format(29, 5, "唐朝估值", &subheader_fmt)?;
        worksheet.write_string(29, 6, "无风险收益率(高估区域)")?;
        worksheet.write_string(29, 7, "2%")?;
        worksheet.write_formula(29, 9, "=1/H30")?;

        worksheet.write_string_with_format(30, 5, "唐朝估值", &subheader_fmt)?;
        worksheet.write_string_with_format(30, 6, "低估买入点", &subheader_fmt)?;
        worksheet.write_formula_with_format(30, 7, "=(C19*POWER(1+H28,3))*J29", &highlight_fmt)?;
        worksheet.write_formula_with_format(30, 8, "=H31/H20", &highlight_fmt)?;

        worksheet.write_string_with_format(31, 5, "唐朝估值", &subheader_fmt)?;
        worksheet.write_string(31, 6, "再打个7折")?;
        worksheet.write_formula_with_format(31, 7, "=H31*0.7", &highlight_fmt)?;
        worksheet.write_formula_with_format(31, 8, "=H32/H20", &highlight_fmt)?;

        worksheet.write_string_with_format(32, 5, "唐朝估值", &subheader_fmt)?;
        worksheet.write_string_with_format(32, 6, "高估卖出点", &subheader_fmt)?;
        worksheet.write_formula_with_format(32, 7, "=(C19*POWER(1+H28,3))*J30", &highlight_fmt)?;
        worksheet.write_formula_with_format(32, 8, "=H33/H20", &highlight_fmt)?;

        Ok(())
    }

    // Sheet 1: 资产&负债结构分析 (简化版，引用sheet2)
    fn write_sheet1_asset_liability(&self, workbook: &mut Workbook, result: &AnalysisResult) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("资产&负债结构分析")?;
        
        let years = &result.asset_structure.years;
        let (header_fmt, subheader_fmt, number_fmt, _, _, _, _) = Self::create_formats();
        
        // Headers
        worksheet.write_string_with_format(1, 3, years[0].to_string(), &header_fmt)?;
        worksheet.write_string_with_format(1, 4, years[1].to_string(), &header_fmt)?;
        worksheet.write_string_with_format(1, 5, years[2].to_string(), &header_fmt)?;
        
        worksheet.write_string_with_format(2, 1, "项目", &subheader_fmt)?;
        
        // 流动资产
        worksheet.write_string_with_format(3, 0, "流动资产", &subheader_fmt)?;
        worksheet.write_string(3, 1, "货币资金")?;
        worksheet.write_formula_with_format(3, 3, "='(经营性&金融性)资产&负债结构分析'!C4", &number_fmt)?;
        worksheet.write_formula_with_format(3, 4, "='(经营性&金融性)资产&负债结构分析'!D4", &number_fmt)?;
        worksheet.write_formula_with_format(3, 5, "='(经营性&金融性)资产&负债结构分析'!E4", &number_fmt)?;
        
        worksheet.write_string_with_format(4, 0, "流动资产", &subheader_fmt)?;
        worksheet.write_string(4, 1, "应收账款")?;
        worksheet.write_formula_with_format(4, 3, "='(经营性&金融性)资产&负债结构分析'!C7", &number_fmt)?;
        worksheet.write_formula_with_format(4, 4, "='(经营性&金融性)资产&负债结构分析'!D7", &number_fmt)?;
        worksheet.write_formula_with_format(4, 5, "='(经营性&金融性)资产&负债结构分析'!E7", &number_fmt)?;
        
        worksheet.write_string_with_format(5, 0, "流动资产", &subheader_fmt)?;
        worksheet.write_string(5, 1, "存货")?;
        worksheet.write_formula_with_format(5, 3, "='(经营性&金融性)资产&负债结构分析'!C9", &number_fmt)?;
        worksheet.write_formula_with_format(5, 4, "='(经营性&金融性)资产&负债结构分析'!D9", &number_fmt)?;
        worksheet.write_formula_with_format(5, 5, "='(经营性&金融性)资产&负债结构分析'!E9", &number_fmt)?;
        
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
        let (header_fmt, subheader_fmt, number_fmt, _, highlight_fmt, _, _) = Self::create_formats();
        
        // Headers
        worksheet.write_string_with_format(2, 3, years[0].to_string(), &header_fmt)?;
        worksheet.write_string_with_format(2, 4, years[1].to_string(), &header_fmt)?;
        worksheet.write_string_with_format(2, 5, years[2].to_string(), &header_fmt)?;
        
        worksheet.write_string_with_format(3, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string_with_format(3, 1, "项目", &subheader_fmt)?;
        
        // 关键指标
        worksheet.write_string_with_format(4, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string(4, 1, "货币资金")?;
        worksheet.write_formula_with_format(4, 3, "='(经营性&金融性)资产&负债结构分析'!C4", &number_fmt)?;
        worksheet.write_formula_with_format(4, 4, "='(经营性&金融性)资产&负债结构分析'!D4", &number_fmt)?;
        worksheet.write_formula_with_format(4, 5, "='(经营性&金融性)资产&负债结构分析'!E4", &number_fmt)?;
        
        worksheet.write_string_with_format(5, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string(5, 1, "存货")?;
        worksheet.write_formula_with_format(5, 3, "='(经营性&金融性)资产&负债结构分析'!C9", &number_fmt)?;
        worksheet.write_formula_with_format(5, 4, "='(经营性&金融性)资产&负债结构分析'!D9", &number_fmt)?;
        worksheet.write_formula_with_format(5, 5, "='(经营性&金融性)资产&负债结构分析'!E9", &number_fmt)?;
        
        worksheet.write_string_with_format(6, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string(6, 1, "固定资产")?;
        worksheet.write_formula_with_format(6, 3, "='(经营性&金融性)资产&负债结构分析'!C5", &number_fmt)?;
        worksheet.write_formula_with_format(6, 4, "='(经营性&金融性)资产&负债结构分析'!D5", &number_fmt)?;
        worksheet.write_formula_with_format(6, 5, "='(经营性&金融性)资产&负债结构分析'!E5", &number_fmt)?;
        
        worksheet.write_string_with_format(7, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string(7, 1, "核心利润")?;
        worksheet.write_formula_with_format(7, 3, "='利润&现金流结构分析'!C22", &highlight_fmt)?;
        worksheet.write_formula_with_format(7, 4, "='利润&现金流结构分析'!D22", &highlight_fmt)?;
        worksheet.write_formula_with_format(7, 5, "='利润&现金流结构分析'!E22", &highlight_fmt)?;
        
        worksheet.write_string_with_format(8, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string(8, 1, "经营活动产生的现金流量净额")?;
        worksheet.write_formula_with_format(8, 3, "='利润&现金流结构分析'!C16", &number_fmt)?;
        worksheet.write_formula_with_format(8, 4, "='利润&现金流结构分析'!D16", &number_fmt)?;
        worksheet.write_formula_with_format(8, 5, "='利润&现金流结构分析'!E16", &number_fmt)?;
        
        worksheet.write_string_with_format(9, 0, "综合实力分析", &subheader_fmt)?;
        worksheet.write_string(9, 1, "资产总计")?;
        worksheet.write_formula_with_format(9, 3, "='(经营性&金融性)资产&负债结构分析'!C21", &number_fmt)?;
        worksheet.write_formula_with_format(9, 4, "='(经营性&金融性)资产&负债结构分析'!D21", &number_fmt)?;
        worksheet.write_formula_with_format(9, 5, "='(经营性&金融性)资产&负债结构分析'!E21", &number_fmt)?;
        
        // 评价框架
        worksheet.write_string_with_format(21, 1, "股票的价格=", &subheader_fmt)?;
        worksheet.write_string_with_format(21, 3, "价值x情绪", &subheader_fmt)?;
        
        worksheet.write_string_with_format(23, 0, "收益评价指标", &subheader_fmt)?;
        worksheet.write_string_with_format(23, 3, "标准", &subheader_fmt)?;
        worksheet.write_string_with_format(23, 4, "风险评价指标", &subheader_fmt)?;
        worksheet.write_string_with_format(23, 6, "标准", &subheader_fmt)?;
        
        worksheet.write_string_with_format(24, 0, "收益评价指标", &subheader_fmt)?;
        worksheet.write_string(24, 1, "净收入")?;
        worksheet.write_string(24, 3, "利润是否为正")?;
        worksheet.write_string_with_format(24, 4, "风险评价指标", &subheader_fmt)?;
        worksheet.write_string(24, 5, "现金流质量")?;
        worksheet.write_string(24, 6, "净收入与现金流查询是否越来越大")?;
        
        worksheet.write_string_with_format(25, 0, "收益评价指标", &subheader_fmt)?;
        worksheet.write_string(25, 1, "资产回报率(ROA)")?;
        worksheet.write_string(25, 3, "ROA是否每年改善")?;
        worksheet.write_string_with_format(25, 4, "风险评价指标", &subheader_fmt)?;
        worksheet.write_string(25, 5, "库存周转率")?;
        worksheet.write_string(25, 6, "库存周转天数是否在增加")?;
        
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
        let (header_fmt, subheader_fmt, number_fmt, _, _, _, _) = Self::create_formats();
        
        worksheet.write_string_with_format(0, 0, "科目", &header_fmt)?;
        worksheet.write_string_with_format(0, 1, years[0].to_string(), &header_fmt)?;
        worksheet.write_string_with_format(0, 2, years[1].to_string(), &header_fmt)?;
        worksheet.write_string_with_format(0, 3, years[2].to_string(), &header_fmt)?;
        
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
}
