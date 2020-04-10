mod config;
mod pb;
mod project;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:10000".parse().unwrap();

    let route_guide = ReicService {};

    let svc = ReicServer::new(route_guide);

    Server::builder().add_service(svc).serve(address).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
