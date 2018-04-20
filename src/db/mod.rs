extern crate bson;
extern crate mongodb;

use bson::Bson;
use mongodb::Client;


pub struct DBConfig {
    address: String,
    host: u32,
    db: String,
    collection: String
}

pub struct DB {
    config: DBConfig,
    collection: Client
}


impl DB {
    pub fn connect(config: DBConfig) -> DB {
        println!("连接到DB{}:{}", config.address, config.port);
//        DB {
//            config,
//            collection
//        }
    }
}
