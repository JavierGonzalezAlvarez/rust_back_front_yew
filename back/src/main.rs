mod config;
mod db;
mod errors;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};

//sino lo uso lo quito
use crate::handlers::*;
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> io::Result<()> {
  dotenv().ok();
  let config = crate::config::Config::from_env().unwrap();
  let pool = config.pg.create_pool(None, NoTls).unwrap();
  println!("Server running at http://{}/", config.server_addr);
  let server = HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone()))
      .route("/get_all", web::get().to(get_all_records))
      .route("/post_one", web::post().to(post_one_record))
  })
  .bind(config.server_addr.clone())?
  .run();
  server.await
}
