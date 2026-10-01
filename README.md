<div align="center">

# Abhilekh

**A local-first memory server for AI agents, written in Rust.**

Raw history stays intact, facts carry time windows, and every index can be rebuilt from one append-only log.

[![CI](https://github.com/AarambhDevHub/abhilekh/actions/workflows/ci.yml/badge.svg)](https://github.com/AarambhDevHub/abhilekh/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)
[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange)](https://www.rust-lang.org)

[Architecture](ARCHITECTURE.md) · [Roadmap](ROADMAP.md) · [Contributing](CONTRIBUTING.md) · [Changelog](CHANGELOG.md)

</div>

> **Status: pre-alpha, built in public in small versions.** Nothing below is released yet. The [roadmap](ROADMAP.md) shows exactly which version adds which feature, and this README is updated as each milestone lands.

*Abhilekh* (अभिलेख) means "a written record".

---

## What is it?

An AI agent forgets everything when a session ends. Abhilekh is a single-binary server that any [MCP](https://modelcontextprotocol.io)-compatible agent (Claude Code, Codex, Cursor and others) can use to store what happened, remember facts that change over time, and search all of it later. Everything stays on your machine.

## Why another memory server?

Many memory tools put everything into one vector store. Recent research on agent memory points to three problems with that:

- **Old evidence gets lost.** Similarity search gets worse as the answer moves further back in time.
- **Changed facts go stale.** "Lives in Paris" and later "moved to London" should not both come back as true.
- **Summaries destroy detail.** Replacing raw text with a summary loses exact names, dates, and numbers.

Abhilekh is built around five rules:

1. **Keep the raw text forever.** Every write is an immutable event.
2. **The log is the truth.** Every index can be deleted and rebuilt from it.
3. **Each kind of memory lives where it fits.** Rules in Markdown, facts in a structured store, text in a keyword index, vectors optional.
4. **Facts have a time window.** A changed fact is closed, never overwritten, so history stays answerable.
5. **No LLM inside the server.** The connected agent already is one, so it does the extraction through tool calls.

### Honest note on prior art

This space is crowded. Open-source Rust projects such as memory_mcp, kronroe, memory-mcp-1file, and jamjet-engram already tackle agent memory. Abhilekh is a learning-first, measured, small-steps take on the problem: its results are compared against a plain-`grep` baseline and published, including the bad numbers. See [ARCHITECTURE.md](ARCHITECTURE.md), Part 0.

## How it works

```
 AI agent ──MCP (stdio)──►  abhilekh serve
                                │
              ┌─────────────────┼──────────────────┐
              ▼                 ▼                  ▼
        Event log (truth)   Core notes (.md)   Derived stores
        JSONL, append-only  always-loaded      facts (redb), keyword
                            rules              index (tantivy), optional
                                               vectors
```

## Planned features by milestone

| Milestone | Roadmap version | What you get | Status |
|---|---|---|---|
| M0 | v0.0.4 | Workspace, config, CLI shell | ☐ |
| **M1** | v0.3.5 | Episodes plus keyword search over MCP, working with Claude Code | ☐ |
| M2 | v0.5.9 | Facts that change over time, with history and `as_of` queries | ☐ |
| M3 | v0.7.5 | Evaluation against simple baselines | ☐ |
| M4 | v0.8.6 | Optional semantic (vector) search | ☐ |
| M5 | v0.9.6 | Archiving, dedupe, and forgetting probes | ☐ |
| M6 | v1.0.0 | Public release, HTTP transport, binaries | ☐ |

## Quick start

> Available from milestone M1. Until the first release, build from source.

```bash
git clone https://github.com/AarambhDevHub/abhilekh
cd abhilekh
cargo build --release

# Register with Claude Code for all your projects
claude mcp add --transport stdio abhilekh --scope user -- /full/path/to/target/release/abhilekh serve
claude mcp list
```

Requires Rust 1.88 or newer.

## MCP tools (planned)

| Tool | Purpose |
|---|---|
| `memory_remember` | Store a raw episode |
| `memory_assert_fact` | Store a structured fact with a time window |
| `memory_search` | Hybrid search with `as_of`, `since`, `until`, and evidence |
| `memory_history` | Show how a fact changed over time |
| `memory_core_read` / `memory_core_write` | Read and edit always-loaded Markdown notes |
| `memory_invalidate` / `memory_forget` | End a fact, or archive a derived item |
| `memory_stats` / `memory_maintain` | Health and bounded cleanup |

Full details are in [ARCHITECTURE.md](ARCHITECTURE.md), Part 10.

## Repository layout

```
crates/
  abhilekh-types   shared types, IDs, errors, config
  abhilekh-log     append-only event log
  abhilekh-notes   core Markdown notes
  abhilekh-facts   entities, facts, edges
  abhilekh-index   keyword index and optional vectors
  abhilekh-core    writer, planner, search, maintenance
  abhilekh-mcp     MCP server
  abhilekh-eval    benchmarks and probes
  abhilekh-cli     the `abhilekh` binary
```

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md): how everything fits together, with worked examples
- [ROADMAP.md](ROADMAP.md): small versions, each finished and tested before the next
- [CONTRIBUTING.md](CONTRIBUTING.md): how to help
- [SECURITY.md](SECURITY.md): how to report a vulnerability

## Evaluation

Results will be published in `docs/EVAL.md` at roadmap version v0.7.5.

## Contributing

Contributions are welcome, especially bug reports, tests, evaluation scenarios, Windows testing, and documentation fixes. Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

## Support the project

Abhilekh is free and open source, built by Aarambh Dev Hub. If it helps you, you can support the work here:

- [GitHub Sponsors](https://github.com/sponsors/aarambh-darshan)
- [Buy Me a Coffee](https://www.buymeacoffee.com/aarambhdevhub)
- [Razorpay](https://razorpay.me/@aarambhdevhub)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project, as defined in the Apache-2.0 license, will be dual licensed as above, without any additional terms or conditions.

## Acknowledgements and further reading

- Zhou et al., *Are We Ready For An Agent-Native Memory System?* (arXiv 2606.24775)
- Rasmussen et al., *Zep: A Temporal Knowledge Graph Architecture for Agent Memory* (arXiv 2501.13956)
- Packer et al., *MemGPT* (arXiv 2310.08560)
- Letta, *Benchmarking AI Agent Memory: Is a Filesystem All You Need?*
- The Model Context Protocol specification and the official Rust SDK (`rmcp`)
