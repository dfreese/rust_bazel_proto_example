load("@io_bazel_rules_rust//proto:proto.bzl", "rust_grpc_library")

package(
    default_visibility = [
        "//visibility:public",
    ],
)

proto_library(
    name = "echo_proto",
    srcs = ["echo.proto"],
)

rust_grpc_library(
    name = "echo_rust_grpc_proto",
    deps = [":echo_proto"],
)
