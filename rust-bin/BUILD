load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")
load("@rules_pkg//pkg:zip.bzl", "pkg_zip")

rust_library(
    name = "lib",
    srcs = ["src/lib.rs"],
    crate_name = "rust_bin",
    edition = "2021",
    compile_data = ["//:version.txt"],
)

rust_binary(
    name = "rust-bin",
    srcs = ["src/main.rs"],
    edition = "2021",
    deps = [":lib"],
    compile_data = ["//:version.txt"],
)

rust_test(
    name = "test",
    crate = ":lib",
)

pkg_zip(
    name = "package",
    srcs = [":rust-bin"],
    out = "continuous-release-bazel.zip",
)
