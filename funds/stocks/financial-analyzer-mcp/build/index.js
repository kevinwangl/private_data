#!/usr/bin/env node
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { CallToolRequestSchema, ListToolsRequestSchema } from '@modelcontextprotocol/sdk/types.js';
import { exec } from 'child_process';
import { promisify } from 'util';
const execAsync = promisify(exec);
const ANALYZER_PATH = '/Users/sm4299/Downloads/bryan/private_data/funds/stocks/financial-analyzer';
const server = new Server({
    name: 'financial-analyzer',
    version: '1.1.0',
}, {
    capabilities: {
        tools: {},
    },
});
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
                    output: {
                        type: 'string',
                        description: '输出文件路径，默认为 {stock_code}_财务分析.xlsx',
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
        const { stock_code, years, source = 'akshare', output, enable_validation, discount_rate, perpetual_growth_rate, fcf_growth_rate, net_profit_growth_rate, low_risk_free_rate, high_risk_free_rate } = request.params.arguments;
        let cmd = `cd ${ANALYZER_PATH} && cargo run --release -- analyze --stock ${stock_code} --years ${years} --source ${source}`;
        if (output)
            cmd += ` --output ${output}`;
        if (enable_validation)
            cmd += ` --enable-validation`;
        if (discount_rate !== undefined)
            cmd += ` --discount-rate=${discount_rate}`;
        if (perpetual_growth_rate !== undefined)
            cmd += ` --perpetual-growth-rate=${perpetual_growth_rate}`;
        if (fcf_growth_rate !== undefined)
            cmd += ` --fcf-growth-rate=${fcf_growth_rate}`;
        if (net_profit_growth_rate !== undefined)
            cmd += ` --net-profit-growth-rate=${net_profit_growth_rate}`;
        if (low_risk_free_rate !== undefined)
            cmd += ` --low-risk-free-rate=${low_risk_free_rate}`;
        if (high_risk_free_rate !== undefined)
            cmd += ` --high-risk-free-rate=${high_risk_free_rate}`;
        try {
            const { stdout, stderr } = await execAsync(cmd);
            return {
                content: [
                    {
                        type: 'text',
                        text: `✅ 分析完成！已生成 Excel 和 TXT 双格式报告\n\n${stdout}\n${stderr ? `⚠️ ${stderr}` : ''}`,
                    },
                ],
            };
        }
        catch (error) {
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
