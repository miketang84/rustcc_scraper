# RustCC Scraper

This is a bot to scrape the reddit latest Rust lang news, automatically clear the data, and use claude.ai API to summarize the content.

## Usage

1. install geckodriver or chromedriver

2. e.g.  ./geckdriver

3. cargo run --bin rustcc_scraper

wait for the finish of it.

This also runs `call_claude` with the same date to generate the report.

4. (optional) cargo run --bin call_claude -- 2026-01-24

wait for the finish of it.
