use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::Instant};

#[derive(Clone)]
pub struct Database {
    db: Arc<Mutex<HashMap<String, String>>>,
    expiry: Arc<Mutex<HashMap<String, Instant>>>
}

impl Database {
    pub fn new() -> Database {
        Database {
            db: Arc::new(Mutex::new(HashMap::new())),
            expiry: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn set(&self, key: String, value: String, ttl: Option<u64>) {
        let mut db_map = self.db.lock().await;
        db_map.insert(key.clone(), value);

        if let Some(seconds) = ttl {
            let mut exp_map = self.expiry.lock().await;
            exp_map.insert(key.clone(), Instant::now() + Duration::from_secs(seconds));
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        if self.is_expired(key).await {
            self.delete(key).await;
            return None;
        }
        let db_map = self.db.lock().await;
        db_map.get(key).cloned()
    }

    pub async fn get_keys(&self) -> Option<Vec<String>> {
        let db_map = self.db.lock().await;
        let mut keys= Vec::<String>::new();
        if db_map.len() == 0 {
            return None;
        }
        for key in db_map.keys() {
            keys.push(key.clone());
        }
        Some(keys)
    }

    pub async fn get_all(&self) -> Option<HashMap<String, String>> {
        let db_map = self.db.lock().await;
        if db_map.len() == 0 {
            return None;
        }
        let values = db_map.clone();
        Some(values)
    }

    pub async fn get_sw(&self, pattern: &str) -> Option<HashMap<String, String>> {
        let keys = self.get_keys().await;
        if let Some(keys) = keys {
            let target_keys: Vec<&String> = keys
                .iter()
                .filter(|&target_key| target_key.starts_with(pattern))
                .collect();
            
            if target_keys.len() == 0 {
                return None;
            }
            
            let db_map = self.db.lock().await;
            let mut target = HashMap::<String, String>::new();

            for key in target_keys {
                target.insert(key.clone(), db_map.get(key).unwrap().clone());
            }

            return Some(target);
        }
        else {
            return None;
        }
    }

    pub async fn get_ew(&self, pattern: &str) -> Option<HashMap<String, String>> {
        let keys = self.get_keys().await;
        if let Some(keys) = keys {
            let target_keys: Vec<&String> = keys
                .iter()
                .filter(|&target_key| target_key.ends_with(pattern))
                .collect();
            
            if target_keys.len() == 0 {
                return None;
            }
            
            let db_map = self.db.lock().await;
            let mut target = HashMap::<String, String>::new();

            for key in target_keys {
                target.insert(key.clone(), db_map.get(key).unwrap().clone());
            }

            return Some(target);
        }
        else {
            return None;
        }
    }

    pub async fn is_expired(&self, key: &str) -> bool {
        let exp_map = self.expiry.lock().await;
        if let Some(&exp_time) = exp_map.get(key) {
            return Instant::now() > exp_time;
        }
        false
    }

    pub async fn delete(&self, key: &str) -> bool {
        let mut db_map = self.db.lock().await;
        let mut exp_map = self.expiry.lock().await;
        exp_map.remove(key);
        db_map.remove(key).is_some()
    }
}