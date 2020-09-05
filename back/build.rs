fn main()->Result<(),Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .format(true)
        .out_dir("health/src/services/api")
        .compile(&["proto/health.proto"], &["proto"])
        .unwrap();
    Ok(())
}