# apex-dev-formatter-meter

`apex-dev-formatter-meter` is a compact Rust repository for developer tools, centered on this goal: Build a Rust toolkit that studies formatter behavior through windowed input fixtures, with late-data behavior checks and no network dependency.

## Problem It Tries To Make Smaller

The project exists to keep a narrow engineering decision visible and testable. For this repo, that decision is how change width and review cost should influence a review result.

## Apex Dev Formatter Meter Review Notes

For a quick review, compare `safe rewrite` with `change width` before reading the middle cases.

## Working Pieces

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/apex-dev-formatter-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `safe rewrite` and `change width`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Design Notes

The core code exposes a scoring path and the added review layer uses `signal`, `slack`, `drag`, and `confidence`. The domain terms are `change width`, `diagnostic quality`, `review cost`, and `safe rewrite`.

The Rust addition stays small enough to inspect in one sitting.

## Example Run

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Tests

The same command runs the local verification path. The highest-scoring domain case is `recovery` at 204, which lands in `ship`. The most cautious case is `baseline` at 113, which lands in `watch`.

## Known Limits

This remains a local project with deterministic fixtures. It does not depend on credentials, hosted services, or live data. Future work should add richer malformed inputs before widening the public API.
