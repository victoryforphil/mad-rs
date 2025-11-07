use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]    
pub struct StoredType{
    pub key: String,
    pub data: Vec<u8>,
}


impl StoredType{
    pub fn new(key: String, data: Vec<u8>) -> Self{
        Self{ key, data }
    }

    pub fn new_blank(key: String) -> Self{
        Self{ key, data: Vec::new() }
    }
}

impl Default for StoredType{
    fn default() -> Self{
        Self::new_blank("NULL".to_string())
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new(){
        let stored_type = StoredType::new("test".to_string(), vec![1, 2, 3]);
        assert_eq!(stored_type.key, "test");
        assert_eq!(stored_type.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_new_blank(){
        let stored_type = StoredType::new_blank("test".to_string());
        assert_eq!(stored_type.key, "test");
        assert_eq!(stored_type.data, Vec::new());
    }

    #[test]
    fn test_default(){
        let stored_type = StoredType::default();
        assert_eq!(stored_type.key, "NULL");
        assert_eq!(stored_type.data, Vec::new());
    }
}