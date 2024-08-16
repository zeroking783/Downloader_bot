mod connect_database;

use connect_database::connect_database::{connect_to_database, disconnect_from_database};
use tokio_postgres::{Client, NoTls, Error};
use tokio::time::{interval, Duration};
use sys_info;
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cpu = sys_info::cpu_num().unwrap() as i32;
    println!("Cpu num: {}", cpu);

    let mut interval = interval(Duration::from_secs(10));

    loop {
        interval.tick().await;
        let client = connect_to_database().await?;

        let now: DateTime<Utc> = Utc::now();
        let now_str = now.to_rfc3339();

        let insert = "INSERT INTO test_table (cpu_num, time_now) VALUES ($1, $2)";
        client.execute(insert, &[&cpu, &now_str]).await?;

        disconnect_from_database(&client).await;

        println!("10 sec");
    }



    Ok(())

}