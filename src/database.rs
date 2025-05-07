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

    // pub async fn print_all(&self) {
    //     let db_map = self.db.lock().await;
    //     let exp_map = self.expiry.lock().await;
    //     for key in db_map.keys() {
    //         if exp_map.contains_key(key) {
    //             println!("{} = {} ({:?})", key, db_map.get(key).unwrap(), exp_map.get(key).unwrap());
    //         } else {
    //             println!("{} = {}", key, db_map.get(key).unwrap());
    //         }
    //     }
    // }
}