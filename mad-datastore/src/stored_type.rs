use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]    
pub struct StoredType{
    pub key: String,
    pub data: Vec<u8>,
}
