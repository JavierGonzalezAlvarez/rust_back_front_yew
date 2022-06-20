use crate::errors::MyError;
use crate::models::Employee;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<Employee>, MyError> {
    let res = client
        .query("SELECT * FROM tbemployee", &[])
        .await?
        .iter()
        .map(|row| Employee::from_row_ref(row).unwrap())
        .collect::<Vec<Employee>>();
    Ok(res)
}
