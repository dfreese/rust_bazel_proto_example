load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Works
http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "ef8932ed811d960611d4d32594a0db7d60179341a706de7a2c10ff215804ab46",
    strip_prefix = "rules_rust-5c0e90a24e9b9e8ab81b183de4d14eb95fb86747",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/5c0e90a24e9b9e8ab81b183de4d14eb95fb86747.tar.gz",
    ],
)

# Doesn't Work
# http_archive(
#     name = "io_bazel_rules_rust",
#     sha256 = "b56ab11bed62abbddc03aa7af127dc945da4ee725c2b0622ac110c5f028a1d1e",
#     strip_prefix = "rules_rust-7cde1e46e9e5a97eb2650ee70a81ec8f9578e3e7",
#     urls = [
#         "https://github.com/bazelbuild/rules_rust/archive/7cde1e46e9e5a97eb2650ee70a81ec8f9578e3e7.tar.gz",
#     ],
# )

http_archive(
    name = "bazel_skylib",
    sha256 = "97e70364e9249702246c0e9444bccdc4b847bed1eb03c5a3ece4f83dfe6abc44",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
    ],
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repository_set")

RUST_VERSION = "1.42.0"

rust_repository_set(
    name = "rust_linux_x86_64",
    exec_triple = "x86_64-unknown-linux-gnu",
    extra_target_triples = [],
    version = RUST_VERSION,
)

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")

bazel_version(name = "bazel_version")

load("@io_bazel_rules_rust//proto:repositories.bzl", "rust_proto_repositories")

rust_proto_repositories()
