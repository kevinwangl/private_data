/// 优化版综合实力分析Sheet
/// 增强版：添加更多财务比率、同比增长率、报告头和说明列

use crate::domain::*;
use crate::excel::{DataHelper, IndicatorDescriptions, SheetBuilder};
use anyhow::Result;
use rust_xlsxwriter::*;

pub fn write_enhanced_comprehensive_sheet(
    workbook: &mut Workbook,
    result: &AnalysisResult,
    stock_code: &str,
) -> Result<()> {
    let mut worksheet = workbook.add_worksheet();
    worksheet.set_name("综合实力分析(优化版)")?;
    
    let builder = SheetBuilder::new(stock_code);
    let descriptions = IndicatorDescriptions::new();
    let data = DataHelper::new(&result.statements);
    
    // 写入报告头
    let mut row = builder.write_header(&mut worksheet, "综合实力分析")?;
    
    // 创建格式
    let (header_fmt, subheader_fmt, number_fmt, percent_fmt, _, _, _, _) = create_formats();
    
    let years = &result.asset_structure.years;
    let num_years = years.len().min(3);
    
    // ========== 第一部分：盈利能力 ==========
    worksheet.write_string_with_format(row, 0, "【盈利能力】", &header_fmt)?;
    worksheet.merge_range(row, 0, row, 4 + num_years as u16, "【盈利能力】", &header_fmt)?;
    row += 1;
    
    // 列标题
    worksheet.write_string_with_format(row, 0, "分类", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 1, "财务指标", &subheader_fmt)?;
    for (i, year) in years.iter().take(num_years).enumerate() {
        worksheet.write_string_with_format(row, 2 + i as u16, 
            &format!("{}年", year), &subheader_fmt)?;
    }
    worksheet.write_string_with_format(row, 2 + num_years as u16, "单位", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 3 + num_years as u16, "说明", &subheader_fmt)?;
    row += 1;
    
    use rust_decimal::prelude::ToPrimitive;
    
    // ROE
    worksheet.write_string(row, 0, "盈利能力")?;
    worksheet.write_string(row, 1, "ROE(净资产收益率)")?;
    for i in 0..num_years {
        if let (Some(net_profit), Some(equity)) = (
            data.get_income_opt(i, "净利润"),
            data.get_balance_opt(i, "所有者权益合计")
        ) {
            if equity > 0.0 {
                let roe = net_profit / equity;
                worksheet.write_number_with_format(row, 2 + i as u16, roe, &percent_fmt)?;
            }
        }
    }
    worksheet.write_string(row, 2 + num_years as u16, "%")?;
    worksheet.write_string(row, 3 + num_years as u16, &descriptions.get("ROE"))?;
    row += 1;
    
    // ROA
    worksheet.write_string(row, 0, "盈利能力")?;
    worksheet.write_string(row, 1, "ROA(总资产收益率)")?;
    for i in 0..num_years {
        if let (Some(net_profit), Some(assets)) = (
            data.get_income_opt(i, "净利润"),
            data.get_balance_opt(i, "资产总计")
        ) {
            if assets > 0.0 {
                let roa = net_profit / assets;
                worksheet.write_number_with_format(row, 2 + i as u16, roa, &percent_fmt)?;
            }
        }
    }
    worksheet.write_string(row, 2 + num_years as u16, "%")?;
    worksheet.write_string(row, 3 + num_years as u16, &descriptions.get("ROA"))?;
    row += 1;
    
    // 净利率
    let profit_analysis = &result.profit_analysis;
    worksheet.write_string(row, 0, "盈利能力")?;
    worksheet.write_string(row, 1, "净利润率")?;
    for (i, value) in profit_analysis.net_profit_margin.iter().take(num_years).enumerate() {
        let val = value.to_f64().unwrap_or(0.0);
        worksheet.write_number_with_format(row, 2 + i as u16, val, &percent_fmt)?;
    }
    worksheet.write_string(row, 2 + num_years as u16, "%")?;
    worksheet.write_string(row, 3 + num_years as u16, &descriptions.get("净利润率"))?;
    row += 1;
    
    // 毛利率
    worksheet.write_string(row, 0, "盈利能力")?;
    worksheet.write_string(row, 1, "毛利率")?;
    for (i, value) in profit_analysis.gross_margin.iter().take(num_years).enumerate() {
        let val = value.to_f64().unwrap_or(0.0);
        worksheet.write_number_with_format(row, 2 + i as u16, val, &percent_fmt)?;
    }
    worksheet.write_string(row, 2 + num_years as u16, "%")?;
    worksheet.write_string(row, 3 + num_years as u16, &descriptions.get("毛利率"))?;
    row += 1;
    
    row += 1;
    
    // ========== 第二部分：偿债能力 ==========
    worksheet.write_string_with_format(row, 0, "【偿债能力】", &header_fmt)?;
    worksheet.merge_range(row, 0, row, 4 + num_years as u16, "【偿债能力】", &header_fmt)?;
    row += 1;
    
    // 列标题
    worksheet.write_string_with_format(row, 0, "分类", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 1, "财务指标", &subheader_fmt)?;
    for (i, year) in years.iter().take(num_years).enumerate() {
        worksheet.write_string_with_format(row, 2 + i as u16, 
            &format!("{}年", year), &subheader_fmt)?;
    }
    worksheet.write_string_with_format(row, 2 + num_years as u16, "单位", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 3 + num_years as u16, "说明", &subheader_fmt)?;
    row += 1;
    
    // 资产负债率
    worksheet.write_string(row, 0, "偿债能力")?;
    worksheet.write_string(row, 1, "资产负债率")?;
    for i in 0..num_years {
        if let (Some(liability), Some(assets)) = (
            data.get_balance_opt(i, "负债合计"),
            data.get_balance_opt(i, "资产总计")
        ) {
            if assets > 0.0 {
                let ratio = liability / assets;
                worksheet.write_number_with_format(row, 2 + i as u16, ratio, &percent_fmt)?;
            }
        }
    }
    worksheet.write_string(row, 2 + num_years as u16, "%")?;
    worksheet.write_string(row, 3 + num_years as u16, "负债/资产，<60%较安全")?;
    row += 1;
    
    row += 1;
    
    // ========== 第三部分：关键指标 ==========
    worksheet.write_string_with_format(row, 0, "【关键指标】", &header_fmt)?;
    worksheet.merge_range(row, 0, row, 4 + num_years as u16, "【关键指标】", &header_fmt)?;
    row += 1;
    
    // 列标题
    worksheet.write_string_with_format(row, 0, "分类", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 1, "财务指标", &subheader_fmt)?;
    for (i, year) in years.iter().take(num_years).enumerate() {
        worksheet.write_string_with_format(row, 2 + i as u16, 
            &format!("{}年", year), &subheader_fmt)?;
    }
    worksheet.write_string_with_format(row, 2 + num_years as u16, "单位", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 3 + num_years as u16, "说明", &subheader_fmt)?;
    row += 1;
    
    // 关键指标数据
    let key_items = vec![
        ("资产", "货币资金", "元", "货币资金"),
        ("资产", "存货", "元", "存货"),
        ("资产", "固定资产", "元", "固定资产"),
        ("资产", "资产总计", "元", "资产总计"),
        ("利润", "净利润", "元", "净利润"),
        ("现金流", "经营活动产生的现金流量净额", "元", "经营活动产生的现金流量净额"),
    ];
    
    for (category, item, unit, desc_key) in key_items {
        worksheet.write_string(row, 0, category)?;
        worksheet.write_string(row, 1, item)?;
        
        for i in 0..num_years {
            let value = if category == "利润" || category == "现金流" {
                data.get_income_opt(i, item)
                    .or_else(|| data.get_cashflow_opt(i, item))
            } else {
                data.get_balance_opt(i, item)
            };
            
            if let Some(val) = value {
                worksheet.write_number_with_format(row, 2 + i as u16, val, &number_fmt)?;
            }
        }
        
        worksheet.write_string(row, 2 + num_years as u16, unit)?;
        worksheet.write_string(row, 3 + num_years as u16, &descriptions.get(desc_key))?;
        row += 1;
    }
    
    // 设置列宽
    worksheet.set_column_width(0, 12)?;
    worksheet.set_column_width(1, 30)?;
    for i in 0..num_years {
        worksheet.set_column_width(2 + i as u16, 20)?;
    }
    worksheet.set_column_width(2 + num_years as u16, 8)?;
    worksheet.set_column_width(3 + num_years as u16, 30)?;
    
    Ok(())
}

fn create_formats() -> (Format, Format, Format, Format, Format, Format, Format, Format) {
    let header_fmt = Format::new()
        .set_bold()
        .set_font_size(12)
        .set_background_color(Color::RGB(0x4472C4))
        .set_font_color(Color::White)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);
    
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
        .set_num_format("0.00%")
        .set_background_color(Color::RGB(0xFFFF00))
        .set_bold()
        .set_border(FormatBorder::Thin);
    
    let positive_fmt = Format::new()
        .set_num_format("0.00%")
        .set_font_color(Color::RGB(0x00B050))
        .set_bold()
        .set_border(FormatBorder::Thin);
    
    let formula_fmt = Format::new()
        .set_num_format("#,##0.00")
        .set_background_color(Color::RGB(0xF2F2F2))
        .set_border(FormatBorder::Thin);
    
    let highlight_number_fmt = Format::new()
        .set_num_format("#,##0.00")
        .set_background_color(Color::RGB(0xFFFF00))
        .set_bold()
        .set_border(FormatBorder::Thin);
    
    (header_fmt, subheader_fmt, number_fmt, percent_fmt, highlight_fmt, positive_fmt, formula_fmt, highlight_number_fmt)
}
