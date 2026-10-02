# Abhilekh: Roadmap (small versions)

> Every version is **one small, finished, tested piece**. You never build a big thing at once. You build a tiny thing, prove it works, tag it, and only then start the next one.
> Read `ARCHITECTURE.md` first (read Part 0, then Parts 1, 2, 5 and 7, and you can begin).

---

## How to use this roadmap

### Rules for every version
1. **One version at a time.** Do not start v0.2.3 until v0.2.2 is tagged.
2. **One branch, one tag.** Branch `v0.2.2`, merge to `main`, tag `v0.2.2`.
3. **Definition of done** (all must be true):
   - [ ] `cargo fmt --check` passes
   - [ ] `cargo clippy --all-targets -- -D warnings` passes
   - [ ] `cargo test` passes, and new behavior has tests
   - [ ] The **Demo** command for that version works on your machine
   - [ ] `CHANGELOG.md` has one line for it
   - [ ] If the design changed, `ARCHITECTURE.md` is updated
4. **If a version takes more than double its size estimate, split it** into two smaller versions before continuing.
5. **Every worked example in `ARCHITECTURE.md` becomes a unit test.** If code and document disagree, find out which one is wrong.
6. **Never skip the crash and replay tests** (Stage 1). Everything else stands on them.

### Size guide
| Size | Time |
|---|---|
| **S** | up to about 2 hours |
| **M** | about half a day |
| **L** | about a day or more |

### Version template
```
### vX.Y.Z  Name   (size)
Build:      what to make (a few lines)
Done when:  the test or check that proves it works
Demo:       one command you can run and show
```

---

## Before you start (about 30 minutes)

- [ ] Install Rust **1.88 or newer** (the MCP SDK needs it) and use edition 2024.
- [ ] Check that **abhilekh** is free on crates.io and GitHub. If you pick another name, run one find-and-replace over both documents.
- [ ] Skim the READMEs of two similar projects (memory_mcp and kronroe are good starting points) for ideas. Do not copy code, and check licenses first (kronroe lists AGPL-3.0).
- [ ] Create the empty GitHub repo.

---

## Milestones

| Milestone | After version | What you have | Good video moment |
|---|---|---|---|
| **M0** | v0.0.4 | Clean workspace and CLI shell | Project intro and architecture tour |
| **M1** | v0.3.5 | **Working memory tool for Claude Code** (episodes plus keyword search over MCP) | "I built AI memory in Rust" demo |
| **M2** | v0.5.9 | **Facts that change over time** with history | "Memory that handles changing facts" |
| **M3** | v0.7.5 | **Measured results** against simple baselines | "Does it beat grep?" benchmarks |
| **M4** | v0.8.6 | Optional semantic (vector) search | "Do you really need vectors?" |
| **M5** | v0.9.6 | Healthy forgetting and cleanup | "Teaching AI what to forget" |
| **M6** | v1.0.0 | Public release | Launch video |

**Stop points:** M1 alone is already a useful project. You can pause at any milestone and still have something that works.

---

## Stage 0. Foundation (v0.0.x)

```
empty folder ──► workspace ──► types ──► config ──► CLI shell
                 (9 crates)   (IDs,      (config    (`abhilekh doctor`)
                              errors)     .toml)
```

**Common beginner questions**
- *Why nine crates already?* Empty crates cost nothing, and separate crates stop the stores from tangling together later.
- *Why UUIDv7?* IDs made at different times sort in time order, which helps debugging.
- *Why a `doctor` command this early?* It gives you a quick way to check paths and state at every later step.

### v0.0.1  Workspace skeleton   (S)
- **Build:** repo, workspace `Cargo.toml` with the 9 crates from Architecture Part 4 (all empty), `rust-toolchain.toml` and `rust-version = "1.88"`, edition 2024, `MIT OR Apache-2.0` license files (change if you prefer), `.gitignore`, README stub, CI running fmt, clippy, and test.
- **Done when:** `cargo build`, `cargo test`, and clippy pass locally and in CI.
- **Demo:** `cargo run -p abhilekh-cli` prints `abhilekh`.

