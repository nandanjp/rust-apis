use std::rc::Rc;

use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::error::Result;
use mongodb::{Client, Collection, Database};

use crate::models::abilities::Ability;
use crate::models::moves::Move;
use crate::models::pokemon::Pokemon;
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
        self.get_collection::<User>("user").find_one(doc! {"_id": result.inserted_id.as_object_id().unwrap()}, None)
            .await.map(|u| u.unwrap())
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

impl GeneralServices<Pokemon> for Service {
    async fn find_all(&self) -> Result<Vec<Pokemon>> {
        let users: Result<Vec<Pokemon>> = self
            .get_collection::<Pokemon>("pokemon")
            .find(None, None)
            .await?
            .try_collect()
            .await;
        users
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Pokemon> {
        self.get_collection::<Pokemon>("pokemon")
            .find_one(Some(doc! {"_id": id}), None)
            .await
            .map(|u| u.unwrap())
    }
    async fn find_query(&self, query_obj: Document) -> Result<Vec<Pokemon>> {
        self.get_collection::<Pokemon>("pokemon")
            .find(Some(query_obj), None)
            .await?
            .try_collect()
            .await
    }
    async fn create(&self, data: Pokemon) -> Result<Pokemon> {
        let result = self
            .get_collection::<Pokemon>("pokemon")
            .insert_one(data, None)
            .await?;
        self.get_collection::<Pokemon>("pokemon").find_one(doc! {"_id": result.inserted_id.as_object_id().unwrap()}, None)
            .await.map(|u| u.unwrap())
    }
    async fn update(&self, id: ObjectId, data: Pokemon) -> Result<Pokemon> {
        self.get_collection::<Pokemon>("pokemon")
            .find_one_and_update(doc! {"_id": id}, data.into_doc(), None)
            .await
            .map(|u| u.unwrap())
    }
    async fn delete(&self, id: ObjectId) -> Result<Pokemon> {
        self.get_collection::<Pokemon>("pokemon")
            .find_one_and_delete(doc! {"_id": id}, None)
            .await
            .map(|u| u.unwrap())
    }
}

impl GeneralServices<Ability> for Service {
    async fn find_all(&self) -> Result<Vec<Ability>> {
        let users: Result<Vec<Ability>> = self
            .get_collection::<Ability>("abilities")
            .find(None, None)
            .await?
            .try_collect()
            .await;
        users
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Ability> {
        self.get_collection::<Ability>("abilities")
            .find_one(Some(doc! {"_id": id}), None)
            .await
            .map(|u| u.unwrap())
    }
    async fn find_query(&self, query_obj: Document) -> Result<Vec<Ability>> {
        self.get_collection::<Ability>("abilities")
            .find(Some(query_obj), None)
            .await?
            .try_collect()
            .await
    }
    async fn create(&self, data: Ability) -> Result<Ability> {
        let result = self
            .get_collection::<Ability>("abilities")
            .insert_one(data, None)
            .await?;
        self.get_collection::<Ability>("abilities").find_one(doc! {"_id": result.inserted_id.as_object_id().unwrap()}, None)
            .await.map(|u| u.unwrap())
    }
    async fn update(&self, id: ObjectId, data: Ability) -> Result<Ability> {
        self.get_collection::<Ability>("abilities")
            .find_one_and_update(doc! {"_id": id}, data.into_doc(), None)
            .await
            .map(|u| u.unwrap())
    }
    async fn delete(&self, id: ObjectId) -> Result<Ability> {
        self.get_collection::<Ability>("abilities")
            .find_one_and_delete(doc! {"_id": id}, None)
            .await
            .map(|u| u.unwrap())
    }
}

impl GeneralServices<Move> for Service {
    async fn find_all(&self) -> Result<Vec<Move>> {
        let users: Result<Vec<Move>> = self
            .get_collection::<Move>("moves")
            .find(None, None)
            .await?
            .try_collect()
            .await;
        users
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Move> {
        self.get_collection::<Move>("moves")
            .find_one(Some(doc! {"_id": id}), None)
            .await
            .map(|u| u.unwrap())
    }
    async fn find_query(&self, query_obj: Document) -> Result<Vec<Move>> {
        self.get_collection::<Move>("moves")
            .find(Some(query_obj), None)
            .await?
            .try_collect()
            .await
    }
    async fn create(&self, data: Move) -> Result<Move> {
        let result = self
            .get_collection::<Move>("moves")
            .insert_one(data, None)
            .await?;
        self.get_collection::<Move>("moves").find_one(doc! {"_id": result.inserted_id.as_object_id().unwrap()}, None)
            .await.map(|u| u.unwrap())
    }
    async fn update(&self, id: ObjectId, data: Move) -> Result<Move> {
        self.get_collection::<Move>("moves")
            .find_one_and_update(doc! {"_id": id}, data.into_doc(), None)
            .await
            .map(|u| u.unwrap())
    }
    async fn delete(&self, id: ObjectId) -> Result<Move> {
        self.get_collection::<Move>("moves")
            .find_one_and_delete(doc! {"_id": id}, None)
            .await
            .map(|u| u.unwrap())
    }
}


