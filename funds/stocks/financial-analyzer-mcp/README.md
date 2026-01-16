# Financial Analyzer MCP Server 配置指南

## 安装步骤

### 1. 构建 MCP Server
```bash
cd financial-analyzer-mcp
npm install
npm run build
```

### 2. 配置到 Kiro CLI

编辑 Kiro 配置文件 `~/.kiro/mcp.json`:

```json
{
  "mcpServers": {
    "financial-analyzer": {
      "command": "node",
      "args": [
        "/Users/sm4299/Downloads/bryan/private_data/funds/stocks/financial-analyzer-mcp/build/index.js"
      ]
    }
  }
}
```

### 3. 配置到 Claude Desktop

编辑 `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "financial-analyzer": {
      "command": "node",
      "args": [
        "/Users/sm4299/Downloads/bryan/private_data/funds/stocks/financial-analyzer-mcp/build/index.js"
      ]
    }
  }
}
```

## 使用示例

在 Kiro CLI 或 Claude Desktop 中:

### 基础分析
```
分析茅台的财务数据，股票代码 600519.SH，分析 2019-2021 年的数据
```

### 带敏感性分析
```
分析 600519.SH，年份 2019,2018,2017，使用 akshare 数据源，
设置折现率 0.10，永续增长率 0.05
```

### 完整参数示例
```
使用 analyze_stock 工具分析股票：
- stock_code: 600519.SH
- years: 2019,2018,2017
- source: akshare
- enable_validation: true
- discount_rate: 0.10
- perpetual_growth_rate: 0.05
- fcf_growth_rate: -0.08
- net_profit_growth_rate: 0.12
```

AI 会自动调用 `analyze_stock` 工具。

## 前置条件

### 使用 AKShare (推荐)
```bash
pip3 install akshare
```

### 使用 Tushare
```bash
export TUSHARE_TOKEN="your_token"
```

## 工具说明

### analyze_stock

**参数:**
- `stock_code` (必需): 股票代码，如 600519.SH
- `years` (必需): 分析年份，如 "2019,2018,2017"
- `source` (可选): 数据源 mock/akshare/tushare，默认 akshare
- `output` (可选): 输出文件路径
- `enable_validation` (可选): 是否启用数据验证
- `discount_rate` (可选): 敏感性分析 - 折现率
- `perpetual_growth_rate` (可选): 敏感性分析 - 永续增长率
- `fcf_growth_rate` (可选): 敏感性分析 - FCF增长率
- `net_profit_growth_rate` (可选): 敏感性分析 - 净利润增长率
- `low_risk_free_rate` (可选): 敏感性分析 - 无风险收益率(低估)
- `high_risk_free_rate` (可选): 敏感性分析 - 无风险收益率(高估)

**输出:**
生成双格式报告:
- **Excel 报告** ({stock_code}_财务分析.xlsx):
  - 资产结构分析
  - 利润分析
  - 现金流分析
  - DCF 估值
  - 唐朝估值法
  - 敏感性分析（如提供参数）
- **TXT 报告** ({stock_code}_财务分析.txt):
  - 关键财务指标摘要
  - 敏感性分析结果（如提供参数）
  - 文本格式便于快速查看

## 故障排查

### 检查 MCP Server 是否运行
```bash
echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | node build/index.js
```

### 查看 Kiro 日志
```bash
tail -f ~/.kiro/logs/mcp.log
```
