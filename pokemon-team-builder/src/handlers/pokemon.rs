use axum::{Router, routing::get, routing::post, response::Json};
use axum::extract::{Path, Query, Json, extract::Extension};

use crate::models::user::User;

use super::service::Service;

pub struct PokemonHandler {
    router: Router,
    service: Service,
}

impl PokemonHandler {

}