### v0.0.2  IDs, timestamps, errors   (S)
- **Build:** in `abhilekh-types`: `EpisodeId`, `FactId`, `EntityId` (UUIDv7 newtypes), a `Timestamp` type that serializes as RFC 3339 text, and a `AbhilekhError` enum.
- **Done when:** tests show IDs are unique and sort by creation time, and a timestamp survives a JSON round trip.
- **Demo:** `cargo test -p abhilekh-types`.

### v0.0.3  Config   (S)
- **Build:** `Config` struct with the defaults from Architecture Part 12. Load `config.toml` if it exists, expand `~`, create missing directories.
- **Done when:** tests for defaults, a partial file overriding only some values, and a missing file.
- **Demo:** run twice with different `data_dir` values and see two different folders created.

### v0.0.4  CLI shell and doctor   (S)
- **Build:** `clap` setup with `abhilekh --version` and `abhilekh doctor` (prints data dir, config path, which folders exist, lock status).
- **Done when:** output is correct on a fresh machine and on one with existing data.
- **Demo:** `abhilekh doctor`.

---

## Stage 1. The event log (v0.1.x)

```
Event ──► serialize ──► one JSON line ──► append ──► fsync ──► seq number
                                            │
                    crash? ──► on next start: drop a half-written last line
```

**Common beginner questions**
- *Why a plain text log instead of a database?* You can read it, back it up, and rebuild everything else from it.
- *What is `fsync`?* It tells the operating system to really write to disk now, so a crash right after cannot lose the line.
- *What is a "torn write"?* A crash in the middle of writing a line, leaving half a line. Startup must detect and remove it.

### v0.1.1  Event types   (S)
- **Build:** the event envelope (`v`, `seq`, `ts`, `kind`, `data`) and only the `EpisodeAdded` payload for now.
- **Done when:** JSON round trip tests, plus one golden-file test that locks the format.
- **Demo:** print one event as JSON.

### v0.1.2  Append one event   (M)
- **Build:** in `abhilekh-log`: open a segment file, write one JSON line, `fsync`, return the `seq`. On open, read the last line to continue the sequence.
- **Done when:** appending 3 events gives a file with 3 lines and seqs 1, 2, 3, also after closing and reopening.
- **Demo:** small example program that appends events.

### v0.1.3  Replay   (S)
- **Build:** `replay(from_seq)` returns events in order. Bad lines produce an error that names the line number.
- **Done when:** replay returns everything in order, and `from_seq` skips earlier events.
- **Demo:** print all events from a log file.

### v0.1.4  Segments and offsets   (M)
- **Build:** start a new segment when the size limit is reached. Write `offsets.idx` (`seq → segment, byte offset`). Rebuild it if missing.
- **Done when:** with a tiny size limit in tests, 1,000 events across many segments replay in order, and deleting `offsets.idx` gets it rebuilt.
- **Demo:** append 10,000 events and jump straight to seq 7,500.

### v0.1.5  Crash safety and lock   (M)
- **Build:** on open, detect and truncate a torn last line. Add an exclusive `LOCK` file so a second writer fails with `locked`.
- **Done when:** a test cuts the log at **every byte position** and startup always recovers to a valid state. A second open returns `locked`.
- **Demo:** start two processes and see the second refuse to write.

---

## Stage 2. Episodes and keyword search (v0.2.x)

```
abhilekh add "text" ──► core::remember ──► LOG (truth)
                                     └─► chunk ──► tantivy index
abhilekh search "words" ──► BM25 ──► ranked snippets
abhilekh reindex ──► delete index ──► rebuild from LOG
```

**Common beginner questions**
- *What is BM25?* A classic ranking method that scores text by how often and how rarely query words appear. It is very good at names and exact words.
- *Why split text into chunks?* Long text matches too loosely. Small overlapping chunks match more precisely.
- *Why a checkpoint file?* It records how far the index has read, so a restart only replays the missing part.

### v0.2.1  Episode view in memory   (S)
- **Build:** fold `EpisodeAdded` events into an in-memory map at startup.
- **Done when:** a property test shows "state after replay equals state built live".
- **Demo:** `cargo test -p abhilekh-core`.

