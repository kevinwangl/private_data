#!/bin/bash

# 财务分析系统快速启动脚本

set -e

echo "🚀 财务分析系统 - 快速启动"
echo "================================"
echo ""

# 检查Rust是否安装
if ! command -v cargo &> /dev/null; then
    echo "❌ 错误: 未检测到Rust环境"
    echo "请访问 https://rustup.rs/ 安装Rust"
    exit 1
fi

echo "✓ Rust环境检测通过"
echo ""

# 编译项目
echo "📦 正在编译项目..."
cargo build --release --quiet

if [ $? -eq 0 ]; then
    echo "✓ 编译成功"
else
    echo "❌ 编译失败"
    exit 1
fi

echo ""
echo "🎯 运行示例分析..."
echo ""

# 运行示例
./target/release/financial-analyzer analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --output ./example_output.xlsx

echo ""
echo "================================"
echo "✅ 完成！"
echo ""
echo "📊 生成的报告: ./example_output.xlsx"
echo ""
echo "💡 更多用法:"
echo "  ./target/release/financial-analyzer --help"
echo ""
