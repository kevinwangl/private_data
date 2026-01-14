use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use anyhow::Result;

/// 科目映射配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountMapping {
    pub operating_assets: HashMap<String, Vec<String>>,
    pub financial_assets: HashMap<String, Vec<String>>,
    pub operating_liabilities: HashMap<String, Vec<String>>,
    pub financial_liabilities: HashMap<String, Vec<String>>,
}

impl AccountMapping {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mapping: AccountMapping = toml::from_str(&content)?;
        Ok(mapping)
    }

    pub fn default_path() -> &'static str {
        "config/account_mapping.toml"
    }
}

/// 验证规则配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ValidationRules {
    pub allow_negative: AllowNegative,
    pub required_accounts: RequiredAccounts,
    pub ratio_ranges: RatioRanges,
    pub yoy_thresholds: YoyThresholds,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AllowNegative {
    pub accounts: HashSet<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequiredAccounts {
    pub balance_sheet: Vec<String>,
    pub income_statement: Vec<String>,
    pub cashflow_statement: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RatioRanges {
    pub current_ratio: RangeLimit,
    pub debt_to_asset: RangeLimit,
    pub gross_margin: RangeLimit,
    pub roe: RangeLimit,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RangeLimit {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct YoyThresholds {
    pub revenue_change: f64,
    pub profit_change: f64,
    pub asset_change: f64,
}

impl ValidationRules {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let rules: ValidationRules = toml::from_str(&content)?;
        Ok(rules)
    }

    pub fn default_path() -> &'static str {
        "config/validation_rules.toml"
    }
}

/// 系统配置
#[derive(Debug, Clone)]
pub struct Config {
    pub account_mapping: AccountMapping,
    pub validation_rules: ValidationRules,
}

impl Config {
    pub fn load() -> Result<Self> {
        let account_mapping = AccountMapping::load(Path::new(AccountMapping::default_path()))?;
        let validation_rules = ValidationRules::load(Path::new(ValidationRules::default_path()))?;

        Ok(Self {
            account_mapping,
            validation_rules,
        })
    }
}
