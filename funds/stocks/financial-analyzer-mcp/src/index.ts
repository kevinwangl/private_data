#!/usr/bin/env node
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { CallToolRequestSchema, ListToolsRequestSchema } from '@modelcontextprotocol/sdk/types.js';
import { exec } from 'child_process';
import { promisify } from 'util';
import { fileURLToPath } from 'url';
import { dirname, resolve, join } from 'path';
import { mkdir, writeFile } from 'fs/promises';

const execAsync = promisify(exec);

// 解析财务数据并生成分析报告
function parseFinancialData(stdout: string): string {
  const lines = stdout.split('\n');
  
  // 提取关键数据
  const extractValue = (pattern: RegExp): string => {
    const line = lines.find(l => pattern.test(l));
    return line ? line.trim() : '-';
  };
  
  const revenue = extractValue(/营业总收入/);
  const netProfit = extractValue(/^净利润\s+/);
  const cashFlow = extractValue(/经营活动现金流量净额/);
  const roe = extractValue(/ROE.*净资产收益率/);
  const roa = extractValue(/ROA.*总资产收益率/);
  const netProfitMargin = extractValue(/净利润率\s+/);
  const grossMargin = extractValue(/毛利率/);
  const cash = extractValue(/货币资金\s+/);
  const dcfValue = extractValue(/每股价值:/);
  
  return `
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 财务分析摘要
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

【盈利能力】
${revenue}
${netProfit}
${grossMargin}
${netProfitMargin}

【资产回报率】
${roe}
${roa}

【现金流状况】
${cashFlow}
${cash}

【估值参考】
${dcfValue}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
`;
}
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const ANALYZER_PATH = resolve(__dirname, '../../financial-analyzer');
const DEFAULT_OUTPUT_DIR = resolve(__dirname, '../../analyzer-report');

const server = new Server(
  {
    name: 'financial-analyzer',
    version: '1.1.0',
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [
    {
      name: 'analyze_stock',
      description: '分析股票财务数据，生成Excel和TXT双格式报告。支持Mock、AKShare、Tushare数据源',
      inputSchema: {
        type: 'object',
        properties: {
          stock_code: {
            type: 'string',
            description: '股票代码，如 600519.SH',
          },
          years: {
            type: 'string',
            description: '分析年份，逗号分隔，如 2019,2018,2017',
          },
          source: {
            type: 'string',
            enum: ['mock', 'akshare', 'tushare'],
            description: '数据源: mock(测试), akshare(免费), tushare(需Token)',
            default: 'akshare',
          },
          output_dir: {
            type: 'string',
            description: '输出目录路径，默认为 stocks/analyzer-report',
          },
          output: {
            type: 'string',
            description: '输出文件名(不含路径)，默认为 {stock_code}_财务分析.xlsx',
          },
          enable_validation: {
            type: 'boolean',
            description: '是否启用数据验证',
            default: false,
          },
          discount_rate: {
            type: 'number',
            description: '敏感性分析 - 折现率',
          },
          perpetual_growth_rate: {
            type: 'number',
            description: '敏感性分析 - 永续增长率',
          },
          fcf_growth_rate: {
            type: 'number',
            description: '敏感性分析 - FCF增长率',
          },
          net_profit_growth_rate: {
            type: 'number',
            description: '敏感性分析 - 净利润增长率',
          },
          low_risk_free_rate: {
            type: 'number',
            description: '敏感性分析 - 无风险收益率(低估)',
          },
          high_risk_free_rate: {
            type: 'number',
            description: '敏感性分析 - 无风险收益率(高估)',
          },
        },
        required: ['stock_code', 'years'],
      },
    },
  ],
}));

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  if (request.params.name === 'analyze_stock') {
    const { 
      stock_code, 
      years, 
      source = 'akshare',
      output_dir,
      output, 
      enable_validation,
      discount_rate = 0.08,
      perpetual_growth_rate = 0.04,
      fcf_growth_rate = 0.05,
      net_profit_growth_rate = 0.10,
      low_risk_free_rate = 0.04,
      high_risk_free_rate = 0.02
    } = request.params.arguments as any;
    
    // 创建输出目录
    const outputPath = output_dir ? resolve(output_dir) : DEFAULT_OUTPUT_DIR;
    try {
      await mkdir(outputPath, { recursive: true });
    } catch (error: any) {
      return {
        content: [
          {
            type: 'text',
            text: `创建输出目录失败: ${error.message}`,
          },
        ],
        isError: true,
      };
    }
    
    // 构建完整输出路径
    const fileName = output || `${stock_code.replace('.', '_')}_财务分析.xlsx`;
    const fullOutputPath = join(outputPath, fileName);
    
    let cmd = `cd ${ANALYZER_PATH} && cargo run --release -- analyze --stock ${stock_code} --years ${years} --source ${source} --output "${fullOutputPath}"`;
    
    if (enable_validation) cmd += ` --enable-validation`;
    
    // 始终添加敏感性分析参数
    cmd += ` --discount-rate=${discount_rate}`;
    cmd += ` --perpetual-growth-rate=${perpetual_growth_rate}`;
    cmd += ` --fcf-growth-rate=${fcf_growth_rate}`;
    cmd += ` --net-profit-growth-rate=${net_profit_growth_rate}`;
    cmd += ` --low-risk-free-rate=${low_risk_free_rate}`;
    cmd += ` --high-risk-free-rate=${high_risk_free_rate}`;
    
    try {
      const { stdout, stderr } = await execAsync(cmd);
      
      // 解析关键财务数据
      const analysisReport = parseFinancialData(stdout);
      
      return {
        content: [
          {
            type: 'text',
            text: `✅ 分析完成！已生成 Excel 和 TXT 双格式报告

📁 输出目录: ${outputPath}
📄 文件名: ${fileName}
📄 完整路径: ${fullOutputPath}

${analysisReport}

${stdout}
${stderr ? `⚠️ ${stderr}` : ''}`,
          },
        ],
      };
    } catch (error: any) {
      return {
        content: [
          {
            type: 'text',
            text: `分析失败: ${error.message}`,
          },
        ],
        isError: true,
      };
    }
  }
  
  throw new Error(`Unknown tool: ${request.params.name}`);
});

async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
}

main().catch(console.error);
