use mongodb::{
    bson::{oid::ObjectId, Document},
    error::Result,
};

pub trait GeneralServices<T> {
    async fn find_by_id(&self, id: ObjectId) -> Result<T>;
    async fn find_all(&self) -> Result<Vec<T>>;
    async fn find_query(&self, query_obj: Document) -> Result<Vec<T>>;
    async fn create(&self, data: T) -> Result<T>;
    async fn update(&self, id: ObjectId, data: T) -> Result<T>;
    async fn delete(&self, id: ObjectId) -> Result<T>;
}

pub trait IntoDocument {
    fn into_doc(self) -> Document;
}
