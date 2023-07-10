# Copyright 2023 The Pigweed Authors
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not
# use this file except in compliance with the License. You may obtain a copy of
# the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
# WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
# License for the specific language governing permissions and limitations under
# the License.

### Fails with upstream
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    patch_args = ["-p1"],
    patches = [
        "//:0001-rustdoc_test-Apply-prefix-stripping-to-proc_macro-de.patch",
    ],
    sha256 = "4a9cb4fda6ccd5b5ec393b2e944822a62e050c7c06f1ea41607f14c4fdec57a2",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.25.1/rules_rust-v0.25.1.tar.gz"],
)

### Succeeds with PR
# load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

# git_repository(
#     name = "rules_rust",
#     commit = "a6e43874ce6de22c4ddf568c339e5b8b923533d9",
#     remote = "https://github.com/konkers/rules_rust",
# )

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

crates_repository(
    name = "rust_crates",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:Cargo.Bazel.lock",
    manifests = [
        "//:Cargo.toml",
        "//dep_crate:Cargo.toml",
        "//lib_crate:Cargo.toml",
        "//proc_macro_crate:Cargo.toml",
    ],
)

load("@rust_crates//:defs.bzl", "crate_repositories")

crate_repositories()
