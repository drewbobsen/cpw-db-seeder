# CompareTheWord - Database Seeder

A lightning-fast, synchronous Rust utility designed to parse OSIS XML Bible texts and generate a highly optimized `seed.sql` file for PostgreSQL initialization.

By decoupling the data generation from the main application stack, this seeder prevents high memory consumption and long build times on production servers.

## Features
* **Extremely Fast Parsing:** Utilizes `quick-xml` to stream through large XML files with minimal memory overhead.
* **Optimized SQL Output:** Wraps individual Bible translations in standard `BEGIN;` and `COMMIT;` SQL transaction blocks, drastically reducing disk write operations and slashing Postgres import times.
* **Automated CI/CD:** Powered by GitHub Actions. Tagging a release automatically compiles the code, generates the `seed.sql` file, and attaches it as a downloadable release asset.

## Project Structure
```text
cpw-db-seeder/
├── src/
│   └── main.rs       # The synchronous XML parser and SQL writer
├── text/
│   └── en/           # Directory containing source OSIS XML files
├── data/             # Output directory for the generated seed.sql
├── Cargo.toml
└── .github/workflows/# CI/CD release pipeline
