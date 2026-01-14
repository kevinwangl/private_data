# AKShare 数据源使用指南

## 简介

AKShare是一个完全免费、开源的Python财经数据接口库，无需Token即可使用。本系统已集成AKShare作为数据源之一。

## 安装依赖

### 1. 安装Python3

确保系统已安装Python 3.7+：

```bash
python3 --version
```

### 2. 安装AKShare

```bash
pip3 install akshare lxml
```

或使用国内镜像加速：

```bash
pip3 install akshare lxml -i https://pypi.tuna.tsinghua.edu.cn/simple
```

**注意**：AKShare依赖`lxml`库进行HTML解析，必须同时安装。

## 使用方法

### 基础使用

```bash
./target/release/financial-analyzer analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source akshare
```

### 指定输出文件

```bash
./target/release/financial-analyzer analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source akshare \
  --output 茅台分析_akshare.xlsx
```

### 启用数据验证

```bash
./target/release/financial-analyzer analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source akshare \
  --enable-validation
```

## 股票代码格式

AKShare支持以下格式：
- `600519.SH` - 上交所（系统会自动转换为 `600519`）
- `000001.SZ` - 深交所（系统会自动转换为 `000001`）
- `600519` - 直接使用代码

## 数据来源

AKShare从以下公开数据源获取数据：
- 东方财富网
- 新浪财经
- 同花顺
- 雪球

## 优势

✅ **完全免费** - 无需注册、无需Token
✅ **数据全面** - 覆盖A股所有上市公司
✅ **更新及时** - 数据源来自主流财经网站
✅ **易于使用** - 无需配置环境变量

## 限制

⚠️ **依赖Python** - 需要系统安装Python3和akshare库
⚠️ **网络依赖** - 需要访问公开财经网站
⚠️ **速度较慢** - 相比API调用，通过Python脚本会稍慢

## 故障排查

### 错误：执行Python失败

**原因**：系统未安装Python3或akshare库

**解决**：
```bash
# 检查Python
python3 --version

# 安装akshare
pip3 install akshare
```

### 错误：Python脚本执行错误

**原因**：akshare库版本过旧或网络问题

**解决**：
```bash
# 升级akshare
pip3 install --upgrade akshare

# 检查网络连接
ping www.eastmoney.com
```

### 错误：解析JSON失败

**原因**：数据格式变化或股票代码错误

**解决**：
1. 检查股票代码是否正确
2. 尝试使用其他数据源（mock或tushare）
3. 查看详细错误信息

## 与其他数据源对比

| 特性 | AKShare | Tushare | Mock |
|------|---------|---------|------|
| 免费 | ✅ 完全免费 | ⚠️ 有限制 | ✅ 免费 |
| Token | ❌ 不需要 | ✅ 需要 | ❌ 不需要 |
| 数据质量 | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ |
| 速度 | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| 依赖 | Python | 网络 | 无 |

## 推荐使用场景

- ✅ 个人学习和研究
- ✅ 小规模数据分析
- ✅ 不想注册API账号
- ✅ 快速验证想法

## 不推荐场景

- ❌ 高频数据获取
- ❌ 生产环境
- ❌ 对速度要求极高
- ❌ 需要历史悠久的数据

## 技术实现

系统通过以下方式调用AKShare：

```rust
// 调用Python脚本
let script = format!(
    r#"
import akshare as ak
import json
df = ak.stock_balance_sheet_by_report_em(symbol="{}")
print(df.to_json(orient='records'))
"#,
    stock_code
);

// 执行并解析结果
let output = Command::new("python3")
    .arg("-c")
    .arg(script)
    .output()?;
```

## 相关链接

- AKShare官网：https://www.akshare.xyz/
- AKShare GitHub：https://github.com/akfamily/akshare
- AKShare文档：https://akshare.akfamily.xyz/

## 更新日志

- **2026-01-14**：首次集成AKShare数据源
