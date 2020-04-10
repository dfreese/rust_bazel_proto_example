This is an attempt at a minimally reproducible example for an error that I was
seeing with building rust and protobuf libraries.  The issue seems to be
introduced by
[this commit](https://github.com/bazelbuild/rules_rust/commit/7cde1e46e9e5a97eb2650ee70a81ec8f9578e3e7),
which was from pull request
[#240](https://github.com/bazelbuild/rules_rust/pull/240).


The error that this generates, when uncommenting the bad version in the
WORKSPACE file is as follows:
```
ERROR: /tmp/rust_bazel_proto_example/bin/BUILD:4:1: error executing shell command: '/bin/bash -c CARGO_MANIFEST_DIR=$(pwd)/bin external/rust_linux_x86_64/bin/rustc "$@" --remap-path-prefix="$(pwd)"=__bazel_redacted_pwd  bin/main.rs --crate-name=server --crate-type=bin --codegen=me...' failed (Exit 1) bash failed: error executing command /bin/bash -c 'CARGO_MANIFEST_DIR=$(pwd)/bin external/rust_linux_x86_64/bin/rustc "$@" --remap-path-prefix="$(pwd)"=__bazel_redacted_pwd' '' bin/main.rs '--crate-name=server' '--crate-type=bin' ... (remaining 89 argument(s) skipped)

Use --sandbox_debug to see verbose messages from the sandbox
error[E0277]: the trait bound `server::EchoImpl: echo_rust_grpc_proto::Echo` is not satisfied
  --> bin/main.rs:10:9
   |
10 |         server::EchoImpl,
   |         ^^^^^^^^^^^^^^^^ the trait `echo_rust_grpc_proto::Echo` is not implemented for `server::EchoImpl`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
```

What seems to happen is that an additional protoc call is invoked.  This was
checked by running `bazel build -s //...` on a clean repository.  There are two
`SUBCOMMAND: # //proto:echo_proto` listed.  The first are identical between the
two differnent rules_rust commits.  The second is identical to the first, but
bazel puts it into a different folder using the --descriptor_set_out flag, but
diffing the two outputs, the outputs are the same binaries.

Both of these get built into rust libraries, which then get linked to the final
rust binary as an additional at the end
`-Ldependency=bazel-out/k8-fastbuild-ST-{hash}/bin/proto/echo_rust_grpc_proto.grpc.rust`
in the `SUBCOMMAND: # //bin:server`.
