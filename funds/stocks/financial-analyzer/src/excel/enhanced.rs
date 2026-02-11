/// 增强版Excel报告生成器
/// 实现方案A + 部分方案B + 说明列
use crate::domain::*;
use anyhow::Result;
use rust_xlsxwriter::*;
use std::path::Path;

pub struct EnhancedExcelWriter;

impl EnhancedExcelWriter {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate(&self, result: &AnalysisResult, output_path: &Path, stock_code: &str) -> Result<()> {
        let mut workbook = Workbook::new();
        
        // Sheet1: 资产负债表分析（合并原Sheet1和Sheet2）
        self.write_balance_sheet_analysis(&mut workbook, result, stock_code)?;
        
        // Sheet2: 利润表分析（从原Sheet3拆分）
        self.write_income_analysis(&mut workbook, result, stock_code)?;
        
        // Sheet3: 现金流分析（从原Sheet3拆分）
        self.write_cashflow_analysis(&mut workbook, result, stock_code)?;
        
        // Sheet4: 综合实力分析（增强版）
        self.write_comprehensive_analysis(&mut workbook, result, stock_code)?;
        
        // Sheet5: 估值分析（合并原Sheet3估值部分和Sheet6）
        self.write_valuation_analysis(&mut workbook, result, stock_code)?;
        
        // Sheet6: 敏感性分析（优化版）
        if result.sensitivity.is_some() {
            super::enhanced_sensitivity::write_enhanced_sensitivity_sheet(&mut workbook, result, stock_code)?;
        }
        
        workbook.save(output_path)?;
        Ok(())
    }
    
    /// 创建统一的报告头
    fn write_report_header(&self, worksheet: &mut Worksheet, stock_code: &str, 
                          sheet_title: &str, row: &mut u32) -> Result<()> {
        let title_fmt = Format::new()
            .set_bold()
            .set_font_size(16)
            .set_align(FormatAlign::Left);
            
        let meta_fmt = Format::new()
            .set_font_size(10)
            .set_font_color(Color::RGB(0x666666));
        
        // 标题
        worksheet.write_string_with_format(*row, 0, 
            &format!("{}财务分析报告", stock_code), &title_fmt)?;
        *row += 1;
        
        // 元数据
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        worksheet.write_string_with_format(*row, 0,
            &format!("工作表: {}  |  生成时间: {}", sheet_title, now), &meta_fmt)?;
        *row += 2; // 空一行
        
        Ok(())
    }
    
    /// 创建标准格式
    fn create_formats() -> (Format, Format, Format, Format, Format) {
        let header_fmt = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
            .set_border(FormatBorder::Thin)
            .set_align(FormatAlign::Center);
        
        let subheader_fmt = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0xD9E1F2))
            .set_border(FormatBorder::Thin);
        
        let number_fmt = Format::new()
            .set_num_format("#,##0.00")
            .set_border(FormatBorder::Thin);
        
        let percent_fmt = Format::new()
            .set_num_format("0.00%")
            .set_border(FormatBorder::Thin);
        
        let highlight_fmt = Format::new()
            .set_num_format("#,##0.00")
            .set_background_color(Color::RGB(0xFFFF00))
            .set_bold()
            .set_border(FormatBorder::Thin);
        
        (header_fmt, subheader_fmt, number_fmt, percent_fmt, highlight_fmt)
    }
    
    fn write_balance_sheet_analysis(&self, workbook: &mut Workbook, result: &AnalysisResult, 
                                   stock_code: &str) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("资产负债表分析")?;
        
        let mut row = 0u32;
        self.write_report_header(worksheet, stock_code, "资产负债表分析", &mut row)?;
        
        let (header_fmt, subheader_fmt, number_fmt, _percent_fmt, _highlight_fmt) = Self::create_formats();
        
        // 列标题
        worksheet.write_string_with_format(row, 0, "分类", &header_fmt)?;
        worksheet.write_string_with_format(row, 1, "财务指标", &header_fmt)?;
        
        let years = &result.asset_structure.years;
        for (i, year) in years.iter().enumerate() {
            worksheet.write_string_with_format(row, 2 + i as u16, 
                &format!("{}年", year), &header_fmt)?;
        }
        worksheet.write_string_with_format(row, 2 + years.len() as u16, "单位", &header_fmt)?;
        worksheet.write_string_with_format(row, 3 + years.len() as u16, "说明", &header_fmt)?;
        
        row += 1;
        
        // TODO: 实现详细内容
        worksheet.write_string_with_format(row, 0, "流动资产", &subheader_fmt)?;
        worksheet.write_string(row, 1, "货币资金")?;
        worksheet.write_string(row, 5, "元")?;
        worksheet.write_string(row, 6, "企业可随时支配的现金")?;
        
        Ok(())
    }
    
    fn write_income_analysis(&self, workbook: &mut Workbook, result: &AnalysisResult,
                            stock_code: &str) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("利润表分析")?;
        
        let mut row = 0u32;
        self.write_report_header(worksheet, stock_code, "利润表分析", &mut row)?;
        
        // TODO: 实现详细内容
        
        Ok(())
    }
    
    fn write_cashflow_analysis(&self, workbook: &mut Workbook, result: &AnalysisResult,
                              stock_code: &str) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("现金流分析")?;
        
        let mut row = 0u32;
        self.write_report_header(worksheet, stock_code, "现金流分析", &mut row)?;
        
        // TODO: 实现详细内容
        
        Ok(())
    }
    
    fn write_comprehensive_analysis(&self, workbook: &mut Workbook, result: &AnalysisResult,
                                   stock_code: &str) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("综合实力分析")?;
        
        let mut row = 0u32;
        self.write_report_header(worksheet, stock_code, "综合实力分析", &mut row)?;
        
        // TODO: 实现详细内容
        
        Ok(())
    }
    
    fn write_valuation_analysis(&self, workbook: &mut Workbook, result: &AnalysisResult,
                               stock_code: &str) -> Result<()> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("估值分析")?;
        
        let mut row = 0u32;
        self.write_report_header(worksheet, stock_code, "估值分析", &mut row)?;
        
        // TODO: 实现详细内容
        
        Ok(())
    }
}
