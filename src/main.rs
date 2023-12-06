use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::header};
use std::fs::File;
use std::io::Write;
use chrono::Utc;
use tracing::{info, error};
use tracing_subscriber;

async fn save_feedback(body: String) -> impl Responder {
    let filename = format!("app/feedback_{}.txt", Utc::now().format("%Y%m%d%H%M%S%f"));

    let mut file = match File::create(&filename) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to create file: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Err(e) = writeln!(file, "{}", body) {
        error!("Failed to write to file: {}", e);
        return HttpResponse::InternalServerError().finish();
    }

    info!("Feedback saved to {}", filename);
    HttpResponse::Ok().body("Feedback received")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    info!("Starting server...");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000") // Replace with the actual origin you want to allow
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/feedback", web::post().to(save_feedback))
            // ... other routes and services ...
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
