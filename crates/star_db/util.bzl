# hack: crates_repository generates the wrong target name
bad_target_mappings = {
    "@crate_index__salsa-2022-0.1.0//:salsa-2022": "@crate_index__salsa-2022-0.1.0//:salsa_2022",
}

def map_maybe_bad_dep(dep):
    return bad_target_mappings.get(dep, dep)
