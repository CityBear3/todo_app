use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, ResponseError};
//use thiserror::Error;
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Deserialize;
use actix_web::middleware::Logger;

mod error;
use error::Myerror;

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[derive(Deserialize)]
struct AddParams {
    text: String,
}

#[derive(Deserialize)]
struct DeleteParams {
    id: u32,
}

impl ResponseError for Myerror {}

#[post("/add")]
async fn add_todo(
    params: web::Form<AddParams>,
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Myerror> {
    let conn = db.get()?;
    conn.execute("INSERT INTO todo (text) VALUES (?)", &[&params.text])?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

#[post("/delete")]
async fn delete_todo(
    params: web::Form<DeleteParams>,
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Myerror> {
    let conn = db.get()?;
    conn.execute("DELETE FROM todo WHERE id=?", &[&params.id])?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())        
}

#[post("/reset")]
async fn reset_todo(
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>
) -> Result<HttpResponse, Myerror> {
    let conn = db.get()?;
    conn.execute("DELETE FROM todo ", params![]).expect("Faild delete all data");
    conn.execute("DELETE FROM sqlite_sequence WHERE name='todo'", params![]).expect("Faild delete autoincrement");
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, Myerror> {
    let conn = db.get()?;
    let mut statement = conn.prepare("SELECT id, text FROM todo")?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id, text })
    })?;

    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
    
}


#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Faild to initialize the connection pool.");
    let conn = pool
        .get()
        .expect("Faild to get the connection pool.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        )",
        params![],
    )
    .expect("Faild to create a table `todo`.");

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(add_todo)
            .service(delete_todo)
            .service(reset_todo)
            .data(pool.clone())
            .wrap(Logger::default())
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}

