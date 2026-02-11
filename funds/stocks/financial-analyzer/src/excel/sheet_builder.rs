/// Excel报告增强版生成器 - 辅助方法
/// 提供通用的sheet创建和格式化功能

use rust_xlsxwriter::*;
use anyhow::Result;

pub struct SheetBuilder {
    stock_code: String,
}

impl SheetBuilder {
    pub fn new(stock_code: &str) -> Self {
        Self {
            stock_code: stock_code.to_string(),
        }
    }
    
    /// 写入报告头并返回下一行行号
    pub fn write_header(&self, worksheet: &mut Worksheet, sheet_title: &str) -> Result<u32> {
        let title_fmt = Format::new()
            .set_bold()
            .set_font_size(14)
            .set_font_color(Color::RGB(0x1F4E78));
            
        let meta_fmt = Format::new()
            .set_font_size(9)
            .set_font_color(Color::RGB(0x7F7F7F));
        
        let mut row = 0u32;
        
        // 标题
        worksheet.write_string_with_format(row, 0, 
            &format!("【{}】财务分析报告", self.stock_code), &title_fmt)?;
        row += 1;
        
        // 元数据
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M");
        worksheet.write_string_with_format(row, 0,
            &format!("工作表: {}  |  生成时间: {}", sheet_title, now), &meta_fmt)?;
        row += 1;
        
        // 空一行
        row += 1;
        
        Ok(row)
    }
    
    /// 写入列标题（带说明列）
    pub fn write_column_headers(&self, worksheet: &mut Worksheet, row: u32, 
                               years: &[i32], header_fmt: &Format) -> Result<()> {
        worksheet.write_string_with_format(row, 0, "分类", header_fmt)?;
        worksheet.write_string_with_format(row, 1, "财务指标", header_fmt)?;
        
        for (i, year) in years.iter().enumerate() {
            worksheet.write_string_with_format(row, 2 + i as u16, 
                &format!("{}年", year), header_fmt)?;
        }
        
        let unit_col = 2 + years.len() as u16;
        let desc_col = unit_col + 1;
        
        worksheet.write_string_with_format(row, unit_col, "单位", header_fmt)?;
        worksheet.write_string_with_format(row, desc_col, "说明", header_fmt)?;
        
        Ok(())
    }
    
    /// 设置标准列宽（包含说明列）
    pub fn set_column_widths(&self, worksheet: &mut Worksheet, num_years: usize) -> Result<()> {
        worksheet.set_column_width(0, 15)?;  // 分类
        worksheet.set_column_width(1, 30)?;  // 财务指标
        
        // 年份数据列
        for i in 0..num_years {
            worksheet.set_column_width(2 + i as u16, 20)?;
        }
        
        let unit_col = 2 + num_years as u16;
        let desc_col = unit_col + 1;
        
        worksheet.set_column_width(unit_col, 10)?;  // 单位
        worksheet.set_column_width(desc_col, 35)?;  // 说明
        
        Ok(())
    }
}
