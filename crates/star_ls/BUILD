load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")

package(default_visibility = ["//visibility:public"])

# hack: crates_repository generates the wrong target name
incorrect_salsa_2022_target = "@crate_index__salsa-2022-0.1.0//:salsa-2022"

correct_salsa_2022_target = "@crate_index__salsa-2022-0.1.0//:salsa_2022"

aliases = {dep if dep != incorrect_salsa_2022_target else correct_salsa_2022_target: alias for (dep, alias) in aliases().items()}

deps = [dep if dep != incorrect_salsa_2022_target else correct_salsa_2022_target for dep in all_crate_deps(normal = True)]

rust_library(
    name = "star_ls_lib",
    srcs = glob(["src/*.rs"]),
    aliases = aliases,
    deps = deps + ["//crates/star_syntax"],
)

rust_binary(
    name = "star_ls",
    srcs = ["src/bin/main.rs"],
    deps = [":star_ls_lib"],
)
