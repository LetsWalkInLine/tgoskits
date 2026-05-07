# AGENTS.md

## Mandatory Environment Rule

- Do not install toolchains or dependencies on the host machine.
- Run all build/test/lint/fmt commands inside the Docker container only.
- Preferred container workflow in this repo:
  - `docker build -t tgoskits-env -f container/Dockerfile .`
  - `docker run -it --rm -v "$(pwd)":/workspace -w /workspace tgoskits-env`
- If using Compose, run commands via `docker compose run --rm tgoskits <command>`.

## Command Policy (Repo-Specific)

- For ArceOS/StarryOS/Axvisor, prefer `cargo xtask ...` over raw `cargo build/test/run`.
- After logic changes, run targeted checks in container:
  - `cargo fmt`
  - `cargo xtask clippy --package <crate>`
- Do not silence clippy warnings with `#[allow(...)]` unless user explicitly asks.
- If a crate passes clippy and is missing from `scripts/test/clippy_crates.csv`, add it in the same change.

## CI-Accurate Local Verification

- CI order is `fmt` first, then `cargo xtask clippy`, then tests (`cargo xtask test` and OS-specific xtask tests).
- Use focused test commands first (single package/single case), then broader suites only as needed.

## High-Value Skills In This Repo

- `update-std-tests`: for `scripts/test/std_crates.csv` audits/updates.
- `starry-test-suit`: for `test-suit/starryos` case/group/config updates.
- `cross-kernel-driver`: for driver work under `drivers/`.
- `review-open-prs`: for repository PR review tasks.
- `board-uboot-fsck-repair`: for OrangePi-5-Plus ext4 fsck recovery flow.

## PR/Communication Conventions

- Keep PR/issues/review text neutral and project-focused.
- Use Conventional Commits title style: `type(scope): content`.
- Do not add agent/AI branding text in commits, PRs, or issue comments.
