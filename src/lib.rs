#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate bson;
extern crate mongodb;
extern crate rand;
extern crate redis;
extern crate iron;
extern crate router;
extern crate handlebars_iron;
extern crate params;
extern crate serde_urlencoded;
extern crate reqwest;
extern crate mount;
extern crate staticfile;

pub mod translate_result;
pub mod mongo_db;
pub mod session;
pub mod page_handler;
