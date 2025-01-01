use std::path::Path;

fn main() {
    let p = Path::new("./src/proto/account/account.proto");
    let p1 = Path::new("./src/proto");
    println!("cargo:rerun-if-changed={}", p.exists());
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .build_transport(true)
        .out_dir("./src/generated_proto_rs")
        .compile(&[&p], &[p1])
        .unwrap();

    // tonic_build::configure()
    //     .build_server(true)
    //     .build_client(true)
    //     .out_dir("./src/generated_proto_rs")
    //     .compile(&["./src/proto/posts/posts.proto"], &["./src/proto"])
    //     .unwrap();
    //
    // tonic_build::configure()
    //     .build_server(true)
    //     .build_client(true)
    //     .out_dir("./src/generated_proto_rs")
    //     .compile(&["./src/proto/reactions/reactions.proto"], &["./src/proto"])
    //     .unwrap();
}

// fn main() {
//     println!("cargo:rerun-if-changed=build.rs");
// }
