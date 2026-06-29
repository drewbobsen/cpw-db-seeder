# CompareTheWord - Database Seeder

A lightning-fast, synchronous Rust utility designed to parse OSIS XML Bible texts and generate a highly optimized `seed.sql` file for PostgreSQL initialization.

By decoupling the data generation from the main application stack, this seeder prevents high memory consumption and long build times on production servers.

## Features
* **Extremely Fast Parsing:** Utilizes `quick-xml` to stream through large XML files with minimal memory overhead.
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
```

## Local Development

**Prerequisites:**
* Rust toolchain
* Source XML files placed in ./text/en, from Repo [https://github.com/bzerangue/osis-bibles]

**Running the Seeder:**
TO compile and run the seeder locally, execute:
```
cargo run --release
```

Seeder will scan the etxt directory, parse the translations and output a ready-to-use seed.sql file in the ./data/ directory.

## Production Usage

You DO NOT need to run this binary on your produciton server. Instead, pull the pre-generated SQL directly from the GitHub Releases page into your database initialization folder:

```
wget https://github.com/drewbobsen/cpw-db-seeder/releases/latest/download/seed.sql -O ./data/seed.sql
```



