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
verbosity: all

.cargo: &cargo !inline |-
  cargo +nightly fmt --all -- --check
  cargo test --all

hooks:
  "pre-commit":
    # use YAML anchors
    - command: *cargo
    # use inline command
    - command: !inline 'cargo doc --no-deps'
      verbosity: stderr
      severity: warn
    # reference a script file (path is relative to Git repo root)
    - command: !file "./hello_world.sh"
    # referemce a script file with custom program
    - command: !file "./hello_world.py"
      program: ["python3", "-c"]
      verbosity: stderr
      severity: error

  "pre-push":
    # re-use YAML anchor
    - command: *cargo

  "prepare-commit-msg":
    # write to $COMMIT_MSG_FILE ($1) which is going to be the template commit message for this commit
    # which is subsequently opened in the $EDITOR program.
    # $0 = path to "hoox.yaml" file in any hook
    - command: !inline |-
        COMMIT_MSG_FILE=$1
        echo "Work in progress" > $COMMIT_MSG_FILE
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
