# AKShare 数据源集成完成

## 更新内容

### 新增文件
1. **src/data_source/akshare.rs** - AKShare数据源实现
2. **AKSHARE_GUIDE.md** - AKShare使用指南

### 修改文件
1. **src/data_source/mod.rs** - 导出AkshareClient
2. **src/main.rs** - 添加akshare数据源选项
3. **src/cli/mod.rs** - 更新CLI帮助信息
4. **README.md** - 更新文档说明

## 技术实现

### 核心特性
- 通过Python子进程调用AKShare库
- 自动转换股票代码格式（600519.SH → 600519）
- JSON数据解析和类型转换
- 日期范围过滤
- 完整的错误处理

### 数据接口
```rust
// 资产负债表
ak.stock_balance_sheet_by_report_em(symbol="600519")

// 利润表
ak.stock_profit_sheet_by_report_em(symbol="600519")

// 现金流量表
ak.stock_cash_flow_sheet_by_report_em(symbol="600519")
```

### 字段映射
| AKShare字段 | 系统字段 |
|------------|---------|
| TOTAL_ASSETS | 资产总计 |
| TOTAL_LIABILITIES | 负债合计 |
| TOTAL_EQUITY | 所有者权益合计 |
| MONETARYFUNDS | 货币资金 |
| FIXED_ASSETS | 固定资产 |
| ACCOUNTS_RECE | 应收账款 |
| INVENTORY | 存货 |
| TOTAL_OPERATE_INCOME | 营业总收入 |
| OPERATE_COST | 营业总成本 |
| NETPROFIT | 净利润 |
| OPERATE_CASH_FLOW | 经营活动现金流 |

## 使用方法

### 前置条件
```bash
# 安装Python3
python3 --version

# 安装AKShare
pip3 install akshare
```

### 基础使用
```bash
# 编译
cargo build --release

# 运行分析
./target/release/financial-analyzer analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source akshare
```

### 输出示例
```
🔍 分析股票: 600519.SH
📅 年份: [2019, 2018, 2017]
📊 数据源: akshare
✓ AKShare客户端已初始化
⏳ 正在获取数据...
📝 正在生成Excel报告...
✅ 分析完成！
📄 报告已保存到: 600519_SH_财务分析.xlsx
```

## 数据源对比

| 特性 | Mock | Tushare | AKShare |
|------|------|---------|---------|
| 免费 | ✅ | ⚠️ 有限制 | ✅ |
| Token | ❌ | ✅ 需要 | ❌ |
| 真实数据 | ❌ | ✅ | ✅ |
| 数据质量 | ⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| 速度 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| 依赖 | 无 | 网络 | Python |
| 推荐场景 | 测试 | 生产 | 学习/研究 |

## 优势

✅ **完全免费** - 无需注册、无需Token
✅ **数据真实** - 来自东方财富等主流财经网站
✅ **易于使用** - 无需配置环境变量
✅ **数据全面** - 覆盖所有A股上市公司
✅ **开源透明** - AKShare是开源项目

## 限制

⚠️ **依赖Python** - 需要系统安装Python3和akshare库
⚠️ **速度较慢** - 通过Python脚本调用，比直接API慢
⚠️ **网络依赖** - 需要访问公开财经网站
⚠️ **数据延迟** - 可能有1-2天的数据延迟

## 故障排查

### 问题1：执行Python失败
```
错误: 执行Python失败: No such file or directory
```

**解决方案**：
```bash
# 检查Python安装
which python3

# 如果未安装，安装Python3
# macOS
brew install python3

# Ubuntu/Debian
sudo apt install python3
```

### 问题2：找不到akshare模块
```
错误: ModuleNotFoundError: No module named 'akshare'
```

**解决方案**：
```bash
# 安装akshare
pip3 install akshare

# 或使用国内镜像
pip3 install akshare -i https://pypi.tuna.tsinghua.edu.cn/simple
```

### 问题3：网络连接失败
```
错误: HTTPError: 404 Client Error
```

**解决方案**：
1. 检查网络连接
2. 检查股票代码是否正确
3. 尝试使用其他数据源（tushare或mock）

## 测试验证

### 编译测试
```bash
cargo build --release
# 输出: 编译成功，无错误
```

### 帮助信息测试
```bash
./target/release/financial-analyzer analyze --help
# 输出: 显示akshare在数据源选项中
```

### 功能测试（需要Python和akshare）
```bash
# 测试AKShare数据获取
./target/release/financial-analyzer analyze \
  --stock 600519 \
  --years 2019 \
  --source akshare

# 预期输出: 成功生成Excel文件
```

## 后续优化建议

1. **性能优化**
   - 考虑使用Python HTTP服务代替子进程调用
   - 添加数据缓存机制

2. **功能增强**
   - 支持更多AKShare数据接口
   - 添加数据质量检查
   - 支持批量股票分析

3. **用户体验**
   - 添加进度条显示
   - 提供更详细的错误信息
   - 支持自定义Python路径

## 相关文档

- [AKShare使用指南](./AKSHARE_GUIDE.md)
- [Tushare使用指南](./TUSHARE_GUIDE.md)
- [架构设计文档](./ARCHITECTURE.md)
- [详细设计文档](./DESIGN.md)

## 版本信息

- **版本**: v1.1.0
- **日期**: 2026-01-14
- **作者**: Financial Analyzer Team
- **更新**: 新增AKShare数据源支持
