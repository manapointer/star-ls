load("@rules_rust//rust:defs.bzl", "rust_library")
load("@crate_index//:defs.bzl", "all_crate_deps")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "star_syntax",
    srcs = glob(["src/**/*.rs"]),
    deps = all_crate_deps(
        normal = True,
    ),
)
