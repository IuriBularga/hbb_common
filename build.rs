fn main() {
    // TradingMD: RENDEZVOUS_SERVER / RS_PUB_KEY are baked into config.rs via
    // option_env!, so a change in either must trigger a rebuild.
    println!("cargo:rerun-if-env-changed=RENDEZVOUS_SERVER");
    println!("cargo:rerun-if-env-changed=RS_PUB_KEY");
    for var in ["RENDEZVOUS_SERVER", "RS_PUB_KEY"] {
        if std::env::var(var).map_or(true, |v| v.is_empty()) {
            println!("cargo:warning={var} is not set, building against the public RustDesk server");
        }
    }

    let out_dir = format!("{}/protos", std::env::var("OUT_DIR").unwrap());

    std::fs::create_dir_all(&out_dir).unwrap();

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(out_dir)
        .inputs(["protos/rendezvous.proto"])
        .include("protos")
        .customize(protobuf_codegen::Customize::default().tokio_bytes(true))
        .run()
        .expect("Codegen failed.");
}
