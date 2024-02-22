//use std::env;

use clap::Parser;
use dotenv::dotenv;

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
}

impl MongoConfig {
    pub fn from_env_and_args() -> Self {
        dotenv().ok();
        Self::parse()
    }

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
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct UserConfig {
    /// API Port
    #[clap(long, env("PORT"), default_value_t = 5000)]
    pub port: u16,
}

impl UserConfig {
    fn from_env_and_args() -> Self {
        dotenv().ok();
        Self::parse()
    }
}

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
