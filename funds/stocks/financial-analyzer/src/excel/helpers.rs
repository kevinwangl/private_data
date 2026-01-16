use crate::domain::*;
use rust_xlsxwriter::*;
use rust_decimal::prelude::ToPrimitive;
use anyhow::Result;

/// 数据获取辅助器
pub struct DataHelper<'a> {
    statements: &'a [FinancialStatement],
}

impl<'a> DataHelper<'a> {
    pub fn new(statements: &'a [FinancialStatement]) -> Self {
        Self { statements }
    }

    pub fn get_balance(&self, year_idx: usize, account: &str) -> f64 {
        self.statements
            .iter()
            .filter(|s| s.report_type == ReportType::BalanceSheet)
            .nth(year_idx)
            .and_then(|s| s.items.get(account))
            .and_then(|v| v.to_f64())
            .unwrap_or(0.0)
    }

    pub fn get_income(&self, year_idx: usize, account: &str) -> f64 {
        self.statements
            .iter()
            .filter(|s| s.report_type == ReportType::IncomeStatement)
            .nth(year_idx)
            .and_then(|s| s.items.get(account))
            .and_then(|v| v.to_f64())
            .unwrap_or(0.0)
    }

    pub fn get_cashflow(&self, year_idx: usize, account: &str) -> f64 {
        self.statements
            .iter()
            .filter(|s| s.report_type == ReportType::CashflowStatement)
            .nth(year_idx)
            .and_then(|s| s.items.get(account))
            .and_then(|v| v.to_f64())
            .unwrap_or(0.0)
    }

    pub fn get_balance_opt(&self, year_idx: usize, account: &str) -> Option<f64> {
        self.statements
            .iter()
            .filter(|s| s.report_type == ReportType::BalanceSheet)
            .nth(year_idx)
            .and_then(|s| s.items.get(account))
            .and_then(|v| v.to_f64())
    }

    pub fn get_income_opt(&self, year_idx: usize, account: &str) -> Option<f64> {
        self.statements
            .iter()
            .filter(|s| s.report_type == ReportType::IncomeStatement)
            .nth(year_idx)
            .and_then(|s| s.items.get(account))
            .and_then(|v| v.to_f64())
    }

    pub fn get_cashflow_opt(&self, year_idx: usize, account: &str) -> Option<f64> {
        self.statements
            .iter()
            .filter(|s| s.report_type == ReportType::CashflowStatement)
            .nth(year_idx)
            .and_then(|s| s.items.get(account))
            .and_then(|v| v.to_f64())
    }
}

/// 格式管理器
pub struct ExcelFormatter {
    pub header: Format,
    pub subheader: Format,
    pub number: Format,
    pub percent: Format,
    pub highlight: Format,
    pub positive: Format,
    pub formula: Format,
    pub highlight_number: Format,
}

impl ExcelFormatter {
    pub fn new() -> Self {
        Self {
            header: Format::new()
                .set_bold()
                .set_font_size(12)
                .set_background_color(Color::RGB(0x4472C4))
                .set_font_color(Color::White)
                .set_align(FormatAlign::Center)
                .set_border(FormatBorder::Thin),
            
            subheader: Format::new()
                .set_bold()
                .set_background_color(Color::RGB(0xD9E1F2))
                .set_border(FormatBorder::Thin),
            
            number: Format::new()
                .set_num_format("#,##0.00")
                .set_border(FormatBorder::Thin),
            
            percent: Format::new()
                .set_num_format("0.00%")
                .set_border(FormatBorder::Thin),
            
            highlight: Format::new()
                .set_num_format("0.00%")
                .set_background_color(Color::RGB(0xFFFF00))
                .set_bold()
                .set_border(FormatBorder::Thin),
            
            positive: Format::new()
                .set_num_format("0.00%")
                .set_font_color(Color::RGB(0x00B050))
                .set_bold()
                .set_border(FormatBorder::Thin),
            
            formula: Format::new()
                .set_num_format("#,##0.00")
                .set_background_color(Color::RGB(0xF2F2F2))
                .set_border(FormatBorder::Thin),
            
            highlight_number: Format::new()
                .set_num_format("#,##0.00")
                .set_background_color(Color::RGB(0xFFFF00))
                .set_bold()
                .set_border(FormatBorder::Thin),
        }
    }
}

/// Sheet写入辅助器
pub struct SheetWriter<'a> {
    worksheet: &'a mut Worksheet,
    fmt: &'a ExcelFormatter,
}

impl<'a> SheetWriter<'a> {
    pub fn new(worksheet: &'a mut Worksheet, fmt: &'a ExcelFormatter) -> Self {
        Self { worksheet, fmt }
    }

    /// 写入年份表头
    pub fn write_year_headers(&mut self, row: u32, start_col: u16, years: &[i32]) -> Result<()> {
        for (i, year) in years.iter().enumerate() {
            self.worksheet.write_string_with_format(
                row, 
                start_col + i as u16, 
                year.to_string(), 
                &self.fmt.header
            )?;
        }
        Ok(())
    }

    /// 写入科目行
    pub fn write_account_row(&mut self, row: u32, label_col: u16, data_start_col: u16, 
                             label: &str, values: &[f64]) -> Result<()> {
        self.worksheet.write_string_with_format(row, label_col, label, &self.fmt.subheader)?;
        for (i, &value) in values.iter().enumerate() {
            if value != 0.0 {
                self.worksheet.write_number_with_format(
                    row, 
                    data_start_col + i as u16, 
                    value, 
                    &self.fmt.number
                )?;
            }
        }
        Ok(())
    }

    /// 写入百分比行
    pub fn write_percent_row(&mut self, row: u32, label_col: u16, data_start_col: u16,
                            label: &str, values: &[f64], highlight: bool) -> Result<()> {
        self.worksheet.write_string_with_format(row, label_col, label, &self.fmt.subheader)?;
        let fmt = if highlight { &self.fmt.highlight } else { &self.fmt.percent };
        for (i, &value) in values.iter().enumerate() {
            self.worksheet.write_number_with_format(
                row, 
                data_start_col + i as u16, 
                value, 
                fmt
            )?;
        }
        Ok(())
    }

    /// 设置标准列宽
    pub fn set_standard_columns(&mut self) -> Result<()> {
        self.worksheet.set_column_width(0, 20)?;
        self.worksheet.set_column_width(1, 35)?;
        for col in 2..15 {
            self.worksheet.set_column_width(col, 30)?;
        }
        Ok(())
    }

    /// 设置行高范围
    pub fn set_row_heights(&mut self, start_row: u32, end_row: u32, height: f64) -> Result<()> {
        for row in start_row..=end_row {
            self.worksheet.set_row_height(row, height)?;
        }
        Ok(())
    }
}