### v0.2.2  `abhilekh add` and `abhilekh list`   (S)
- **Build:** `Abhilekh::remember` in `abhilekh-core` appends the event. Add the two CLI commands.
- **Done when:** added episodes survive a restart and appear in `list`.
- **Demo:** `abhilekh add "I like dark mode"` then `abhilekh list`.

### v0.2.3  Chunking   (S)
- **Build:** split text into roughly 800-character chunks with about 100 characters of overlap, preferring sentence and paragraph boundaries.
- **Done when:** tests cover short text, very long text, one giant word, and Unicode text.
- **Demo:** print the chunks of a sample paragraph.

### v0.2.4  Keyword index   (M)
- **Build:** tantivy schema (chunk text, episode id, namespace, recorded time). Index on every `remember`, commit after each write, and **reload the reader after each commit** so new text is searchable at once.
- **Done when:** a test indexes 100 episodes and finds a known word **immediately after the last write**.
- **Demo:** add 5 episodes, inspect the index folder.

### v0.2.5  `abhilekh search`   (M)
- **Build:** top-k BM25 search with snippet output, episode id, and a namespace filter.
- **Done when:** exact words, names, and multi-word queries return the right episode first.
- **Demo:** `abhilekh search "dark mode"`.

### v0.2.6  `abhilekh reindex`   (S)
- **Build:** delete `index/` and rebuild it from the log.
- **Done when:** search results are identical before and after a reindex.
- **Demo:** `rm -rf ~/.abhilekh/index && abhilekh reindex && abhilekh search ...`.

### v0.2.7  Index checkpoint   (S)
- **Build:** a checkpoint file with the last indexed `seq`. On startup, replay only newer events into the index.
- **Done when:** a test kills the process between "log append" and "index update" and startup repairs the index.
- **Demo:** kill the process mid-write, restart, search still finds everything.

---

## Stage 3. MCP: first contact with a real agent (v0.3.x)

```
Claude Code ──stdio──► abhilekh serve ──► tools: stats, remember, search
                          (logs go to STDERR only, never stdout)
```

**Common beginner questions**
- *What is MCP?* A standard way for AI tools to call programs. Abhilekh exposes its abilities as "tools".
- *What is stdio transport?* The agent starts Abhilekh as a child process and talks to it through standard input and output.
- *Why must logs go to stderr?* Stdout carries the protocol messages. One stray `println!` breaks the connection.

*Verified October 2026: the current MCP spec is 2026-07-28 and `rmcp` 3.x implements it (no handshake, no sessions, stdio logs go to stderr). Still skim the rmcp docs before v0.3.1.*

### v0.3.1  MCP skeleton   (M)
- **Build:** `abhilekh serve` speaks MCP over stdio using `rmcp` (enable its server and stdio features, check docs.rs) and advertises one tool, `memory_stats` (counts and last seq). All logging goes to stderr.
- **Done when:** a small test client lists the tools and calls `memory_stats`.
- **Demo:** run the test client.

### v0.3.2  `memory_remember`   (S)
- **Build:** tool that appends an episode (text, namespace, session, tags, importance). If `CLAUDE_PROJECT_DIR` is set, the default namespace is `project:<folder name>`.
- **Done when:** calling it twice increases the stats count by 2.
- **Demo:** test client call.

### v0.3.3  `memory_search` (episodes only)   (S)
- **Build:** tool returning the result JSON shape from Architecture 10.2 (episodes only for now).
- **Done when:** results include text, score, source, and recorded time.
- **Demo:** test client search.

### v0.3.4  Connect to Claude Code   (S)
- **Build:** register the server with `claude mcp add --transport stdio abhilekh --scope user -- abhilekh serve` and check it with `claude mcp list`. Write `docs/claude-code-setup.md`.
- **Done when:** in one session you ask the agent to remember something, and in a **new session** it recalls it using the tool.
- **Demo:** the two-session test. **This is your first real milestone.**

### v0.3.5  MCP hardening   (S)
- **Build:** error codes (`invalid_input`, `not_found`, `locked`, `internal`), input size limits, `tracing` to stderr, clean shutdown.
- **Done when:** bad input produces a clear error, never a crash.
- **Demo:** send an empty text and an oversized text.

