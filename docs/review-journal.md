# Review Journal

The repository goal stays the same: build a Rust toolkit that studies formatter behavior through windowed input fixtures, with late-data behavior checks and no network dependency. This note explains the added review angle.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its developer tools focus without claiming live deployment or external usage.

## Cases

- `baseline`: `change width`, score 113, lane `watch`
- `stress`: `diagnostic quality`, score 151, lane `ship`
- `edge`: `review cost`, score 151, lane `ship`
- `recovery`: `safe rewrite`, score 204, lane `ship`
- `stale`: `change width`, score 172, lane `ship`

## Note

The repository should be understandable without pretending it is larger than it is.
