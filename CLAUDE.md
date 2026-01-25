# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RustCC Scraper is a two-stage pipeline that scrapes Rust news from Reddit's r/rust subreddit and generates Chinese-language daily reports ("Rust日报") using the Claude API.

## Build and Run Commands

```bash
# Build
cargo build --release

# Stage 1: Web scraping (requires geckodriver running on localhost:4444)
./geckodriver &
cargo run --bin rustcc_scraper [YYYY-MM-DD]

# Stage 2: Claude API summarization
cargo run --bin call_claude [YYYY-MM-DD]
```

Date argument is optional and defaults to today's date.

## Environment Setup

- Requires `.env` file with `ANTHROPIC_API_KEY`
- WebDriver server (geckodriver or chromedriver) must be running on `http://localhost:4444`

## Architecture

**Two-stage pipeline:**

1. **Scraping stage** (`src/tmp/main.rs` → `rustcc_scraper` binary):
   - Uses Fantoccini to control Firefox/Chrome via WebDriver
   - Scrapes top 15 posts from old.reddit.com/r/rust
   - Extracts content using DOM density analysis (`dom-content-extraction`)
   - Outputs to `tmp/rust_diary-{date}.txt`

2. **Summarization stage** (`src/call_claude.rs` → `call_claude` binary):
   - Reads intermediate file from stage 1
   - Filters entries by length (300-5000 chars)
   - Calls Claude API for Chinese summarization
   - Uses `rust_diary_tmpl.md` template
   - Outputs to `outputs/rust_diary-{date}.md`

**Data format between stages:**
- Entries separated by `======>`
- URL and content separated by `-->>-->>`

**Key modules:**
- `src/tmp/claude.rs` - Claude API client (uses claude-3-sonnet model)
- `src/tmp/main.rs` - Web scraping with Fantoccini WebDriver

## Key Constants

- `SLEEP_TIME: 1` - Delay between requests (seconds)
- `ITEM_LEN: 15` - Number of Reddit posts to scrape
- Content truncated to 5000 chars before summarization
