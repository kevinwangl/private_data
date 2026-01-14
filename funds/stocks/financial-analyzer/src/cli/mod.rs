use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "financial-analyzer")]
#[command(about = "财务报表分析系统", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 分析单只股票
    Analyze {
        /// 股票代码
        #[arg(short, long)]
        stock: String,

        /// 年份列表（逗号分隔）
        #[arg(short, long, value_delimiter = ',')]
        years: Vec<i32>,

        /// 数据源
        #[arg(long, default_value = "mock")]
        source: String,

        /// 输出路径（默认为：股票代码_财务分析.xlsx）
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// 启用数据验证
        #[arg(long, default_value = "false")]
        enable_validation: bool,
    },
}
