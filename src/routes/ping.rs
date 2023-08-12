use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn ping(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
