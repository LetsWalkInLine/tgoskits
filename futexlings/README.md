# Futexlings

Futexlings is a Rustlings community exercise track for building futex intuition from Rust synchronization primitives up to Linux futex ABI details and kernel-side design models.

The course is intentionally independent from the surrounding repository. It uses normal local Rust tooling and does not depend on tgoskits, StarryOS, Docker, or OS build scripts.

## Requirements

- Rust stable toolchain.
- Rustlings 6 or newer. Older Rustlings 5.x releases do not include `rustlings dev` or community exercise support.
- Linux is required for the raw futex syscall exercises in `02_linux_abi`. Non-Linux platforms skip those tests so the rest of the course remains usable.

## Usage

```powershell
cargo install rustlings
cd futexlings
rustlings
```

In watch mode, press `l` for the exercise list and `h` for the current hint. Search each exercise for `TODO` or `todo!()` and edit files under `exercises/`.

Solutions are available under `solutions/` for comparison after solving an exercise.

## Local Course Maintenance

With Rustlings 6 or newer:

```powershell
rustlings dev check --require-solutions
```

The `Cargo.toml` binary list is included so solutions can also be checked directly with Cargo.
