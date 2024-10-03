use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

const MAX_SIZE: usize = 10 * 1024 * 1024; // 10 MB limit

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::default().allow_any_origin())
            .app_data(web::PayloadConfig::new(MAX_SIZE))
            .service(web::resource("/convert").route(web::post().to(convert_file)))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}

async fn convert_file(input_buffer: web::Bytes) -> Result<HttpResponse, Error> {
    let temp_input_file_path = "temp_input_file";
    let output_file_path = "output.png";

    let mut file = File::create(temp_input_file_path)?;
    file.write_all(&input_buffer)?;

    let output = Command::new("gm")
        .arg("convert")
        .arg(temp_input_file_path)
        .arg(output_file_path)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("GraphicsMagick error: {}", stderr);
                return Ok(HttpResponse::InternalServerError().body("Conversion failed"));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute GraphicsMagick: {:?}", e);
            return Ok(HttpResponse::InternalServerError().body("Conversion failed"));
        }
    }

    let mut output_file = File::open(output_file_path)?;
    let mut output_buffer = Vec::new();
    output_file.read_to_end(&mut output_buffer)?;

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(output_buffer))
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("GraphicsMagick conversion server is running")
}


