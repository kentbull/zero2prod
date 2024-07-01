use actix_web::dev::Server;
use actix_web::{
    middleware::Logger, web, web::Bytes, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
struct HelloKeriCredEnvelope {
    action: String,
    actor: String,
    data: HelloKeriCredIssue,
}

#[derive(Debug, Deserialize)]
struct HelloKeriCredIssue {
    schema: String,
    issuer: String,
    #[serde(rename(deserialize = "issueTimestamp"))]
    issue_timestamp: String,
    credential: String,
    recipient: String,
    hello_msg: String,
}

impl fmt::Display for HelloKeriCredEnvelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{action: {}, actor: {}, data: {{{}}} }}",
            self.action, self.actor, self.data
        )
    }
}

impl fmt::Display for HelloKeriCredIssue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "schema: {}, issuer: {}, issueTimestamp: {}, credential: {}, recipient: {}, hello_msg: {}",
        self.schema, self.issuer, self.issue_timestamp, self.credential, self.recipient, self.hello_msg)
    }
}

async fn index(bytes: Bytes) -> std::io::Result<HttpResponse> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(text) => {
            println!("Request body {}", text);
            let envelope: HelloKeriCredEnvelope = serde_json::from_str(&text).unwrap();
            println!("{}", envelope);
            Ok(HttpResponse::Ok().body(format!("Request body is {}", text)))
        }
        Err(_) => Ok(HttpResponse::BadRequest().finish()),
    }
    // let body = format!("{}", cred);
    // println!("Received credential");
    // println!("{}", body);
    // Ok(body)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            // .wrap(Cors::permissive())
            .wrap(Logger::default())
            .route("/health", web::get().to(health_check))
            .route("/", web::post().to(index))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8000))?
    .run();
    Ok(server)
}