---

## Stage 4. Core notes (v0.4.x)

```
core/user-preferences.md ◄──► CoreNoteWritten events in the LOG
        │
        └─► SessionStart hook + memory_core_read tool  (agent gets the notes at session start)
```

**Common beginner questions**
- *Why Markdown files?* You can open and edit them yourself, and they work like the always-loaded rule files agents already know.
- *Why also log every change?* If a note is deleted or ruined, you can restore an older version.
- *What is an atomic write?* Write to a temporary file, then rename it, so a crash never leaves a half-written note.

### v0.4.1  Read and list notes   (S)
- **Build:** `abhilekh-notes`: list `.md` files, parse simple front matter (`title`, `pinned`).
- **Done when:** tests with and without front matter.
- **Demo:** `abhilekh notes list`.

### v0.4.2  Write a note   (M)
- **Build:** atomic file write plus a `CoreNoteWritten` event with the full body.
- **Done when:** the file and the log agree, and a crash test leaves no half file.
- **Demo:** `abhilekh notes write prefs "Prefer dark mode"`.

### v0.4.3  MCP core tools and resource   (S)
- **Build:** `memory_core_read`, `memory_core_write`, and an optional `abhilekh://core` resource. Resources are normally attached by the user or client, so do not rely on them to load automatically.
- **Done when:** an agent can write a note in one session and read it in the next.
- **Demo:** two-session test again.

### v0.4.4  Restore notes from the log   (S)
- **Build:** `reindex` recreates the latest version of every note from events.
- **Done when:** deleting `core/` and running `reindex` restores all notes.
- **Demo:** delete the folder, reindex, notes return.

### v0.4.5  Session-start hook   (S)
- **Build:** `abhilekh core print --hook-json` reads `core/*.md` directly (read-only, no lock needed) and prints `{"hookSpecificOutput":{"hookEventName":"SessionStart","additionalContext":"..."}}`. Register it as a SessionStart **command** hook in `~/.claude/settings.json`. (The hooks docs say `mcp_tool` hooks are not available for SessionStart.)
- **Done when:** a brand-new Claude Code session already knows a core note without any tool call. Test on your Claude Code version, since hook output handling has had bugs in some releases. Backup: one `CLAUDE.md` line telling the agent to call `memory_core_read` first.
- **Demo:** start a fresh session and ask "what are my preferences?".

---

## Stage 5. Facts that change over time (v0.5.x)

```
assert_fact(User, lives_in, Paris)   ──► fact F1  valid_from Jan, valid_to None
assert_fact(User, lives_in, London)  ──► F1.valid_to = Jul ; F1.superseded_by = F2
                                         F2  valid_from Jul, valid_to None
as_of(March) → F1     as_of(now) → F2
```

**Common beginner questions**
- *What is a fact here?* A small structured statement: subject, predicate, object, plus the time window when it was true.
- *Why not just overwrite the old value?* Then you could never answer "what was true in March?", and you could never fix a wrong update.
- *What about facts with many values?* Mark those predicates as multi-valued so a new value does not close the old one.

### v0.5.1  Pick the store and define a trait   (S)
- **Build:** use `redb` 4.x as the default (decided in Architecture Part 0.3; switch to SQLite for this tier only if redb blocks you). Define a `FactStore` trait and an in-memory implementation for tests.
- **Done when:** the trait is small (apply event, query by entity, query by time) and the in-memory version passes basic tests.
- **Demo:** `cargo test -p abhilekh-facts`.

### v0.5.2  Entities and aliases   (M)
- **Build:** `EntityUpserted` event, name normalization (lowercase, trim), alias lookup, `find_entity(name)`.
- **Done when:** "Darshan", "darshan", and an added alias all resolve to one entity.
- **Demo:** `abhilekh entity add` then look up by alias.

### v0.5.3  Assert a fact   (M)
- **Build:** `FactAsserted` event, fact stored with evidence links; entity objects create edges.
- **Done when:** a fact can be stored and read back with its source episodes.
- **Demo:** `abhilekh facts assert User lives_in Paris`.

