//! Excel格式和样式定义
//! 
//! 提供统一的格式创建函数，避免代码重复

use rust_xlsxwriter::*;

/// Excel格式集合
pub struct ExcelFormats {
    pub title: Format,
    pub header: Format,
    pub subheader: Format,
    pub number: Format,
    pub percent: Format,
    pub highlight: Format,
    pub normal: Format,
}

impl ExcelFormats {
    /// 创建标准格式集合
    pub fn new() -> Self {
        Self {
            title: Self::create_title_format(),
            header: Self::create_header_format(),
            subheader: Self::create_subheader_format(),
            number: Self::create_number_format(),
            percent: Self::create_percent_format(),
            highlight: Self::create_highlight_format(),
            normal: Self::create_normal_format(),
        }
    }
    
    fn create_title_format() -> Format {
        Format::new()
            .set_bold()
            .set_font_size(14)
            .set_align(FormatAlign::Center)
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
    }
    
    fn create_header_format() -> Format {
        Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0x4472C4))
            .set_font_color(Color::White)
            .set_border(FormatBorder::Thin)
            .set_align(FormatAlign::Center)
    }
    
    fn create_subheader_format() -> Format {
        Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0xD9E1F2))
            .set_border(FormatBorder::Thin)
    }
    
    fn create_number_format() -> Format {
        Format::new()
            .set_num_format("#,##0.00")
            .set_border(FormatBorder::Thin)
    }
    
    fn create_percent_format() -> Format {
        Format::new()
            .set_num_format("0.00%")
            .set_border(FormatBorder::Thin)
    }
    
    fn create_highlight_format() -> Format {
        Format::new()
            .set_num_format("0.00%")
            .set_background_color(Color::RGB(0xFFFF00))
            .set_border(FormatBorder::Thin)
    }
    
    fn create_normal_format() -> Format {
        Format::new()
            .set_border(FormatBorder::Thin)
    }
}

impl Default for ExcelFormats {
    fn default() -> Self {
        Self::new()
    }
}

/// 设置标准列宽
pub fn set_standard_column_widths(worksheet: &mut Worksheet) -> Result<(), XlsxError> {
    worksheet.set_column_width(0, 25)?;
    worksheet.set_column_width(1, 20)?;
    worksheet.set_column_width(2, 15)?;
    worksheet.set_column_width(3, 15)?;
    worksheet.set_column_width(4, 15)?;
    Ok(())
}

/// 设置标准行高
pub fn set_standard_row_heights(worksheet: &mut Worksheet, start_row: u32, end_row: u32) -> Result<(), XlsxError> {
    for row in start_row..=end_row {
        worksheet.set_row_height(row, 20)?;
    }
    Ok(())
}
