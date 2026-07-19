# Acme UI Kit

A clean-room Rust desktop GUI component library built on **GPUI** (Zed's native UI framework). Entirely new implementation — no copied source code.

## Current Status — V1+V2+V3+V4

| Phase | Components | Status |
|-------|-----------|--------|
| **V1 Foundation** | Button, Card, Badge, Progress, Switch, FieldShell, Tabs, Separator, Skeleton, Theme (Light/Dark), primitives | ✅ |
| **V2 Controls** | TextInput, Textarea, Checkbox, Radio/RadioGroup, Select, Combobox, Menu, Dialog, Popover, Tooltip, Notification, IconProvider | ✅ |
| **V3 Data & Layout** | Pagination (+10 tests), Sidebar, Resizable, LoadingState, VirtualList, **Tree**, **Form+Validation**, **Table (sortable)** | ✅ |
| **V4 Rich Content** | **SettingsPage**, **Tiles**, **Markdown**, **BarChart**, **Dock** | ✅ |

**Total**: 28 components, ~8000+ lines, all compiling with zero warnings.

## Quick Start

```powershell
# Prerequisites:
#   - Visual Studio 2022 Build Tools (Desktop C++ workload)
#   - Windows 10/11 SDK
#   - Rust nightly (rust-toolchain.toml sets it automatically)

# Run the Gallery app:
cargo run -p acme-gallery
```

Gallery features: theme toggle (Light/Dark), interactive demos for every component.

## Project Structure

```
acme-ui-kit/
├── apps/acme-gallery/       # Interactive component demo
├── crates/acme-ui/src/      # 32 source files
│   ├── lib.rs               # Module declarations + re-exports
│   ├── theme.rs             # Theme, FontSizes, Spacing, ThemeColors
│   ├── styled.rs            # StyledExt helpers (h_flex, v_flex)
│   ├── primitives.rs        # Size / Tone enums
│   ├── icons.rs             # IconProvider, IconName
│   └── *.rs                 # One file per component
├── docs/                    # Architecture, Design System, API, Roadmap
├── scripts/                 # Windows / Unix helpers
├── AGENTS.md                # Agent workflow rules (incl. git push)
├── UI_DESIGN_PRINCIPLES.md  # Typography & spacing constraints
├── spec.md / plan.md / todos.md / test.md
└── Cargo.toml               # GPUI pinned to one Zed revision
```

## Design Principles

- **Theme-first**: All colors from `cx.theme().colors.*`, no hardcoded hex.
- **Token scale**: `FontSizes` (heading/body/caption), `Spacing` (widget/group/section/panel).
- **RenderOnce default**: Stateless views → `RenderOnce`; stateful (focus, async, IME) → `Entity + Render`.
- **Gallery-driven**: Every new component must have an interactive Gallery demo.
- **Clean-room**: Reimplemented API shape and UX, never copied source.

## Using Components

```rust
use acme_ui::prelude::*;
// or explicitly:
use acme_ui::{Button, ActiveTheme, StyledExt};

// Builder pattern everywhere:
Button::new("id")
    .label("Click me")
    .primary()
    .small()
    .on_click(|_event, _window, cx| { cx.notify(); })
```

## Building & Verification

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## GPUI Upgrades

GPUI is pinned to one Zed revision in the workspace `Cargo.toml`. All GPUI-family deps must be updated together. See `docs/UPGRADE_GPUI.md` for the process.

## Agent Instructions

This project is designed for AI agent-driven development. See `AGENTS.md` for workflow rules, component conventions, and git push procedures.

## Traditional Chinese

See `README.zh-TW.md` for Traditional Chinese documentation.