### v0.5.4  The validity rule   (S)
- **Build:** `valid(fact, T)` exactly as in Architecture 8.4.
- **Done when:** both worked examples from the document pass, plus edge cases (exact boundary, open-ended fact).
- **Demo:** unit tests.

### v0.5.5  Supersede logic   (M)
- **Build:** same subject and predicate with a different object closes the old fact and links `superseded_by`. Same object adds evidence only. Multi-valued predicates skip closing.
- **Done when:** the Paris to London scenario works, and a multi-valued predicate keeps both.
- **Demo:** run the London story from Architecture 1.3.

### v0.5.6  Invalidate and history   (S)
- **Build:** `FactInvalidated` event and a history query returning the version chain, oldest first.
- **Done when:** history shows Paris then London with correct windows.
- **Demo:** `abhilekh history User lives_in`.

### v0.5.7  Persist in the real store and rebuild   (M)
- **Build:** real `redb` (or SQLite) implementation of `FactStore`. Store `last_applied_seq` in the same transaction. `reindex` rebuilds it from the log.
- **Done when:** a property test shows "replay equals live state", and killing the process mid-write recovers correctly.
- **Demo:** delete `state/`, reindex, facts return.

### v0.5.8  MCP fact tools   (M)
- **Build:** `memory_assert_fact`, `memory_invalidate`, `memory_history`, `memory_get`.
- **Done when:** an agent stores a fact, later stores a changed fact, and history is correct.
- **Demo:** two-session test with a changing fact.

### v0.5.9  CLI fact commands   (S)
- **Build:** `abhilekh facts` (list, filter by entity and `--as-of`) and `abhilekh history`.
- **Done when:** output is readable and correct on the London story.
- **Demo:** `abhilekh facts --entity User --as-of 2025-03-01`.

---

## Stage 6. Smarter search (v0.6.x)

```
query ─► PLAN ─► keyword ─┐
            │             ├─► RRF fuse ─► valid at as_of ─► add evidence ─► strength ─► results
            └─► entities ─┘
                (edge expansion)
```

**Common beginner questions**
- *Why not one search method?* Each finds different things. Keywords find exact names, entity links find related facts, and fusing them is more reliable than either alone.
- *What is RRF?* Merging ranked lists by rank position only, so you never compare unrelated scores.
- *Does strength delete old memories?* No. It only changes ranking.

### v0.6.1  Index fact statements   (S)
- **Build:** add each fact's readable `statement` to the keyword index, tagged as a fact.
- **Done when:** a search finds facts as well as episodes.
- **Demo:** `abhilekh search "London"`.

### v0.6.2  Planner: entity mentions   (M)
- **Build:** find 1-3 word chunks in the query that match entity names or aliases.
- **Done when:** "where does Darshan live" finds the entity, and unrelated words do not.
- **Demo:** print the plan for a query.

### v0.6.3  Planner: time phrases   (M)
- **Build:** rules for "yesterday", "last week", "last month", "in June", "in June 2025", "since March", and ISO dates, using an injectable "now" so tests are stable.
- **Done when:** a table of phrase to expected range passes for a fixed "now".
- **Demo:** print the plan for "what did we decide last week".

### v0.6.4  Edge expansion   (M)
- **Build:** from seed entities, collect facts within 1-2 hops of edges.
- **Done when:** a question about a project also surfaces facts about its owner.
- **Demo:** query a project name and see linked facts.

### v0.6.5  RRF fusion   (S)
- **Build:** merge keyword and entity result lists with `k = 60`.
- **Done when:** both RRF worked examples from Architecture 8.3 pass as tests.
- **Demo:** unit tests.

### v0.6.6  `as_of` filtering in search   (S)
- **Build:** default to facts valid now. Support `as_of` (a single instant), `include_history`, and the **overlap rule** for time phrases (Architecture 8.4.1).
- **Done when:** the March 2025 vs now scenario returns Paris vs London, and both overlap worked examples from 8.4.1 pass as tests.
- **Demo:** `abhilekh search "where does the user live" --as-of 2025-03-01`.

### v0.6.7  Evidence expansion   (M)
- **Build:** attach source episodes to facts and derived facts to episodes, capped per result.
- **Done when:** the London fact result includes the "We just moved to London" episode.
- **Demo:** search output shows evidence lines.

