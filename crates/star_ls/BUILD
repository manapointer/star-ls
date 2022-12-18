load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load(":util.bzl", "map_maybe_bad_dep")

package(default_visibility = ["//visibility:public"])

aliases = {map_maybe_bad_dep(dep): alias for (dep, alias) in aliases().items()}

deps = [map_maybe_bad_dep(dep) for dep in all_crate_deps(normal = True)]

rust_library(
    name = "star_ls_lib",
    srcs = glob(["src/*.rs"]),
    aliases = aliases,
    deps = deps + ["//crates/star_syntax"],
)

rust_binary(
    name = "star_ls",
    srcs = ["src/bin/main.rs"],
    aliases = {k: v for (k, v) in aliases.items() + [(":star_ls_lib", "star_ls")]},
    deps = deps + [":star_ls_lib"],
)
