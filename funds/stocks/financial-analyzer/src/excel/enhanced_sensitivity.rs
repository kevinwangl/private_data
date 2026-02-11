/// 优化版敏感性分析Sheet生成器
/// 展示完整优化后的效果：报告头 + 说明列 + 清晰布局

use crate::domain::*;
use crate::excel::{DataHelper, IndicatorDescriptions, SheetBuilder};
use anyhow::Result;
use rust_xlsxwriter::*;

pub fn write_enhanced_sensitivity_sheet(
    workbook: &mut Workbook,
    result: &AnalysisResult,
    stock_code: &str,
) -> Result<()> {
    let mut worksheet = workbook.add_worksheet();
    worksheet.set_name("敏感性分析(优化版)")?;
    
    let builder = SheetBuilder::new(stock_code);
    let descriptions = IndicatorDescriptions::new();
    
    // 写入报告头
    let mut row = builder.write_header(&mut worksheet, "敏感性分析")?;
    
    // 创建格式
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
    
    // 获取数据
    let data = DataHelper::new(&result.statements);
    let sensitivity = result.sensitivity.as_ref().unwrap();
    
    // 参数部分标题
    worksheet.write_string_with_format(row, 0, "可编辑参数", &header_fmt)?;
    worksheet.merge_range(row, 0, row, 3, "可编辑参数", &header_fmt)?;
    row += 1;
    
    // 参数列标题
    worksheet.write_string_with_format(row, 0, "参数名称", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 1, "参数值", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 2, "单位", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 3, "说明", &subheader_fmt)?;
    row += 1;
    
    // 参数数据（记录起始行用于公式引用）
    let param_start_row = row;
    let params = vec![
        ("折现率(r)", sensitivity.params.discount_rate, "%", descriptions.get("折现率")),
        ("永续增长率(g)", sensitivity.params.perpetual_growth_rate, "%", descriptions.get("永续增长率")),
        ("FCF增长率(G)", sensitivity.params.fcf_growth_rate, "%", descriptions.get("FCF增长率")),
        ("净利润增长率", sensitivity.params.net_profit_growth_rate, "%", descriptions.get("净利润增长率")),
        ("无风险收益率(低估)", sensitivity.params.low_risk_free_rate, "%", "用于计算低估PE倍数".to_string()),
        ("无风险收益率(高估)", sensitivity.params.high_risk_free_rate, "%", "用于计算高估PE倍数".to_string()),
    ];
    
    for (name, value, unit, desc) in params {
        worksheet.write_string(row, 0, name)?;
        worksheet.write_number_with_format(row, 1, value, &percent_fmt)?;
        worksheet.write_string(row, 2, unit)?;
        worksheet.write_string(row, 3, &desc)?;
        row += 1;
    }
    
    row += 1;
    
    // 基础数据部分（记录起始行用于公式引用）
    worksheet.write_string_with_format(row, 0, "基础数据（最近一年）", &header_fmt)?;
    worksheet.merge_range(row, 0, row, 3, "基础数据（最近一年）", &header_fmt)?;
    row += 1;
    
    worksheet.write_string_with_format(row, 0, "数据项", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 1, "数值", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 2, "单位", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 3, "说明", &subheader_fmt)?;
    row += 1;
    
    let base_data_start_row = row;
    let fcf = data.get_cashflow_opt(0, "经营活动产生的现金流量净额").unwrap_or(0.0)
        - data.get_cashflow_opt(0, "购建固定资产、无形资产和其他长期资产支付的现金").unwrap_or(0.0);
    let net_profit = data.get_income_opt(0, "净利润").unwrap_or(0.0);
    let total_shares = data.get_balance_opt(0, "股本").unwrap_or(100_000_000.0);
    
    worksheet.write_string(row, 0, "自由现金流(FCF)")?;
    worksheet.write_number_with_format(row, 1, fcf, &number_fmt)?;
    worksheet.write_string(row, 2, "元")?;
    worksheet.write_string(row, 3, &descriptions.get("自由现金流"))?;
    row += 1;
    
    worksheet.write_string(row, 0, "净利润")?;
    worksheet.write_number_with_format(row, 1, net_profit, &number_fmt)?;
    worksheet.write_string(row, 2, "元")?;
    worksheet.write_string(row, 3, &descriptions.get("净利润"))?;
    row += 1;
    
    worksheet.write_string(row, 0, "总股本")?;
    worksheet.write_number_with_format(row, 1, total_shares, &number_fmt)?;
    worksheet.write_string(row, 2, "股")?;
    worksheet.write_string(row, 3, "公司发行的股票总数")?;
    row += 1;
    
    row += 1;
    
    // 估值结果部分
    worksheet.write_string_with_format(row, 0, "估值结果（基于上述参数计算）", &header_fmt)?;
    worksheet.merge_range(row, 0, row, 3, "估值结果（基于上述参数计算）", &header_fmt)?;
    row += 1;
    
    worksheet.write_string_with_format(row, 0, "估值方法", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 1, "估值结果", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 2, "单位", &subheader_fmt)?;
    worksheet.write_string_with_format(row, 3, "说明", &subheader_fmt)?;
    row += 1;
    
    // 计算单元格引用（Excel行号从1开始）
    let r_cell = format!("B{}", param_start_row + 1);      // 折现率
    let g_cell = format!("B{}", param_start_row + 2);      // 永续增长率
    let fcf_g_cell = format!("B{}", param_start_row + 3);  // FCF增长率
    let np_g_cell = format!("B{}", param_start_row + 4);   // 净利润增长率
    let low_rf_cell = format!("B{}", param_start_row + 5); // 无风险收益率(低估)
    let high_rf_cell = format!("B{}", param_start_row + 6);// 无风险收益率(高估)
    
    let fcf_cell = format!("B{}", base_data_start_row + 1);    // FCF
    let np_cell = format!("B{}", base_data_start_row + 2);     // 净利润
    let shares_cell = format!("B{}", base_data_start_row + 3); // 总股本
    
    // DCF企业价值公式: FCF * (1+G) / (r-g)
    let dcf_value_row = row;
    worksheet.write_string(row, 0, "DCF企业价值")?;
    let dcf_formula = format!("={}*(1+{})/(MAX({}-{},0.0001))", fcf_cell, fcf_g_cell, r_cell, g_cell);
    worksheet.write_formula_with_format(row, 1, dcf_formula.as_str(), &number_fmt)?;
    worksheet.write_string(row, 2, "元")?;
    worksheet.write_string(row, 3, &descriptions.get("DCF企业价值"))?;
    row += 1;
    
    // DCF每股价值公式: DCF企业价值 / 总股本
    worksheet.write_string(row, 0, "DCF每股价值")?;
    let dcf_price_formula = format!("=B{}/{}", dcf_value_row + 1, shares_cell);
    worksheet.write_formula_with_format(row, 1, dcf_price_formula.as_str(), &number_fmt)?;
    worksheet.write_string(row, 2, "元/股")?;
    worksheet.write_string(row, 3, &descriptions.get("DCF每股价值"))?;
    row += 1;
    
    // 唐朝低估价公式: 净利润 * (1+净利润增长率) * (1/无风险收益率(低估)) / 总股本
    worksheet.write_string(row, 0, "唐朝低估价")?;
    let low_formula = format!("={}*(1+{})*(1/MAX({},0.0001))/{}", np_cell, np_g_cell, low_rf_cell, shares_cell);
    worksheet.write_formula_with_format(row, 1, low_formula.as_str(), &number_fmt)?;
    worksheet.write_string(row, 2, "元/股")?;
    worksheet.write_string(row, 3, &descriptions.get("唐朝低估价"))?;
    row += 1;
    
    // 唐朝高估价公式: 净利润 * (1+净利润增长率) * (1/无风险收益率(高估)) / 总股本
    worksheet.write_string(row, 0, "唐朝高估价")?;
    let high_formula = format!("={}*(1+{})*(1/MAX({},0.0001))/{}", np_cell, np_g_cell, high_rf_cell, shares_cell);
    worksheet.write_formula_with_format(row, 1, high_formula.as_str(), &number_fmt)?;
    worksheet.write_string(row, 2, "元/股")?;
    worksheet.write_string(row, 3, &descriptions.get("唐朝高估价"))?;
    row += 1;
    
    // 唐朝安全边际价公式: (低估价 + 高估价) / 2
    let low_price_row = row - 2;
    let high_price_row = row - 1;
    worksheet.write_string(row, 0, "唐朝安全边际价")?;
    let safety_formula = format!("=(B{}+B{})/2", low_price_row + 1, high_price_row + 1);
    worksheet.write_formula_with_format(row, 1, safety_formula.as_str(), &number_fmt)?;
    worksheet.write_string(row, 2, "元/股")?;
    worksheet.write_string(row, 3, &descriptions.get("唐朝安全边际价"))?;
    
    // 设置列宽
    worksheet.set_column_width(0, 25)?;
    worksheet.set_column_width(1, 20)?;
    worksheet.set_column_width(2, 10)?;
    worksheet.set_column_width(3, 40)?;
    
    Ok(())
}
