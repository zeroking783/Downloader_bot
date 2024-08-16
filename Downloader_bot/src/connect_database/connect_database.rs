use tokio_postgres::{NoTls, Client, Error};

pub use crate::connect_database::parse_config::parse;

pub async fn connect_to_database() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(&parse().unwrap_or_else(
        |err| {panic!("Problem parsing arguments: {err}")}
    ), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection to database is error: {}", e);
        }
    });

    Ok(client)
}

pub async fn disconnect_from_database(client: &Client) {
    println!("Закрываем соединение (по завершении работы клиента)...");
}