# Object codegen benchmark

This package compares object conversion codegen modes:

- `baseline`: explicit inline in the default build, used as the pre-auto reference
- `inline`: `#[napi(object, codegen = "inline")]`
- `compact`: `#[napi(object, codegen = "compact")]`
- `auto`: `#[napi(object, codegen = "auto")]`

Run the current branch with all modes:

```sh
yarn workspace @examples/bench-codegen bench:modes
```

To compare against `main`, apply or copy this package to a `main` checkout and
run the baseline-only build. It disables the feature that contains the new
`codegen` attribute syntax, so the same benchmark cases compile with the old
macro implementation:

```sh
yarn workspace @examples/bench-codegen bench:baseline
```

For faster smoke checks:

```sh
ITERATIONS=1000 SAMPLES=3 yarn workspace @examples/bench-codegen bench:modes
ITERATIONS=1000 SAMPLES=3 yarn workspace @examples/bench-codegen bench:baseline
```
