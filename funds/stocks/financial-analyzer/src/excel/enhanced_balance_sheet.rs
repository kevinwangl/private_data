/// 优化版资产负债表分析Sheet
/// 合并Sheet1和Sheet2的内容，添加报告头和说明列

use crate::domain::*;
use crate::excel::{DataHelper, IndicatorDescriptions, SheetBuilder};
use anyhow::Result;
use rust_xlsxwriter::*;

pub fn write_enhanced_balance_sheet(
    workbook: &mut Workbook,
    result: &AnalysisResult,
    stock_code: &str,
) -> Result<()> {
    let mut worksheet = workbook.add_worksheet();
    worksheet.set_name("资产负债表分析(优化版)")?;
    
    let builder = SheetBuilder::new(stock_code);
    let descriptions = IndicatorDescriptions::new();
    let data = DataHelper::new(&result.statements);
    
    // 写入报告头
    let mut row = builder.write_header(&mut worksheet, "资产负债表分析")?;
    
    // 创建格式
    let (header_fmt, subheader_fmt, number_fmt, percent_fmt, _, _, _, _) = create_formats();
    
    let years = &result.asset_structure.years;
    let num_years = years.len().min(3);
    
    // ========== 第一部分：资产 ==========
    worksheet.write_string_with_format(row, 0, "【资产】", &header_fmt)?;
    worksheet.merge_range(row, 0, row, 4 + num_years as u16, "【资产】", &header_fmt)?;
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
    
    // 资产数据 - 使用公式引用
    let asset_items = vec![
        ("流动资产", "货币资金", "元", "货币资金", 4),
        ("流动资产", "应收账款", "元", "应收账款", 7),
        ("流动资产", "存货", "元", "存货", 9),
        ("非流动资产", "固定资产", "元", "固定资产", 5),
        ("非流动资产", "无形资产", "元", "无形资产", 10),
        ("合计", "资产总计", "元", "资产总计", 21),
    ];
    
    for (category, item, unit, desc_key, source_row) in asset_items {
        worksheet.write_string(row, 0, category)?;
        worksheet.write_string(row, 1, item)?;
        
        for i in 0..num_years {
            let col_letter = match i {
                0 => "C",
                1 => "D",
                2 => "E",
                _ => "F",
            };
            let formula = format!("='(经营性&金融性)资产&负债结构分析'!{}{}", col_letter, source_row);
            worksheet.write_formula_with_format(row, 2 + i as u16, formula.as_str(), &number_fmt)?;
        }
        
        worksheet.write_string(row, 2 + num_years as u16, unit)?;
        worksheet.write_string(row, 3 + num_years as u16, &descriptions.get(desc_key))?;
        row += 1;
    }
    
    row += 1;
    
    // ========== 第二部分：负债 ==========
    worksheet.write_string_with_format(row, 0, "【负债】", &header_fmt)?;
    worksheet.merge_range(row, 0, row, 4 + num_years as u16, "【负债】", &header_fmt)?;
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
    
    // 负债数据 - 使用公式引用
    let liability_items = vec![
        ("流动负债", "应付账款", "元", "应付账款", 5),
        ("流动负债", "短期借款", "元", "短期借款", 18),
        ("非流动负债", "长期借款", "元", "长期借款", 17),
        ("非流动负债", "应付债券", "元", "应付债券", 14),
        ("合计", "负债合计", "元", "负债合计", 20),
        ("权益", "所有者权益合计", "元", "所有者权益合计", 21),
    ];
    
    for (category, item, unit, desc_key, source_row) in liability_items {
        worksheet.write_string(row, 0, category)?;
        worksheet.write_string(row, 1, item)?;
        
        for i in 0..num_years {
            let col_letter = match i {
                0 => "I",
                1 => "J",
                2 => "K",
                _ => "L",
            };
            let formula = format!("='(经营性&金融性)资产&负债结构分析'!{}{}", col_letter, source_row);
            worksheet.write_formula_with_format(row, 2 + i as u16, formula.as_str(), &number_fmt)?;
        }
        
        worksheet.write_string(row, 2 + num_years as u16, unit)?;
        worksheet.write_string(row, 3 + num_years as u16, &descriptions.get(desc_key))?;
        row += 1;
    }
    
    row += 1;
    
    // ========== 第三部分：结构分析 ==========
    worksheet.write_string_with_format(row, 0, "【结构分析】", &header_fmt)?;
    worksheet.merge_range(row, 0, row, 4 + num_years as u16, "【结构分析】", &header_fmt)?;
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
    
    // 结构比率 - 直接引用已计算的比率
    worksheet.write_string(row, 0, "资产结构")?;
    worksheet.write_string(row, 1, "经营性资产占比")?;
    for i in 0..num_years {
        let col_letter = match i {
            0 => "C",
            1 => "D",
            2 => "E",
            _ => "F",
        };
        let formula = format!("='(经营性&金融性)资产&负债结构分析'!{}{}", col_letter, 22);
        worksheet.write_formula_with_format(row, 2 + i as u16, formula.as_str(), &percent_fmt)?;
    }
    worksheet.write_string(row, 2 + num_years as u16, "%")?;
    worksheet.write_string(row, 3 + num_years as u16, &descriptions.get("经营性资产占比"))?;
    row += 1;
    
    worksheet.write_string(row, 0, "资产结构")?;
    worksheet.write_string(row, 1, "金融性资产占比")?;
    for i in 0..num_years {
        let col_letter = match i {
            0 => "C",
            1 => "D",
            2 => "E",
            _ => "F",
        };
        let formula = format!("='(经营性&金融性)资产&负债结构分析'!{}{}", col_letter, 23);
        worksheet.write_formula_with_format(row, 2 + i as u16, formula.as_str(), &percent_fmt)?;
    }
    worksheet.write_string(row, 2 + num_years as u16, "%")?;
    worksheet.write_string(row, 3 + num_years as u16, &descriptions.get("金融性资产占比"))?;
    row += 1;
    
    worksheet.write_string(row, 0, "负债结构")?;
    worksheet.write_string(row, 1, "经营性负债占比")?;
    for i in 0..num_years {
        let col_letter = match i {
            0 => "I",
            1 => "J",
            2 => "K",
            _ => "L",
        };
        let formula = format!("='(经营性&金融性)资产&负债结构分析'!{}{}", col_letter, 22);
        worksheet.write_formula_with_format(row, 2 + i as u16, formula.as_str(), &percent_fmt)?;
    }
    worksheet.write_string(row, 2 + num_years as u16, "%")?;
    worksheet.write_string(row, 3 + num_years as u16, &descriptions.get("经营性负债占比"))?;
    row += 1;
    
    worksheet.write_string(row, 0, "负债结构")?;
    worksheet.write_string(row, 1, "金融性负债占比")?;
    for i in 0..num_years {
        let col_letter = match i {
            0 => "I",
            1 => "J",
            2 => "K",
            _ => "L",
        };
        let formula = format!("='(经营性&金融性)资产&负债结构分析'!{}{}", col_letter, 23);
        worksheet.write_formula_with_format(row, 2 + i as u16, formula.as_str(), &percent_fmt)?;
    }
    worksheet.write_string(row, 2 + num_years as u16, "%")?;
    worksheet.write_string(row, 3 + num_years as u16, &descriptions.get("金融性负债占比"))?;
    
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
