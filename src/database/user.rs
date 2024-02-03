use crate::database::models::UserDB;
use log::{error, info};
use mongodb::{bson::{doc, Document}, error::Result as MongoResult, results::InsertOneResult};
use super::db::CollectionDB;

impl UserDB {
    pub async fn is_unique(&self) -> bool {
        let users = CollectionDB::new("Users").await;

        let doc = users.doc(&(self._id.0 as i64)).await.unwrap_or(None);

        if let Some(_) = doc {
            false
        } else {
            true
        }
    }
   pub async fn save(&self) -> MongoResult<InsertOneResult> {
        let users = CollectionDB::new("Users").await;
        
        match users.insert(self.to_doc()).await {
            Ok(result) => {
                info!("[mongo] Inserted User: {0}", self._id);
                Ok(result)
            },
            Err(err) => {
                error!("[mongo] Failed to Insert User: {0}\nError: {1}", self._id, err);
                Err(err)
            }
        }
    }
    fn to_doc(&self) -> Document {
        let _id = self._id.0 as i64;
        doc! {
            "_id": _id,  // Assuming UserId implements the necessary trait for conversion
            "name": &self.name,
            "username": &self.username,
        }
    }
}