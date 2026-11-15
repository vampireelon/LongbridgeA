# LongbridgeA

**A-share stock selection & analysis tool** inspired by Longbridge Pro’s market data and research experience.

Built with **Rust + gpui-kit + WASM/Desktop**, using **Tencent market data APIs** as the primary quote source.

> **Scope**: Pure stock selection and analysis. **No trading / order placement / simulated trading** features.
>
> Status: Phase 0 complete (docs + Cargo workspace skeleton). Application UI not started yet.

## Repository

- **Work Plan**: [WORK_PLAN.md](./WORK_PLAN.md)
- **Project structure**: [docs/PROJECT_STRUCTURE.md](./docs/PROJECT_STRUCTURE.md)
- **Tencent Quote API research**: [docs/tencent-quote-api.md](./docs/tencent-quote-api.md)
- **UI / UX Design**: [docs/DESIGN.md](./docs/DESIGN.md)
- GitHub: https://github.com/vampireelon/LongbridgeA

## Workspace crates

| Crate | Path | Role |
|-------|------|------|
| `longbridgea-core` | `crates/core` | Domain models |
| `tencent-quote` | `crates/tencent-quote` | Tencent HTTP client (stub) |
| `longbridgea-chart` | `crates/chart` | Indicators / chart math |
| `longbridgea-ui` | `crates/ui` | Pages (gpui-kit later) |
| `longbridgea-app` | `crates/app` | Binary entry |

```bash
cargo check
cargo test -p longbridgea-chart
cargo run -p longbridgea-app
```

## Tech Stack (planned)

- **Language**: Rust
- **UI**: [gpui-kit](https://github.com/longbridge/gpui-kit)
- **Targets**: Native Desktop (macOS / Windows / Linux) + WASM
- **Market Data**: Tencent public quote APIs (`qt.gtimg.cn` etc.)

## License

[MIT](./LICENSE)
