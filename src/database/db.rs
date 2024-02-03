// src/database/db.rs
use crate::config::Config;

use log::error;
use mongodb::{
    bson::{doc, Document},
    error::Result,
    results::InsertOneResult,
    Client, Collection, Database,
};

pub async fn db() -> Database {
    let config = Config::new();

    let client = match Client::with_uri_str(&config.mongodb_uri).await {
        Ok(client) => client,
        Err(err) => {
            error!("Error Initializing Database: {:?}", err);
            std::process::exit(1)
        }
    };

    let database = client.database(&config.db_name);
    database
}

// fn get_value<'a>(doc: &'a Document, key: &'a str) -> Option<&'a Bson> {
//     let mut current_doc = doc;

//     for part in key.split('.') {
//         match current_doc.get(part) {
//             Some(value) => {
//                 if let Bson::Document(next_doc) = value {
//                     current_doc = next_doc;
//                 } else {
//                     return Some(value);
//                 }
//             }
//             None => return None,
//         }
//     }

//     None
// }

pub struct CollectionDB {
    collection: Collection<Document>,
}

impl CollectionDB {
    pub async fn new(collection_name: &str) -> Self {
        let database = db().await;
        let collection = database.collection::<Document>(collection_name);
        Self { collection }
    }

    pub async fn doc(&self, _id: &i64) -> Result<Option<Document>> {
        let filter = doc! { "_id": _id };
        self.collection.find_one(filter, None).await
    }

 

    pub async fn insert(&self, doc: Document) -> Result<InsertOneResult> {
        self.collection.insert_one(doc, None).await
    }

    // pub async fn set(&self, _id: &i64, opts: Document) -> Result<()> {
    //     let filter = doc! { "_id": _id };
    //     let update = doc! { "$set": opts };
    //     self.collection.update_one(filter, update, None).await?;
    //     Ok(())
    // }

    // pub async fn push(&self, _id: &i64, opts: Document) -> Result<()> {
    //     let filter = doc! { "_id": _id };
    //     let update = doc! { "$push": opts };
    //     self.collection.update_one(filter, update, None).await?;
    //     Ok(())
    // }

    // pub async fn pull(&self, _id: &i64, opts: Document) -> Result<()> {
    //     let filter = doc! { "_id": _id };
    //     let update = doc! { "$pull": opts };
    //     self.collection.update_one(filter, update, None).await?;
    //     Ok(())
    // }

    // pub async fn inc(&self, _id: &i64, opts: Document) -> Result<()> {
    //     let filter = doc! { "_id": _id };
    //     let update = doc! { "$inc": opts };
    //     self.collection.update_one(filter, update, None).await?;
    //     Ok(())
    // }

    // pub async fn dec(&self, _id: &i64, opts: Document) -> Result<()> {
    //     let filter = doc! { "_id": _id };
    //     let update = doc! { "$dec": opts };
    //     self.collection.update_one(filter, update, None).await?;
    //     Ok(())
    // }

    // pub async fn delete(&self, _id: &i64) -> Result<()> {
    //     let filter = doc! { "_id": _id };
    //     self.collection.delete_one(filter, None).await?;
    //     Ok(())
    // }

    // pub async fn get_values(&self, _id: &i64, keys: &[&str]) -> Vec<(String, Option<Bson>)> {
    //     if let Some(doc) = self.doc(_id).await.unwrap() {
    //         let mut result = Vec::new();

    //         for key in keys {
    //             let value = get_value(&doc, key).cloned();
    //             result.push((key.to_string(), value));
    //         }

    //         return result;
    //     }

    //     Vec::new()
    // }

    // pub async fn distinct(&self, key: &str) -> Result<Vec<Bson>> {
    //     self.collection.distinct(key, None, None).await
    // }
}
