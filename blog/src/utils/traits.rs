use clap::Parser;
use dotenv::dotenv;

pub trait ConfigParser: Parser {
    fn from_env_and_args() -> Self {
        dotenv().ok();
        Self::parse()
    }
}
