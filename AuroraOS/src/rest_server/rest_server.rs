use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
// Estructura de datos para API
#[derive(Serialize, Deserialize)]
struct ApiData {
    input: Vec<f64>,
    output: Vec<f64>,
}
// Implementación de servidor REST
pub struct RestServer;
impl RestServer {
    // Endpoint para predicciones
    async fn predict(data: web::Json<ApiData>) -> impl Responder {
        // Llamada a modelo de autogradiente...
        HttpResponse::Ok().json(data)
    }
    // Inicialización del servidor
    pub async fn init() -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(web::resource("/predict").route(web::post().to(Self::predict)))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
}