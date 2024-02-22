use crate::utils::traits::ConfigParser;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct MongoConfig {
    /// Database Username
    #[clap(short, long, env("MONGO_USER"))]
    user_mongo: String,
    /// Database Password
    #[clap(short, long, env("MONGO_PASS"))]
    pass_mongo: String,
    /// Database URI
    #[clap(short, long, env("MONGO_URI"))]
    mongo_uri: String,
    /// Database Connection Timeout
    #[clap(short, long, env("MONGO_CONNECTION_TIMEOUT"))]
    connection_timeout: u32,
    /// Database Maximum Pool Size
    #[clap(long, env("MONGO_MAX_POOL_SIZE"))]
    max_pool_size: u32,
    /// Database Minimum Pool Size
    #[clap(long, env("MONGO_MIN_POOL_SIZE"))]
    min_pool_size: u32,
}

impl MongoConfig {
    pub fn create_uri(&mut self) -> Option<()> {
        //mongodb+srv://nandanjp17:<password>@cluster0.u0cjfh5.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0
        let password = self
            .mongo_uri
            .find("<password>")
            .map(|start| start..=start + "<password".len())?;
        self.mongo_uri
            .replace_range(password, self.pass_mongo.clone().as_str());
        Some(())
    }

    pub fn mongo_uri(&self) -> &String {
        &self.mongo_uri
    }
    pub fn connection_timeout(&self) -> u32 {
        self.connection_timeout
    }
    pub fn max_pool_size(&self) -> u32 {
        self.max_pool_size
    }
    pub fn min_pool_size(&self) -> u32 {
        self.min_pool_size
    }
}

impl ConfigParser for MongoConfig {}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct UserConfig {
    /// API Port
    #[clap(long, env("PORT"), default_value_t = 5000)]
    pub port: u16,
}
impl ConfigParser for UserConfig {}

#[derive(Debug)]
pub struct Config {
    mongo_config: MongoConfig,
    user_config: UserConfig,
}

impl Config {
    pub fn new() -> Self {
        let mut mongo_config = MongoConfig::from_env_and_args();
        match mongo_config.create_uri() {
            Some(_) => println!("Successfully parsed the uri"),
            None => panic!("failed to parse the mongodb uri"),
        }
        Config {
            mongo_config,
            user_config: UserConfig::from_env_and_args(),
        }
    }

    pub fn mongo_config(&self) -> &MongoConfig {
        &self.mongo_config
    }

    pub fn user_config(&self) -> &UserConfig {
        &self.user_config
    }
}
