load("@io_bazel_rules_rust//proto:toolchain.bzl", "GRPC_COMPILE_DEPS")
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library", "rust_test")

package_group(
    name = "internal",
    packages = ["//services/echo/rust/..."],
)

package(
    default_visibility = ["//:__subpackages__"],
)

rust_library(
    name = "server",
    srcs = ["server.rs"],
    deps = [
        "//proto:echo_rust_grpc_proto",
    ] + GRPC_COMPILE_DEPS,
)
