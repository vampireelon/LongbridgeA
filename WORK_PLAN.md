# LongbridgeA 工作计划

> **目标**：使用 Rust + gpui-kit + WASM/Desktop 技术栈，复刻面向 A 股市场的 Longbridge Pro 专业桌面交易终端，并集成腾讯行情接口作为核心数据源。
>
> **当前阶段**：仅规划与文档，暂不编码。本仓库用于跟踪文件更新与进度。

---

## 1. 项目愿景与范围

### 1.1 核心目标
- 打造高性能、跨平台（macOS / Windows / Linux）的 A 股专业行情 + 交易桌面端。
- 视觉与交互对标 Longbridge Pro：多屏布局、自定义 Workspace、图表内交易、专业快捷键、AI 辅助分析等。
- 数据层优先使用腾讯公开行情接口（qt.gtimg.cn 等），后续可扩展其他源。
- 支持 WASM 目标（浏览器端原型）与原生 Desktop 双端。

### 1.2 非目标（初期）
- 不实现真实券商下单通道（先做模拟盘 / 演示交易）。
- 不覆盖港股、美股完整功能（可预留扩展接口）。
- 不实现完整量化回测引擎（可后期加入）。

---

## 2. 技术栈确认

| 层级 | 技术 | 说明 |
|------|------|------|
| 语言 | Rust | 主语言 |
| GUI 框架 | gpui-kit (基于 GPUI) | Longbridge 官方开源，生产级组件库，支持 Desktop + WASM |
| 渲染/窗口 | GPUI + gpui-platform | 高性能 GPU 加速 |
| 数据获取 | reqwest / tokio + 腾讯行情 HTTP 接口 | 实时报价、K线、分时等 |
| 状态管理 | gpui 内置 Entity + 可能结合 Redux-like 或自定义 Store | |
| 图表 | 自研或集成 plotters / 自定义 Canvas 绘制 | K线、分时、技术指标 |
| 序列化 | serde + serde_json | |
| 异步运行时 | tokio | |
| 构建 | cargo + wasm-bindgen / trunk（WASM） | |
| 包管理 | Cargo Workspace | 多 crate 结构 |

**参考资源**：
- [gpui-kit GitHub](https://github.com/longbridge/gpui-kit)
- [gpui-kit 文档](https://gpui-kit.com)
- 腾讯行情接口：`http://qt.gtimg.cn/q=sh600519` 等公开接口

---

## 3. 功能模块拆分（按优先级）

### Phase 0：基础设施（当前）
- [x] 创建 GitHub 仓库 `LongbridgeA`
- [ ] 完善 README、LICENSE、.gitignore
- [ ] 定义项目结构（Cargo Workspace）
- [ ] 调研并文档化腾讯行情接口字段与限流策略
- [ ] 确定主题系统与设计 tokens（对标 Longbridge Pro 深色主题）

### Phase 1：数据层（核心）
- [ ] 腾讯行情客户端 crate（`tencent-quote`）
  - 实时报价（单只 / 批量）
  - 分时数据
  - K线（日/周/月/分钟）
  - 股票搜索
  - 指数与板块
- [ ] 数据模型定义（Quote、Kline、Tick 等）
- [ ] 缓存与订阅机制（内存 + 可选本地 SQLite）
- [ ] 错误处理与重试策略

### Phase 2：基础 UI 框架
- [ ] 使用 gpui-kit 搭建主窗口与 Dock / Panel 布局系统
- [ ] 侧边栏导航（行情、自选、交易、资讯、设置）
- [ ] 多窗口 / 多屏支持基础
- [ ] 主题切换（深色 / 浅色）
- [ ] 快捷键框架

### Phase 3：核心行情界面
- [ ] 自选股列表（Watchlist）
- [ ] 个股详情页（报价、五档、分时、K线）
- [ ] 专业 K线图表（支持画线、常用技术指标：MA、MACD、RSI、BOLL 等）
- [ ] 分时图
- [ ] 市场概览（指数、涨跌分布、热门板块）

### Phase 4：交易相关（模拟）
- [ ] 模拟账户与持仓
- [ ] 下单面板（限价 / 市价）
- [ ] 图表内一键交易原型
- [ ] 委托与成交记录
- [ ] 快捷键交易

### Phase 5：高级功能
- [ ] 条件选股器（Screener）
- [ ] 资金流向、龙虎榜等扩展数据（可结合其他接口）
- [ ] AI 辅助分析入口（预留接口，可对接本地/云端模型）
- [ ] 自定义 Workspace 布局保存/加载
- [ ] 资讯与公告模块

### Phase 6：跨端与打磨
- [ ] WASM 版本（浏览器可运行原型）
- [ ] 打包与自动更新（Desktop）
- [ ] 性能优化（虚拟列表、图表渲染）
- [ ] 无障碍与国际化（中英文）
- [ ] 单元测试 + UI 集成测试

---

## 4. 建议项目结构（初期）

```
LongbridgeA/
├── Cargo.toml                 # Workspace root
├── README.md
├── WORK_PLAN.md               # 本文件
├── LICENSE
├── crates/
│   ├── app/                   # 主应用入口（Desktop + WASM）
│   ├── tencent-quote/         # 腾讯行情客户端
│   ├── core/                  # 共享数据模型、业务逻辑
│   ├── ui/                    # 自定义组件与页面
│   └── chart/                 # 图表引擎
├── assets/                    # 图标、字体、主题资源
└── docs/                      # 接口文档、设计文档
```

---

## 5. 近期下一步行动（本周）

1. **完善仓库基础文件**
   - 写详细 README（中英文）
   - 添加 MIT / Apache-2.0 LICENSE
   - 配置 .gitignore（Rust 标准）

2. **接口调研文档**
   - 在 `docs/tencent-quote-api.md` 中整理：
     - 实时报价字段解析
     - K线与分时接口
     - 批量请求限制与最佳实践

3. **设计文档**
   - 核心页面线框图描述
   - 主题色板与组件规范

4. **技术验证（仍不写业务代码）**
   - 确认 gpui-kit 最新版本与 WASM 支持情况
   - 本地跑通 gpui-kit 官方示例

---

## 6. 风险与注意事项

- **数据合规**：腾讯公开接口为非官方，存在变更或限流风险，需做好抽象层，方便切换数据源。
- **性能**：A股全市场实时订阅对网络与渲染压力较大，需合理设计订阅与刷新策略。
- **gpui-kit 成熟度**：虽已被 Longbridge Pro 使用，但仍在快速迭代，需锁定版本并关注 breaking changes。
- **交易合规**：初期仅做模拟交易，避免涉及真实资金与券商接口。

---

## 7. 进度跟踪方式

- 本仓库 Issues 用于任务拆分与讨论
- Projects（GitHub Projects）用于看板管理
- 每次重要文档更新直接 push 到 main
- 代码阶段开始后采用 feature branch + PR 流程

---

**最后更新**：2026-11-15  
**仓库地址**：https://github.com/vampireelon/LongbridgeA
