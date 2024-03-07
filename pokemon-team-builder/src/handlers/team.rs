use serde::{Deserialize, Serialize};

use crate::{
    models::{
        enums::tier::Tier,
        teams::{Team, TeamSerial},
    },
    utils::traits::{GeneralAdaptor, IntoSerial},
};

#[derive(Debug, Clone, Serialize)]
struct TeamResponse {
    success: bool,
    team: Option<TeamSerial>,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ListTeamResponse {
    success: bool,
    teams: Option<Vec<TeamSerial>>,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTeam {
    name: String,
    description: String,
    user_id: i32,
    tier: Tier,
    is_favourite: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTeam {
    name: Option<String>,
    description: Option<String>,
    is_favourite: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct QueryTeam {
    name: Option<String>,
    is_favourite: Option<bool>,
    tier: Option<Tier>,
}

#[derive(Debug, Clone)]
pub struct TeamError(String);
impl std::fmt::Display for TeamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve the team resource due to the following error {:#?}",
            self.0
        )
    }
}

struct TeamService;
impl GeneralAdaptor for TeamService {
    type Error = TeamError;
    type Query = QueryTeam;
    type CreateBody = CreateTeam;
    type UpdateBody = UpdateTeam;
    type Response = Team;
    type ListResponse = Vec<Team>;

    async fn get_all_items(
        pool: &sqlx::PgPool,
        query: Self::Query,
    ) -> Result<Self::ListResponse, Self::Error> {
        match query {
            QueryTeam {name: Some(name), tier: Some(tier), is_favourite: Some(favourite) } => {
              sqlx::query!(r#"select id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at from team where name = $1 and tier = $2 and is_favourite = $3"#, name, tier as Tier, favourite).fetch_all(pool).await.map(|t| t.into_iter().map(|t| Team {
                  id: t.id,
                  name: t.name,
                  description: t.description,
                  created_at: t.created_at.unwrap(),
                  is_favourite: t.is_favourite.unwrap_or_default(),
                  tier: t.tier,
                updated_at: t.updated_at.unwrap(),
                user_id: t.user_id,
              }).collect::<Vec<Team>>()).map_err(|e| TeamError(format!("failed to retrieve all pokemon teams due to the following error: {e:#?}")))
            },
            QueryTeam {tier: Some(tier), is_favourite: Some(favourite), .. } => {
                sqlx::query!(r#"select id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at from team where tier = $1 and is_favourite = $2"#, tier as Tier, favourite).fetch_all(pool).await.map(|t| t.into_iter().map(|t| Team {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.unwrap(),
                    is_favourite: t.is_favourite.unwrap_or_default(),
                    tier: t.tier,
                  updated_at: t.updated_at.unwrap(),
                  user_id: t.user_id,
                }).collect::<Vec<Team>>()).map_err(|e| TeamError(format!("failed to retrieve all pokemon teams due to the following error: {e:#?}")))
            },
            QueryTeam {tier: Some(tier), .. } => {
                sqlx::query!(r#"select id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at from team where tier = $1"#, tier as Tier).fetch_all(pool).await.map(|t| t.into_iter().map(|t| Team {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.unwrap(),
                    is_favourite: t.is_favourite.unwrap_or_default(),
                    tier: t.tier,
                  updated_at: t.updated_at.unwrap(),
                  user_id: t.user_id,
                }).collect::<Vec<Team>>()).map_err(|e| TeamError(format!("failed to retrieve all pokemon teams due to the following error: {e:#?}")))
            },
            QueryTeam {is_favourite: Some(favourite), .. } => {
                sqlx::query!(r#"select id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at from team where is_favourite = $1"#, favourite).fetch_all(pool).await.map(|t| t.into_iter().map(|t| Team {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.unwrap(),
                    is_favourite: t.is_favourite.unwrap_or_default(),
                    tier: t.tier,
                  updated_at: t.updated_at.unwrap(),
                  user_id: t.user_id,
                }).collect::<Vec<Team>>()).map_err(|e| TeamError(format!("failed to retrieve all pokemon teams due to the following error: {e:#?}")))
            },
            _ => {
                sqlx::query!(r#"select id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at from team"#).fetch_all(pool).await.map(|t| t.into_iter().map(|t| Team {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.unwrap(),
                    is_favourite: t.is_favourite.unwrap_or_default(),
                    tier: t.tier,
                  updated_at: t.updated_at.unwrap(),
                  user_id: t.user_id,
                }).collect::<Vec<Team>>()).map_err(|e| TeamError(format!("failed to retrieve all pokemon teams due to the following error: {e:#?}")))
            }
        }
    }

    async fn get_one_item(id: i32, pool: &sqlx::PgPool) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"select id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at from team where id = $1"#, id).fetch_one(pool).await.map(|t| Team {
            id: t.id,
            name: t.name,
            description: t.description,
            created_at: t.created_at.unwrap(),
            is_favourite: t.is_favourite.unwrap_or_default(),
            tier: t.tier,
          updated_at: t.updated_at.unwrap(),
          user_id: t.user_id,
        }).map_err(|e| TeamError(format!("failed to retrieve a team with the id = {id} due to the following error: {e:#?}")))
    }

    async fn create_one_item(
        pool: &sqlx::PgPool,
        create: Self::CreateBody,
    ) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"insert into team (name, description, user_id, tier, is_favourite) values ($1, $2, $3, $4, $5) returning id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at"#, create.name, create.description, create.user_id, create.tier as Tier, create.is_favourite).fetch_one(pool).await.map(|t| Team {
            id: t.id,
            name: t.name,
            description: t.description,
            created_at: t.created_at.unwrap(),
            is_favourite: t.is_favourite.unwrap_or_default(),
            tier: t.tier,
          updated_at: t.updated_at.unwrap(),
          user_id: t.user_id,
        }).map_err(|e| TeamError(format!("failed to create a team with the provided details due to the following error: {e:#?}")))
    }

    async fn update_one_item(
        id: i32,
        pool: &sqlx::PgPool,
        update: Self::UpdateBody,
    ) -> Result<Self::Response, Self::Error> {
        match update {
            UpdateTeam{name: Some(name), description: Some(des), ..} => {
                sqlx::query!(r#"update team set name = $1, description = $2, is_favourite = $3 where id = $4 returning id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at"#, name, des, update.is_favourite, id).fetch_one(pool).await.map(|t| Team {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.unwrap(),
                    is_favourite: t.is_favourite.unwrap_or_default(),
                    tier: t.tier,
                  updated_at: t.updated_at.unwrap(),
                  user_id: t.user_id,
                }).map_err(|e| TeamError(format!("failed to update the team with id = {id} with the provided details due to the following error: {e:#?}")))
            },
            UpdateTeam{description: Some(des), ..} => {
                sqlx::query!(r#"update team set description = $1, is_favourite = $2 where id = $3 returning id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at"#, des, update.is_favourite, id).fetch_one(pool).await.map(|t| Team {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.unwrap(),
                    is_favourite: t.is_favourite.unwrap_or_default(),
                    tier: t.tier,
                  updated_at: t.updated_at.unwrap(),
                  user_id: t.user_id,
                }).map_err(|e| TeamError(format!("failed to update the team with id = {id} with the provided details due to the following error: {e:#?}")))
            },
            UpdateTeam{name: Some(name), ..} => {
                sqlx::query!(r#"update team set name = $1, is_favourite = $2 where id = $3 returning id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at"#, name, update.is_favourite, id).fetch_one(pool).await.map(|t| Team {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.unwrap(),
                    is_favourite: t.is_favourite.unwrap_or_default(),
                    tier: t.tier,
                  updated_at: t.updated_at.unwrap(),
                  user_id: t.user_id,
                }).map_err(|e| TeamError(format!("failed to update the team with id = {id} with the provided details due to the following error: {e:#?}")))
            },
            _ => {
                sqlx::query!(r#"select id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at from team where id = $1"#, id).fetch_one(pool).await.map(|t| Team {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.unwrap(),
                    is_favourite: t.is_favourite.unwrap_or_default(),
                    tier: t.tier,
                  updated_at: t.updated_at.unwrap(),
                  user_id: t.user_id,
                }).map_err(|e| TeamError(format!("failed to update the team with id = {id} with the provided details due to the following error: {e:#?}")))
            }
        }
    }

    async fn delete_one_item(id: i32, pool: &sqlx::PgPool) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"delete from team where id = $1 returning id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at"#, id).fetch_one(pool).await.map(|t| Team {
            id: t.id,
            name: t.name,
            description: t.description,
            created_at: t.created_at.unwrap(),
            is_favourite: t.is_favourite.unwrap_or_default(),
            tier: t.tier,
          updated_at: t.updated_at.unwrap(),
          user_id: t.user_id,
        }).map_err(|e| TeamError(format!("failed to delete the team with the id = {id} due to the following error: {e:#?}")))
    }
}

