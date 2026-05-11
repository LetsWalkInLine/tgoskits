# Rust Analyzer Workspace Modes

Use the `*-full.code-workspace` files when you open the repository inside the dev container.
Use the `*-lite.code-workspace` files when you open the repository on the host and want the
best chance of responsive indexing, completion, and jump-to-definition.

## Recommended pairings

- `tgoskits-arceos-full.code-workspace`: ArceOS work in the container.
- `tgoskits-arceos-lite.code-workspace`: ArceOS work on the host.
- `tgoskits-starryos-full.code-workspace`: StarryOS work in the container.
- `tgoskits-starryos-lite.code-workspace`: StarryOS work on the host.
- `tgoskits-axvisor-full.code-workspace`: Axvisor work in the container.
- `tgoskits-axvisor-lite.code-workspace`: Axvisor work on the host.
- `tgoskits-tooling-full.code-workspace`: `xtask` and `scripts/axbuild` in the container.
- `tgoskits-tooling-lite.code-workspace`: `xtask` and `scripts/axbuild` on the host.

## Mode differences

Full mode keeps normal rust-analyzer behavior and is intended to run with the real toolchain and
dependencies available inside the dev container.

Lite mode is intentionally degraded:

- `procMacro` and `buildScripts` are disabled.
- dependency loading is reduced with `cargo.noDeps`.
- Cargo metadata is forced offline.
- clearly unrelated top-level areas are excluded per subsystem.

Use lite mode when full mode is too slow or gets stuck on host indexing.
