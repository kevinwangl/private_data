# EMV TLV Parser — 架构、技术方案与使用文档

## 1. 项目概述

EMV TLV Parser 是一个纯 Java 实现的 BER-TLV 数据解析/序列化工具库，专用于 EMV（Europay, Mastercard, Visa）支付领域的 TLV 数据处理。

核心能力：
- 解析 BER-TLV 二进制数据为树形结构
- 将树形结构序列化回二进制
- 以可视化方式展示 TLV 结构，标注 Tag/Length/Value 边界
- 自动识别 EMV 标准 Tag 名称
- 对 Issuer Script Command（Tag 86）自动拆解 APDU 命令

---

## 2. 架构设计

### 2.1 模块总览

```
┌─────────────────────────────────────────────────────────┐
│                        Main                             │
│                    (入口 & 演示)                         │
└──────────┬──────────────────────┬───────────────────────┘
           │                      │
           ▼                      ▼
┌─────────────────┐    ┌─────────────────────┐
│   TLVParser     │    │  TLVPrettyPrinter   │
│  (二进制→树)     │    │  (树→可视化输出)     │
└────────┬────────┘    └──┬──────────┬───────┘
         │                │          │
         ▼                ▼          ▼
┌─────────────────┐  ┌──────────┐ ┌───────────┐
│   TLVElement    │  │EMVTagDict│ │APDUParser │
│   (节点模型)     │  │(Tag字典) │ │(APDU拆解) │
└────────┬────────┘  └──────────┘ └───────────┘
         │
         ▼
┌─────────────────┐    ┌─────────────────────┐
│   TLVSerializer │    │  TLVParserException  │
│  (树→二进制)     │    │  (异常定义)           │
└────────┬────────┘    └──────────────────────┘
         │
         ▼
┌─────────────────┐
│    TLVUtils     │
│  (底层工具方法)   │
└─────────────────┘
```

### 2.2 文件清单

| 文件 | 职责 | 行数 |
|------|------|------|
| `TLVElement.java` | TLV 节点模型，支持树形父子关系 | ~230 |
| `TLVParser.java` | 从 InputStream 解析 BER-TLV 二进制数据 | ~180 |
| `TLVSerializer.java` | 将 TLVElement 树序列化为二进制字节流 | ~170 |
| `TLVUtils.java` | 底层工具：hex/bin 转换、位操作、字节插入、树查找 | ~320 |
| `TLVParserException.java` | 自定义异常，支持异常链 | ~45 |
| `EMVTagDict.java` | EMV 标准 Tag 名称字典（约 80 个 Tag） | ~107 |
| `APDUParser.java` | ISO 7816 APDU 命令拆解器 | ~88 |
| `TLVPrettyPrinter.java` | 树形可视化输出（方案 E 混合紧凑式） | ~154 |
| `Main.java` | 入口程序 & 使用演示 | ~35 |

### 2.3 数据流

```
Hex String
    │
    ▼ TLVUtils.hex2Bin()
byte[]
    │
    ▼ ByteArrayInputStream
InputStream
    │
    ▼ TLVParser.parseAllTLVElement()
ArrayList<TLVElement>  ──────────────────────▶  TLVPrettyPrinter.print()
    │                                                    │
    ▼ TLVSerializer.writeTLVElement()                    ▼
byte[] (序列化输出)                              终端可视化输出
```

---

## 3. 技术方案

### 3.1 BER-TLV 解析规则

遵循 ISO/IEC 7816-4 和 EMV Book 3 规范：

**Tag 解析：**
- 首字节后 5 位全为 1（`& 0x1F == 0x1F`）→ 多字节 Tag，继续读取
- 后续字节最高位为 1 → 还有更多 Tag 字节
- 后续字节最高位为 0 → Tag 结束
- 首字节 bit 6（`& 0x20`）为 1 → 结构化（Constructed）对象

**Length 解析：**
- 首字节最高位为 0 → 短格式，值即长度（0~127）
- 首字节最高位为 1 → 长格式，低 7 位表示后续长度字节数

**Value 解析：**
- 原始（Primitive）对象 → 直接读取 Length 个字节
- 结构化（Constructed）对象 → 读取 Length 个字节后递归解析子 TLV

### 3.2 可视化方案（方案 E 混合紧凑式）

设计原则：
- 每个节点一行展示 `[Tag] [Len] [Value] ── 名称`
- 下方用 `T L V` 小标签标注字段边界
- 结构化节点省略 Value，展开子树
- Tag 86（Issuer Script Command）自动拆解 APDU
- 顶部展示完整原始 Hex

