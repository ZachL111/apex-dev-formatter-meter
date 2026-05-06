# Failure Modes

For `apex-dev-formatter-meter`, I would look first for these mistakes:

- `change width` cases moving lanes without a matching threshold change.
- `review cost` scoring higher after drag increases.
- Duplicate fixture ids hiding a stale golden row.
- README examples drifting away from the verifier.

The local checks are intentionally strict about these cases.
