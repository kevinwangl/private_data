/// 优化版利润&现金流分析Sheet
/// 包含：报告头 + 说明列 + 杠杆分析 + 清晰分区

use crate::domain::*;
use crate::excel::{DataHelper, IndicatorDescriptions, SheetBuilder};
use anyhow::Result;
use rust_xlsxwriter::*;

pub fn write_enhanced_profit_cashflow_sheet(
    workbook: &mut Workbook,
    result: &AnalysisResult,
    stock_code: &str,
) -> Result<()> {
    let mut worksheet = workbook.add_worksheet();
    worksheet.set_name("利润&现金流分析(优化版)")?;
    
    let builder = SheetBuilder::new(stock_code);
    let descriptions = IndicatorDescriptions::new();
    let data = DataHelper::new(&result.statements);
    
    // 写入报告头
    let mut row = builder.write_header(&mut worksheet, "利润&现金流分析")?;
    
    // 创建格式
    let (header_fmt, subheader_fmt, number_fmt, percent_fmt, _, _, _, _) = create_formats();
    
    let years = &result.asset_structure.years;
    let num_years = years.len().min(3);
    
    // ========== 第一部分：利润表 ==========
    worksheet.write_string_with_format(row, 0, "【利润表】", &header_fmt)?;
    worksheet.merge_range(row, 0, row, 4 + num_years as u16, "【利润表】", &header_fmt)?;
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
    
    // 利润表数据
    let income_items = vec![
        ("收入", "营业总收入", "元", "营业总收入"),
        ("成本", "营业成本", "元", "营业成本"),
        ("成本", "营业总成本", "元", "营业总成本"),
        ("费用", "税金及附加", "元", "税金及附加"),
        ("费用", "销售费用", "元", "销售费用"),
        ("费用", "管理费用", "元", "管理费用"),
        ("费用", "研发费用", "元", "研发费用"),
        ("费用", "财务费用", "元", "财务费用"),
        ("利润", "营业利润", "元", "营业利润"),
        ("利润", "净利润", "元", "净利润"),
        ("利润", "持续经营净利润", "元", "持续经营净利润"),
    ];
    
    for (category, item, unit, desc_key) in income_items {
        worksheet.write_string(row, 0, category)?;
        worksheet.write_string(row, 1, item)?;
        
        for i in 0..num_years {
            if let Some(value) = data.get_income_opt(i, item) {
                worksheet.write_number_with_format(row, 2 + i as u16, value, &number_fmt)?;
            }
        }
        
        worksheet.write_string(row, 2 + num_years as u16, unit)?;
        worksheet.write_string(row, 3 + num_years as u16, &descriptions.get(desc_key))?;
        row += 1;
    }
    
    row += 1;
    
    // ========== 第二部分：财务比率 ==========
    worksheet.write_string_with_format(row, 0, "【财务比率】", &header_fmt)?;
    worksheet.merge_range(row, 0, row, 4 + num_years as u16, "【财务比率】", &header_fmt)?;
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
    
    // 财务比率数据
    let profit_analysis = &result.profit_analysis;
    let ratios = vec![
        ("盈利能力", "毛利率", &profit_analysis.gross_margin, "%", "毛利率"),
        ("盈利能力", "核心利润率", &profit_analysis.core_profit_margin, "%", "核心利润率"),
        ("盈利能力", "净利润率", &profit_analysis.net_profit_margin, "%", "净利润率"),
    ];
    
    use rust_decimal::prelude::ToPrimitive;
    for (category, name, values, unit, desc_key) in ratios {
        worksheet.write_string(row, 0, category)?;
        worksheet.write_string(row, 1, name)?;
        
        for (i, value) in values.iter().take(num_years).enumerate() {
            let val = value.to_f64().unwrap_or(0.0);
            worksheet.write_number_with_format(row, 2 + i as u16, val, &percent_fmt)?;
        }
        
        worksheet.write_string(row, 2 + num_years as u16, unit)?;
        worksheet.write_string(row, 3 + num_years as u16, &descriptions.get(desc_key))?;
        row += 1;
    }
    
    row += 1;
    
    // ========== 第三部分：杠杆分析 ==========
    if let Some(leverage) = &result.leverage_analysis {
        worksheet.write_string_with_format(row, 0, "【杠杆分析】", &header_fmt)?;
        worksheet.merge_range(row, 0, row, 4 + num_years as u16, "【杠杆分析】", &header_fmt)?;
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
        
        // 杠杆数据
        let leverage_items = vec![
            ("杠杆", "经营杠杆(DOL)", &leverage.operating_leverage, "倍", "经营杠杆DOL"),
            ("杠杆", "财务杠杆(DFL)", &leverage.financial_leverage, "倍", "财务杠杆DFL"),
            ("杠杆", "总杠杆(DTL)", &leverage.total_leverage, "倍", "总杠杆DTL"),
        ];
        
        for (category, name, values, unit, desc_key) in leverage_items {
            worksheet.write_string(row, 0, category)?;
            worksheet.write_string(row, 1, name)?;
            
            for (i, value) in values.iter().take(num_years).enumerate() {
                let val = value.to_f64().unwrap_or(0.0);
                if val.abs() > 0.01 {
                    worksheet.write_number_with_format(row, 2 + i as u16, val, &number_fmt)?;
                } else {
                    worksheet.write_string(row, 2 + i as u16, "-")?;
                }
            }
            
            worksheet.write_string(row, 2 + num_years as u16, unit)?;
            worksheet.write_string(row, 3 + num_years as u16, &descriptions.get(desc_key))?;
            row += 1;
        }
        
        row += 1;
    }
    
    // ========== 第四部分：现金流量表 ==========
    worksheet.write_string_with_format(row, 0, "【现金流量表】", &header_fmt)?;
    worksheet.merge_range(row, 0, row, 4 + num_years as u16, "【现金流量表】", &header_fmt)?;
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
    
    // 现金流数据
    let cashflow_items = vec![
        ("经营活动", "经营活动产生的现金流量净额", "元", "经营活动产生的现金流量净额"),
        ("投资活动", "投资活动产生的现金流量净额", "元", "投资活动产生的现金流量净额"),
        ("筹资活动", "筹资活动产生的现金流量净额", "元", "筹资活动产生的现金流量净额"),
        ("资本支出", "购建固定资产、无形资产和其他长期资产支付的现金", "元", "购建固定资产、无形资产和其他长期资产支付的现金"),
    ];
    
    for (category, item, unit, desc_key) in cashflow_items {
        worksheet.write_string(row, 0, category)?;
        worksheet.write_string(row, 1, item)?;
        
        for i in 0..num_years {
            if let Some(value) = data.get_cashflow_opt(i, item) {
                worksheet.write_number_with_format(row, 2 + i as u16, value, &number_fmt)?;
            }
        }
        
        worksheet.write_string(row, 2 + num_years as u16, unit)?;
        worksheet.write_string(row, 3 + num_years as u16, &descriptions.get(desc_key))?;
        row += 1;
    }
    
    // 设置列宽
    worksheet.set_column_width(0, 12)?;
    worksheet.set_column_width(1, 35)?;
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
