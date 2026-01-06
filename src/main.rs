use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{
    post, App, HttpResponse, HttpServer,
    http::header,
};
use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[post("/upload")]
async fn upload(mut payload: Multipart) -> HttpResponse {
    let txt_path = "input.txt";
    let mp3_path = "output.mp3";

    // Save uploaded file
    let mut has_data = false;
    let mut f = File::create(txt_path).unwrap();

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();


        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            if data.is_empty() {
                continue;
            }
            has_data = true;
            f.write_all(&data).unwrap();
            
        }
    }
    if !has_data {
        return HttpResponse::BadRequest().body("No file or text uploaded");
    }
    // Call Python TTS
    let status = Command::new("python")
        .arg("speech.py")
        .arg(txt_path)
        .arg(mp3_path)
        .status()
        .unwrap();

    if !status.success() {
        return HttpResponse::InternalServerError().body("TTS failed");
    }

    let mp3_bytes = std::fs::read(mp3_path).unwrap();

    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "audio/mpeg"))
        .body(mp3_bytes)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173") // Vite
            .allowed_origin("http://localhost:3000") // Docker / Prod
            .allowed_origin("https://speech-service-front-2.onrender.com") // Render Prod
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
            ])
            .expose_headers([header::CONTENT_TYPE])
            .supports_credentials();

        App::new()
            .wrap(cors)   // ✅ THIS LINE FIXES CORS
            .service(upload)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
