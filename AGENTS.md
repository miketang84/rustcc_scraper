# Repository Guidelines

## Project Overview

`rustcc_scraper` is a two-stage Rust pipeline that scrapes the latest posts from
[r/rust](https://old.reddit.com/r/rust/) and generates Chinese-language daily
reports ("Rust日报") via the Claude API. Final reports are published to the
Rustcc community.

---

## Project Structure

```
rustcc_scraper/
├── src/
│   ├── main.rs          # Stage 1: WebDriver scraper (rustcc_scraper binary)
│   ├── call_claude.rs   # Stage 2: Claude summarizer (call_claude binary)
│   └── claude.rs        # Claude API client module
├── tmp/                 # Intermediate scraped text (rust_diary-YYYY-MM-DD.txt)
├── outputs/             # Final Markdown reports (rust_diary-YYYY-MM-DD.md)
├── rust_diary_tmpl.md   # Report template (uses $DATE$ and $CONTENT$ placeholders)
├── Cargo.toml
└── .env                 # Local secrets (not committed)
```

**Data flow between stages:**
- Entries are separated by `======>`
- Within each entry, URL and body are separated by `-->>`

---

## Environment Setup

Create a `.env` file in the project root:

```
ANTHROPIC_API_KEY=sk-ant-...
```

A WebDriver server must be running on `http://localhost:4444` before the
scraper starts:

```bash
./geckodriver &   # or chromedriver
```

---

## Build, Test, and Run Commands

```bash
# Check compilation
cargo check

# Build both binaries (release)
cargo build --release

# Run the full pipeline (scrape + summarize) for today
cargo run --bin rustcc_scraper

# Run for a specific date
cargo run --bin rustcc_scraper -- 2026-03-13

# Run in unattended daily mode (fires at 02:00 by default)
cargo run --bin rustcc_scraper -- --daily
cargo run --bin rustcc_scraper -- --daily --clock 08:30

# Run only the summarization stage (Stage 2)
cargo run --bin call_claude -- 2026-03-13

# Run tests
cargo test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt
```

---

## Coding Style & Naming Conventions

- **Edition**: Rust 2021.
- **Formatting**: enforced by `rustfmt` — run `cargo fmt` before committing.
- **Linting**: `cargo clippy -- -D warnings` must pass with no errors.
- **Naming**: follow standard Rust conventions — `snake_case` for functions and
  variables, `PascalCase` for types and traits, `SCREAMING_SNAKE_CASE` for
  constants.
- **Error handling**: use `Result` with `Box<dyn std::error::Error>` at binary
  entry points; avoid `unwrap()` in production paths — prefer `?` propagation
  or `log::warn!` with a fallback.
- **Logging**: use the `log` crate macros (`log::info!`, `log::warn!`,
  `log::error!`). Do not use `println!` in production code paths.

---

## Testing Guidelines

The project currently has no automated test suite. When adding tests:

- Place unit tests in an inline `#[cfg(test)]` module within the source file.
- Name test functions descriptively: `test_<function>_<scenario>`.
- Run all tests with `cargo test`.
- Integration tests that require a live WebDriver or Claude API key should be
  gated with `#[ignore]` and documented accordingly.

---

## Commit & Pull Request Guidelines

Commit messages in this repository are short and imperative. Follow the same
pattern:

```
fix a bug in content extraction
add daily scheduler flag
update deps to latest versions
```

- One logical change per commit.
- Reference any relevant issue or context in the commit body if the subject
  line is insufficient.
- For pull requests: include a brief description of what changed and why,
  and note any external requirements (e.g., new env vars, updated
  geckodriver version).

---

## Key Constants (src/main.rs)

| Constant     | Value | Purpose                              |
|--------------|-------|--------------------------------------|
| `SLEEP_TIME` | `3`   | Seconds to wait between page fetches |
| `ITEM_LEN`   | `15`  | Number of Reddit posts scraped       |

Content is truncated to **5 000 characters** before being sent to the Claude
API. Entries shorter than **300 characters** are skipped entirely.
