use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]    
pub struct StoredType{
    pub key: String,
    pub data: Vec<u8>,
}
