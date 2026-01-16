# 敏感性分析使用指南

## 功能说明

敏感性分析模块允许你调整估值参数，观察不同参数对估值结果的影响。系统会生成：
- **Excel工作表**：包含可编辑参数和自动计算的估值结果 ✅
- **TXT报告**：包含完整的敏感性分析数据

### ✨ 核心特性

- **实时更新**：在Excel中直接修改参数值，估值结果会自动重新计算
- **公式驱动**：所有估值结果都使用Excel公式，无需重新运行程序
- **双格式输出**：Excel（可交互）+ TXT（便于查看）

## 参数说明

### DCF估值参数

| 参数 | CLI参数 | 默认值 | 说明 | 合理范围 |
|------|---------|--------|------|----------|
| 折现率(r) | `--discount-rate` | 0.08 | 反映投资风险和机会成本 | 0.06-0.12 |
| 永续增长率(g) | `--perpetual-growth-rate` | 0.04 | 长期稳定增长率 | 0.02-0.05 |
| FCF增长率(G) | `--fcf-growth-rate` | -0.10 | 自由现金流增长率 | -0.20-0.20 |

### 唐朝估值参数

| 参数 | CLI参数 | 默认值 | 说明 | 合理范围 |
|------|---------|--------|------|----------|
| 净利润增长率 | `--net-profit-growth-rate` | 0.10 | 未来净利润增长率 | 0.05-0.20 |
| 无风险收益率(低估) | `--low-risk-free-rate` | 0.04 | 低估区域PE倍数=1/该值 | 0.03-0.06 |
| 无风险收益率(高估) | `--high-risk-free-rate` | 0.02 | 高估区域PE倍数=1/该值 | 0.01-0.03 |

## 使用示例

### 示例1: 基础敏感性分析

使用默认参数的变体：

```bash
cargo run -- analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source mock \
  --discount-rate=0.10 \
  --perpetual-growth-rate=0.05
```

### 示例2: 悲观情景分析

高折现率、低增长率：

```bash
cargo run -- analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source akshare \
  --discount-rate=0.12 \
  --perpetual-growth-rate=0.02 \
  --fcf-growth-rate=-0.15 \
  --net-profit-growth-rate=0.05
```

### 示例3: 乐观情景分析

低折现率、高增长率：

```bash
cargo run -- analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source akshare \
  --discount-rate=0.06 \
  --perpetual-growth-rate=0.05 \
  --fcf-growth-rate=0.10 \
  --net-profit-growth-rate=0.15
```

### 示例4: 完整参数分析

指定所有参数：

```bash
cargo run -- analyze \
  --stock 600519.SH \
  --years 2019,2018,2017 \
  --source akshare \
  --discount-rate=0.10 \
  --perpetual-growth-rate=0.05 \
  --fcf-growth-rate=-0.08 \
  --net-profit-growth-rate=0.12 \
  --low-risk-free-rate=0.05 \
  --high-risk-free-rate=0.025
```

## 输出说明

### Excel输出

生成的Excel文件会包含新的"敏感性分析"工作表，包括：

1. **参数部分**：显示所有6个参数及其值（可直接编辑）✅
2. **基础数据部分**：显示最近一年的FCF、净利润、总股本
3. **估值结果部分**（自动计算）：
   - DCF企业价值（元）
   - DCF每股价值（元/股）
   - 唐朝低估价（元/股）
   - 唐朝高估价（元/股）
   - 唐朝安全边际价（元/股）
4. **使用说明**：参数解释和使用建议

**重要**：在Excel中直接修改参数值，估值结果会立即自动更新，无需重新运行程序！✅

### TXT输出

文本报告会在末尾添加"敏感性分析"部分，格式化显示所有参数和结果。

## 参数调节建议

### 1. 单因素敏感性分析

每次只改变一个参数，观察对估值的影响：

```bash
# 测试折现率影响
for rate in 0.06 0.08 0.10 0.12; do
  cargo run -- analyze --stock 600519.SH --years 2019 --source mock \
    --discount-rate=$rate --output "dcf_r_${rate}.xlsx"
done
```

### 2. 情景对比分析

创建3个情景进行对比：

