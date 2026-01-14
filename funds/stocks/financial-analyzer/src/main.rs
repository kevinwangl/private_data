mod domain;
mod data_source;
mod analyzer;
mod excel;
mod cli;
mod utils;
mod validation;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use data_source::{DataSource, MockDataSource, TushareClient};
use analyzer::FinancialAnalyzer;
use excel::ExcelWriter;
use utils::Config;
use validation::DataValidator;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            stock,
            years,
            source,
            output,
            enable_validation,
        } => {
            println!("🔍 分析股票: {}", stock);
            println!("📅 年份: {:?}", years);
            println!("📊 数据源: {}", source);

            // 创建数据源
            let data_source: Box<dyn DataSource> = match source.as_str() {
                "mock" => Box::new(MockDataSource::new()),
                "tushare" => {
                    match TushareClient::from_env() {
                        Ok(client) => {
                            println!("✓ Tushare客户端已初始化");
                            Box::new(client)
                        }
                        Err(e) => {
                            eprintln!("❌ Tushare初始化失败: {}", e);
                            eprintln!("💡 请设置环境变量: export TUSHARE_TOKEN=your_token");
                            std::process::exit(1);
                        }
                    }
                }
                _ => {
                    eprintln!("❌ 不支持的数据源: {}", source);
                    eprintln!("💡 当前支持: mock, tushare");
                    std::process::exit(1);
                }
            };

            // 创建分析器
            let mut analyzer = FinancialAnalyzer::new();

            // 如果启用验证，加载配置
            if enable_validation {
                println!("🔐 启用数据验证...");
                match Config::load() {
                    Ok(config) => {
                        let validator = DataValidator::new(config.validation_rules);
                        analyzer = analyzer.with_validator(validator);
                        println!("✓ 验证规则已加载");
                    }
                    Err(e) => {
                        eprintln!("⚠️  警告: 无法加载配置文件: {}", e);
                        eprintln!("   继续执行但不进行数据验证");
                    }
                }
            }

            // 执行分析
            println!("⏳ 正在获取数据...");
            let result = analyzer.analyze(&stock, years, data_source.as_ref()).await?;

            // 确定输出文件名
            let output_path = output.unwrap_or_else(|| {
                PathBuf::from(format!("{}_财务分析.xlsx", stock.replace(".", "_")))
            });

            // 生成Excel
            println!("📝 正在生成Excel报告...");
            let excel_writer = ExcelWriter::new();
            excel_writer.generate(&result, &output_path)?;

            println!("✅ 分析完成！");
            println!("📄 报告已保存到: {}", output_path.display());
        }
    }

    Ok(())
}