### v0.6.8  Strength scoring   (S)
- **Build:** access tracking and the strength formula with `α = 0.3`. **Decision to make here:** store access counts in the state store, not the log (so the log stays small). A rebuild resets counts to zero, and that is acceptable.
- **Done when:** both strength worked examples from Architecture 8.6 pass.
- **Demo:** search the same item repeatedly and watch its score rise.

### v0.6.9  End-to-end London test   (S)
- **Build:** one golden test that runs the full walkthrough from Architecture 8.7.
- **Done when:** both queries return the documented results with evidence.
- **Demo:** `cargo test london`.

### v0.6.10  Augmented index keys   (S)
- **Build:** when indexing, also add entity names and aliases found in the text, and date markers (`2025-07`, `July 2025`, `2025-07-03`), as extra searchable text (Architecture 8.8).
- **Done when:** a question like "what happened in July 2025" finds the right episode. **Keep it only if the Stage 7 eval shows a gain.**
- **Demo:** compare results with the feature on and off.

### v0.6.11  Time-ordered search   (S)
- **Build:** `since`, `until`, and `order = relevance | time` on search (Architecture 8.9), in both the CLI and `memory_search`.
- **Done when:** "what did we do last week, in order" returns episodes oldest to newest.
- **Demo:** `abhilekh search "deploy" --since 2026-09-01 --order time`.

---

## Stage 7. Measure it (v0.7.x)

```
scenarios.jsonl ─► run through each configuration ─► metrics ─► report table
   configs: grep │ BM25 │ BM25+entities │ full
```

**Common beginner questions**
- *Why measure before adding vectors?* You need a baseline to know whether vectors actually help.
- *What is Recall@5?* How often the correct memory appears in the top 5 results.
- *What is update correctness?* After a fact changes, does search return the new value and not the old one?

### v0.7.1  Scenario format   (S)
- **Build:** a JSONL format: a list of events to load, then questions with expected item ids and (optional) `as_of`.
- **Done when:** a loader parses and validates a sample file.
- **Demo:** `abhilekh-eval validate scenarios/`.

### v0.7.2  Write the scenarios   (M)
- **Build:** about 30 hand-written scenarios, including changing facts, old evidence, several sessions, and time questions.
- **Done when:** every scenario has an unambiguous expected answer.
- **Demo:** file count and a quick review.

### v0.7.3  Metrics   (S)
- **Build:** Recall@1/5/10, MRR, update correctness, as-of correctness, latency.
- **Done when:** metrics are unit-tested on tiny hand-made inputs.
- **Demo:** run on one scenario and print numbers.

### v0.7.4  Baselines   (M)
- **Build:** baseline A (plain grep over the log) and baseline B (BM25 only).
- **Done when:** all configurations run on all scenarios without errors.
- **Demo:** `abhilekh-eval run --all`.

### v0.7.5  Report   (S)
- **Build:** a results table written to `docs/EVAL.md` and copied into the README. Include bad results.
- **Done when:** you can see clearly where the full system beats grep and where it does not.
- **Demo:** open the table.

---

## Stage 8. Optional vectors (v0.8.x)

```
text ─► Embedder ─► vector ─► vectors.idx (file)         [always rebuildable]
query ─► Embedder ─► compare against all vectors ─► ranked list ─► joins the RRF fusion
```

**Common beginner questions**
- *Why are vectors optional?* The research found flat vector search is weak on its own, and the system must still work without them.
- *What does an embedding model do?* It turns text into a list of numbers so similar meanings end up close together.
- *Why brute force?* Comparing against every vector is simple and fast enough for tens of thousands of items. Add a fancier index only after measuring.

### v0.8.1  Embedder trait and fake embedder   (S)
- **Build:** the `Embedder` trait and a deterministic fake embedder for tests.
- **Done when:** the same text always gives the same vector, and dimensions are checked.
- **Demo:** unit tests.

