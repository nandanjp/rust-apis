use std::rc::Rc;

use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::error::Result;
use mongodb::{Client, Collection, Database};

use crate::models::traits::{GeneralServices, IntoDocument};
use crate::models::user::User;

pub struct Service {
    db: Rc<Database>,
}

impl Service {
    fn new(client: &Client, db_name: &String) -> Self {
        let db = client.database(db_name.as_str());
        Service { db: Rc::new(db) }
    }

    fn get_collection<T>(&self, collection: &'static str) -> Collection<T> {
        self.db.clone().collection::<T>(collection)
    }
}

impl GeneralServices<User> for Service {
    async fn find_all(&self) -> Result<Vec<User>> {
        let users: Result<Vec<User>> = self
            .get_collection::<User>("user")
            .find(None, None)
            .await?
            .try_collect()
            .await;
        users
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<User> {
        self.get_collection::<User>("user")
            .find_one(Some(doc! {"_id": id}), None)
            .await
            .map(|u| u.unwrap())
    }
    async fn find_query(&self, query_obj: Document) -> Result<Vec<User>> {
        self.get_collection::<User>("user")
            .find(Some(query_obj), None)
            .await?
            .try_collect()
            .await
    }
    async fn create(&self, data: User) -> Result<User> {
        let result = self
            .get_collection::<User>("user")
            .insert_one(data, None)
            .await?;
        self.find_by_id(result.inserted_id.as_object_id().unwrap())
            .await
    }
    async fn update(&self, id: ObjectId, data: User) -> Result<User> {
        self.get_collection::<User>("user")
            .find_one_and_update(doc! {"_id": id}, data.into_doc(), None)
            .await
            .map(|u| u.unwrap())
    }
    async fn delete(&self, id: ObjectId) -> Result<User> {
        self.get_collection::<User>("user")
            .find_one_and_delete(doc! {"_id": id}, None)
            .await
            .map(|u| u.unwrap())
    }
}
