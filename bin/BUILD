load("@io_bazel_rules_rust//proto:toolchain.bzl", "GRPC_COMPILE_DEPS")
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "server",
    srcs = ["main.rs"],
    deps = [
        "//proto:echo_rust_grpc_proto",
        "//services:server",
    ] + GRPC_COMPILE_DEPS,
)