### v0.8.2  Local embedding model   (L)
- **Build:** local embeddings behind a cargo feature (for example `embed-local`). Option A: `fastembed` 5.x (ONNX, default BGE-small-en-v1.5, 384 dimensions, downloads on first run into `models/`). Option B: Candle, pure Rust. fastembed pulls in the ONNX runtime, which weakens the single-binary goal, so keep it feature-gated.
- **Done when:** embedding a batch works, and similar sentences score higher than unrelated ones.
- **Demo:** print similarity for 3 sentence pairs.

### v0.8.3  Vector file and brute-force search   (M)
- **Build:** store vectors in `index/vectors.idx` and search top-k by cosine similarity.
- **Done when:** search over 10,000 fake vectors returns the correct nearest neighbor.
- **Demo:** time the search.

### v0.8.4  Background embedding   (M)
- **Build:** a queue that embeds new chunks and facts after the write returns. `reindex --vectors` rebuilds everything.
- **Done when:** writes stay fast, and killing the process never breaks the log or keyword index.
- **Demo:** add 1,000 episodes and watch the queue drain.

### v0.8.5  Fuse vectors and re-run eval   (M)
- **Build:** add the vector list into RRF, then run the Stage 7 evaluation.
- **Done when:** the report shows whether vectors help. **Keep the feature only if the numbers improve.**
- **Demo:** the updated results table.

### v0.8.6  Near-duplicate detection   (S)
- **Build:** cosine check with a 0.95 threshold, and a normalized-text fallback when vectors are off.
- **Done when:** both cosine worked examples from Architecture 9.1 pass.
- **Demo:** add the same sentence twice and see one item.

---

## Stage 9. Healthy forgetting (v0.9.x)

```
low-strength derived items ─► ARCHIVE (hidden from default recall)
near-identical items       ─► DEDUPE  (linked, original kept)
everything logged as events ─► every action can be UNDONE
raw log ─────────────────────► never touched
```

**Common beginner questions**
- *Does forgetting delete my memories?* No. It hides derived items from default search. The raw log is untouched.
- *Why only one entity at a time?* Small, local cleanups are cheap and safe. Whole-store rewrites are slow and risky.
- *Why not summarize old memories?* Summaries lose exact details. The optional digest is a separate item, never a replacement.

### v0.9.1  Archive and restore   (S)
- **Build:** `ItemArchived` and `ItemRestored` events. Archived items skip default recall unless `include_archived=true`.
- **Done when:** archive then restore returns the item to results.
- **Demo:** `abhilekh archive <id>` / `abhilekh restore <id>`.

### v0.9.2  Pin and unpin   (S)
- **Build:** `ItemPinned` and `ItemUnpinned` events. Pinned items use `strength = 1` and are never archived.
- **Done when:** a pinned old item still ranks normally.
- **Demo:** pin an item and run maintenance.

### v0.9.3  Decay-based archive job   (M)
- **Build:** a maintenance pass over one entity or session window that archives low-strength derived items older than 24 hours.
- **Done when:** a simulated-time test archives the right items and skips pinned and recent ones.
- **Demo:** `abhilekh maintain --entity User`.

### v0.9.4  Dedupe merge   (M)
- **Build:** merge near-duplicates by linking them, keeping the original reachable.
- **Done when:** duplicates disappear from default results but remain in history.
- **Demo:** add duplicates, run maintenance, check results.

### v0.9.5  Forgetting probes   (M)
- **Build:** eval scenarios that simulate weeks passing, run maintenance, then check that important items survive and the **raw log is byte-identical** to before.
- **Done when:** probes pass and appear in the report.
- **Demo:** `abhilekh-eval probes`.

### v0.9.6  Digests (optional)   (L)
- **Build:** a short digest per entity or session window, stored as a separate linked item. Skip this if the eval does not show a benefit.
- **Done when:** digests improve or at least do not hurt results, and raw items remain searchable.
- **Demo:** search shows a digest plus its source items.

---

## Stage 10. Release (v1.0.x)

```
HTTP transport ─► export/import ─► purge ─► docs + numbers ─► binaries ─► launch
```

**Common beginner questions**
- *Why localhost only for HTTP?* Memory can contain private data. Keep it on your machine unless you deliberately expose it, and always require a token.
- *Why is `purge` separate?* It is the one command that really deletes raw data, so it should be explicit and hard to trigger by accident.

