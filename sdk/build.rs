use std::path::Path;

fn main() {
    println!("THIS IS PROST");
    let p = Path::new("./src/proto/account/account.proto");
    let c = Path::new("./src/proto/channel/channel.proto");
    let m = Path::new("./src/proto/messaging/messaging.proto");
    let e = Path::new("./src/proto/extras/extras.proto");
    let p1 = Path::new("./src/proto");
    println!("cargo:rerun-if-changed={}", p.exists());

    tonic_build::configure()
        .build_server(true)
        // .build_client(true)
        .build_transport(true)
        .out_dir("./src/generated_proto_rs")
        .compile(&[&p, &c, &m, &e], &[p1])
        .unwrap();
}
