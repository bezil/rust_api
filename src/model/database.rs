use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};
use std::env;
use std::net::SocketAddr;
use crate::model::client::ClientResponse;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        // Retrieve the value of the environment variable
        let address_str = env::var("SURREAL_ADDRESS").expect("Environment variable not found");
        // Parse the string into a SocketAddr
        let socket_addr: SocketAddr = address_str.parse().expect("Failed to parse address");
        // Connect to the server
        let client = Surreal::new::<Ws>(socket_addr).await?;

        let user = env::var("SURREAL_USER").expect("Username key is missing");
        let password = env::var("SURREAL_PASSWORD").expect("Password key is missing");
        let dbname = env::var("SURREAL_DB_NAME").expect("Database name key is missing");
        let namespace = env::var("SURREAL_NAME_SPACE").expect("Namespace key is missing");


        // Signin as a namespace, database, or root user
        client.signin(Root {
            username: &user,
            password: &password,
        })
        .await?;

        // Select a specific namespace / database
        client.use_ns(&namespace).use_db(&dbname).await.unwrap();
        Ok(Database {
            client,
            name_space: namespace.clone(),
            db_name: dbname.clone()
        })
    }

    pub async fn get_clients_from_db(&self) -> Option<Vec<ClientResponse>> {
        let result = self.client.select("user").await;

        match result {
            Ok(found_clients) => Some(found_clients),
            Err(_) => None
        }
    }

    pub async fn add_client_to_db(&self, new_entry: ClientResponse) -> Option<ClientResponse> {
        // Create a new client
        let created = self.client
            .create(("user", new_entry.uuid.clone()))
            .content(new_entry)
        .await;

        dbg!(&created);

       match created {
            Ok(created_entry) => created_entry,
            Err(_) => None
       }
    }
}
