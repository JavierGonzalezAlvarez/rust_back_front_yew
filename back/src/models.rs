use serde::{Deserialize, Serialize};
extern crate tokio_pg_mapper;
extern crate tokio_pg_mapper_derive;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "tbemployee")]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub comments: String,
    //pub comments: Option<String>,
}
