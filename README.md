# Acme UI Kit

A clean-room Rust desktop GUI development kit built directly on GPUI. It is designed as a practical starting point for recreating a modern component library similar in scope and workflow to `gpui-component`, without copying its implementation.

## Included in V1

- Workspace architecture: reusable UI crate + Gallery application
- Light/dark design tokens
- Shared size and tone primitives
- Button, Card, Badge, Progress, Switch, FieldShell, Tabs, Separator, Skeleton
- Interactive Gallery with theme toggle, counter, switch, tabs, and progress examples
- Windows PowerShell and Unix shell helper scripts
- CI, formatting, linting, documentation, test plan, roadmap, and agent instructions

## Run

```powershell
./scripts/bootstrap-windows.ps1
./scripts/run-gallery.ps1
```

Or:

```bash
cargo run -p acme-gallery
```

GPUI is pinned to one Zed revision in the workspace `Cargo.toml`. Upgrade every GPUI-family dependency together.

Read `README.zh-TW.md` for Traditional Chinese instructions.
