use serde::{Serialize, Deserialize };
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientResponse {
    pub name: String,
    pub uuid: String
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct ClientRequest {
    #[validate(length(min=1, message="Minimum one character required"))]
    pub name: String,
}

impl ClientResponse {
    pub fn new(uuid: String, name: String) -> ClientResponse {
        ClientResponse { uuid, name }
    }
}
