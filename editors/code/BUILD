load("@aspect_rules_swc//swc:defs.bzl", "swc")
load("@aspect_rules_ts//ts:defs.bzl", "ts_project")
load("@bazel_skylib//lib:partial.bzl", "partial")
load("@npm//:defs.bzl", "npm_link_all_packages")

npm_link_all_packages(name = "node_modules")

ts_project(
    name = "swc",
    srcs = ["src/main.ts"],
    declaration = True,
    root_dir = "src",
    source_map = True,
    transpiler = partial.make(
        swc,
        out_dir = "out",
        root_dir = "src",
        source_maps = "true",
    ),
    deps = [
        ":node_modules/@types/node",
        ":node_modules/@types/vscode",
        ":node_modules/vscode-languageclient",
    ],
)
