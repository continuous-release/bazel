# bazel

Bazel + Rust testbed for
[continuous-release](https://github.com/continuous-release).

Every push to `main` cuts a release. CI computes the version from conventional
commits, writes it to `version.txt`, builds with Bazel, runs the test target,
and attaches the zip to the GitHub Release.

The version is a compile-time input: `rust-bin/src/lib.rs` uses
`include_str!("../../version.txt")`, and the file is declared as
`compile_data`, so Bazel rebuilds when the version changes without any
stamping or workspace-status machinery.

Nx counterpart: [continuous-release/nx](https://github.com/continuous-release/nx).

## Local

```bash
bazel test //...
bazel build //rust-bin:package
python3 -m zipfile -l bazel-bin/rust-bin/continuous-release-bazel.zip
```

To reproduce a release build:

```bash
echo 1.2.3 > version.txt
bazel build //rust-bin:package
bazel-bin/rust-bin/rust-bin
```

## Layout

| Path              | Purpose                                      |
| ----------------- | -------------------------------------------- |
| `MODULE.bazel`    | bzlmod deps and Rust toolchain               |
| `rust-bin/BUILD`  | Library, binary, test, and zip targets       |
| `version.txt`     | Release version, rewritten by CI             |
| `rust-bin/src/`   | `lib.rs` reads the version; `main.rs` prints it |

## Contract

See [continuous-release/.github](https://github.com/continuous-release/.github)
for the shared workflow, version rules, and release notes format.
