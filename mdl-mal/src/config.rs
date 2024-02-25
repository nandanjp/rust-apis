use clap::Parser;
use crate::utils::traits::ConfigParser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct MongoConfig {
    #[clap(short, long, env("MONGO_PASSWORD"))]
    pass_mongo: String,
    #[clap(short, long, env("MONGO_URI"))]
    uri_mongo: String,
    #[clap(short, long, env("CONNECTION_TIMEOUT"))]
    connection_timeout: u32,
    #[clap(long, env("MAX_POOL_SIZE"))]
    max_pool_size: u32,
    #[clap(long, env("MIN_POOL_SIZE"))]
    min_pool_size: u32,
}
impl ConfigParser for MongoConfig {}
impl MongoConfig {
    pub fn create_uri(&mut self) -> Option<()> {
        let password = self.uri_mongo.find("<password>").map(|start| start..start + "<password>".len())?;
        self.uri_mongo.replace_range(password, self.pass_mongo.clone().as_str());
        Some(())
    }

    pub fn get_uri(&self) -> &String {
        &self.uri_mongo
    }
    #[allow(dead_code)]
    pub fn get_password(&self) -> &String {
        &self.pass_mongo
    }
    pub fn get_connection(&self) -> u32 {
        self.connection_timeout
    }
    pub fn get_max_pool(&self) -> u32 {
        self.max_pool_size
    }
    pub fn get_min_pool(&self) -> u32 {
        self.min_pool_size
    }
}

#[derive(Parser, Clone, Debug)]
#[command(author, version, about)]
pub struct UserConfig {
    #[clap(short, long, env("PORT"))]
    port: u32,
}
impl ConfigParser for UserConfig {}

impl UserConfig {
    pub fn get_port(&self) -> u32 {
        self.port
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    mongo_config: MongoConfig,
    user_config: UserConfig,
}

impl Config {
    pub fn new() -> Self {
        let mut mongo_config = MongoConfig::from_env_and_args();
        match mongo_config.create_uri() {
            Some(_) => println!("Successfully parsed the uri"),
            None => panic!("Failed to parse the mongodb uri"),
        }
        Config {
            mongo_config,
            user_config: UserConfig::from_env_and_args()
        }
    }

    pub fn get_mongo_config(&self) -> &MongoConfig {
        &self.mongo_config
    }

    pub fn get_user_config(&self) -> &UserConfig {
        &self.user_config
    }
}