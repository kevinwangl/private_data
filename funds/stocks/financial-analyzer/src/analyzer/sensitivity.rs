use crate::analyzer::valuation::{ValuationParams, DCFParams, TangchaoParams};
use crate::domain::*;
use anyhow::Result;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 敏感性分析参数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SensitivityParams {
    pub discount_rate: f64,
    pub perpetual_growth_rate: f64,
    pub fcf_growth_rate: f64,
    pub net_profit_growth_rate: f64,
    pub low_risk_free_rate: f64,
    pub high_risk_free_rate: f64,
}

impl Default for SensitivityParams {
    fn default() -> Self {
        Self {
            discount_rate: 0.08,
            perpetual_growth_rate: 0.04,
            fcf_growth_rate: -0.10,
            net_profit_growth_rate: 0.10,
            low_risk_free_rate: 0.04,
            high_risk_free_rate: 0.02,
        }
    }
}

impl SensitivityParams {
    pub fn to_valuation_params(&self, total_shares: Decimal) -> ValuationParams {
        ValuationParams {
            dcf: DCFParams {
                discount_rate: self.discount_rate,
                perpetual_growth_rate: self.perpetual_growth_rate,
                fcf_growth_rate: self.fcf_growth_rate,
            },
            tangchao: TangchaoParams {
                net_profit_growth_rate: self.net_profit_growth_rate,
                low_risk_free_rate: self.low_risk_free_rate,
                high_risk_free_rate: self.high_risk_free_rate,
                safety_margin: 0.7,
            },
            total_shares,
        }
    }
}

/// 敏感性分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityResult {
    pub params: SensitivityParams,
    pub dcf_enterprise_value: Decimal,
    pub dcf_price_per_share: Decimal,
    pub tangchao_low_estimate: Decimal,
    pub tangchao_high_estimate: Decimal,
    pub tangchao_safety_margin_price: Decimal,
}
