mod connect_database;

use connect_database::connect_database::{connect_to_database, disconnect_from_database};
use tokio_postgres::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = connect_to_database().await?;

    // Выполните запрос или другую работу с клиентом здесь

    disconnect_from_database(&client).await;

    Ok(())
}