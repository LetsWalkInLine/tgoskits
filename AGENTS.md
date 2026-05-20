# AGENTS.md

## Mandatory Environment Rule

- Do not install toolchains or dependencies on the host machine.
- Run all build/test/lint/fmt commands inside the Docker container only.
- Preferred container workflow in this repo:
  - Build the image if needed: `docker build -t tgoskits-env -f container/Dockerfile .`
  - Reuse a single long-lived Compose container for the entire task instead of creating a fresh container per command.
  - First check whether the service container is already running: `docker compose ps --status running -q tgoskits`
  - If it is already running, execute commands in it via `docker compose exec -T tgoskits <command>`.
  - If it exists but is stopped, start it via `docker compose start tgoskits`, then execute commands via `docker compose exec -T tgoskits <command>`.
  - If it does not exist yet, create and start it via `docker compose up -d tgoskits`, then execute commands via `docker compose exec -T tgoskits <command>`.
  - Keep this container running for the full development task and stop it only after the task is complete.
  - Do not use `docker run`, `docker compose run --rm`, or any other per-command container workflow for normal development in this repo.
  - Because `docker-compose.yml` enables `tty: true`, prefer `docker compose exec -T ...` for agent-driven or other non-interactive commands so command output is captured correctly.
  - If the container state does not match the cases above, or a command truly requires an interactive TTY session, ask the user before proceeding.

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
- For StarryOS grouped QEMU cases such as `bugfix` and `syscall`, avoid running the full group during normal iteration. Prefer targeted subcase commands, for example:
  - `cargo xtask starry test qemu --arch x86_64 -g normal -c bugfix --subcase bug-nginx-fioasync`
  - `cargo xtask starry test qemu --arch x86_64 -g normal -c syscall --subcase test_ioctl_fionbio_int`
  - Shorthand is also supported: `cargo xtask starry test qemu --arch x86_64 -g normal -c syscall/test_ioctl_fionbio_int`
- Run full grouped cases such as `-c bugfix` or `-c syscall` only for final sweeps or when broad regression coverage is explicitly needed.

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
