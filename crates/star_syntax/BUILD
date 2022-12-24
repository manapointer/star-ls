load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")
load("@crate_index//:defs.bzl", "all_crate_deps")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "star_syntax",
    srcs = glob(["src/**/*.rs"]),
    deps = all_crate_deps(
        normal = True,
    ),
)

rust_test(
    name = "star_syntax_test",
    crate = ":star_syntax",
    data = glob(["src/parser/test_data/**"]),
    deps = all_crate_deps(
        normal_dev = True,
    ),
)