```bash
# 悲观情景
cargo run -- analyze --stock 600519.SH --years 2019 --source akshare \
  --discount-rate=0.12 --perpetual-growth-rate=0.02 \
  --output "scenario_pessimistic.xlsx"

# 基准情景
cargo run -- analyze --stock 600519.SH --years 2019 --source akshare \
  --discount-rate=0.08 --perpetual-growth-rate=0.04 \
  --output "scenario_base.xlsx"

# 乐观情景
cargo run -- analyze --stock 600519.SH --years 2019 --source akshare \
  --discount-rate=0.06 --perpetual-growth-rate=0.06 \
  --output "scenario_optimistic.xlsx"
```

## 参数选择指南

### 折现率选择

- **6-7%**: 低风险企业（如公用事业、消费龙头）
- **8-10%**: 中等风险企业（如制造业、科技企业）
- **11-12%**: 高风险企业（如初创企业、周期性行业）

### 永续增长率选择

- **2-3%**: 成熟市场GDP增长率
- **3-4%**: 中国长期GDP增长预期
- **4-5%**: 行业龙头企业

### FCF增长率选择

- **负值**: 企业处于投资期或现金流下降
- **0-5%**: 稳定增长
- **5-10%**: 快速增长
- **>10%**: 高速增长（不可持续）

### 净利润增长率选择

- **5-8%**: 成熟企业
- **8-12%**: 稳健增长企业
- **12-20%**: 高成长企业

## 注意事项

1. **参数合理性**: 确保参数符合企业实际情况和行业特点
2. **折现率 > 永续增长率**: DCF模型要求折现率必须大于永续增长率
3. **负数参数**: 使用等号格式，如 `--fcf-growth-rate=-0.10`
4. **多次运行**: 可以多次运行不同参数组合，对比结果
5. **结合基本面**: 敏感性分析结果应结合企业基本面分析

## 常见问题

### Q: 为什么我的估值结果差异很大？

A: 估值对折现率和增长率非常敏感，微小的参数变化可能导致较大的估值差异。建议：
- 使用多个情景进行对比
- 关注估值区间而非单一数值
- 结合其他估值方法验证

### Q: 如何选择合适的参数？

A: 建议步骤：
1. 研究企业历史财务数据
2. 分析行业平均水平
3. 参考同类企业估值参数
4. 使用保守估计（安全边际原则）

### Q: 可以不提供任何参数吗？

A: 可以。如果不提供敏感性参数，系统会使用默认估值参数，不会生成敏感性分析工作表。

## 示例输出

运行命令后，你会看到：

```
🔍 分析股票: 600519.SH
📅 年份: [2019, 2018, 2017]
📊 数据源: akshare
✓ AKShare客户端已初始化
⏳ 正在获取数据...
🔬 计算敏感性分析...
✓ 敏感性分析完成

📊 生成文本报告...

【敏感性分析】
====================================================================================================

--- 敏感性参数 ---
参数名称                                          参数值
--------------------------------------------------
折现率(r)                                     10.00%
永续年金增长率(g)                                  5.00%
FCF增长率(G)                                  -8.00%
净利润增长率                                     12.00%
无风险收益率(低估区域)                                5.00%
无风险收益率(高估区域)                                2.50%

--- 估值结果 ---
估值方法                                         估值结果         单位
------------------------------------------------------------
DCF企业价值                                  1296.61万          元
DCF每股价值                                      0.13        元/股
唐朝低估价                                        0.22        元/股
唐朝高估价                                        0.45        元/股
唐朝安全边际价                                      0.16        元/股
```

## 进阶用法

### 批量分析脚本

创建一个shell脚本进行批量敏感性分析：

```bash
#!/bin/bash
# sensitivity_batch.sh

STOCK="600519.SH"
YEARS="2019,2018,2017"
SOURCE="akshare"

# 折现率敏感性
for r in 0.06 0.08 0.10 0.12; do
  echo "Testing discount rate: $r"
  cargo run -- analyze --stock $STOCK --years $YEARS --source $SOURCE \
    --discount-rate=$r \
    --output "sensitivity_r_${r}.xlsx"
done

# 永续增长率敏感性
for g in 0.02 0.03 0.04 0.05; do
  echo "Testing perpetual growth rate: $g"
  cargo run -- analyze --stock $STOCK --years $YEARS --source $SOURCE \
    --perpetual-growth-rate=$g \
    --output "sensitivity_g_${g}.xlsx"
done
```

运行：
```bash
chmod +x sensitivity_batch.sh
./sensitivity_batch.sh
```

---

**提示**: 敏感性分析是估值的重要工具，但不应作为唯一依据。请结合企业基本面、行业分析和其他估值方法综合判断。
