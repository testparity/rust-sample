<p align="center">
  <img src="https://testparity.com/brand/testparity-readme-banner.jpg" alt="TestParity" width="100%">
</p>

# Rust Sample

[![Parity Sample](https://github.com/testparity/rust-sample/actions/workflows/parity.yml/badge.svg)](https://github.com/testparity/rust-sample/actions/workflows/parity.yml)

This repository is an end-to-end Parity demo for Rust and Cargo. It runs the real Cargo suite, generates one Cobertura report per integration-test target with `cargo-llvm-cov`, and lets Parity prove which test owns each module.

## What this sample proves

`lib.rs` reaches 80% when every test is combined, but `lib_test.rs` owns only 40%. `discount_test.rs` supplies the other coverage by using library helpers inside its own scenario.

| Source file | Belonging test | All tests | Belonging test | Attribution |
| --- | --- | ---: | ---: | ---: |
| `src/lib.rs` | `tests/lib_test.rs` | 80% | 40% | `2|1` |
| `src/discount.rs` | `tests/discount_test.rs` | 100% | 100% | `1|0` |
| Project total | - | 84.21% | - | - |

`coverage-attribution` uses `total covering tests|non-matching covering tests`. The `2|1` result proves that another Cargo test target contributes coverage to `lib.rs`.

## How the proof works

The matching library test calls `double` and `increment`. The discount test also calls `square` and `absolute`, while `triple` remains intentionally uncovered. `parity test` invokes each integration-test target separately through `cargo llvm-cov`, writes `.parity/per-test`, and runs `parity check` automatically.

## Run locally

Requirements: stable Rust, `cargo-llvm-cov`, PHP 8.4+, and Composer.

```bash
cargo install cargo-llvm-cov --locked
cargo test --locked
composer global require testparity/parity:^1.2
```

Generate fresh per-test evidence and check it:

```bash
parity test --config=parity.yaml --format=json
```

To inspect the same generated reports without rerunning Cargo:

```bash
parity check --config=parity.yaml --format=json
```

The expected result is `passed = true`, `global_coverage = 84.21`, and the file-level values shown above. No coverage fixture is committed.

## CI

GitHub Actions installs Rust and `cargo-llvm-cov`, runs the complete locked Cargo suite, installs public `testparity/parity:^1.2`, and executes `parity test`. The generated `.parity/per-test` directory and JSON result are uploaded as the `parity-per-test-evidence` artifact for 14 days. No private token is required.
