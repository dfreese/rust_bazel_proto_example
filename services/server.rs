extern crate echo_rust_grpc_proto;
extern crate grpc;

pub struct EchoImpl;

use echo_rust_grpc_proto::{Echo, EchoRequest, EchoResponse};

impl Echo for EchoImpl {
    fn echo(
        &self,
        _m: grpc::RequestOptions,
        req: EchoRequest,
    ) -> grpc::SingleResponse<EchoResponse> {
        // Simply send the request payload back as the response.
        let mut r = EchoResponse::new();
        r.set_payload(req.payload);
        grpc::SingleResponse::completed(r)
    }
}
