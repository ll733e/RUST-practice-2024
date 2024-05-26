use actix_files::Files;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use sysinfo::{System, SystemExt, ProcessorExt};

struct AppState {
    start_time: DateTime<Utc>,
    system: Mutex<System>,
}

#[get("/status")]
async fn status(data: web::Data<Arc<AppState>>, req: HttpRequest) -> impl Responder {
    let app_state = data.get_ref();

    // Calculate uptime
    let current_time = Utc::now();
    let uptime = current_time.signed_duration_since(app_state.start_time);

    // Get system info
    let mut sys = app_state.system.lock().unwrap();
    sys.refresh_all();

    // Get CPU usage and memory usage
    let cpu_usage = sys.global_processor_info().cpu_usage();
    let total_memory = sys.total_memory() / 1024; // Convert KB to MB
    let used_memory = sys.used_memory() / 1024; // Convert KB to MB
    let memory_usage_percentage = (used_memory as f64 / total_memory as f64) * 100.0;

    // Get client's IP address
    let ip_address = req.connection_info().realip_remote_addr().unwrap_or("Unknown").to_string();

    // Generate JSON response
    let json = format!(
        r#"{{
            "uptime": {},
            "cpu_usage": {:.2},
            "memory_usage": {:.2},
            "used_memory": {},
            "total_memory": {},
            "ip_address": "{}"
        }}"#,
        uptime.num_seconds(),
        cpu_usage,
        memory_usage_percentage,
        used_memory,
        total_memory,
        ip_address
    );

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let start_time = Utc::now();
    let system = System::new_all();
    let app_state = Arc::new(AppState {
        start_time,
        system: Mutex::new(system),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(status)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
