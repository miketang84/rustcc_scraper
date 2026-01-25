# Repository Guidelines

## Project Structure & Module Organization
- `src/tmp/main.rs`: stage 1 scraper binary (`rustcc_scraper`), uses WebDriver to collect r/rust posts.
- `src/tmp/claude.rs`: Claude API client used by the summarizer.
- `src/call_claude.rs`: stage 2 summarizer binary (`call_claude`), reads stage 1 output and writes reports.
- `tmp/`: intermediate scrape outputs like `tmp/rust_diary-YYYY-MM-DD.txt`.
- `outputs/`: final Markdown reports like `outputs/rust_diary-YYYY-MM-DD.md`.
- `rust_diary_tmpl.md`: report template.

## Build, Test, and Development Commands
- `cargo build --release`: build optimized binaries.
- `./geckodriver &` (or `chromedriver`): start WebDriver on `http://localhost:4444`.
- `cargo run --bin rustcc_scraper [YYYY-MM-DD]`: run the scraper; date defaults to today.
- `cargo run --bin call_claude [YYYY-MM-DD]`: run the summarizer; date defaults to today.
- `cargo test`: run tests if/when added.

## Coding Style & Naming Conventions
- Rust 2021 edition with `rustfmt` defaults (4-space indentation, trailing commas where idiomatic).
- Naming: `snake_case` for modules/functions, `PascalCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Keep logging consistent with the existing `log`/`env_logger` usage.

## Testing Guidelines
- No automated tests are currently present. If adding tests, prefer `tests/` for integration tests and inline `#[cfg(test)]` modules for unit tests.
- Name tests descriptively (e.g., `test_parse_entry_format`) and ensure they run with `cargo test`.

## Commit & Pull Request Guidelines
- Commit history uses short, lowercase summaries (e.g., "update.", "fix a bug."); follow that style unless a clearer imperative message is needed.
- PRs should describe which stage (scraping or summarization) was changed, include any affected output files, and mention API behavior changes.
- Keep `.env` files and API keys out of commits.

## Configuration & Runtime Notes
- Create a `.env` file with `ANTHROPIC_API_KEY` for summarization.
- Ensure a WebDriver server is running on `localhost:4444` before stage 1.
