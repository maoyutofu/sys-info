use sys_info::{config, server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();
    let config = match config::Config::new() {
        Err(e) => panic!("{}", e),
        Ok(conf) => conf,
    };
    server::start(config).await
}
