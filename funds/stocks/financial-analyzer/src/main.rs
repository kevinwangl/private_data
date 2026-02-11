mod domain;
mod data_source;
mod analyzer;
mod excel;
mod cli;
mod utils;
mod validation;
mod report;
mod error;

pub use error::{AnalyzerError, Result as AnalyzerResult};
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use data_source::{DataSource, MockDataSource, TushareClient, AkshareClient};
use analyzer::FinancialAnalyzer;
use excel::ExcelWriter;
use utils::Config;
use validation::DataValidator;
use report::TextReporter;
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
            discount_rate,
            perpetual_growth_rate,
            fcf_growth_rate,
            net_profit_growth_rate,
            low_risk_free_rate,
            high_risk_free_rate,
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
                "akshare" => {
                    println!("✓ AKShare客户端已初始化");
                    Box::new(AkshareClient::new())
                }
                _ => {
                    eprintln!("❌ 不支持的数据源: {}", source);
                    eprintln!("💡 当前支持: mock, tushare, akshare");
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
            let mut result = analyzer.analyze(&stock, years, data_source.as_ref()).await?;

            // 默认启用敏感性分析
            println!("🔬 计算敏感性分析...");
            
            let mut sensitivity_params = analyzer::SensitivityParams::default();
            
            if let Some(r) = discount_rate {
                sensitivity_params.discount_rate = r;
            }
            if let Some(g) = perpetual_growth_rate {
                sensitivity_params.perpetual_growth_rate = g;
            }
            if let Some(fcf_g) = fcf_growth_rate {
                sensitivity_params.fcf_growth_rate = fcf_g;
            }
            if let Some(np_g) = net_profit_growth_rate {
                sensitivity_params.net_profit_growth_rate = np_g;
            }
            if let Some(low_rf) = low_risk_free_rate {
                sensitivity_params.low_risk_free_rate = low_rf;
            }
            if let Some(high_rf) = high_risk_free_rate {
                sensitivity_params.high_risk_free_rate = high_rf;
            }
            
            analyzer.calculate_sensitivity(&mut result, sensitivity_params)?;
            println!("✓ 敏感性分析完成");

            // 确定输出文件名
            let output_path = output.unwrap_or_else(|| {
                PathBuf::from(format!("../analyzer-report/{}_财务分析.xlsx", stock.replace(".", "_")))
            });

            // 创建输出目录（如果不存在）
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // 生成文本报告（控制台输出 + 保存文件）
            println!("\n📊 生成文本报告...\n");
            TextReporter::generate(&result, &stock, output_path.to_str().unwrap_or("output.xlsx"))?;

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
