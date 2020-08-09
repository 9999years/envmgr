# envmgr

A helper for declaring common operations on environment variables (like "add
these directories to `$PATH` on these platforms / if they exist") in a
shell-agnostic way.


## Config

At the top level, we have:

```yaml
---
tests:
  test_name1: expr
  test_name2: expr
env:
  VAR1: value
  VAR2: value...
```

`env` configures the environment. Valid values are:

  - Configuration maps; name-value mappings to configure environment variables. Values can be:

    - A single string, e.g. `~/.go`
    - A list of strings, e.g.

      ```yaml
      - ~/.cargo/bin
      - /bin
      ```

    - A map:

      ```yaml
      # `sep` defaults to `:` on Linux/MacOS and `;` on Windows
      sep: '!!!'
      paths:
        - /sbin
        - /bin
      ```

  - A block, containing nested `env` values.

    ```yaml
    block:
      GOPATH: ~/.go
    ```

    Blocks may be conditional.

  - A list whose elements are any of the above.

    ```yaml
    - block:
        GOPATH: ~/.go
    - PATH:
      - /bin
    ```
