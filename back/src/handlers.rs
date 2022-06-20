use crate::db;
use crate::errors::MyError;
use crate::models::Employee;

use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsertEmployee {
    name: String,
    phone: String,
    email: String,
    comments: String,
}

pub async fn post_one_record(
    db_pool: web::Data<Pool>,
    items: web::Json<InsertEmployee>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = post_one(&client, items).await;
    println!("result {:#?}", result);

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn post_one(
    client: &Client,
    items: web::Json<InsertEmployee>,
) -> Result<Vec<Employee>, MyError> {
    let name = &items.name;
    let phone = &items.phone;
    let email = &items.email;
    let comments = &items.comments;
    let item = InsertEmployee {
        name: name.into(),
        phone: phone.into(),
        email: email.into(),
        comments: comments.into(),
    };
    //
    let value = serde_json::to_string(&item);
    println!("value {:#?}", value);
    println!("json {:#?}", items);
    println!("name {:#?}", item.name);
    //
    let res = client
        .query(
            "INSERT INTO tbemployee (name, phone, email, comments) VALUES ($1, $2, $3, $4)",
            &[&item.name, &item.phone, &item.email, &item.comments],
        )
        .await?
        .iter()
        .map(|row| Employee::from_row_ref(row).unwrap())
        .collect::<Vec<Employee>>();
    Ok(res)
}

pub async fn get_all_records(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::get_todos(&client).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
