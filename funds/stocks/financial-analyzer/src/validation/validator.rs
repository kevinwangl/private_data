use crate::domain::*;
use crate::utils::config::ValidationRules;
use anyhow::{anyhow, Result};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};

/// 验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub reliability_score: f64,
}

/// 验证错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub rule: String,
    pub message: String,
    pub severity: Severity,
}

/// 验证警告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub field: String,
    pub message: String,
}

/// 严重程度
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Severity {
    Critical,  // 致命错误
    High,      // 高风险
    Medium,    // 中等风险
    Low,       // 低风险
}

/// 数据验证器
pub struct DataValidator {
    rules: ValidationRules,
}

impl DataValidator {
    pub fn new(rules: ValidationRules) -> Self {
        Self { rules }
    }

    /// 验证资产负债表
    pub fn validate_balance_sheet(&self, bs: &BalanceSheet) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // 1. 会计恒等式验证
        if let Err(e) = self.check_accounting_equation(bs) {
            errors.push(e);
        }

        // 2. 必需科目检查
        errors.extend(self.check_required_accounts_balance(bs));

        // 3. 数值合理性检查
        errors.extend(self.check_value_ranges(bs));

        let reliability_score = self.calculate_reliability_score(&errors, &warnings);

        ValidationResult {
            is_valid: !errors.iter().any(|e| matches!(e.severity, Severity::Critical)),
            errors,
            warnings,
            reliability_score,
        }
    }

    /// 验证利润表
    pub fn validate_income_statement(&self, is: &IncomeStatement) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // 检查必需科目
        errors.extend(self.check_required_accounts_income(is));

        // 检查利润率合理性
        if is.revenue != Decimal::ZERO {
            let gross_margin = (is.gross_profit / is.revenue).to_f64().unwrap_or(0.0);
            if gross_margin < self.rules.ratio_ranges.gross_margin.min
                || gross_margin > self.rules.ratio_ranges.gross_margin.max
            {
                warnings.push(ValidationWarning {
                    field: "毛利率".to_string(),
                    message: format!("毛利率 {:.2}% 超出合理范围", gross_margin * 100.0),
                });
            }
        }

        let reliability_score = self.calculate_reliability_score(&errors, &warnings);

        ValidationResult {
            is_valid: !errors.iter().any(|e| matches!(e.severity, Severity::Critical)),
            errors,
            warnings,
            reliability_score,
        }
    }

    /// 检查会计恒等式
    fn check_accounting_equation(&self, bs: &BalanceSheet) -> Result<(), ValidationError> {
        let total_assets = bs.statement.items.get("资产总计")
            .copied()
            .unwrap_or(Decimal::ZERO);

        let total_liabilities = bs.statement.items.get("负债合计")
            .copied()
            .unwrap_or(Decimal::ZERO);

        let total_equity = bs.statement.items.get("所有者权益合计")
            .copied()
            .unwrap_or(Decimal::ZERO);

        let diff = (total_assets - (total_liabilities + total_equity)).abs();
        let tolerance = Decimal::new(1000, 0);

        if diff > tolerance {
            return Err(ValidationError {
                field: "会计恒等式".to_string(),
                rule: "资产 = 负债 + 所有者权益".to_string(),
                message: format!(
                    "不平衡: 资产({}) ≠ 负债({}) + 权益({}), 差异: {}",
                    total_assets, total_liabilities, total_equity, diff
                ),
                severity: Severity::Critical,
            });
        }

        Ok(())
    }

    /// 检查必需科目（资产负债表）
    fn check_required_accounts_balance(&self, bs: &BalanceSheet) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for account in &self.rules.required_accounts.balance_sheet {
            if !bs.statement.items.contains_key(account) {
                errors.push(ValidationError {
                    field: account.clone(),
                    rule: "必需科目".to_string(),
                    message: format!("缺少必需科目: {}", account),
                    severity: Severity::High,
                });
            }
        }

        errors
    }

    /// 检查必需科目（利润表）
    fn check_required_accounts_income(&self, is: &IncomeStatement) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for account in &self.rules.required_accounts.income_statement {
            if !is.statement.items.contains_key(account) {
                errors.push(ValidationError {
                    field: account.clone(),
                    rule: "必需科目".to_string(),
                    message: format!("缺少必需科目: {}", account),
                    severity: Severity::High,
                });
            }
        }

        errors
    }

    /// 检查数值合理性
    fn check_value_ranges(&self, bs: &BalanceSheet) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for (account, value) in &bs.statement.items {
            // 检查负值
            if *value < Decimal::ZERO && !self.rules.allow_negative.accounts.contains(account) {
                errors.push(ValidationError {
                    field: account.clone(),
                    rule: "非负约束".to_string(),
                    message: format!("{} 不应为负值: {}", account, value),
                    severity: Severity::High,
                });
            }

            // 检查异常大值
            let max_reasonable = Decimal::new(1_000_000_000_000, 0);
            if value.abs() > max_reasonable {
                errors.push(ValidationError {
                    field: account.clone(),
                    rule: "数值范围".to_string(),
                    message: format!("{} 数值异常大: {}", account, value),
                    severity: Severity::Medium,
                });
            }
        }

        errors
    }

    /// 计算可靠性评分
    fn calculate_reliability_score(&self, errors: &[ValidationError], warnings: &[ValidationWarning]) -> f64 {
        let mut score = 100.0;

        for error in errors {
            score -= match error.severity {
                Severity::Critical => 50.0,
                Severity::High => 20.0,
                Severity::Medium => 10.0,
                Severity::Low => 5.0,
            };
        }

        score -= warnings.len() as f64 * 2.0;

        score.max(0.0)
    }
}
