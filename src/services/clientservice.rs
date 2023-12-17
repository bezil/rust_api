use actix_web::{ get, post, Responder, HttpResponse, web::{Data, Json} };
use crate::model::{database::Database, client::{ClientRequest, ClientResponse}};
use validator::Validate;

#[post("/create-client")]
async fn create_client(body: Json<ClientRequest>, db_data: Data<Database>) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let new_name = body.name.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let created_client = db_data.add_client_to_db(ClientResponse::new(
                new_name,
                String::from(new_uuid),
            )).await;


            match created_client {
                Some(created)  => HttpResponse::Ok().body(format!("Client created {:?}", created)),
                None => HttpResponse::Ok().body("Client Creation failed")
            }
        }
        Err(_) => HttpResponse::Ok().body("No clients")
    }
}

#[get("/clients")]
async fn get_clients(db_data: Data<Database>) -> impl Responder {
    let fetched_clients = db_data.get_clients_from_db().await;
    match fetched_clients {
        Some(found_clients) => HttpResponse::Ok().body(format!("{:?}", found_clients)),
        None => HttpResponse::Ok().body("No clients"),
    }
}
