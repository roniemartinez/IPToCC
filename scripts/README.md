# Maintenance scripts

- `bump.py`: bump version across crate, python, and wasm binding manifests.
- `tag.py`: create the corresponding git tags (`rust-vX.Y.Z`, `python-vX.Y.Z`, `wasm-vX.Y.Z`, `all-vX.Y.Z`).

Run via Taskfile:

```bash
task bump -- wasm patch
task tag  -- python
```