输出示例：
```
Raw: 72 0F 9F 18 04 00 01 02 04 86 05 80 CA 9F 36 00

[72] [0F] ── Issuer Script Template 2 ── 15 bytes, Constructed
 T    L
 │
 ├─ [9F18] [04] [00010204] ── Issuer Script Identifier
 │   T      L    V
 │
 └─ [86] [05] [80CA9F3600] ── Issuer Script Command
     T    L    V
                └─ APDU
                   ├─ CLA  | 80     | Proprietary
                   ├─ INS  | CA     | GET DATA
                   ├─ P1P2 | 9F36   | → Application Transaction Counter (ATC)
                   └─ Le   | 00
```

### 3.3 APDU 拆解规则

按 ISO 7816-4 APDU 结构拆解：

```
┌─────┬─────┬────┬────┬────┬──────┬────┐
│ CLA │ INS │ P1 │ P2 │ Lc │ Data │ Le │
│ 1B  │ 1B  │ 1B │ 1B │ 1B │ var  │ 1B │
└─────┴─────┴────┴────┴────┴──────┴────┘
```

特殊处理：
- `INS` 自动映射命令名（如 `CA` → `GET DATA`，`A4` → `SELECT`）
- `GET DATA`/`PUT DATA` 命令的 P1P2 合并显示为 EMV Tag
- `CLA` 标注 Proprietary（bit 8=1）或 ISO/IEC 7816

### 3.4 EMV Tag 字典

内置约 80 个常用 EMV Tag 映射，覆盖：
- 基础数据元素（5A, 57, 5F20 等）
- 交易数据（9A, 9C, 9F02, 9F03 等）
- 安全数据（90, 91, 93, 9F26, 9F27 等）
- 终端数据（9F33, 9F35, 9F40 等）
- Issuer Script 相关（71, 72, 86, 9F18 等）
- 模板（61, 6F, 70, 77, A5 等）

未识别的 Tag 显示为 `Unknown`。

---

## 4. 使用文档

### 4.1 环境要求

- JDK 8 或以上

### 4.2 编译

```bash
cd /path/to/tlv
javac *.java
```

### 4.3 运行演示

```bash
java Main
```

### 4.4 API 使用

#### 解析 TLV 数据

```java
// 1. Hex 字符串转 byte[]
byte[] bin = TLVUtils.hex2Bin("720F9F180400010204860580CA9F3600");

// 2. 创建解析器并解析
InputStream input = new ByteArrayInputStream(bin);
TLVParser parser = new TLVParser(input);
ArrayList<TLVElement> elements = parser.parseAllTLVElement();

// 3. 可视化输出
TLVPrettyPrinter printer = new TLVPrettyPrinter();
printer.print(bin, elements);
```

#### 遍历解析结果

```java
for (TLVElement elem : elements) {
    String tag = elem.getTag();           // "72"
    long length = elem.getLength();       // 15
    boolean constructed = elem.isConstruct(); // true

    if (constructed) {
        for (TLVElement child : elem.getChildren()) {
            // 递归处理子元素
        }
    } else {
        byte[] value = elem.getValue();
        String hex = TLVUtils.convertBytesToString(value);
    }
}
```

#### 按 Tag 查找元素

```java
// 查找所有匹配的 Tag
ArrayList<TLVElement> results = new ArrayList<>();
TLVUtils.findTLVElementByTag(rootElement, "9F36", results);

// 判断是否包含某个 Tag
boolean hasATC = TLVUtils.isContainsTag(rootElement, "9F36");
```

#### 序列化 TLV 数据

```java
// 构建 TLV 元素
TLVElement script = new TLVElement("72");
script.setConstruct(true);

TLVElement scriptId = new TLVElement("9F18", TLVUtils.hex2Bin("00010204"));
TLVElement scriptCmd = new TLVElement("86", TLVUtils.hex2Bin("80CA9F3600"));

script.addChild(scriptId);
script.addChild(scriptCmd);

// 序列化
ByteArrayOutputStream out = new ByteArrayOutputStream();
TLVSerializer serializer = new TLVSerializer(out);
serializer.writeTLVElement(script);
byte[] result = serializer.toByteArray();
```

#### 查询 Tag 名称

```java
String name = EMVTagDict.getName("9F36");
// → "Application Transaction Counter (ATC)"

boolean known = EMVTagDict.isKnown("FF99");
// → false
```

#### 拆解 APDU 命令

```java
byte[] apdu = TLVUtils.hex2Bin("80CA9F3600");
List<String[]> fields = APDUParser.parse(apdu);
// fields: [["CLA","80","Proprietary"], ["INS","CA","GET DATA"], ...]
```

### 4.5 自定义扩展

**添加新的 EMV Tag：** 在 `EMVTagDict.java` 的 `static` 块中添加：
```java
TAGS.put("DF8101", "Card Data Input Capability");
```

**添加新的 APDU INS 命令：** 在 `APDUParser.java` 的 `static` 块中添加：
```java
INS_NAMES.put((byte) 0xB0, "READ BINARY");
```
