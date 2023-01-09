load("@aspect_bazel_lib//lib:copy_to_directory.bzl", "copy_to_directory")
load("@aspect_bazel_lib//lib:write_source_files.bzl", "write_source_files")
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
        out_dir = "dist",
        root_dir = "src",
        source_maps = "true",
    ),
    deps = [
        ":node_modules/@types/node",
        ":node_modules/@types/vscode",
        ":node_modules/vscode-languageclient",
    ],
)

copy_to_directory(
    name = "swc_cp",
    srcs = [":swc"],
    root_paths = ["editors/code/dist"],
)

write_source_files(
    name = "dist",
    files = {
        "dist": ":swc_cp",
    },
)
