fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .format(true)
        .out_dir("../reic-server/src/pb")
        .compile(&["proto/reic.proto"], &["proto"])?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .format(true)
        .out_dir("../reic-client/src/pb")
        .compile(&["proto/reic.proto"], &["proto"])?;
    Ok(())
}
