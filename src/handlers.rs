use actix_web::{web, HttpResponse, Responder};

// ------------------------- Website API ------------------------- //
pub async fn contact() -> impl Responder { // POST /contact
    HttpResponse::Ok().body("contact")
}

// ------------------------ Backoffice API ----------------------- //
pub async fn get() -> impl Responder { // GET /inbox
    HttpResponse::Ok().body("get")
}

pub async fn assign(path: web::Path<String>) -> impl Responder { // POST /inbox/{id}/assign
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("assign message {}", id))
}

pub async fn release(path: web::Path<String>) -> impl Responder { // POST /inbox/{id}/release
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("release message {}", id))
}

pub async fn reply(path: web::Path<String>) -> impl Responder { // POST /inbox/{id}/reply
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("reply to message {}", id))
}

pub async fn delete(path: web::Path<String>) -> impl Responder { // DELETE /inbox/{id}
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("delete message {}", id))
}
