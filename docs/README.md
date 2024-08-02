# hoox

![](hoox.png)

`hoox` is an application / library that allows users to manage git hooks as part of the repository as well as in a central place.

## Workflow

1) `cargo install hoox`
2) `cargo add hoox --dev` - initializes and writes hooks in `.git` directory of the current repo (traverses up until it finds a `.git` folder)
3) modify `./hoox.yaml` to start using Git hooks - see `examples`

## Example

```yaml
version: "0.0.0"
verbosity: all # [all, none, stdout, stderr]

# anchors
.cargo_fmt_check: &cargo_fmt_check |-
  cargo +nightly fmt --all -- --check
.cargo_test: &cargo_test |-
  cargo test --all

hooks:
  "pre-commit": # pre-commit hook
    - command: *cargo_fmt_check # re-use anchor
    - command: *cargo_test
    - command: 'cargo doc --no-deps'
      verbosity: stderr # overrides global verbosity
      severity: warn # [error, warn]
  "pre-push": # pre-push hook
    - command: *cargo_fmt_check
    - command: *cargo_test

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
