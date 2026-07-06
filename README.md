# Rust Sample

This repository is a focused Parity demo for Rust. It intentionally separates aggregate file coverage from matching-test coverage, which is the core Parity use case.

## What this sample proves

The checked-in language-neutral `parity-coverage.json` fixture contains per-line test attribution. Parity reads that attribution and reports both all-test file coverage and how much of each source file is covered by its matching test file.

| Source file | Matching test | All-test file coverage | Matching-test coverage | Other covering tests | Why it matters |
| --- | --- | ---: | ---: | ---: | --- |
| `src/lib.rs` | `tests/lib_test.rs` | 70% | 40% | 2 | Overall coverage looks acceptable, but most covered lines come from incidental tests. |
| `src/discount.rs` | `tests/discount_test.rs` | 90% | 90% | 2 | The matching test owns nearly all of the file's coverage. |
| Project total | - | 80% | - | - | Global coverage is healthy while one file still needs better direct tests. |

Parity enforces these thresholds in `parity.yaml`:

- `min_coverage_global: 80` proves the overall project can be at a release-ready level.
- `minimum-coverage: 70` allows the intentionally weaker file to pass while still surfacing its exact percentage.
- `matched-coverage: 40` proves Parity can distinguish the matching test from incidental coverage.
- `coverage-attribution` is added automatically for the PHPUnit XML fixture and reports the number of covering tests plus how many are non-matching.

## Run locally

Install Parity from the public CLI repository:

```bash
composer global config repositories.parity vcs https://github.com/testparity/cli
composer global require testparity/parity:dev-main
```

Run the Parity proof:

```bash
parity check --config=parity.yaml --format=json
```

Expected highlights from the JSON output:

- `lib.rs`: `minimum-coverage = 70%`, `matched-coverage = 40%`, `coverage-attribution = 3|2`.
- `discount.rs`: `minimum-coverage = 90%`, `matched-coverage = 90%`, `coverage-attribution = 3|2`.
- `global_coverage = 80` and `passed = true`.

## CI

GitHub Actions installs Parity from the public `testparity/cli` repository, runs any native sample test step for this ecosystem, and then runs:

```bash
parity check --config=parity.yaml --format=json
```

No private token is required.
