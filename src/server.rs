
use crate::lexer;
use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct web_query {
    query : String,
}
// Database Handlers
#[post("/execute")]
pub async fn execute(query : web::Json<web_query>) -> Result<String> {
    println!("post request with query : {}", query.query);
    let mut query : lexer::Lexer = lexer::Lexer::new(query.query.clone());
    query.node_tree();
    query.executer();
    println!("{:?}", query);
    Ok(format!("{}", query.query))
}

// Static Pages
#[get("/")]
pub async fn greet() -> impl Responder {
    format!("Hello World!")
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("Server starting...\n http://127.0.0.1:6942");
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(execute)
    })
    .bind(("127.0.0.1", 6942))?
    .run()
    .await
}
