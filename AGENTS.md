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

## Project Skills

- `update-std-tests` (`.claude/skills/update-std-tests/SKILL.md`): use for `scripts/test/std_crates.csv` audits/updates, workspace-vs-whitelist comparisons, and new std-test candidates.
- `starry-test-suit` (`.claude/skills/starry-test-suit/SKILL.md`): use for `test-suit/starryos` cases/configs, `qemu-*.toml`, grouping, success/fail regexes, and Starry test-suit CI behavior.
- `cross-kernel-driver` (`.claude/skills/cross-kernel-driver/SKILL.md`): use for portable Rust driver work under `drivers/`, including layer boundaries, MMIO/DMA handling, IRQ/queue contracts, and OS API coupling audits.
- `review-open-prs` (`.claude/skills/review-open-prs/SKILL.md`): use for auditing or reviewing all open PRs, re-reviewing updated PRs, running validation, and submitting approve/request-changes reviews.
- `review-single-pr` (`.claude/skills/review-single-pr/SKILL.md`): use for focused review of one PR number/URL, overlap/conflict/app-support checks, local validation, Chinese inline comments, and final review submission.
- `reassign-pr-reviewers` (`.claude/skills/reassign-pr-reviewers/SKILL.md`): use for assigning or rebalancing `rcore-os/tgoskits` PR reviewers while preserving bot requests and permission limits.
- `board-uboot-fsck-repair` (`.claude/skills/board-uboot-fsck-repair/SKILL.md`): use for physical-board ext4 recovery through U-Boot, OrangePi-5-Plus `extraboardargs=fsckfix`, and Linux fsck/boot checks around Starry board write tests.
- `crates-io-owner` (`.claude/skills/crates-io-owner/SKILL.md`): use for adding/verifying `github:rcore-os:crates-io` ownership for branch-added crates or explicitly requested `cargo owner` checks.

## PR/Communication Conventions

- Keep PR/issues/review text neutral and project-focused.
- Use Conventional Commits title style: `type(scope): content`.
- Do not add agent/AI branding text in commits, PRs, or issue comments.

## Worklog

- Only write a worklog when the user explicitly asks for it.
- Store worklogs under `target/worklog`.
- Write worklogs in Chinese, using short `- ` bullet points in chronological order.
- Keep entries concise: one or two sentences per key milestone, covering important findings, decisions, commands/results, blockers, and follow-up state.
- Use the style of prior logs under `target/worklog` as reference, but do not assume that directory exists.

## Additional Upstream Requirements

- When changing logic, run a relevant `cargo clippy` check after the code change, using the container workflow above.
- After modifying a crate, ensure that crate passes clippy. Prefer `cargo xtask clippy --package <crate>` for targeted verification.
- Run `cargo fmt` after code edits, using the container workflow above.
- If `cargo xtask` cannot satisfy a special configuration, inspect the `xtask` flow first and only then fall back to native Cargo commands with manually matched arguments.
- For PR titles, follow `type(scope): content` in Conventional Commits style. Prefer the main affected crate name as `scope` when one crate clearly dominates the change; for cross-cutting or infrastructure work, broader scopes such as `ci`, `repo`, or `docs` are acceptable.
- PR title examples: `feat(axbuild): add Starry remote board test flow`, `fix(starry-process): correct tty session cleanup`, `chore(ci): split Starry self-hosted board matrix`.
- When submitting a PR, write the title in English and the body in Chinese.
- PR descriptions must clearly cover: the problem being solved, what was changed to solve it, and the logic behind each step of the solution.
- Before submitting a PR, locally validate the CI flow as much as practical, excluding only physical board tests and self-hosted test flows unless the user explicitly asks to run them. Changes unrelated to building or testing, such as documentation-only updates, do not require local CI validation.
- After adding or changing commits on a PR branch, update the PR description so it stays synchronized with the committed changes.
- Do not insert agent-related labels, signatures, branding, or other advertisement-style wording such as `codex`, `agent`, `AI`, or similar self-promotional tags unless the user explicitly requests it.
