# hoox

`hoox` is an application / library that allows users to manage git hooks as part of the repository as well as in a central place.

## Workflow

1) `cargo install hoox`
2) `cargo add hoox --dev` - initializes and writes hooks in `.git` directory of the current repo (traverses up until it finds a `.git` folder)
3) modify `./hoox.yaml` to start using Git hooks - see `examples`

## Example

```yaml
version: "0.0.0"

.cargo_check: &cargo_check |-
  cargo +nightly fmt --all -- --check

hooks:
  "pre-commit":
    - command: *cargo_check
  "pre-push":
    - command: *cargo_check
```

### Available hooks

- `applypatch-msg`
- `commit-msg`
- `post-applypatch`
- `post-checkout`
- `post-commit`
- `post-merge`
- `post-receive`
- `post-rewrite`
- `post-update`
- `pre-applypatch`
- `pre-auto-gc`
- `pre-commit`
- `pre-push`
- `pre-rebase`
- `pre-receive`
- `prepare-commit-msg`
- `push-to-checkout`
- `sendemail-validate`
- `update`