impl TeamService {
    async fn get_users_team(
        id: i32,
        query: QueryTeam,
        pool: &sqlx::PgPool,
    ) -> Result<Vec<Team>, TeamError> {
        match query {
            QueryTeam {name: Some(name), tier: Some(tier), is_favourite: Some(favourite) } => {
              sqlx::query!(r#"select id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at from team where name = $1 and tier = $2 and is_favourite = $3 and user_id = $4"#, name, tier as Tier, favourite, id).fetch_all(pool).await.map(|t| t.into_iter().map(|t| Team {
                  id: t.id,
                  name: t.name,
                  description: t.description,
                  created_at: t.created_at.unwrap(),
                  is_favourite: t.is_favourite.unwrap_or_default(),
                  tier: t.tier,
                updated_at: t.updated_at.unwrap(),
                user_id: t.user_id,
              }).collect::<Vec<Team>>()).map_err(|e| TeamError(format!("failed to retrieve all pokemon teams due to the following error: {e:#?}")))
            },
            QueryTeam {tier: Some(tier), is_favourite: Some(favourite), .. } => {
                sqlx::query!(r#"select id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at from team where tier = $1 and is_favourite = $2 and user_id = $3"#, tier as Tier, favourite, id).fetch_all(pool).await.map(|t| t.into_iter().map(|t| Team {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.unwrap(),
                    is_favourite: t.is_favourite.unwrap_or_default(),
                    tier: t.tier,
                  updated_at: t.updated_at.unwrap(),
                  user_id: t.user_id,
                }).collect::<Vec<Team>>()).map_err(|e| TeamError(format!("failed to retrieve all pokemon teams due to the following error: {e:#?}")))
            },
            QueryTeam {tier: Some(tier), .. } => {
                sqlx::query!(r#"select id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at from team where tier = $1 and user_id = $2"#, tier as Tier, id).fetch_all(pool).await.map(|t| t.into_iter().map(|t| Team {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.unwrap(),
                    is_favourite: t.is_favourite.unwrap_or_default(),
                    tier: t.tier,
                  updated_at: t.updated_at.unwrap(),
                  user_id: t.user_id,
                }).collect::<Vec<Team>>()).map_err(|e| TeamError(format!("failed to retrieve all pokemon teams due to the following error: {e:#?}")))
            },
            QueryTeam {is_favourite: Some(favourite), .. } => {
                sqlx::query!(r#"select id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at from team where is_favourite = $1 and user_id = $2"#, favourite, id).fetch_all(pool).await.map(|t| t.into_iter().map(|t| Team {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.unwrap(),
                    is_favourite: t.is_favourite.unwrap_or_default(),
                    tier: t.tier,
                  updated_at: t.updated_at.unwrap(),
                  user_id: t.user_id,
                }).collect::<Vec<Team>>()).map_err(|e| TeamError(format!("failed to retrieve all pokemon teams due to the following error: {e:#?}")))
            },
            _ => {
                sqlx::query!(r#"select id, name, description, user_id, tier as "tier: Tier", is_favourite, created_at, updated_at from team where user_id = $1"#, id).fetch_all(pool).await.map(|t| t.into_iter().map(|t| Team {
                    id: t.id,
                    name: t.name,
                    description: t.description,
                    created_at: t.created_at.unwrap(),
                    is_favourite: t.is_favourite.unwrap_or_default(),
                    tier: t.tier,
                  updated_at: t.updated_at.unwrap(),
                  user_id: t.user_id,
                }).collect::<Vec<Team>>()).map_err(|e| TeamError(format!("failed to retrieve all pokemon teams due to the following error: {e:#?}")))
            }
        }
    }
}