### v1.0.0-rc1  HTTP transport   (M)
- **Build:** Streamable HTTP through `rmcp` (stateless under spec 2026-07-28: requests carry `Mcp-Method` and `Mcp-Name` headers, no session id). Bind to `127.0.0.1`, validate the `Origin` header, and require a bearer token. The spec prefers OAuth for HTTP, so document the static token as a local-only shortcut.
- **Done when:** requests without the token are rejected.
- **Demo:** call a tool with and without the token.

### v1.0.0-rc2  Export and import   (S)
- **Build:** `abhilekh export` writes a portable JSONL bundle, and `abhilekh import` loads it into an empty store.
- **Done when:** export, wipe, import, reindex gives identical search results.
- **Demo:** run that round trip.

### v1.0.0-rc3  Purge   (M)
- **Build:** `abhilekh purge <id>` writes a `Purged` tombstone and rewrites the affected segment safely.
- **Done when:** the text is gone from the log and indexes, and a crash during purge cannot corrupt anything.
- **Demo:** purge one episode and confirm it is gone everywhere.

### v1.0.0-rc4  Documentation and numbers   (S)
- **Build:** final README with install steps, Claude Code setup, evaluation table, and donation information instead of contact details. Keep `ARCHITECTURE.md` up to date.
- **Done when:** a new person can install and use it from the README alone.
- **Demo:** follow your own README on a clean machine.

### v1.0.0-rc5  Binaries and release CI   (M)
- **Build:** prebuilt binaries for Linux, macOS, and Windows, `cargo install` instructions, and a release workflow.
- **Done when:** a tagged release produces downloadable binaries that run.
- **Demo:** download and run one.

### v1.0.0  Launch   (S)
- **Build:** tag `v1.0.0`, publish the release, post the launch video, and share the write-ups.
- **Done when:** the release is public.
- **Demo:** the release page.

---

## Progress tracker

Copy this into your repo and tick items as you go.

```
Stage 0  [x] v0.0.1  [x] v0.0.2  [ ] v0.0.3  [ ] v0.0.4
Stage 1  [ ] v0.1.1  [ ] v0.1.2  [ ] v0.1.3  [ ] v0.1.4  [ ] v0.1.5
Stage 2  [ ] v0.2.1  [ ] v0.2.2  [ ] v0.2.3  [ ] v0.2.4  [ ] v0.2.5  [ ] v0.2.6  [ ] v0.2.7
Stage 3  [ ] v0.3.1  [ ] v0.3.2  [ ] v0.3.3  [ ] v0.3.4  [ ] v0.3.5      ← M1
Stage 4  [ ] v0.4.1  [ ] v0.4.2  [ ] v0.4.3  [ ] v0.4.4  [ ] v0.4.5
Stage 5  [ ] v0.5.1  [ ] v0.5.2  [ ] v0.5.3  [ ] v0.5.4  [ ] v0.5.5  [ ] v0.5.6  [ ] v0.5.7  [ ] v0.5.8  [ ] v0.5.9  ← M2
Stage 6  [ ] v0.6.1  [ ] v0.6.2  [ ] v0.6.3  [ ] v0.6.4  [ ] v0.6.5  [ ] v0.6.6  [ ] v0.6.7  [ ] v0.6.8  [ ] v0.6.9  [ ] v0.6.10  [ ] v0.6.11
Stage 7  [ ] v0.7.1  [ ] v0.7.2  [ ] v0.7.3  [ ] v0.7.4  [ ] v0.7.5      ← M3
Stage 8  [ ] v0.8.1  [ ] v0.8.2  [ ] v0.8.3  [ ] v0.8.4  [ ] v0.8.5  [ ] v0.8.6  ← M4
Stage 9  [ ] v0.9.1  [ ] v0.9.2  [ ] v0.9.3  [ ] v0.9.4  [ ] v0.9.5  [ ] v0.9.6  ← M5
Stage 10 [ ] rc1  [ ] rc2  [ ] rc3  [ ] rc4  [ ] rc5  [ ] v1.0.0           ← M6
```

**Start here:** v0.0.1. Nothing else matters until it is done.
