# LongbridgeA

**A-share edition of Longbridge Pro** — high-performance desktop trading terminal focused on Chinese A-shares.

Built with **Rust + gpui-kit + WASM/Desktop**, using **Tencent market data APIs** as the primary quote source.

> Status: Planning & documentation phase. No application code yet.

## Repository

- **Work Plan (详细工作计划)**: [WORK_PLAN.md](./WORK_PLAN.md)
- GitHub: https://github.com/vampireelon/LongbridgeA

## Tech Stack (planned)

- **Language**: Rust
- **UI**: [gpui-kit](https://github.com/longbridge/gpui-kit) (GPUI-based, production-ready components used by Longbridge Pro itself)
- **Targets**: Native Desktop (macOS / Windows / Linux) + WASM
- **Market Data**: Tencent public quote APIs (`qt.gtimg.cn` etc.)

## Current Focus

1. Finalize project structure & documentation
2. Document Tencent quote API fields and usage patterns
3. Design system & layout prototypes (docs only)
4. Then start coding Phase 1 (data layer)

See [WORK_PLAN.md](./WORK_PLAN.md) for the full phased roadmap.

## License

To be determined (likely MIT or Apache-2.0).
