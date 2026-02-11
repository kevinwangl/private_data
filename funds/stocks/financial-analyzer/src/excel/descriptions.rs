/// 财务指标说明文本
use std::collections::HashMap;

pub struct IndicatorDescriptions {
    descriptions: HashMap<String, String>,
}

impl IndicatorDescriptions {
    pub fn new() -> Self {
        let mut descriptions = HashMap::new();
        
        // 资产负债表指标
        descriptions.insert("货币资金".to_string(), "企业可随时支配的现金".to_string());
        descriptions.insert("应收账款".to_string(), "应收取的销售款项".to_string());
        descriptions.insert("存货".to_string(), "原材料、产成品等".to_string());
        descriptions.insert("固定资产".to_string(), "房屋、设备等长期资产".to_string());
        descriptions.insert("无形资产".to_string(), "专利权、商标权等".to_string());
        descriptions.insert("资产总计".to_string(), "企业全部资源".to_string());
        descriptions.insert("负债合计".to_string(), "企业全部债务".to_string());
        descriptions.insert("所有者权益合计".to_string(), "净资产".to_string());
        
        // 利润表指标
        descriptions.insert("营业收入".to_string(), "主营业务收入".to_string());
        descriptions.insert("营业总收入".to_string(), "主营业务收入".to_string());
        descriptions.insert("营业成本".to_string(), "直接成本".to_string());
        descriptions.insert("净利润".to_string(), "最终利润".to_string());
        descriptions.insert("持续经营净利润".to_string(), "扣除非经常性损益".to_string());
        descriptions.insert("销售费用".to_string(), "销售过程费用".to_string());
        descriptions.insert("管理费用".to_string(), "管理活动费用".to_string());
        descriptions.insert("财务费用".to_string(), "利息支出净额".to_string());
        
        // 现金流指标
        descriptions.insert("经营活动产生的现金流量净额".to_string(), "经营现金净额".to_string());
        descriptions.insert("自由现金流".to_string(), "经营现金流-资本支出".to_string());
        
        // 财务比率
        descriptions.insert("毛利率".to_string(), "产品盈利能力".to_string());
        descriptions.insert("净利率".to_string(), "整体盈利能力".to_string());
        descriptions.insert("ROE".to_string(), "股东回报率".to_string());
        descriptions.insert("ROA".to_string(), "资产使用效率".to_string());
        
        // 杠杆指标
        descriptions.insert("经营杠杆DOL".to_string(), ">1表示正向放大".to_string());
        descriptions.insert("财务杠杆DFL".to_string(), "接近1表示低风险".to_string());
        descriptions.insert("总杠杆DTL".to_string(), "综合杠杆效应".to_string());
        
        // 估值指标
        descriptions.insert("DCF企业价值".to_string(), "现金流折现价值".to_string());
        descriptions.insert("DCF每股价值".to_string(), "企业价值/总股本".to_string());
        descriptions.insert("唐朝低估价".to_string(), "保守买入价".to_string());
        descriptions.insert("唐朝高估价".to_string(), "乐观卖出价".to_string());
        
        Self { descriptions }
    }
    
    pub fn get(&self, key: &str) -> String {
        self.descriptions.get(key)
            .cloned()
            .unwrap_or_else(|| "".to_string())
    }
}
