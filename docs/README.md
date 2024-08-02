# hoox

`hoox` is an application / library that allows users to manage git hooks as part of the repository as well as in a central place.

## Workflow

1) `cargo install hoox`
2) `cargo add hoox --dev` - initializes and writes hooks in `.git` directory of the current repo (traverses up until it finds a `.git` folder)
3) modify `./hoox.yaml` to start using Git hooks - see `examples`

## Example

```yaml
version: "0.0.0"

hooks:
  "pre-commit":
    # severity: error
    command: |-
      cargo +nightly fmt --all -- --check
  # "pre-commit":
  #   program: ["python", "-c"]
  #   severity: warn
  #   command: |-
  #     print('executing hook')
  #     print('calling python program')

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
