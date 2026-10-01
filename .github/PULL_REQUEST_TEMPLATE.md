## What does this change?

<!-- One or two sentences. -->

## Related issue or roadmap version

<!-- For example: Closes #12, or ROADMAP v0.2.3 -->

## How was it tested?

<!-- Tests added or changed, and the demo command output if there is one. -->

## Checklist

- [ ] `cargo fmt --all --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes, with new tests for new behavior
- [ ] The demo command for this roadmap version works
- [ ] `CHANGELOG.md` has a line under "Unreleased"
- [ ] `ARCHITECTURE.md` is updated if the design changed
- [ ] If this touches the log: crash and "replay equals live state" tests still pass
- [ ] Nothing in `serve` writes to stdout except protocol messages
