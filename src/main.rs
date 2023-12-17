pub mod model;
pub mod services;
use crate::model::database::Database;
use crate::services::clientservice::{ get_clients, create_client };

use actix_web::{ HttpServer, App, web::Data };
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Retrieve the value of the environment variable
    let address_str = env::var("ADDRESS").expect("Environment variable not found");
    // Parse the string into a SocketAddr
    let socket_addr: SocketAddr = address_str.parse().expect("Failed to parse address");


    let db = Database::init()
        .await.expect("Error connecting to Database");

    let db_data = Data::new(db);

    HttpServer::new(move ||  { App::new()
        .app_data(db_data.clone())
        .service(get_clients)
        .service(create_client)
    }).bind(socket_addr)?.run().await
}
