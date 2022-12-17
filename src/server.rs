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

    let get_url = format!("http://localhost:{}/internal/sys-info", config.http.port);
    loop {
        let resp = client.get(get_url.clone()).send();
        match resp {
            Err(e) => eprintln!("{}", e),
            Ok(resp) => {
                let si = resp.json::<super::system_info::SystemInfo>().unwrap();
                match client.post(url).json(&si).send() {
                    Err(e) => eprintln!("{}", e),
                    Ok(res) => println!("{:#?}", res.text()),
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
        .body("Hello! <a href=\"sys-info\">See.</a>")
}

#[get("/sys-id")]
async fn sys_id() -> impl Responder {
    let id = super::get_device_id().await;

    let result = &Result::success_return_data(id);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string_pretty(result).unwrap())
}

#[get("/sys-platform")]
async fn sys_platform() -> impl Responder {
    let result = match super::sys_platform().await {
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


#[get("/sys-memory")]
async fn sys_memory() -> impl Responder {
    let result = match super::sys_memory().await {
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

#[get("/sys-disk")]
async fn sys_disk() -> impl Responder {
    let result = match super::sys_disk().await {
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


#[get("/sys-net")]
async fn sys_net() -> impl Responder {
    let result = match super::sys_net().await {
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

#[get("/sys-cpu")]
async fn sys_cpu(config: web::Data<Arc<config::Config>>) -> impl Responder {
    let result = match super::sys_cpu(config.sys.timer).await {
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

#[get("/internal/sys-info")]
async fn internal_sys_info(config: web::Data<Arc<config::Config>>) -> impl Responder {
    let result = match super::sys_info(config.sys.timer).await {
        Err(_e) => "{}".to_string(),
        Ok(si) => serde_json::to_string_pretty(&si).unwrap(),
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .body(result)
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
            .service(sys_net)
            .service(sys_platform)
            .service(sys_memory)
            .service(sys_disk)
            .service(sys_cpu)
            .service(sys_id)
            .service(internal_sys_info)
    })
    .bind((bind, port))?
    .run()
    .await
}
