use super::config;
use super::result::Result;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use async_std::task;
use reqwest::blocking::Client;
use std::sync::Arc;
use std::time::Duration;

async fn task(config: Arc<config::Config>) {
    if !config.notification.enable {
        return;
    }
    let url = &config.notification.url;
    let client = Client::new();

    loop {
        match super::sys_info(config.sys.timer).await {
            Err(e) => eprintln!("{}", e),
            Ok(si) => {
                match client.post(url).json(&si).send() {
                    Err(e) => eprintln!("{}", e),
                    Ok(res) => println!("{}", res.text().unwrap()),
                };
            }
        };
        futures_timer::Delay::new(Duration::from_millis(config.notification.interval)).await;
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("Hello! <a href=\"/sys-info\">See.</a>")
}

#[get("/sys-info")]
async fn sys_info(config: web::Data<Arc<config::Config>>) -> impl Responder {
    let result = match super::sys_info(config.sys.timer).await {
        Err(e) => serde_json::to_string_pretty(&Result::error_description(
            Result::SYS_ERROR,
            &e.to_string(),
        )),
        Ok(si) => serde_json::to_string_pretty(&Result::success_return_data(si)),
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .body(result.unwrap())
}

pub async fn start(config: config::Config) -> std::io::Result<()> {
    let config_arc = Arc::new(config);
    let config_arc_clone = config_arc.clone();
    task::spawn(task(config_arc_clone.clone()));
    let config_arc_clone = config_arc.clone();
    let bind = config_arc.http.bind.as_str();
    let port: u16 = config_arc.http.port;
    HttpServer::new(move || {
        App::new()
            .data(config_arc_clone.clone())
            .service(index)
            .service(sys_info)
    })
    .bind((bind, port))?
    .run()
    .await
}
