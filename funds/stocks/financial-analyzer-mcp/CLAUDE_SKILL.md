# Financial Analyzer - Claude Desktop Skill

## 安装到 Claude Desktop

### 1. 配置文件位置

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

### 2. 重启 Claude Desktop

完全退出并重新打开 Claude Desktop 应用。

### 3. 验证安装

在 Claude Desktop 中，你会看到:
- 🔌 右下角显示 "1 tool available" 或类似提示
- 工具名称: `analyze_stock`

## 使用示例

直接在 Claude Desktop 对话:

```
分析茅台 600519.SH 2021-2019年的财务数据
```

```
帮我分析五粮液 000858.SZ 最近3年的财务状况
```

```
用 mock 数据测试分析 600519.SH 2019年的数据
```

```
分析 600519.SH 2019-2017年数据，设置折现率0.10，永续增长率0.05
```

Claude 会自动:
1. 调用 `analyze_stock` 工具
2. 生成 Excel + TXT 报告（含敏感性分析）
3. 读取报告内容
4. 给出投资建议

## 输出位置

报告默认保存在 financial-analyzer 目录:
```
/Users/sm4299/Downloads/bryan/private_data/funds/stocks/financial-analyzer/
├── {股票代码}_财务分析.xlsx
└── {股票代码}_财务分析.txt
```

## 故障排查

### 工具未显示
1. 检查配置文件路径是否正确
2. 完全退出 Claude Desktop (Cmd+Q)
3. 重新打开

### 查看日志
```bash
# Claude Desktop 日志位置
~/Library/Logs/Claude/
```

### 测试 MCP Server
```bash
echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | \
  node /Users/sm4299/Downloads/bryan/private_data/funds/stocks/financial-analyzer-mcp/build/index.js
```

## Windows 配置

配置文件位置: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "financial-analyzer": {
      "command": "node",
      "args": [
        "C:\\path\\to\\financial-analyzer-mcp\\build\\index.js"
      ]
    }
  }
}
```
