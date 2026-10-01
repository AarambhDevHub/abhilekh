# Contributing to Abhilekh

Thanks for wanting to help. This project is built in **small, finished versions**, so contributions work best when they are small and finished too.

## Ways to help

- **Report bugs** with clear steps to reproduce (use the bug template).
- **Write evaluation scenarios** (roadmap v0.7.2): hand-written memory stories with unambiguous expected answers.
- **Test on Windows and macOS**, especially file locking, crash recovery, and paths.
- **Improve documentation**: unclear sentences, missing examples, typos.
- **Add tests**, especially property tests and crash tests.
- **Pick up a roadmap version.** Comment on an issue first so work is not duplicated. Versions depend on earlier ones, so only claim a version whose predecessors are merged.

If you want to propose a large change, open an issue first and describe the problem before the solution.

## Setup

You need Rust **1.88 or newer**.

```bash
git clone https://github.com/AarambhDevHub/abhilekh
cd abhilekh
cargo build --workspace
cargo test --workspace
```

Useful tools (optional but recommended):

```bash
cargo install cargo-deny      # license and advisory checks
```

## The rules every change follows

Read [ROADMAP.md](ROADMAP.md) ("Rules for every version") and [ARCHITECTURE.md](ARCHITECTURE.md) first. In short:

1. **One small change per pull request.** If it grows past about double the estimate in the roadmap, split it.
2. **Definition of done** (CI checks the first three):
   - `cargo fmt --all --check` passes
   - `cargo clippy --workspace --all-targets -- -D warnings` passes
   - `cargo test --workspace` passes, with new tests for new behavior
   - The demo command for that roadmap version works
   - `CHANGELOG.md` has one line under "Unreleased"
   - `ARCHITECTURE.md` is updated if the design changed
3. **Worked examples are tests.** Every worked example in `ARCHITECTURE.md` has a matching unit test. If code and document disagree, find out which one is wrong and fix it.
4. **Never skip crash and replay tests.** Anything that touches the log must keep the crash-recovery and "replay equals live state" tests green.

## Design guardrails

Changes that break these will be asked to change:

- **The log is the truth.** Derived stores and indexes must be rebuildable with `abhilekh reindex`.
- **Raw text is never rewritten.** Maintenance changes derived data only. Only `purge` removes raw data, and it is explicit.
- **Facts are closed, not overwritten.** A changed fact gets `valid_to` and a link to its replacement.
- **No LLM inside the server** in v1. The connected agent does extraction through tool calls.
- **stdout belongs to the protocol.** In `serve`, log to stderr only. A stray `println!` breaks MCP over stdio.
- **Storage crates do not depend on each other.** Only `abhilekh-core` combines them.
- **Blocking libraries stay off the async runtime.** redb, tantivy, and embedding code run on the writer thread or `spawn_blocking`.

## Commit messages and branches

- Branch names: `v0.2.2-add-list-command`, `fix/torn-write-recovery`, `docs/typo-architecture`.
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org): `feat(log): append events with fsync`, `fix(facts): close old fact on supersede`, `docs: clarify overlap rule`, `test(log): crash at every byte`.
- Keep commits focused. Squash fixup commits before review if you can.

## Pull requests

- Fill in the pull request template.
- Link the issue or the roadmap version.
- Say how you tested it, and paste the demo command output if there is one.
- Be ready for small review changes. Reviews are about the code, never the person.

## Adding a dependency

Dependencies are a cost. Before adding one:

1. Check that the standard library or an existing dependency cannot do the job.
2. Check its license (`cargo deny check`). Allowed licenses are listed in `deny.toml`. Copyleft licenses such as AGPL are not accepted.
3. Check that it is maintained and works on Linux, macOS, and Windows.
4. Explain the choice in the pull request.

## AI-assisted contributions

Using AI tools is fine. You are responsible for the result: you must understand the code you submit, it must pass the checks above, and the license rules still apply. Do not submit code copied from projects with incompatible licenses.

## Reporting security problems

Do **not** open a public issue. Follow [SECURITY.md](SECURITY.md).

## Code of Conduct

This project follows the [Code of Conduct](CODE_OF_CONDUCT.md). By taking part, you agree to it.

## License

By contributing, you agree that your contribution is licensed under the same terms as the project: `MIT OR Apache-2.0`.