use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::PgPool;

pub async fn get_all_teams(
    State(pool): State<PgPool>,
    query: Option<Query<QueryTeam>>,
) -> impl IntoResponse {
    match TeamService::get_all_items(&pool, query.unwrap_or_default().0).await {
        Ok(teams) => (
            StatusCode::OK,
            Json(ListTeamResponse {
                success: true,
                teams: Some(
                    teams
                        .into_iter()
                        .map(|t| t.to_serial())
                        .collect::<Vec<TeamSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ListTeamResponse {
                success: false,
                teams: None,
                error: Some(format!(
                    "failed to retrieve all teams due to the following error: {e:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_team_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match TeamService::get_one_item(id, &pool).await {
        Ok(team) => (StatusCode::OK, Json(TeamResponse {
            success: true,
            team: Some(team.to_serial()),
            error: None
        })),
        Err(e) => (StatusCode::BAD_REQUEST, Json(TeamResponse {
            success: false,
            team: None,
            error: Some(format!("failed to retrieve a team with an id = {id} due to the following error: {e:#?}"))
        }))
    }
}

pub async fn create_team(
    State(pool): State<PgPool>,
    Json(create): Json<CreateTeam>,
) -> impl IntoResponse {
    match TeamService::create_one_item(&pool, create).await {
        Ok(team) => (StatusCode::CREATED, Json(TeamResponse {
            success: true,
            team: Some(team.to_serial()),
            error: None,
        })),
        Err(e) => (StatusCode::BAD_REQUEST, Json(TeamResponse {
            success: false,
            team: None,
            error: Some(format!("failed to create a team with the provided details due to the following error: {e:#?}"))
        }))
    }
}

pub async fn update_team(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateTeam>,
) -> impl IntoResponse {
    match TeamService::update_one_item(id, &pool, update).await {
        Ok(team) => (StatusCode::OK, Json(TeamResponse {
            success: true,
            team: Some(team.to_serial()),
            error: None,
        })),
        Err(e) => (StatusCode::BAD_REQUEST, Json(TeamResponse {
            success: false,
            team: None,
            error: Some(format!("failed to update the team with id = {id} with the provided details due to the following error: {e:#?}"))
        }))
    }
}

pub async fn delete_team(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match TeamService::delete_one_item(id, &pool).await {
        Ok(team) => (
            StatusCode::OK,
            Json(TeamResponse {
                success: true,
                team: Some(team.to_serial()),
                error: None,
            }),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(TeamResponse {
                success: false,
                team: None,
                error: Some(format!(
                    "failed to delete the team with id = {id} due to the following error: {e:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_users_teams(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
    query: Option<Query<QueryTeam>>,
) -> impl IntoResponse {
    match TeamService::get_users_team(user_id, query.unwrap_or_default().0, &pool).await {
        Ok(teams) => (
            StatusCode::OK,
            Json(ListTeamResponse {
                success: true,
                teams: Some(
                    teams
                        .into_iter()
                        .map(|t| t.to_serial())
                        .collect::<Vec<TeamSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ListTeamResponse {
                success: false,
                teams: None,
                error: Some(format!(
                    "failed to retrieve all teams due to the following error: {e:#?}"
                )),
            }),
        ),
    }
}
