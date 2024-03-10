pub struct Config {
    pub port: u32,
    pub db_url: String,
    pub spotify_secret: String,
    pub spotify_client_id: String,
    pub jwt_token: String,
}

impl Config {
    pub fn new() -> Result<Self, String> {
        dotenv::dotenv().ok();

        let port = std::env::var("PORT").map_err(|e| format!("failed to parse a port: {e}"))?;
        Ok(Self {
            port: port.parse::<u32>().map_err(|e| format!("failed to parse the provided port into a valid port value: {e}"))?,
            db_url: std::env::var("DATABASE_URL")
                .map_err(|e| format!("failed to parse a database_url: {e}"))?,
            spotify_client_id: std::env::var("SPOTIFY_CLIENT_ID")
                .map_err(|e| format!("failed to parse a spotify client id: {e}"))?,
            spotify_secret: std::env::var("SPOTIFY_CLIENT_SECRET")
                .map_err(|e| format!("failed to parse a spotify secret: {e}"))?,
            jwt_token: std::env::var("JWT_SECRET")
                .map_err(|e| format!("failed to parse a jwt secret: {e}"))?,
        })
    }
}
