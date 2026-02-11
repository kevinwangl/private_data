//! 财务分析系统错误类型定义
//! 
//! 提供统一的错误处理机制，包括：
//! - 数据源错误
//! - 验证错误
//! - 分析计算错误
//! - Excel生成错误

use std::fmt;

/// 财务分析系统错误类型
#[derive(Debug)]
pub enum AnalyzerError {
    /// 数据源错误（网络请求、数据解析等）
    DataSource(String),
    
    /// 数据验证错误
    Validation(String),
    
    /// 计算错误（除零、溢出等）
    Calculation(String),
    
    /// Excel生成错误
    Excel(String),
    
    /// 配置错误
    Config(String),
    
    /// IO错误
    Io(std::io::Error),
    
    /// 其他错误
    Other(String),
}

impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DataSource(msg) => write!(f, "数据源错误: {}", msg),
            Self::Validation(msg) => write!(f, "验证错误: {}", msg),
            Self::Calculation(msg) => write!(f, "计算错误: {}", msg),
            Self::Excel(msg) => write!(f, "Excel生成错误: {}", msg),
            Self::Config(msg) => write!(f, "配置错误: {}", msg),
            Self::Io(err) => write!(f, "IO错误: {}", err),
            Self::Other(msg) => write!(f, "错误: {}", msg),
        }
    }
}

impl std::error::Error for AnalyzerError {}

impl From<std::io::Error> for AnalyzerError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<rust_xlsxwriter::XlsxError> for AnalyzerError {
    fn from(err: rust_xlsxwriter::XlsxError) -> Self {
        Self::Excel(err.to_string())
    }
}

impl From<toml::de::Error> for AnalyzerError {
    fn from(err: toml::de::Error) -> Self {
        Self::Config(err.to_string())
    }
}

/// Result类型别名
pub type Result<T> = std::result::Result<T, AnalyzerError>;
