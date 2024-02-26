use mongodb::{Client, Collection};

pub async fn get_collection<T>(client: &Client, collection: &'static str) -> Collection<T> {
    client.database("mdl_mal").collection::<T>(collection)
}