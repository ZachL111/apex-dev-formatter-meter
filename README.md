# apex-dev-formatter-meter

`apex-dev-formatter-meter` is a Rust project for Developer tools. It turns build a Rust toolkit that studies formatter behavior through windowed input fixtures, with late-data behavior checks and no network dependency into a small local model with readable fixtures and a direct verification command.

## Reading Apex Dev Formatter Meter

Start with the README, then open `metadata/project.json` to check the constants behind the examples. After that, `fixtures/cases.csv` shows the compact path and `examples/extended_cases.csv` gives a wider look at the same rule.

## Purpose

The repository exists to keep a technical idea small enough to reason about. The implementation avoids external dependencies where possible, then uses fixtures to make changes easy to review.

## Files Worth Reading

- `src`: primary implementation
- `tests`: verification harness
- `fixtures`: compact golden scenarios
- `examples`: expanded scenario set
- `metadata`: project constants and verification metadata
- `docs`: operations and extension notes
- `scripts`: local verification and audit commands
- `Cargo.toml`: Rust package metadata

## What It Does

- Includes extended examples for safe defaults, including `recovery` and `degraded`.
- Documents repeatable output tradeoffs in `docs/operations.md`.
- Runs locally with a single verification command and no external credentials.
- Stores project constants and verification metadata in `metadata/project.json`.
- Adds a repository audit script that checks structure before running the language verifier.

## Design Sketch

The project is organized around a compact model rather than a large framework. Inputs are scored, classified, and checked against golden fixtures. The constants live in code and are mirrored in metadata so documentation drift is easy to catch. The Rust code keeps ownership and data movement plain, which makes the tests useful for checking both behavior and API shape.

## Setup

Install Rust and run the commands from the repository root. The project does not need credentials or a hosted service.

## Fixture Notes

`boundary` is the first example I would inspect because it lands on the `review` path with a score of 38. The broader file also keeps `degraded` at -106 and `recovery` at 174, which gives the model a useful low-to-high spread.

## Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

This runs the language-level build or test path against the compact fixture set.

## Verification

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/audit.ps1
```

The audit command checks repository structure and README constraints before it delegates to the verifier.

## Next Directions

- Add malformed input fixtures so the failure path is as visible as the happy path.
- Split the scoring constants into a typed configuration object and validate it before use.
- Add a comparison mode that shows how decisions change when one signal is adjusted.
- Add one more developer tools fixture that focuses on a malformed or borderline input.

## Limits

This code is local-first. It makes no claim about deployed usage and avoids credentials, hosted state, and environment-specific setup.
