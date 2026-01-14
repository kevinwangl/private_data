use crate::domain::*;
use anyhow::Result;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};

/// 估值参数配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ValuationParams {
    pub dcf: DCFParams,
    pub tangchao: TangchaoParams,
    pub total_shares: Decimal,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DCFParams {
    pub discount_rate: f64,
    pub perpetual_growth_rate: f64,
    pub fcf_growth_rate: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TangchaoParams {
    pub net_profit_growth_rate: f64,
    pub low_risk_free_rate: f64,
    pub high_risk_free_rate: f64,
    pub safety_margin: f64,
}

impl Default for ValuationParams {
    fn default() -> Self {
        Self {
            dcf: DCFParams {
                discount_rate: 0.08,
                perpetual_growth_rate: 0.03,
                fcf_growth_rate: 0.10,
            },
            tangchao: TangchaoParams {
                net_profit_growth_rate: 0.10,
                low_risk_free_rate: 0.04,
                high_risk_free_rate: 0.02,
                safety_margin: 0.7,
            },
            total_shares: Decimal::new(100_000_000, 0), // 1亿股
        }
    }
}

/// 估值结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationResult {
    pub dcf: DCFValuation,
    pub tangchao: TangchaoValuation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DCFValuation {
    pub enterprise_value: Decimal,
    pub price_per_share: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangchaoValuation {
    pub low_estimate: Decimal,
    pub high_estimate: Decimal,
    pub safety_margin_price: Decimal,
}

/// 估值器
pub struct Valuator {
    params: ValuationParams,
}

impl Valuator {
    pub fn new(params: ValuationParams) -> Self {
        Self { params }
    }

    pub fn with_default() -> Self {
        Self::new(ValuationParams::default())
    }

    /// 计算估值
    pub fn calculate(
        &self,
        income_statements: &[IncomeStatement],
        cashflow_statements: &[CashflowStatement],
    ) -> Result<ValuationResult> {
        let dcf = self.calculate_dcf(cashflow_statements)?;
        let tangchao = self.calculate_tangchao(income_statements)?;

        Ok(ValuationResult { dcf, tangchao })
    }

    /// DCF估值
    fn calculate_dcf(&self, cashflows: &[CashflowStatement]) -> Result<DCFValuation> {
        if cashflows.is_empty() {
            return Ok(DCFValuation {
                enterprise_value: Decimal::ZERO,
                price_per_share: Decimal::ZERO,
            });
        }

        // 计算平均自由现金流
        let total_fcf: Decimal = cashflows.iter().map(|cf| cf.free_cashflow).sum();
        let avg_fcf = total_fcf / Decimal::from(cashflows.len());

        let discount_rate = Decimal::from_f64_retain(self.params.dcf.discount_rate).unwrap();
        let fcf_growth = Decimal::from_f64_retain(self.params.dcf.fcf_growth_rate).unwrap();
        let perpetual_growth = Decimal::from_f64_retain(self.params.dcf.perpetual_growth_rate).unwrap();

        // 计算前3年现值
        let mut pv_sum = Decimal::ZERO;
        for year in 1..=3 {
            let mut growth_factor = Decimal::ONE;
            let mut discount_factor = Decimal::ONE;
            
            for _ in 0..year {
                growth_factor *= Decimal::ONE + fcf_growth;
                discount_factor *= Decimal::ONE + discount_rate;
            }
            
            let fcf = avg_fcf * growth_factor;
            let pv = fcf / discount_factor;
            pv_sum += pv;
        }

        // 计算永续价值
        let mut terminal_growth_factor = Decimal::ONE;
        for _ in 0..3 {
            terminal_growth_factor *= Decimal::ONE + fcf_growth;
        }
        let terminal_fcf = avg_fcf * terminal_growth_factor;
        
        let terminal_value = terminal_fcf * (Decimal::ONE + perpetual_growth) / (discount_rate - perpetual_growth);
        
        let mut terminal_discount = Decimal::ONE;
        for _ in 0..3 {
            terminal_discount *= Decimal::ONE + discount_rate;
        }
        let pv_terminal = terminal_value / terminal_discount;

        let enterprise_value = pv_sum + pv_terminal;
        let price_per_share = enterprise_value / self.params.total_shares;

        Ok(DCFValuation {
            enterprise_value,
            price_per_share,
        })
    }

    /// 唐朝估值法
    fn calculate_tangchao(&self, income_statements: &[IncomeStatement]) -> Result<TangchaoValuation> {
        if income_statements.is_empty() {
            return Ok(TangchaoValuation {
                low_estimate: Decimal::ZERO,
                high_estimate: Decimal::ZERO,
                safety_margin_price: Decimal::ZERO,
            });
        }

        let latest_net_profit = income_statements[0].net_profit;
        let growth_rate = Decimal::from_f64_retain(self.params.tangchao.net_profit_growth_rate).unwrap();

        // 计算PE倍数
        let low_pe = Decimal::ONE / Decimal::from_f64_retain(self.params.tangchao.low_risk_free_rate).unwrap();
        let high_pe = Decimal::ONE / Decimal::from_f64_retain(self.params.tangchao.high_risk_free_rate).unwrap();

        // 3年后净利润
        let mut future_profit = latest_net_profit;
        for _ in 0..3 {
            future_profit *= Decimal::ONE + growth_rate;
        }

        // 估值
        let low_estimate = (future_profit * low_pe) / self.params.total_shares;
        let high_estimate = (future_profit * high_pe) / self.params.total_shares;
        let safety_margin_price = low_estimate * Decimal::from_f64_retain(self.params.tangchao.safety_margin).unwrap();

        Ok(TangchaoValuation {
            low_estimate,
            high_estimate,
            safety_margin_price,
        })
    }
}
