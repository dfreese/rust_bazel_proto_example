extern crate echo_rust_grpc_proto;
extern crate grpc;
extern crate server;
extern crate tls_api_stub;

fn main() {
    let mut server = grpc::ServerBuilder::<tls_api_stub::TlsAcceptor>::new();
    server.http.set_port(50051);
    server.add_service(echo_rust_grpc_proto::EchoServer::new_service_def(
        server::EchoImpl,
    ));
    server.http.set_cpu_pool_threads(4);

    let server = server.build().expect("server");
    let port = server.local_addr().port().unwrap();
    println!("Echo server started on port {}", port);

    loop {
        std::thread::park();
    }
}
